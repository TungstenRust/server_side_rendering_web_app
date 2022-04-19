use serde::{Deserialize, Serialize};
use super::schema::certs;
#[derive(Queryable, Serialize)]
pub struct Cert {
    pub id: i32,
    pub name: String,
    pub image_path:String
}