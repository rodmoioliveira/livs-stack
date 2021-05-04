use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\+|\-)(\w+)").unwrap();
}

#[derive(Clone, Debug, Deserialize)]
pub struct Order {
    pub order_by: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl Order {
    pub fn to_sql(self) -> String {
        let limit = match self.limit {
            Some(value) => format!("{}", value),
            None => "NULL".to_string(),
        };
        let offset = self.offset.unwrap_or(0);
        let order: String = self
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
            .collect::<Vec<String>>()
            .join(", ");

        let sql = format!("ORDER BY {} LIMIT {} OFFSET {};", order, limit, offset);

        sql
    }
}
