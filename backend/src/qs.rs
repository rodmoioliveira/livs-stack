use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\+|\-)(\w+)").unwrap();
}

#[derive(Debug, Deserialize)]
pub struct Order {
    pub order_by: Option<String>,
}

impl Order {
    pub fn to_sql(self) -> String {
        let order = self.order_by.unwrap_or(format!("{} id", "ORDER_BY"));
        let split: Vec<String> = order
            .split(",")
            .filter(|&s| s != "")
            .map(|s| {
                RE.replace_all(s, "$2 $1")
                    .into_owned()
                    .replace("-", "DESC")
                    .replace("+", "ASC")
            })
            .collect();
        let result = format!("{} {};", "ORDER_BY", split.join(", "));
        result
    }
}
