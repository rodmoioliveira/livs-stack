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

#[derive(Debug, Deserialize, Serialize)]
pub struct DataWithPagination<T, Y> {
    pub data: T,
    pub pagination: Y,
}

impl<T, Y> DataWithPagination<T, Y> {
    pub fn new(
        data: T,
        pagination: Y,
    ) -> Self {
        DataWithPagination { data, pagination }
    }
}
