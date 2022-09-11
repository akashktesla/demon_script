#![allow(warnings)]
// use demon_script::ds;
use demon_script::ds_compiler;
use demon_script::rustpp::read_file;
fn main(){ 
let vstr:String = format!("&a:&a:1&r:&add:&b:4&r:&add:&b:2");
ds_compiler::main()
// let ret = ds::run_ds(&vstr);
// println!("ret:{}",ret);
    }
