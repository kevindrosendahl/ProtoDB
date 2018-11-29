use std::sync::Arc;

use super::super::generated::protodb::wasm;
use super::Handler;

use crate::{
    catalog::database::DatabaseCatalogEntry,
    wasm::{ProtoDBModule, ProtoDBModuleImportHashes},
};

use tower_grpc::Request;

impl Handler {
    pub(crate) fn handle_register_wasm_module(
        &mut self,
        request: &Request<wasm::RegisterModuleRequest>,
    ) -> wasm::RegisterModuleResponse {
        let module = ProtoDBModule::new(
            request.get_ref().wasm.clone(),
            "./wasm".to_string(),
            ProtoDBModuleImportHashes {
                log: "5a667b7fd4c3dac9".to_string(),
                get_object: "fd5ad791f53a8133".to_string(),
            },
        );
        self.storage_engine
            .clone()
            .catalog()
            .get_database_entry(&request.get_ref().database)
            .ok_or(wasm::register_module_response::ErrorCode::InvalidDatabase)
            .and_then(|db: Arc<dyn DatabaseCatalogEntry>| {
                Ok(db.create_wasm_module(&request.get_ref().name, module))
            })
            .and(Ok(wasm::RegisterModuleResponse {
                error_code: wasm::register_module_response::ErrorCode::NoError as i32,
            }))
            .unwrap_or_else(|error_code| wasm::RegisterModuleResponse {
                error_code: error_code as i32,
            })
    }

    pub(crate) fn handle_run_wasm_module(
        &mut self,
        request: &Request<wasm::RunModuleRequest>,
    ) -> wasm::RunModuleResponse {
        self.storage_engine
            .clone()
            .catalog()
            .get_database_entry(&request.get_ref().database)
            .ok_or(wasm::run_module_response::ErrorCode::InvalidDatabase)
            .and_then(|db: Arc<dyn DatabaseCatalogEntry>| {
                db.get_wasm_module(&request.get_ref().name)
                    .ok_or(wasm::run_module_response::ErrorCode::InvalidModule)
            })
            .and_then(|module: Arc<ProtoDBModule>| {
                Ok(wasm::RunModuleResponse {
                    error_code: wasm::run_module_response::ErrorCode::NoError as i32,
                    result: self.wasm_interpreter.run(&module),
                })
            })
            .unwrap_or_else(|error_code| wasm::RunModuleResponse {
                error_code: error_code as i32,
                result: vec![],
            })
    }
}
