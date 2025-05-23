use serde::Serialize;
use superposition_types::database::models::cac::FunctionType;

#[derive(Serialize)]
pub struct FunctionCreateRequest {
    pub function_name: String,
    pub function: String,
    pub runtime_version: String,
    pub description: String,
    pub change_reason: String,
    pub function_type: FunctionType,
}

#[derive(Serialize)]
pub struct FunctionUpdateRequest {
    pub function: String,
    pub runtime_version: String,
    pub description: String,
    pub change_reason: String,
    pub function_type: FunctionType,
}
