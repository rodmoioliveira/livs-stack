use serde::Deserialize;

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
                    .map(|s| format!("{}", s.to_owned()))
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
