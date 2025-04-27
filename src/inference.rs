use crate::errors::DomainError;
use async_trait::async_trait;
use ort::inputs;
use ort::session::Session;
use ort::value::{DynValue, Tensor};
use std::sync::Arc;

pub fn load_session(model_path: String) -> anyhow::Result<Session> {
    let session = Session::builder()?.commit_from_file(model_path)?;
    Ok(session)
}

#[async_trait]
pub trait Inference: Send + Sync + 'static {
    async fn infer(&self, input: Tensor<f32>) -> Result<Tensor<f32>, DomainError>;
}

pub struct InferenceImpl {
    pub session: Arc<Session>,
}

#[async_trait]
impl Inference for InferenceImpl {
    async fn infer(&self, input: Tensor<f32>) -> Result<Tensor<f32>, DomainError> {
        let mut outputs = self
            .session
            .run(inputs!(input).map_err(|e| {
                eprintln!("Error during inference: {:?}", e);
                DomainError::InferenceFailed(e.to_string())
            })?)
            .map_err(|e| {
                eprintln!("Error during inference: {:?}", e);
                DomainError::ProcessFailed(e.to_string())
            })?;
        let value: DynValue = outputs
            .remove("Plus214_Output_0")
            .ok_or_else(|| DomainError::InferenceFailed("Output not found".to_string()))?;

        let tensor: Tensor<f32> = value
            .downcast()
            .map_err(|e| DomainError::InferenceFailed(format!("Failed to downcast: {:?}", e)))?;
        Ok(tensor)
    }
}
