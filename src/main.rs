#![allow(unused)]

mod file_io;
use file_io::*;
use std::env::{self, *};
use std::fs::{self, *};
use std::path::{self, *};

fn main() {
    let var = file_io::FileManager::get_files(Path::new("C:\\"));
    for i in var {
        println!("{:?}", i.to_str().expect(""));
    }
}