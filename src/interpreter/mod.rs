
use crate::pointers::{Pointers, PointersIndex};

mod commands {
        pub const MOVE_FORWARD:         &'static u8 = &62; // >
        pub const MOVE_BACKWARD:        &'static u8 = &60; // <
        pub const INCREMENT:            &'static u8 = &43; // +
        pub const DECREMENT:            &'static u8 = &45; // -
        pub const BEGIN_LOOP:           &'static u8 = &91; // [
        pub const END_LOOP:             &'static u8 = &93; // ]
        pub const READ_VALUE:           &'static u8 = &44; // ,
        pub const PRINT_VALUE:          &'static u8 = &46; // .
}

pub trait BFInterpretor {
        fn new(code: Vec<u8>, user_inputs: Vec<u8>) -> Interpretor;
        fn start(&mut self) -> Vec<u8>;

        fn read_next_user_input(&mut self) -> u8;
        fn discover_end_of_code(&self) -> usize; // should be up to about 30000
        fn discover_end_of_loop(&self, start_of_loop_index: usize) -> Result<usize, &'static str>;
}

pub struct Interpretor {
        code:           Vec<u8>,
        user_inputs:    Vec<u8>
}



impl BFInterpretor for Interpretor {
        fn new(code: Vec<u8>, user_inputs: Vec<u8>) -> Interpretor {
                Interpretor { code, user_inputs: user_inputs.into_iter().rev().collect::<Vec<u8>>() }
        }
        fn start(&mut self) -> Vec<u8> {
                let mut pointers = Pointers::new();
                let end_of_code = self.discover_end_of_code();
                let mut code_index: usize = 0;

                let mut loop_layer = 0;
                
                while code_index < end_of_code {
                        
                }

                Vec::new()
        }

        fn read_next_user_input(&mut self) -> u8 {
                match self.user_inputs.pop() {
                        Some(value) => value,
                        None => 0,
                }
        }
        fn discover_end_of_code(&self) -> usize {
                self.code.len()
        }

        fn discover_end_of_loop(&self, start_of_loop_index: usize) -> Result<usize, &'static str> {
                if start_of_loop_index >= self.code.len() {
                        panic!("start_of_loop_index is greater than code.len()");
                }

                let mut loop_stack = 0;

                for index in (start_of_loop_index+1)..self.code.len() {
                        if self.code[index] == *commands::BEGIN_LOOP {
                                loop_stack += 1;
                        }
                        else if self.code[index] == *commands::END_LOOP && loop_stack == 0 {
                                return Ok(index)
                        }
                        else if self.code[index] == *commands::END_LOOP {
                                loop_stack -= 1;
                        }
                }

                Err("Could not find the end of loop")
        }
}
