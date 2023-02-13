



pub trait PointersIndex {
        fn new() -> Pointers;
        fn move_index_forward(&mut self) -> Result<(), &'static str>;
        fn move_index_backward(&mut self) -> Result<(), &'static str>;
        fn increment_value(&mut self) -> Result<(), &'static str>;
        fn decrement_value(&mut self) -> Result<(), &'static str>;
        fn set_value(&mut self, value: u8) -> Result<(), &'static str>;
        fn read_value(&self) -> Option<u8>;

        fn adjust_values_forward(&mut self) -> Result<(), &'static str>;
        fn adjust_values_backward(&mut self) -> Result<(), &'static str>;
        fn increment_u8(value: u8) -> u8;
        fn decrement_u8(value: u8) -> u8;
}


pub struct Pointers {
        values:         Vec<u8>,
        index:          usize,
}

impl PointersIndex for Pointers {
        fn new() -> Pointers {
                Pointers { values: vec![0u8], index: 0 }
        }

        fn move_index_forward(&mut self) -> Result<(), &'static str> {
                self.index += 1;
                if self.index == self.values.len() {
                        return match self.adjust_values_forward() {
                                Ok(_) => Ok(()),
                                Err(err) => Err(err)
                        }
                }

                Ok(())
        }
        fn move_index_backward(&mut self) -> Result<(), &'static str> {
                if self.index == 0 {
                        return match self.adjust_values_backward() {
                                Ok(_) => Ok(()),
                                Err(err) => Err(err)
                        }
                }
                else {
                        self.index -= 1;
                }

                Ok(())
        }
        fn increment_value(&mut self) -> Result<(), &'static str> {
                if self.index >= self.values.len() {
                        return Err("Index is out of bounds")
                }
                self.values[self.index] = Pointers::increment_u8(self.values[self.index]);

                Ok(())
        }
        fn decrement_value(&mut self) -> Result<(), &'static str> {
                if self.index >= self.values.len() {
                        return Err("Index is out of bounds")
                }
                self.values[self.index] = Pointers::decrement_u8(self.values[self.index]);

                Ok(())
        }

        fn set_value(&mut self, value: u8) -> Result<(), &'static str> {
                self.values[self.index] = value;
                Ok(())
        }
        fn read_value(&self) -> Option<u8> {
                if self.index >= self.values.len() {
                        return None
                }
                Some(self.values[self.index])
        }

        fn adjust_values_forward(&mut self) -> Result<(), &'static str> {
                self.values.push(0);
                Ok(())
        }
        fn adjust_values_backward(&mut self) -> Result<(), &'static str> {
                self.values.insert(0, 0);
                Ok(())
        }
        fn increment_u8(value: u8) -> u8 {
                return match value {
                        255 => 0,
                        _ => value + 1,
                }
        }
        fn decrement_u8(value: u8) -> u8 {
                return match value {
                        0 => 255,
                        _ => value - 1,
                }
        }
}

/* test
let mut pointers = pointers::Pointers::new();

        if let Err(err) = pointers.set_value(70) {
                panic!("{}", err);
        }

        println!("{}", match pointers.read_value() {
                Some(value) => value,
                None => "Error".to_string()
        });


        for _ in 0..5 {
                if let Err(err) = pointers.decrement_value() {
                        panic!("{}", err)
                }
        }

        println!("{}", match pointers.read_value() {
                Some(value) => value,
                None => "Error".to_string()
        });

        if let Err(err) = pointers.move_index_forward() {
                panic!("{}", err);
        }

        if let Err(err) = pointers.set_value(30) {
                panic!("{}", err);
        }
        for _ in 0..45 {
                if let Err(err) = pointers.increment_value() {
                        panic!("{}", err)
                }
        }

        println!("{}", match pointers.read_value() {
                Some(value) => value,
                None => "Error".to_string()
        });

        if let Err(err) = pointers.move_index_backward() {
                panic!("{}", err);
        }
        println!("{}", match pointers.read_value() {
                Some(value) => value,
                None => "Error".to_string()
        }); */




