use crate::errors::DomainError;
use crate::inference::Inference;
use crate::models::{Features, Prediction};
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Extension, Json};
use ort::value::Tensor;
use std::sync::Arc;

async fn predict(
    Extension(inference): Extension<Arc<dyn Inference>>,
    Json(features): Json<Features>,
) -> Result<(StatusCode, Json<Prediction>), DomainError> {
    let inputs: Tensor<f32> = features.try_into()?;
    let outputs = inference.infer(inputs).await?;
    Ok((StatusCode::OK, Json(outputs.try_into()?)))
}

pub fn router(inference: Arc<dyn Inference>) -> axum::Router {
    axum::Router::new()
        .route("/", post(predict))
        .layer(Extension(inference))
}
