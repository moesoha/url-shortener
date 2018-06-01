#[derive(Debug,PartialEq,Eq)]
pub struct ShortToken{
	pub token: Option<String>,
	pub target: Option<String>
}
pub fn find_token(conn:&mut mysql::PooledConn,short_token:&String)->Option<ShortToken>{
	conn.first_exec("SELECT `token`,`target` from `short_token` WHERE `token`=:token",params!{
		"token"=>short_token.as_str()
	}).unwrap().map(|row|{
		let (token,target)=mysql::from_row(row);
		ShortToken{
			token: token,
			target: target
		}
	})
}
