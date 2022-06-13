mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::user_api::{create_user, get_user, update_user, delete_user, get_all_users,login,create_profile}; //import the handler here
use repository::mongodb_repo::{UserEntry, HomeUserDetails};

#[launch]
fn rocket() -> _ {
    let db = UserEntry::init();
    let db1 = HomeUserDetails::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])
        .mount("/", routes![login])
        .manage(db1)
        .mount("/", routes![create_profile])
}