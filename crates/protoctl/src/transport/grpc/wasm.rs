use super::generated::{protodb, protodb::wasm::register_module_request::ModuleMetadata};
use super::{Client, ClientError};

use futures::Future;
use prost_types::DescriptorProto;
use tower_grpc::Request;

impl Client {
    pub fn get_wasm_module_info(
        &mut self,
        database: String,
        name: String,
    ) -> Result<protodb::wasm::GetModuleInfoResponse, ClientError> {
        let request = self
            .client
            .get_wasm_module_info(Request::new(protodb::wasm::GetModuleInfoRequest {
                database,
                name,
            }))
            .and_then(|response| Ok(response.into_inner()));

        Ok(self.core.run(request)?)
    }

    pub fn register_wasm_module(
        &mut self,
        database: String,
        name: String,
        metadata: ModuleMetadata,
        wasm: Vec<u8>,
        result_schema: DescriptorProto,
    ) -> Result<protodb::wasm::RegisterModuleResponse, ClientError> {
        let request = self
            .client
            .register_wasm_module(Request::new(protodb::wasm::RegisterModuleRequest {
                database,
                name,
                metadata: Some(metadata),
                wasm,
                result_schema: Some(result_schema),
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
