use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\+|\-)(\w+)").unwrap();
}

#[derive(Debug, Deserialize)]
pub struct Order {
    pub order_by: Option<String>,
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

impl Order {
    pub fn to_sql(self) -> String {
        let limit = match self.limit {
            Some(value) => format!("{}", value),
            None => "NULL".to_string(),
        };
        let offset = self.offset.unwrap_or(0);
        let order: Vec<String> = self
            .order_by
            .unwrap_or("id".to_string())
            .split(",")
            .filter(|&s| s != "")
            .map(|s| {
                RE.replace_all(s.trim(), "$2 $1")
                    .into_owned()
                    .replace("-", "DESC")
                    .replace("+", "ASC")
            })
            .collect();

        let sql = format!(
            "ORDER BY {} LIMIT {} OFFSET {};",
            order.join(", "),
            limit,
            offset
        );

        sql
    }
}
