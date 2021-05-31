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
    pub fn to_sql(&self) -> String {
        let limit = match self.limit {
            Some(value) => format!("{}", value),
            None => "NULL".to_string(),
        };
        let offset = self.offset.unwrap_or(0);
        let order: String = self
            .clone()
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

        let sql = format!("ORDER BY {} LIMIT {} OFFSET {}", order, limit, offset);

        sql
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Filters {
    pub formats: Option<String>,
    pub genres: Option<String>,
    pub languages: Option<String>,
    pub title_id: Option<String>,
    pub customer_id: Option<String>,
}

impl Filters {
    pub fn to_sql(&self) -> String {
        let mut filters: Vec<String> = vec![];
        let copy = self.clone();

        match copy.formats {
            Some(value) => filters.push(format!("format IN ({})", value)),
            None => (),
        };

        match copy.genres {
            Some(value) => filters.push(format!("genre IN ({})", value)),
            None => (),
        };

        match copy.languages {
            Some(value) => filters.push(format!("language IN ({})", value)),
            None => (),
        };

        match copy.customer_id {
            Some(value) => filters.push(format!("customer_id IN ({})", value)),
            None => (),
        };

        match copy.title_id {
            Some(value) => filters.push(format!("title_id IN ({})", value)),
            None => (),
        };

        let where_cause = if filters.len() == 0 { "" } else { "WHERE" };
        let sql = format!("{} {}", where_cause, filters.join(" AND "));

        sql
    }
}
