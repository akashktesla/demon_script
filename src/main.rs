#![allow(warnings)]
use std::any::{Any, TypeId};
use std::collections::HashMap;
use core::any::type_name;
mod test;
fn main(){ 
    test::main()
    }
pub fn ds(){
    let mut tis:HashMap<String,String> = HashMap::new();
    tis.insert(String::from("&out:"),String::from("out_ds(frame=var.a,content=var.b)"));
}

struct Arb{
a:String,
r:String,
b:String
}
impl Arb{
fn new(_a:String,_r:String,_b:String)->Arb{
    Arb{
    a:_a,
    r:_r,
    b:_b
    }}
fn to_arb(self)->Arb{
self
}
}

trait IsArb{
fn is_arb(&self)->bool{
    return false
}}

impl IsArb for String{
fn is_arb(&self)->bool{
    self.contains("&a:") && self.contains("&r:") && self.contains("&b:") 
}}

impl IsArb for Arb{
fn is_arb(&self)->bool{
    true
}}
impl ToString for Arb{
fn to_string(&self)->String{
    format!("&a:{}&r:{}&b:{}",self.a.to_string(),self.r.to_string(),self.b.to_string())
}}
trait ToArb{
fn to_arb(&self)->Arb{
    Arb::new(String::from(""),String::from(""),String::from(""))
}
}

impl ToArb for String{
fn to_arb(&self)->Arb{
    Arb::new(String::from(""),String::from(""),String::from(""))
}

}



fn arb_to_str(varb:Arb)->String
{
return "aldkfj".to_string()
}


fn run_ds(_ds:&String){
loop{}
}
fn loop_ds(cond:bool,ablock:String){
    while cond{
        run_ds(&ablock);
    }
}
