#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
extern crate regex;
#[macro_use]
extern crate mysql;

use rocket::fairing;
use rocket::response::{Redirect,status};

mod db;
mod documents;

#[get("/")]
fn index()->&'static str{
	"Welcome to use URL Shortener! GitHub: https://github.com/moesoha/url-shortener"
}

#[get("/<short_token>")]
fn short_token(mut conn:db::DbConn,short_token:String)->Result<Redirect,status::NotFound<String>>{
	let conn=&mut *conn;
	let find_token:Option<documents::ShortToken>=conn.first_exec("SELECT `token`,`target` from `short_token` WHERE `token`=:token",params!{
		"token"=>short_token.as_str()
	}).unwrap().map(|row|{
		let (token,target)=mysql::from_row(row);
		documents::ShortToken{
			token: token,
			target: target
		}
	});
	match find_token{
		Option::Some(token_result)=>{
			Result::Ok(Redirect::to(&token_result.target.unwrap()))
		},
		Option::None=>{
			Result::Err(status::NotFound(format!("No such short token `{}`, please check the URL.",short_token).to_string()))
		}
	}
}

fn main(){
	rocket::ignite()
	.manage(db::connect())
	.attach(fairing::AdHoc::on_response(|_,res|{
		res.set_raw_header("X-Powered-By","url-shortener/0.1");
		res.set_raw_header("X-Developed-By","Tianhai_IT/tianhai.info Soha_Jin/sohaj.in");
		res.set_raw_header("X-Git-Repo","https://github.com/moesoha/url-shortener");
	}))
	.mount("/",routes![
		index,
		short_token
	])
	.launch();
}
