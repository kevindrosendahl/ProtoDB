use super::generated::{protodb, protodb::wasm::register_module_request::ModuleMetadata};
use super::{Client, ClientError};

use futures::Future;
use tower_grpc::Request;

impl Client {
    pub fn register_wasm_module(
        &mut self,
        database: String,
        name: String,
        metadata: ModuleMetadata,
        wasm: Vec<u8>,
    ) -> Result<protodb::wasm::RegisterModuleResponse, ClientError> {
        let request = self
            .client
            .register_wasm_module(Request::new(protodb::wasm::RegisterModuleRequest {
                database,
                name,
                metadata: Some(metadata),
                wasm,
            }))
            .and_then(|response| Ok(response.into_inner()));

        Ok(self.core.run(request)?)
    }

    pub fn run_wasm_module(
        &mut self,
        database: String,
        name: String,
    ) -> Result<protodb::wasm::RunModuleResponse, ClientError> {
        let request = self
            .client
            .run_wasm_module(Request::new(protodb::wasm::RunModuleRequest {
                database,
                name,
            }))
            .and_then(|response| Ok(response.into_inner()));

        Ok(self.core.run(request)?)
    }
}
