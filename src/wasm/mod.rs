use std::{
    collections::HashMap,
    io::Cursor,
    sync::{
        atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT},
        Arc,
    },
};

use crate::{
    index::{Direction, IteratorMode},
    storage::{errors::InternalStorageEngineError, StorageEngine},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use prost_types::DescriptorProto;
use protodb_schema::encoding::FieldValue;
use wasmi::{
    Error as InterpreterError, ExternVal, Externals, FuncInstance, FuncRef, ImportsBuilder,
    MemoryRef, Module, ModuleImportResolver, ModuleInstance, ModuleRef, NopExternals, RuntimeArgs,
    RuntimeValue, Signature, Trap, ValueType,
};

static GLOBAL_ARGUMENT_PTR_EXPORT: &'static str = "__wbindgen_global_argument_ptr";
static FREE_EXPORT: &'static str = "__wbindgen_free";
static MALLOC_EXPORT: &'static str = "__wbindgen_malloc";
static MEMORY_EXPORT: &'static str = "memory";
static RUN_EXPORT: &'static str = "run";

// TODO: for some reason the match in the import resolver would always match THROW_IMPORT
static THROW_IMPORT: &'static str = "__wbindgen_throw";
const THROW_IMPORT_INDEX: usize = 0;
static FIND_OBJECT_IMPORT_PREFIX: &'static str = "__wbg_findobject_";
const FIND_OBJECT_IMPORT_INDEX: usize = 1;
static FIND_OBJECTS_ITER_IMPORT_PREFIX: &'static str = "__wbg_findobjectsiter_";
const FIND_OBJECTS_ITER_IMPORT_INDEX: usize = 2;
static FIND_OBJECTS_ITER_NEXT_IMPORT_PREFIX: &'static str = "__wbg_findobjectsiternext_";
const FIND_OBJECTS_ITER_NEXT_IMPORT_INDEX: usize = 3;
static LOG_IMPORT_PREFIX: &'static str = "__wbg_log_";
const LOG_IMPORT_INDEX: usize = 4;
static INDEX_ITER_IMPORT_PREFIX: &'static str = "__wbg_indexiter_";
const INDEX_ITER_IMPORT_INDEX: usize = 5;
static INDEX_ITER_NEXT_ID_IMPORT_PREFIX: &'static str = "__wbg_indexiternextid_";
const INDEX_ITER_NEXT_ID_IMPORT_INDEX: usize = 6;
static INDEX_ITER_NEXT_VALUE_IMPORT_PREFIX: &'static str = "__wbg_indexiternextvalue_";
const INDEX_ITER_NEXT_VALUE_IMPORT_INDEX: usize = 7;

pub struct ProtoDBModule {
    wasm_module: Module,
    name: String,
    hashes: ProtoDBModuleImportHashes,
    pub result_schema: DescriptorProto,
}

impl ProtoDBModule {
    pub fn new(
        wasm_binary: Vec<u8>,
        name: String,
        hashes: ProtoDBModuleImportHashes,
        result_schema: DescriptorProto,
    ) -> Self {
        ProtoDBModule {
            wasm_module: Module::from_buffer(wasm_binary).expect("failed to load_wasm"),
            name,
            hashes,
            result_schema,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProtoDBModuleImportHashes {
    pub log: String,
    pub find_object: String,
    pub find_object_iter: String,
    pub find_object_iter_next: String,
    pub index_iter: String,
    pub index_iter_next_id: String,
    pub index_iter_next_value: String,
}

pub struct Interpreter {
    storage_engine: Arc<dyn StorageEngine>,
}

impl Interpreter {
    pub fn new(storage_engine: Arc<dyn StorageEngine>) -> Self {
        Interpreter { storage_engine }
    }

    pub fn run(&self, module: &ProtoDBModule) -> Vec<u8> {
        let mut instance = ProtoDBModuleInstance::new(module, self.storage_engine.clone());
        instance.invoke_run()
    }
}

struct ProtoDBModuleInstance {
    module_ref: ModuleRef,
    memory_export: ExternVal,

    externals: ProtoDBExternals,
}

impl ProtoDBModuleInstance {
    pub fn new(module: &ProtoDBModule, storage_engine: Arc<dyn StorageEngine>) -> Self {
        let resolver = ProtoDBModuleImportResolver {
            hashes: module.hashes.clone(),
        };
        let mut imports = ImportsBuilder::new();
        imports.push_resolver(module.name.clone(), &resolver);

        let module_ref = ModuleInstance::new(&module.wasm_module, &imports)
            .expect("failed to instantiate wasm module")
            .assert_no_start();

        let memory_export = module_ref
            .export_by_name(MEMORY_EXPORT)
            .unwrap_or_else(|| panic!("failed to find {} export", MEMORY_EXPORT));

        let externals = ProtoDBExternals {
            module_ref: module_ref.clone(),
            memory_export: memory_export.clone(),

            storage_engine,
            find_object_iterators: HashMap::new(),
            find_object_iterator_counter: ATOMIC_USIZE_INIT,
            index_iterators: HashMap::new(),
            index_iterator_counter: ATOMIC_USIZE_INIT,
        };

        ProtoDBModuleInstance {
            module_ref,
            memory_export,

            externals,
        }
    }

    pub fn invoke_run(&mut self) -> Vec<u8> {
        // Ask the guest for the location it will store the return values.
        let ret_val = self
            .module_ref
            .invoke_export(GLOBAL_ARGUMENT_PTR_EXPORT, &[], &mut NopExternals)
            .unwrap_or_else(|_| panic!("failed to execute {}", GLOBAL_ARGUMENT_PTR_EXPORT))
            .unwrap();

        let ret_ptr = match ret_val {
            RuntimeValue::I32(ret_ptr) => ret_ptr,
            _ => panic!(format!(
                "unexpected return type for {}",
                GLOBAL_ARGUMENT_PTR_EXPORT
            )),
        };

        // Actually invoke the exported run function.
        let ret_val = self
            .module_ref
            .invoke_export(
                RUN_EXPORT,
                &[RuntimeValue::I32(ret_ptr)],
                &mut self.externals,
            )
            .unwrap_or_else(|_| panic!("failed to execute {}", RUN_EXPORT));
        assert_eq!(ret_val, None);

        // Read in the result.
        let memory = self.get_memory();
        let guest_ptr: u32 = memory.get_value(ret_ptr as u32).unwrap();
        let len: u32 = memory.get_value((ret_ptr + 4) as u32).unwrap();
        let result = memory.get(guest_ptr, len as usize).unwrap();

        // Free the allocated guest memory.
        self.invoke_free(guest_ptr as i32, len as i32);

        result
    }

    fn invoke_free(&self, guest_ptr: i32, len: i32) {
        let ret_val = self
            .module_ref
            .invoke_export(
                FREE_EXPORT,
                &[RuntimeValue::I32(guest_ptr), RuntimeValue::I32(len)],
                &mut NopExternals,
            )
            .unwrap_or_else(|_| panic!("failed to execute {}", FREE_EXPORT));
        assert_eq!(ret_val, None);
    }

    fn get_memory(&self) -> &MemoryRef {
        self.memory_export
            .as_memory()
            .unwrap_or_else(|| panic!("{} export is not of type 'memory'", MEMORY_EXPORT))
    }
}

type ExternalsFindObjectIterators =
    HashMap<usize, Box<dyn Iterator<Item = Result<Vec<u8>, InternalStorageEngineError>>>>;

struct IndexIteratorState {
    inner: Box<dyn Iterator<Item = (FieldValue, u64)>>,
    last_id: u64,
}

type ExternalsIndexIterators = HashMap<usize, IndexIteratorState>;

struct ProtoDBExternals {
    module_ref: ModuleRef,
    memory_export: ExternVal,

    storage_engine: Arc<dyn StorageEngine>,
    find_object_iterators: ExternalsFindObjectIterators,
    find_object_iterator_counter: AtomicUsize,

    index_iterators: ExternalsIndexIterators,
    index_iterator_counter: AtomicUsize,
}

impl ProtoDBExternals {
    fn get_memory(&self) -> &MemoryRef {
        self.memory_export
            .as_memory()
            .unwrap_or_else(|| panic!("{} export is not of type 'memory'", MEMORY_EXPORT))
    }

    fn get_string(&self, ptr: u32, len: usize) -> String {
        let vec = self.get_memory().get(ptr, len).unwrap();
        String::from_utf8(vec).unwrap()
    }

    fn malloc(&self, len: usize) -> u32 {
        let ret_val = self
            .module_ref
            .invoke_export(
                MALLOC_EXPORT,
                &[RuntimeValue::I32(len as i32)],
                &mut NopExternals,
            )
            .unwrap_or_else(|_| panic!("failed to execute {}", MALLOC_EXPORT))
            .unwrap();

        match ret_val {
            RuntimeValue::I32(i) => i as u32,
            _ => panic!("unexpected return type from {}", MALLOC_EXPORT),
        }
    }

    fn pass_byte_array(&self, arr: &[u8]) -> u32 {
        let ptr = self.malloc(arr.len());
        self.get_memory().set(ptr, arr).unwrap();
        ptr
    }

    fn pass_u64(&self, value: u64, ptr: u32) {
        let mut buf = Vec::new();
        buf.write_u64::<LittleEndian>(value).unwrap();
        self.get_memory().set(ptr, &buf).unwrap();
    }

    fn u64_shim(first: u32, second: u32) -> u64 {
        let mut buf = Vec::new();
        buf.write_u32::<LittleEndian>(first).unwrap();
        buf.write_u32::<LittleEndian>(second).unwrap();

        let mut reader = Cursor::new(buf);
        reader.read_u64::<LittleEndian>().unwrap()
    }
}

impl Externals for ProtoDBExternals {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        match index {
            THROW_IMPORT_INDEX => panic!("something bad happened"),
            FIND_OBJECT_IMPORT_INDEX => {
                let ret: u32 = args.nth(0);

                // Get collection name.
                let collection_ptr: u32 = args.nth(1);
                let collection_len: u32 = args.nth(2);
                let collection = self.get_string(collection_ptr, collection_len as usize);

                // Get id.
                let id_1: u32 = args.nth(3);
                let id_2: u32 = args.nth(4);
                let id = ProtoDBExternals::u64_shim(id_1, id_2);

                // Look up the object.
                let object = self
                    .storage_engine
                    .catalog()
                    .get_database_entry("dev")
                    .unwrap()
                    .get_collection_entry(&collection)
                    .unwrap()
                    .find_object(id)
                    .unwrap()
                    .unwrap();

                // Pass the object down to the guest.
                let ptr = self.pass_byte_array(&object);

                // Pass where the object was allocated down to the guest.
                self.get_memory().set_value(ret, ptr).unwrap();
                self.get_memory()
                    .set_value(ret + 4, object.len() as u32)
                    .unwrap();

                Ok(None)
            }
            FIND_OBJECTS_ITER_IMPORT_INDEX => {
                // Get collection name.
                let collection_ptr: u32 = args.nth(0);
                let collection_len: u32 = args.nth(1);
                let collection = self.get_string(collection_ptr, collection_len as usize);

                // Get id.
                let iter_id = self
                    .find_object_iterator_counter
                    .fetch_add(1, Ordering::SeqCst);

                // Look up the object.
                let iter = self
                    .storage_engine
                    .catalog()
                    .get_database_entry("dev")
                    .unwrap()
                    .get_collection_entry(&collection)
                    .unwrap()
                    .find_all(None);

                self.find_object_iterators.insert(iter_id, iter);
                Ok(Some(RuntimeValue::I32(iter_id as i32)))
            }
            FIND_OBJECTS_ITER_NEXT_IMPORT_INDEX => {
                let ret: u32 = args.nth(0);

                let iter_id: u32 = args.nth(1);
                let iter_id = iter_id as usize;
                let iter = self.find_object_iterators.get_mut(&iter_id).unwrap();

                let (ptr, len) = match iter.next() {
                    Some(object) => {
                        let object = object.unwrap();
                        (self.pass_byte_array(&object), object.len())
                    }
                    None => (0, 0),
                };

                // Pass where the object was allocated down to the guest.
                self.get_memory().set_value(ret, ptr).unwrap();
                self.get_memory().set_value(ret + 4, len as u32).unwrap();

                Ok(None)
            }
            LOG_IMPORT_INDEX => {
                let ptr: u32 = args.nth(0);
                let len: u32 = args.nth(1);

                let message = self.get_string(ptr, len as usize);
                println!("message from wasm: {}", message);
                Ok(None)
            }
            INDEX_ITER_IMPORT_INDEX => {
                // Get collection name.
                let collection_ptr: u32 = args.nth(0);
                let collection_len: u32 = args.nth(1);
                let collection = self.get_string(collection_ptr, collection_len as usize);

                let index_id: u32 = args.nth(2);

                // Get id.
                let iter_id = self.index_iterator_counter.fetch_add(1, Ordering::SeqCst);

                // Look up the object.
                let iter = self
                    .storage_engine
                    .catalog()
                    .get_database_entry("dev")
                    .unwrap()
                    .get_collection_entry(&collection)
                    .unwrap()
                    .index(index_id as usize)
                    .unwrap()
                    .iter(IteratorMode {
                        direction: Direction::Forward,
                        from: None,
                    });

                let iterator_state = IndexIteratorState {
                    inner: iter,
                    last_id: 0,
                };
                self.index_iterators.insert(iter_id, iterator_state);
                Ok(Some(RuntimeValue::I32(iter_id as i32)))
            }
            INDEX_ITER_NEXT_ID_IMPORT_INDEX => {
                let ret: u32 = args.nth(0);

                let iter_id: u32 = args.nth(1);
                let iter_id = iter_id as usize;
                let state = &self.index_iterators[&iter_id];

                self.pass_u64(state.last_id, ret);

                Ok(None)
            }
            INDEX_ITER_NEXT_VALUE_IMPORT_INDEX => {
                let ret: u32 = args.nth(0);

                let iter_id: u32 = args.nth(1);
                let iter_id = iter_id as usize;
                let state = self.index_iterators.get_mut(&iter_id).unwrap();

                let (valid, value) = match state.inner.next() {
                    None => (0, 0),
                    Some((field_value, id)) => {
                        state.last_id = id;
                        let value = match field_value {
                            FieldValue::Uint32(value) => value,
                            _ => panic!("unsupported index iterator FieldValue type"),
                        };
                        (1, value)
                    }
                };

                // Pass the value down to the guest
                self.get_memory().set_value(ret, valid).unwrap();
                self.get_memory().set_value(ret + 4, value).unwrap();

                Ok(None)
            }
            _ => panic!("unknown function index {}", index),
        }
    }
}

struct ProtoDBModuleImportResolver {
    hashes: ProtoDBModuleImportHashes,
}

impl ModuleImportResolver for ProtoDBModuleImportResolver {
    fn resolve_func(
        &self,
        field_name: &str,
        _signature: &Signature,
    ) -> Result<FuncRef, InterpreterError> {
        if field_name == THROW_IMPORT {
            return Ok(FuncInstance::alloc_host(
                Signature::new(&[ValueType::I32, ValueType::I32][..], None),
                THROW_IMPORT_INDEX,
            ));
        }

        if field_name == format!("{}{}", FIND_OBJECT_IMPORT_PREFIX, self.hashes.find_object) {
            return Ok(FuncInstance::alloc_host(
                Signature::new(
                    &[
                        ValueType::I32,
                        ValueType::I32,
                        ValueType::I32,
                        ValueType::I32,
                        ValueType::I32,
                    ][..],
                    None,
                ),
                FIND_OBJECT_IMPORT_INDEX,
            ));
        }

        if field_name
            == format!(
                "{}{}",
                FIND_OBJECTS_ITER_IMPORT_PREFIX, self.hashes.find_object_iter
            )
        {
            return Ok(FuncInstance::alloc_host(
                Signature::new(&[ValueType::I32, ValueType::I32][..], Some(ValueType::I32)),
                FIND_OBJECTS_ITER_IMPORT_INDEX,
            ));
        }

        if field_name
            == format!(
                "{}{}",
                FIND_OBJECTS_ITER_NEXT_IMPORT_PREFIX, self.hashes.find_object_iter_next
            )
        {
            return Ok(FuncInstance::alloc_host(
                Signature::new(&[ValueType::I32, ValueType::I32][..], None),
                FIND_OBJECTS_ITER_NEXT_IMPORT_INDEX,
            ));
        }

        if field_name == format!("{}{}", INDEX_ITER_IMPORT_PREFIX, self.hashes.index_iter) {
            return Ok(FuncInstance::alloc_host(
                Signature::new(
                    &[
                        ValueType::I32,
                        ValueType::I32,
                        ValueType::I32,
                        ValueType::I32,
                        ValueType::I32,
                    ][..],
                    Some(ValueType::I32),
                ),
                INDEX_ITER_IMPORT_INDEX,
            ));
        }

        if field_name
            == format!(
                "{}{}",
                INDEX_ITER_NEXT_ID_IMPORT_PREFIX, self.hashes.index_iter_next_id
            )
        {
            return Ok(FuncInstance::alloc_host(
                Signature::new(&[ValueType::I32, ValueType::I32][..], None),
                INDEX_ITER_NEXT_ID_IMPORT_INDEX,
            ));
        }

        if field_name
            == format!(
                "{}{}",
                INDEX_ITER_NEXT_VALUE_IMPORT_PREFIX, self.hashes.index_iter_next_value
            )
        {
            return Ok(FuncInstance::alloc_host(
                Signature::new(&[ValueType::I32, ValueType::I32][..], None),
                INDEX_ITER_NEXT_VALUE_IMPORT_INDEX,
            ));
        }

        if field_name == format!("{}{}", LOG_IMPORT_PREFIX, self.hashes.log) {
            return Ok(FuncInstance::alloc_host(
                Signature::new(&[ValueType::I32, ValueType::I32][..], None),
                LOG_IMPORT_INDEX,
            ));
        }

        Err(InterpreterError::Function(format!(
            "host module doesn't export function with name {}",
            field_name
        )))
    }
}
