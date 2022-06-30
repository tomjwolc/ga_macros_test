extern crate proc_macro;
use proc_macro::*;

use lazy_static::lazy_static;
use std::{fs, collections::HashMap};

extern crate serde_json;

lazy_static! {
    static ref TYPE: (usize, usize, usize) = {
        if let Ok(str) = fs::read_to_string("type.json") {
            let json: HashMap<&str, Vec<usize>> = serde_json::from_str(str.as_str()).expect("The json file couldn't be parsed");

            (json["type"][0], json["type"][1], json["type"][2])
        } else {    
            (3, 0, 0) // 3D Vectorspace Geometric Algebra is the default
        }
    };
}

#[proc_macro]
pub fn eq(tokens: TokenStream) -> TokenStream {
    format!("{:?}", *TYPE).as_str().parse().unwrap()
}