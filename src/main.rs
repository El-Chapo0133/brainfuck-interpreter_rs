use std::str::from_utf8;

use crate::interpreter::{Interpretor, BFInterpretor};


mod interpreter;
mod pointers;


mod data {
        // pub const ENTRY_CODE: &'static str = "-[--->+<]>.--[----->+<]>.>-[--->+<]>.";
        pub const ENTRY_CODE: &'static str = ",.>,.>,.>,.>,.>,.";
        pub const USER_INPUTS: &'static str = "abc";
}


fn main() {
        println!("Code me daddy!");

        let mut interpreter = Interpretor::new(data::ENTRY_CODE.as_bytes().to_vec(), data::USER_INPUTS.as_bytes().to_vec());

        let result = match interpreter.start() {
                Ok(result) => convert_bytes_to_str(result),
                Err(err) => panic!("Error when executing code: {}", err),
        };

        println!("{:?}", result);
}


fn convert_bytes_to_str(bytes: Vec<u8>) -> String {
        println!("{:?}", bytes);
        match from_utf8(&bytes) {
                Ok(result) => result.to_string(),
                Err(err) => panic!("could not convert Vec<u8> to str, {}", err)
        }
}

