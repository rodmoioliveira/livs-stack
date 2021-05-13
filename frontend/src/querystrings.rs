use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Filters {
    pub formats: Option<String>,
    pub genres: Option<String>,
    pub languages: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Order {
    pub order_by: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
