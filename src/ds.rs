use std::any::{Any, TypeId};
use std::collections::HashMap;
use core::any::type_name;

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
    let a = str2arb(format!("{}",&self));
    a
}}



trait ToBool{
fn to_bool(&self)->bool{
   return true
}
}

impl ToBool for String{
fn to_bool(&self)->bool{
   self=="true"
}}





trait ToI32{
fn to_i32(&self)->i32{
   0
}
}
impl ToI32 for String{
fn to_i32(&self)->i32{
self.parse::<i32>().unwrap()
}
}

trait ToU32{
fn to_u32(&self)->u32{
   0
}
}
impl ToU32 for String{
fn to_u32(&self)->u32{
self.parse::<u32>().unwrap()
}
}

fn str2arb(mut _arb:String)->Arb{
   _arb = _arb.replace(" ",""); 
   _arb = _arb.replace("&a:"," &a: "); 
   _arb = _arb.replace("&r:"," &r: "); 
   _arb = _arb.replace("&b:"," &b: "); 
   let sp = _arb.split_whitespace();
   let cl:Vec<&str> = sp.collect();
   let mut ca = 0;
   let mut cr = 0;
   let mut cb = 0;
   let mut flaga = false;
   let mut flagr = false;
   let mut flagb = false;

   let mut arb = "".to_string();
   for (i,item) in cl.iter().enumerate(){
      let mut flag = true;
      if item == &"&a:"{
         ca +=1;
         if ca-cb ==0 || i ==0{
           arb = format!("{} ",arb);
           flag = false;
           flaga = true;
         }}
      if item == &"&r:" {
         cr += 1;
         if ca-cr ==0&&flaga{
           arb = format!("{} ",arb);
           flag = false;
           flagr = true;
         }}
      if item == &"&b:"{
         cb += 1;
         if cr-cb ==0&&flagr{
           arb = format!("{} ",arb);
           flag = false;
         }}
      if flag{
      arb = format!("{}{}",arb,item);
      }
}//end:for

   let _sp = arb.split_whitespace();
   let _cl:Vec<&str> = _sp.collect();
   Arb{
      a:_cl[0].to_string(),
      r:_cl[1].to_string(),
      b:_cl[2].to_string()
   }//returns

}

fn arb_to_str(varb:Arb)->String
{
return "aldkfj".to_string()
}

trait IsDs{
fn is_ds(&self)->bool{
   return false
}
}

impl IsDs for String{
fn is_ds(&self)->bool{
let ds_tokens = ["&add:","&sub:"];
ds_tokens.iter().any(|x| self.contains(x))
}
}

fn add_ds(a:String,b:String)->String{
let returns = run_ds(&a).to_i32()+run_ds(&b).to_i32();
returns.to_string()
}

fn sub_ds(a:String,b:String)->String{
let returns = run_ds(&a).to_i32()-run_ds(&b).to_i32();
returns.to_string()
}

fn mul_ds(a:String,b:String)->String{
let returns = run_ds(&a).to_i32()*run_ds(&b).to_i32();
returns.to_string()
}

fn div_ds(a:String,b:String)->String{
let returns = run_ds(&a).to_i32()/run_ds(&b).to_i32();
returns.to_string()
}

fn mod_ds(a:String,b:String)->String{
let returns = run_ds(&a).to_i32()%run_ds(&b).to_i32();
returns.to_string()
}

fn pow_ds(a:String,b:String)->String{
let returns = i32::pow(run_ds(&a).to_i32(),run_ds(&b).to_u32());
returns.to_string()
}

fn cmp_ds(a:String,b:String)->String{
let returns = a==b;
returns.to_string()
}

fn ne_ds(a:String,b:String)->String{
let returns = a!=b;
returns.to_string()
}

fn lt_ds(a:String,b:String)->String{
let returns = a<b;
returns.to_string()
}

fn gt_ds(a:String,b:String)->String{
let returns = a>b;
returns.to_string()
}
fn ge_ds(a:String,b:String)->String{
let returns = a>=b;
returns.to_string()
}
fn le_ds(a:String,b:String)->String{
let returns = a<=b;
returns.to_string()
}

fn and_ds(a:String,b:String)->String{
let returns = a.to_bool() && b.to_bool();
returns.to_string()
}

fn or_ds(a:String,b:String)->String{
let returns = a.to_bool() || b.to_bool();
returns.to_string()
}

fn if_ds(a:String,b:String)->String{
   let returns = "".to_string();
   if b.to_bool(){
      let returns = run_ds(&a);
   }
   returns
}

pub fn run_ds(_ds:&String)->String{
   if _ds.is_arb(){
      let mut returns  = "".to_string();
      let _dsarb = _ds.to_arb();
       match _dsarb.r.as_str(){
      "&add:"|"+"=>returns = add_ds(_dsarb.a,_dsarb.b),
      "&sub:"=>returns = sub_ds(_dsarb.a,_dsarb.b),
      "&mul:"=>returns = mul_ds(_dsarb.a,_dsarb.b),
      "&div:"=>returns = div_ds(_dsarb.a,_dsarb.b),
      "&mod:"=>returns = mod_ds(_dsarb.a,_dsarb.b),
      "&pow:"=>returns = pow_ds(_dsarb.a,_dsarb.b),
      "&cmp:"=>returns = cmp_ds(_dsarb.a,_dsarb.b),
      "&ne:"=>returns = ne_ds(_dsarb.a,_dsarb.b),
      "&lt:"=>returns = lt_ds(_dsarb.a,_dsarb.b),
      "&gt:"=>returns = gt_ds(_dsarb.a,_dsarb.b),
      "&le:"=>returns = le_ds(_dsarb.a,_dsarb.b),
      "&ge:"=>returns = ge_ds(_dsarb.a,_dsarb.b),
      "&and:"=>returns = and_ds(_dsarb.a,_dsarb.b),
      "&or:"=>returns = or_ds(_dsarb.a,_dsarb.b),
      "&if:"=>returns = if_ds(_dsarb.a,_dsarb.b),

      
      &_=>returns = panic!("DS:SYNTAX ERROR")
     }
   returns
   }
   else{
      _ds.to_string()
   }
}

fn loop_ds(cond:bool,ablock:String){
    while cond{
        run_ds(&ablock);
    }
}

