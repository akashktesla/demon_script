use std::fs::File;
use std::io::prelude::*;
use serde_json::Value as JsonValue;
use serde::de::DeserializeOwned;

pub fn read_file(path:&String)->String{
    let mut _file = File::open(path).expect("can't open file");
    let mut contents = String::new();
    _file.read_to_string(&mut contents).expect("can't read the file");
    contents = format!(r#"{}"#,contents); 
    return contents
}

pub fn read_json(contents:&String)->JsonValue {
    let res =serde_json::from_str(&contents);
    if res.is_ok(){
        let p:JsonValue = res.unwrap();
        return p
    }
    else{
        panic!("something went wrong!!")
    } 

}

pub fn sync_json<T:DeserializeOwned>(contents:&String)->T {
    let res =serde_json::from_str(&contents);
    if res.is_ok(){
        let p:T = res.unwrap();
        return p
    }
    else{
        panic!("something went wrong!!")
    } 

}

//extracts data from path and converts into appropriate structure
pub fn extractor <T:DeserializeOwned> (_name:String,_path:String)->T{
    let contents:String=read_file(&format!("{}",_path));
    let data:JsonValue = read_json(&contents);
    let data:String = data[_name].to_string(); 
    let returns:T = sync_json(&data);
    returns
}
