
use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc}, 
    results::{ InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection, Database}, options::FindOneOptions,
};
use crate::models::user_model::{User, uname, HomeUser};
fn databaeinit()->Database{
    dotenv().ok();
    let uri = match env::var("MONGOURI") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let client = Client::with_uri_str(uri).unwrap();
    let db = client.database("rustDB");
    return db;
}
pub struct UserEntry {
    col: Collection<User>,
}

impl UserEntry {
    pub fn init() -> Self {
        let db=databaeinit();
        let col: Collection<User> = db.collection("User");
        UserEntry { col }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }
    pub fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "location": new_user.location,
                    "title": new_user.title
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }
    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }
    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }

    pub fn login(&self,user:uname)-> Result<User, Error> {
        let filter=doc! {
            "name": user.name,
        };
        let projection =doc! {"_id": 1};
        let options = FindOneOptions::builder().projection(projection).build();
        let user_detail = self
            .col
            .find_one(filter, options)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }
}
pub struct HomeUserDetails {
    col: Collection<HomeUser>,
}
impl HomeUserDetails {
    pub fn init() -> Self {
        let db = databaeinit();
        let col: Collection<HomeUser> = db.collection("HomeUser");
        HomeUserDetails { col }
    }
    pub fn create_profile(&self, new_user: HomeUser) -> Result<InsertOneResult, Error> {
        let new_doc = HomeUser {
            uid: new_user.uid,
            name: new_user.name,
            ward: new_user.ward,
            locality: new_user.locality,
            houseno: new_user.houseno,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating profile");
        Ok(user)
    }
}



// let projection = // getting the projection doc here based on graphql fields, lets say doc! {"title": 1}

// let options = FindOptions::builder().limit(10).projection(projection).build();


// use async_graphql::*;
// use serde::{Deserialize, Serialize};
// use mongodb::{bson::doc, bson::oid::ObjectId, options::FindOptions, Collection};

// #[derive(SimpleObject, Serialize, Deserialize, Debug)]
// #[graphql(complex)]
// struct Post {
//     #[serde(rename = "_id")]
//     pub id: ObjectId,
//     pub title: String,
//     // I could do something like
//     // #[serde(default)]
//     pub body: String,
// }

// #[derive(SimpleObject, Serialize, Deserialize, Debug)]
// #[graphql(complex)]
// struct PostTitle {
//     #[serde(rename = "_id")]
//     pub id: ObjectId,
//     pub title: String,
// }

// struct Query;
// #[Object]
// impl Query {
//     // fetching posts
//     async fn posts<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<PostTitle> {
//         let posts = ctx.data_unchecked::<Collection<PostTitle>>();
//         let projection = doc! {"title": 1}

//         let options = FindOptions::builder().limit(10).projection(projection).build();
//         let cursor = posts.find(None, options).await.unwrap();

//         cursor.try_collect().await.unwrap_or_else(|_| vec![])
//     }
// }



// let course_collection = database.collection::<CourseDocument>("courses");

// let projection = doc! {"localizedFields": 1};

// let options = FindOptions::builder()
//     .limit(10)
//     .projection(projection)
//     .build();

// let mut cursor = course_collection.find(None, options).await.unwrap();

// while let Some(course) = cursor.try_next().await.unwrap() {
//     println!("{:#?}", course)
// }



// #[derive(Serialize, Deserialize)]
// struct ProjectedCourseDocument {
//     #[serde(rename = "localizedFields")]
//     localized_fields: Vec<CourseDocumentLocalizedFields>,
// }

// let mut cursor = course_collection
//     .clone_with_type::<ProjectedCourseDocument>()
//     .find(None, options)
//     .await
//     .unwrap();
