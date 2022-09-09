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
   _arb = _arb.replace("&a:"," &a: "); 
   _arb = _arb.replace("&r:"," &r: "); 
   _arb = _arb.replace("&b:"," &b: "); 
   let sp = _arb.split_whitespace();
   let cl:Vec<&str> = sp.collect();
   println!("{:?}",cl);
   let mut pa = 0;
   let mut pr = 0;
   let mut pb = 0;
   let mut ca = 0;
   let mut cr = 0;
   let mut cb = 0;
   
   for (i,item) in cl.iter().enumerate(){
      println!("{},{}",i,item);
      if item == &"&a:"{
         ca +=1;
         if ca-cb ==0{
           pa = i; 
         }
      }
    println!("a:{},r:{},b:{}",ca,cr,cb);
   
      if item == &"&r:"{
         cr += 1;
         if ca-cr ==0{
           pr = i;
         }
      }
    println!("a:{},r:{},b:{}",ca,cr,cb);

      if item == &"&b:"{
         cb += 1;
         if cr-cb ==0{
           pb = i; 
         }
      }
 println!("a:{},r:{},b:{}",ca,cr,cb);
}//end:for
 println!("a:{},r:{},b:{}",pa,pr,pb);
}
