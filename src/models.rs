use serde::{Deserialize, Serialize};
use super::schema::certs;
#[derive(Queryable, Serialize)]
pub struct Cert {
    pub id: i32,
    pub name: String,
    pub image_path:String
}
#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "certs"]
pub struct NewCert {
    pub name: String,
    pub image_path: String
}