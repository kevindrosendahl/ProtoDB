extern crate wasmi;

use wasmi::{
    ExternVal, ImportsBuilder, MemoryRef, Module, ModuleInstance, ModuleRef, NopExternals,
    RuntimeValue,
};

static GLOBAL_ARGUMENT_PTR_EXPORT: &'static str = "__wbindgen_global_argument_ptr";
static FREE_EXPORT: &'static str = "__wbindgen_free";
static MEMORY_EXPORT: &'static str = "memory";
static RUN_EXPORT: &'static str = "run";

pub struct Interpreter;

impl Interpreter {
    pub fn run(&self, wasm_binary: Vec<u8>) -> Vec<u8> {
        let instance = ProtoDBModuleInstance::new(wasm_binary);
        instance.invoke_run()
    }
}

struct ProtoDBModuleInstance {
    module_ref: ModuleRef,
    memory_export: ExternVal,
}

impl ProtoDBModuleInstance {
    pub fn new(wasm_binary: Vec<u8>) -> Self {
        let module = Module::from_buffer(&wasm_binary).expect("failed to load_wasm");
        let module_ref = ModuleInstance::new(&module, &ImportsBuilder::new())
            .expect("failed to instantiate wasm module")
            .assert_no_start();

        let memory_export = module_ref
            .export_by_name(MEMORY_EXPORT)
            .expect(&format!("failed to find {} export", MEMORY_EXPORT));

        ProtoDBModuleInstance {
            module_ref,
            memory_export,
        }
    }

    pub fn invoke_run(&self) -> Vec<u8> {
        // Ask the guest for the location it will store the return values.
        let ret_val = self
            .module_ref
            .invoke_export(GLOBAL_ARGUMENT_PTR_EXPORT, &[], &mut NopExternals)
            .expect(&format!("failed to execute {}", GLOBAL_ARGUMENT_PTR_EXPORT))
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
            .invoke_export(RUN_EXPORT, &[], &mut NopExternals)
            .expect(&format!("failed to execute {}", RUN_EXPORT));
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
            .expect(&format!("failed to execute {}", FREE_EXPORT));
        assert_eq!(ret_val, None);
    }

    fn get_memory(&self) -> &MemoryRef {
        self.memory_export
            .as_memory()
            .expect(&format!("{} export is not of type 'memory'", MEMORY_EXPORT))
    }
}
