use serde::Deserialize;

static ORDER_BY: &str = "ORDER BY";

#[derive(Debug, Deserialize)]
pub struct Order {
    pub order_by: Option<String>,
    pub sort_by: Option<String>,
}

impl Order {
    pub fn default(&mut self) {
        self.sort_by = match &self.sort_by {
            Some(value) => Some(format!("{} {}", ORDER_BY, value.to_owned())),
            None => Some(format!("{} {}", ORDER_BY, "id".to_owned())),
        };

        self.order_by = match &self.order_by {
            Some(value) => Some(value.to_owned()),
            None => Some("ASC".to_owned()),
        };
    }
}
