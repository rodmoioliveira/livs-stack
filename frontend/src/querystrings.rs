use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Filters {
    pub formats: Option<String>,
    pub genres: Option<String>,
    pub languages: Option<String>,
}
