use std::ops::{Deref,DerefMut};
use rocket::http::Status;
use rocket::request::{self,FromRequest};
use rocket::{Request,State,Outcome};
use dotenv::dotenv;
use std::env;
use mysql;

pub type MysqlPool=mysql::Pool;
#[derive(Debug)]
pub struct DbConn(pub mysql::PooledConn);

impl<'a,'r> FromRequest<'a,'r> for DbConn{
	type Error=();

	fn from_request(request:&'a Request<'r>)->request::Outcome<Self,Self::Error>{
		let pool=request.guard::<State<mysql::Pool>>()?;
		match pool.get_conn(){
			Ok(conn)=>Outcome::Success(DbConn(conn)),
			Err(_)=>Outcome::Failure((Status::ServiceUnavailable,()))
		}
	}
}

impl Deref for DbConn{
	type Target=mysql::PooledConn;

	fn deref(&self)->&Self::Target{
		&self.0
	}
}
impl DerefMut for DbConn{
	fn deref_mut(&mut self)->&mut mysql::PooledConn{
		&mut self.0
	}
}

pub fn connect()->MysqlPool{
	dotenv().ok();
	let database_url=env::var("DATABASE_URL");
	mysql::Pool::new(database_url.unwrap()).expect("db orz")
}
