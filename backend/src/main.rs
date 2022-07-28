#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod routes;

use rocket_auth::{Error, Users};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let users = Users::open_sqlite("mydb.db").await?;

    rocket::build()
        .mount("/auth", routes::auth::get_routes())
        .manage(users)
        .launch()
        .await
        .unwrap();
    Ok(())
}
