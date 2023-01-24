#![allow(unused)]

mod file_io;
use std::fs::{*,self};
use std::env::{*,self};
use std::path::{*,self};
use file_io::{*};

fn main(){
    let var=get_files(Path::new("C:\\"));
    for i in var{
        println!("{:?}",i.to_str().expect(""));
    }
}