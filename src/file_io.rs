#![allow(unused)]

use std::ffi::OsString;
use std::fs::{*,self};
use std::env::{*,self};
use std::path::{*,self};

pub struct FileManager{
    
}
impl FileManager{
    pub fn get_files(_path:&Path)->Vec<OsString>{
    let mut file_vec:Vec<OsString>=Vec::new();
    if path::Path::exists(_path){
        for i in fs::read_dir(_path).unwrap(){
            file_vec.push(i.expect("").file_name());
        }
    }
    else{
        for i in fs::read_dir(env::current_dir().unwrap()).unwrap(){
            file_vec.push(i.expect("").file_name());
        }
    }
    file_vec
}
}