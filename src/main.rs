#![feature(plugin)]
#![feature(custom_derive)]
#![feature(extern_prelude)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
extern crate regex;
#[macro_use]
extern crate mysql;
extern crate rand;

use rocket::fairing;
use rocket::response::{Redirect,status};
use rand::{Rng};

mod db;
mod documents;

#[get("/")]
fn index()->&'static str{
	"Welcome to use URL Shortener! GitHub: https://github.com/moesoha/url-shortener"
}

#[get("/<short_token>")]
fn short_token(mut conn:db::DbConn,short_token:String)->Result<Redirect,status::NotFound<String>>{
	match documents::find_token(&mut *conn,&short_token){
		Option::Some(token_result)=>{
			Result::Ok(Redirect::to(&token_result.target.unwrap()))
		},
		Option::None=>{
			Result::Err(status::NotFound(format!("No such short token `{}`, please check the URL.",short_token).to_string()))
		}
	}
}

#[derive(FromForm,Debug)]
struct NewShortTokenBody{
	token: Option<String>,
	url: String
}
static TOKEN_STRING:[char;48]=['a','A','b','B','c','D','d','e','E','f','F','G','g','H','h','i','J','j','K','k','L','M','m','N','n','p','Q','q','R','r','s','t','T','u','v','w','x','y','Y','z','2','3','4','5','6','7','8','9'];
const TOKEN_DEFAULT_LENGTH:i32=7;
#[put("/",data="<thebody>")]
/*
	PUT /
	url=https://sohaj.in&token=sohajin

	Params in body:
		url    Redirect to there
		token  (Optional) custom short token
*/
fn new_token(mut conn:db::DbConn,thebody:rocket::request::Form<NewShortTokenBody>)->Result<String,status::BadRequest<String>>{
	let body=thebody.get();
	let mut rng=rand::thread_rng();

	let new_token_doc=documents::ShortToken{
		token: match body.token.clone(){
			Some(token)=>{
				if token.len()<(TOKEN_DEFAULT_LENGTH as usize){
					return Result::Err(status::BadRequest(Some("Custom token too short!".to_string())));
				}
				match documents::find_token(&mut *conn,&token){
					Some(_)=>{
						return Result::Err(status::BadRequest(Some("Custom token has already been registered!".to_string())));
					},
					None=>Some(token)
				}
			},
			None=>{
				let mut gen_ok=false;
				let mut tkn=String::new();
				while !gen_ok{
					for _ in 0..TOKEN_DEFAULT_LENGTH{
						tkn.push(*rng.choose(&TOKEN_STRING).unwrap());
					}
					match documents::find_token(&mut *conn,&tkn){
						Some(_)=>{
							tkn=String::new()
						},
						None=>{
							gen_ok=true;
						}
					}
				}
				Some(tkn)
			}
		},
		target: Some(body.url.clone())
	};
	conn.prep_exec("INSERT INTO `short_token` (`token`,`target`) VALUES (:token,:target)",params!{
		"token"=>&new_token_doc.token,
		"target"=>&new_token_doc.target
	}).unwrap();
	Result::Ok(new_token_doc.token.unwrap())
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
		short_token,
		new_token
	])
	.launch();
}
