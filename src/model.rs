use super::schema::stores;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Store {
    pub id: i32,
    pub data: String,
    pub api_id: String
}

impl Into<serde_json::Value> for Store {
    fn into(self) -> Value {
        let entry_id = self.api_id;
        let entry_data = serde_json::from_str::<Value>(&self.data).unwrap();
        json!({
            "api_id": entry_id,
            "data": entry_data
        })
    }
}

#[derive(Insertable)]
#[table_name = "stores"]
pub struct NewStore<'a> {
    pub data: &'a str,
    pub api_id: &'a str
}
