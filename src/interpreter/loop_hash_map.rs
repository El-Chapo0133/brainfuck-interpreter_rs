use std::collections::HashMap;


pub trait BFLoopHashMap {
        fn new() -> LoopHashMap;
        fn add_entry(&mut self, start: usize, end: usize) -> Result<(), &'static str>;
        fn remove_entry(&mut self, start_index: usize) -> Result<(), &'static str>;
        fn get_end_index(&self, start_index: usize) -> Option<&usize>;
        fn get_start_index(&self, end_index: usize) -> Option<&usize>;

        fn contains_value(&self, lue: &usize) -> bool;
}

pub struct LoopHashMap {
        indexes: HashMap<usize, usize>,
}

impl BFLoopHashMap for LoopHashMap {
        fn new() -> LoopHashMap {
                LoopHashMap { indexes: HashMap::<usize, usize>::new() }
        }

        fn add_entry(&mut self, start: usize, end: usize) -> Result<(), &'static str> {
                if self.indexes.contains_key(&start) || self.contains_value(&end) {
                        return Err("Given key (of value) is already set");
                }

                self.indexes.insert(start, end);

                if !self.indexes.contains_key(&start) || !self.contains_value(&end) {
                        return Err("Given key is not found after inserting it");
                }
                Ok(())
        }

        fn remove_entry(&mut self, start_index: usize) -> Result<(), &'static str> {
                if !self.indexes.contains_key(&start_index) {
                        return Err("Given key is not found");
                }

                self.indexes.remove(&start_index);

                if self.indexes.contains_key(&start_index) {
                        return Err("Given key is still accessible after removing it");
                }
                Ok(())
        }

        fn get_end_index(&self, start_index: usize) -> Option<&usize> {
                self.indexes.get(&start_index)
        }
        fn get_start_index(&self, end_index: usize) -> Option<&usize> {
                if !self.contains_value(&end_index) {
                        return None;
                }
                self.indexes.iter()
                        .find_map(|(key, &val)| if val == end_index { Some(key) } else { None })
        }

        fn contains_value(&self, value: &usize) -> bool {
                self.indexes.values().find(|val| val == &value).is_some()
        }
}


