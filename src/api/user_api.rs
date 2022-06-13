use crate::{models::user_model::{User,ToF, uname, HomeUser}, repository::mongodb_repo::{UserEntry, HomeUserDetails}};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult}; 
use rocket::{http::Status, serde::json::Json, State};
#[post("/user", data = "<new_user>")]
pub fn create_user(
    db: &State<UserEntry>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };
    let user_detail = db.create_user(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}
#[post("/profile", data = "<new_user>")]
pub fn create_profile(
    db: &State<HomeUserDetails>,
    new_user: Json<HomeUser>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = HomeUser {
        uid: new_user.uid,
        name: new_user.name.to_owned(),
        ward: new_user.ward.to_owned(),
        locality: new_user.locality.to_owned(),
        houseno: new_user.houseno.to_owned(),
    };
    let user_detail = db.create_profile(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user/<path>")]
pub fn get_user(db: &State<UserEntry>, path: String) -> Result<Json<User>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user_detail = db.get_user(&id);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/user/<path>", data = "<new_user>")]
pub fn update_user(
    db: &State<UserEntry>,
    path: String,
    new_user: Json<User>,
) -> Result<Json<User>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };
    let update_result = db.update_user(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id);
                return match updated_user_info {
                    Ok(user) => Ok(Json(user)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
#[delete("/user/<path>")]
pub fn delete_user(db: &State<UserEntry>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_user(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("User successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
#[get("/users")]
pub fn get_all_users(db: &State<UserEntry>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users();
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}
#[post("/login", data = "<new_user>")]
pub fn login(
    db: &State<UserEntry>,
    new_user: Json<uname>,
) -> Result<Json<User>, Status> {
    let data = uname {
        name: new_user.name.to_owned()
    };
    let user_detail = db.login(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}
