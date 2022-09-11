pub fn main(){
    
let vstr:String = format!("& a :&a:akash&r:the&b:great&r:son&b:dio");
// let a = Arb{a:"akash".to_string(),r:"son".to_string(),b:"&a:dio&r:the&b:demon".to_string()};
let a = str2arb(vstr);
println!("arb:{}-{}-{}",a.a,a.r,a.b);
}

struct Arb{
a:String,
r:String,
b:String
}
fn str2arb(mut _arb:String)->Arb{
   _arb = _arb.replace(" ",""); 
   _arb = _arb.replace("&a:"," &a: "); 
   _arb = _arb.replace("&r:"," &r: "); 
   _arb = _arb.replace("&b:"," &b: "); 
   let sp = _arb.split_whitespace();
   let cl:Vec<&str> = sp.collect();
   println!("{:?}",cl);
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
   println!("cl::{:?}",_cl[0]);
   Arb{
      a:_cl[0].to_string(),
      r:_cl[1].to_string(),
      b:_cl[2].to_string()
   }//returns

}
