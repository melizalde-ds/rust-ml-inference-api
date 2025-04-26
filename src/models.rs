use ort::value::Tensor;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Features {
    pub values: Vec<f32>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Prediction {
    pub values: Vec<f32>,
}

impl TryFrom<Features> for Tensor<f32> {
    type Error = ();

    fn try_from(value: Features) -> Result<Self, Self::Error> {
        let length = value.values.len();
        if length != 728 {
            return Err(());
        }
        for i in value.values.iter() {
            if *i < 0.0 || *i > 255.0 {
                return Err(());
            }
        }
        Tensor::try_from(value)
    }
}
