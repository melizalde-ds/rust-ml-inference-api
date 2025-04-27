use crate::errors::DomainError;
use ndarray::{ArrayD, ArrayViewD};
use ort::value::Tensor;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Features {
    pub values: Vec<f32>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Prediction {
    pub values: Vec<f32>,
}

impl TryFrom<Features> for Tensor<f32> {
    type Error = DomainError;

    fn try_from(value: Features) -> Result<Self, Self::Error> {
        let shape = vec![1, 1, 28, 28];
        let flat_len: usize = shape.iter().product();

        if value.values.len() != flat_len {
            return Err(DomainError::InvalidInput(format!(
                "expected {} floats, got {}",
                flat_len,
                value.values.len()
            )));
        }

        let arr = ArrayD::from_shape_vec(shape, value.values)
            .map_err(|_| DomainError::InvalidInput("input shape is invalid".into()))?;
        let tensor = Tensor::from_array(arr)
            .map_err(|e| DomainError::ProcessFailed(format!("tensor creation error: {}", e)))?;
        Ok(tensor)
    }
}

impl TryFrom<Tensor<f32>> for Prediction {
    type Error = DomainError;

    fn try_from(value: Tensor<f32>) -> Result<Self, Self::Error> {
        let arr: ArrayViewD<f32> = value.extract_tensor();
        let shape = arr.shape();
        let flat_len: usize = shape.iter().product();
        let values: Vec<f32> = arr.iter().copied().collect();
        if values.len() != flat_len {
            return Err(DomainError::InvalidInput(format!(
                "expected {} floats, got {}",
                flat_len,
                values.len()
            )));
        }

        Ok(Prediction { values })
    }
}
