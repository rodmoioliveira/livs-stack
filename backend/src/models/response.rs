use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Data<T> {
    pub data: T,
}

impl<T> Data<T> {
    pub fn new(data: T) -> Self {
        Data { data }
    }
}
