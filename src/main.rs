use crate::interpreter::{Interpretor, BFInterpretor};


mod interpreter;
mod pointers;


mod data {
        // pub const ENTRY_CODE: &'static str = ",.,.,.";
        // pub const USER_INPUTS: &'static str = "abc";
        pub const ENTRY_CODE: &'static str = "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.";
        pub const USER_INPUTS: &'static str = "";
}


fn main() {
        println!("Code me daddy!");

        let result = Interpretor::start(data::ENTRY_CODE.as_bytes().to_vec(), data::USER_INPUTS.as_bytes().to_vec());

        println!("{:?}", result);
}




