use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\+|\-)(\w+)").unwrap();
}

#[derive(Debug, Deserialize)]
pub struct Filters {
    pub formats: Option<String>,
    pub genres: Option<String>,
    pub languages: Option<String>,
}

impl Filters {
    pub fn to_sql(self) -> String {
        let mut filters: Vec<String> = vec![];

        match self.formats {
            Some(value) => {
                let formats = value
                    .split(",")
                    .map(|s| format!("'{}'", s.to_owned()))
                    .collect::<Vec<String>>()
                    .join(",");
                filters.push(format!("format IN ({})", formats));
            }
            None => (),
        };
        match self.genres {
            Some(value) => filters.push(format!("genre IN ({})", value)),
            None => (),
        };
        match self.languages {
            Some(value) => filters.push(format!("language IN ({})", value)),
            None => (),
        };

        let where_cause = if filters.len() == 0 { "" } else { "WHERE" };
        let sql = format!("{} {}", where_cause, filters.join(" AND "));

        sql
    }
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
