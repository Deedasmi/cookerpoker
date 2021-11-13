#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod database;
mod account;

use database::DbConn;

#[get("/")]
fn index() ->&'static str {
    "This will eventually serve the poker client"
}

#[get("/monies")]
async fn monies_admin(conn: DbConn, a: account::Admin) -> String {
    let v = account::get_settled_balance(&conn, a.0.account_id).await;
    format!("Welcome God-King {}. Your balance is {} pennies", a.0.account_name, v)
}

#[get("/monies", rank = 2)]
async fn monies_user(conn: DbConn, u: account::User) -> String {
    let v = account::get_settled_balance(&conn, u.0.account_id).await;
    format!("Welcome peasent. Your balance is {} pennies", v)
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(DbConn::fairing())
        .mount("/", routes![index, monies_admin, monies_user])
}
