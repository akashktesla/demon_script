use std::fs::File;
use rustpp::fs::{read_file,write_file,create_file};
use rustpp::json::store_var;
use crate::ds::run_ds;
use std::env;

pub fn main(){
    let (file,dot) = get_path();
    let path = format!("{}.{}",file,dot);
    // println!("{}",path);
    if dot=="ds"{
    create_file(".ds".to_string(),format!("{}_vars.json",file));
    create_file(".ds".to_string(),format!("{}_flags.json",file));
    write_file(&format!(".ds/{}_vars.json",file),&"{}".to_string());
    write_file(&format!(".ds/{}_flags.json",file),&"{}".to_string());
    // store_var(&format!(".ds/{}_vars.json",cl[0]),&"test_key".to_string(),&"test_value".to_string());

    let contents = read_file(&path);
    let ret = run_ds(&contents);
    println!("{}",ret);
    }
    else{
    println!("provide a .ds file")
    }
}

pub fn get_path()->(String,String){
    let args:Vec<String> = env::args().collect();
    let path = args.get(1).expect("provide a path to .ds file");
    let sp = path.split('.');
    let cl:Vec<&str> = sp.collect();
    (cl[0].to_string(),cl[1].to_string())
}



