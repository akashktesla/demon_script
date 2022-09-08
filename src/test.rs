pub fn main(){
    
let vstr:String = format!("& a :&a:akash&r:the&b:great&r:son&b:dio");
// let a = Arb{a:"akash".to_string(),r:"son".to_string(),b:"&a:dio&r:the&b:demon".to_string()};
str2arb(vstr)
}

struct Arb{
a:String,
r:String,
b:String
}
fn str2arb(mut _arb:String){
   _arb = _arb.replace(" ",""); 
   _arb = _arb.replace("&a:","_&a:_"); 
   _arb = _arb.replace("&r:","_&r:_"); 
   _arb = _arb.replace("&b:","_&b:_"); 
   let sp = _arb.split("_");
   println!("sp:{:?}",sp);
   println!("arb:{}",_arb);
}
