#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools;



mod auth;
mod models;
mod schema;
mod repositories;




use models::*;
use auth::BasicAuth;
use repositories::*;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Value,json,Json};
use rocket:: response::status::Custom;





#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);




#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db:DbConn)-> Result<Value, Custom<Value>> {
  db.run(|c: &mut SqliteConnection|{
    RustaceanRepository::find_multiple(c, 100)
        .map(|rustaceans|json!(rustaceans))
        .map_err(|e|status::Custom(Status::InternalServerError,json!(e.to_string())))
  }).await 
}

#[get("/rustaceans/<id>")]
async fn view_rustaceans(id:i32, _auth:BasicAuth, db:DbConn)-> Result<Value, Custom<Value>>{
    db.run(move|c: &mut SqliteConnection|{
        RustaceanRepository::find(c, id)
            .map(|rustaceans|json!(rustaceans))
            .map_err(|e|status::Custom(Status::InternalServerError,json!(e.to_string())))
    }).await
    
}

#[post("/rustaceans", format="json", data="<new_rustaceans>")]
async fn create_rustacean(_auth:BasicAuth,db:DbConn, new_rustaceans:Json<NewRustaceans>)-> Result<Value, Custom<Value>>{
    db.run(|c|{
    RustaceanRepository::create(c, new_rustaceans.into_inner())
        .map(|rustaceans|json!(rustaceans))
        .map_err(|e|status::Custom(Status::InternalServerError,json!(e.to_string())))
    }).await
}

#[put("/rustaceans/<id>", format="json", data="<rustaceans>")]
async fn update_rustacean(id:i32,_auth:BasicAuth,db:DbConn,rustaceans:Json<Rustaceans>)-> Result<Value, Custom<Value>>{
    db.run(move|c|{
        RustaceanRepository::save(c, id, rustaceans.into_inner())
        .map(|rustaceans|json!(rustaceans))
        .map_err(|e|status::Custom(Status::InternalServerError,json!(e.to_string())))
    }).await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustaceans(id:i32,_auth:BasicAuth,db:DbConn)-> Result<status::NoContent, Custom<Value>>{
    db.run(move|c|{
        RustaceanRepository::delete(c,id)
        .map(|_| status::NoContent)
        .map_err(|e|Custom(Status::InternalServerError, json!(e.to_string())))
    }).await

}


#[catch(404)]
fn not_found()-> Value{
    json!("Not found!")
}



#[rocket::main]
async fn main(){
    let _ = rocket::build()
    .mount("/",routes![
        get_rustaceans,
        view_rustaceans,
        create_rustacean,
        update_rustacean,
        delete_rustaceans
        ])
    .register("/",catchers![
        not_found
    ])
    .attach(DbConn::fairing())
    .launch()
    .await;
}
