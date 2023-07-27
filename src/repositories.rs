

use crate::models::NewRustaceans;
use crate::schema::rustaceans;
use crate::models::Rustaceans;
use diesel::{SqliteConnection,QueryResult};
use diesel::prelude::*;




pub struct RustaceanRepository;
impl RustaceanRepository{
    pub fn find(c:&mut SqliteConnection,id:i32)->QueryResult<Rustaceans>{
        rustaceans::table.find(id).get_result::<Rustaceans>(c)  
    }

    pub fn find_multiple(c:&mut SqliteConnection, limit:i64)-> QueryResult<Vec<Rustaceans>>{
        rustaceans::table.limit(limit).load::<Rustaceans>(c)
    }

    pub fn create(c:&mut SqliteConnection, new_rustaceans:NewRustaceans)->QueryResult<Rustaceans>{
        diesel::insert_into(rustaceans::table)
            .values(new_rustaceans)
            .execute(c)?;
        
        let last_id= Self::last_inserted_id(c)?;
        Self::find(c, last_id)    
    }

    pub fn save(c:&mut SqliteConnection, id:i32, rustacean:Rustaceans)-> QueryResult<Rustaceans>{
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::email.eq(rustacean.email.to_owned()),
                rustaceans::name.eq(rustacean.name.to_owned()),
        ))
            .execute(c)?;

        Self::find(c, id)
    }

    pub fn delete(c:&mut SqliteConnection, id:i32)->QueryResult<usize>{
        diesel::delete(rustaceans::table.find(id)).execute(c)
    }
   
    fn last_inserted_id(c:&mut SqliteConnection)-> QueryResult<i32>{
        rustaceans::table.select(rustaceans::id).order(rustaceans::id.desc()).first(c)
    }

}

