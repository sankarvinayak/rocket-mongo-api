use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HomeUser{
    pub uid: Option<ObjectId>,
    pub name: Option<String>,
    pub ward: Option<u32>,
    pub locality: Option<String>,
    pub houseno: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToF{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct uname{
    pub name: String,
}