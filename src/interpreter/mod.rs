mod loop_hash_map;
use crate::pointers::{Pointers, PointersIndex};

use self::loop_hash_map::{LoopHashMap, BFLoopHashMap};

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
        fn start(&mut self) -> Result<Vec<u8>, &'static str>;

        fn read_next_user_input(&mut self) -> Option<u8>;
        fn discover_end_of_code(&self) -> usize; // should be up to about 30000
        fn discover_end_of_loop(&self, start_of_loop_index: usize) -> Result<usize, &'static str>;
        fn check_code_loops(&self) -> bool;
}

pub struct Interpretor {
        code:           Vec<u8>,
        user_inputs:    Vec<u8>
}



impl BFInterpretor for Interpretor {
        fn new(code: Vec<u8>, user_inputs: Vec<u8>) -> Interpretor {
                Interpretor { code, user_inputs: user_inputs.into_iter().rev().collect::<Vec<u8>>() }
        }
        fn start(&mut self) -> Result<Vec<u8>, &'static str> {
                if !self.check_code_loops() {
                        return Err("Code contains invalids or unclosed loops")
                }
                let mut result = String::new();

                let mut pointers = Pointers::new();
                let end_of_code = self.discover_end_of_code();
                let mut code_index: usize = 0;

                let mut loop_layer = 0;

                let mut loop_hash_map = LoopHashMap::new();
                
                while code_index < end_of_code {
                        println!("code_index: {}", code_index);
                        match &self.code[code_index] {
                                commands::INCREMENT => {
                                        match pointers.increment_value() {
                                                Ok(_) => {
                                                        code_index += 1;
                                                        continue
                                                },
                                                Err(err) => eprintln!("Could not increment value, {}", err),
                                        }
                                },
                                commands::DECREMENT => {
                                        match pointers.decrement_value() {
                                                Ok(_) => {
                                                        code_index += 1;
                                                        continue
                                                },
                                                Err(err) => eprintln!("Could not decrement value, {}", err),
                                        }
                                },
                                commands::MOVE_FORWARD => {
                                        match pointers.move_index_forward() {
                                                Ok(_) => {
                                                        code_index += 1;
                                                        continue
                                                },
                                                Err(err) => eprintln!("Could not move forward, {}", err),
                                        }
                                },
                                commands::MOVE_BACKWARD => {
                                        match pointers.move_index_backward() {
                                                Ok(_) => {
                                                        code_index += 1;
                                                        continue
                                                },
                                                Err(err) => eprintln!("Could not move backward, {}", err),
                                        }
                                },
                                commands::PRINT_VALUE => {
                                        match pointers.read_value() {
                                                Some(value) => result.push(value as char),
                                                None => {
                                                        code_index += 1;
                                                        continue
                                                },
                                        }
                                },
                                commands::READ_VALUE => {
                                        match self.read_next_user_input() {
                                                Some(value) => match pointers.set_value(value) {
                                                        Ok(_) => {
                                                                code_index += 1;
                                                                continue
                                                        },
                                                        Err(err) => eprintln!("Could not set the value, {}", err),
                                                },
                                                None => {
                                                        code_index += 1;
                                                        continue
                                                },
                                        }
                                }
                                commands::BEGIN_LOOP => {
                                        loop_layer += 1;
                                        match loop_hash_map.add_entry(code_index, match self.discover_end_of_loop(code_index) {
                                                Ok(val) => val,
                                                Err(err) => {
                                                        eprintln!("could not find the end of loop, {}", err);
                                                        0
                                                }
                                        }) {
                                                Ok(_) => (),
                                                Err(err) => eprintln!("{}", err),
                                        };

                                        code_index += 1;
                                        continue;

                                },
                                commands::END_LOOP => {
                                        if let Some(val) = pointers.read_value() {
                                                if val == 0 {
                                                        code_index += 1;
                                                        continue;
                                                }
                                        }


                                        match loop_hash_map.get_start_index(code_index) {
                                                Some(val) => code_index = val + 1,
                                                None => {
                                                        eprintln!("Could not find the start index of the loop, ignoring");
                                                        continue;
                                                }
                                        }

                                        loop_layer -= 1;
                                        code_index += 1;
                                        continue;

                                },
                                _ => {
                                        code_index += 1;
                                        continue
                                },
                        }
                }

                if loop_layer == 0 {
                        println!("Code executed with exit code 0");
                }

                Ok(pointers.get_values_cloned())
        }

        fn read_next_user_input(&mut self) -> Option<u8> {
                // match self.user_inputs.pop() {
                //         Some(value) => value,
                //         None => 0,
                // }
                self.user_inputs.pop()
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
        fn check_code_loops(&self) -> bool {
                let mut loop_stack = 0;

                for index in 0..self.code.len() {
                        if self.code[index] == *commands::BEGIN_LOOP {
                                loop_stack += 1;
                        }
                        else if self.code[index] == *commands::END_LOOP {
                                loop_stack -= 1;
                        }
                }

                loop_stack == 0
        }
}
