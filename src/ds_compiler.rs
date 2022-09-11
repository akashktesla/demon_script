use std::fs::File;
use crate::rustpp::read_file;
use crate::rustpp::create_file;
use crate::rustpp::write_file;
use crate::rustpp::store_var;
use crate::ds::run_ds;
use std::env;
pub fn main(){
let args:Vec<String> = env::args().collect();
let path = args.get(1).expect("provide a path to .ds file");
let sp = path.split('.');
let cl:Vec<&str> = sp.collect();
if cl[1]=="ds"{
create_file(".ds".to_string(),format!("{path}_vars.json",path=cl[0]));
create_file(".ds".to_string(),format!("{path}_flags.json",path=cl[0]));
write_file(&format!(".ds/{}_vars.json",cl[0]),&"{}".to_string());
write_file(&format!(".ds/{}_flags.json",cl[0]),&"{}".to_string());
// store_var(&format!(".ds/{}_vars.json",cl[0]),&"test_key".to_string(),&"test_value".to_string());
let contents = read_file(path);
let ret = run_ds(&contents);
println!("{}",ret);
}
else{
println!("provide a .ds file")
}
}


