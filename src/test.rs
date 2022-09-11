use std::fs::File;
use crate::rustpp::read_file;
use crate::ds::run_ds;
use std::env;
pub fn main(){
let args:Vec<String> = env::args().collect();
let path = args.get(1).expect("provide a path to .ds file");
println!("{}",path);
let sp = path.split('.');
let cl:Vec<&str> = sp.collect();
}
