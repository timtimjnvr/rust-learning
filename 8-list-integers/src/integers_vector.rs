use std::collections::HashMap;

#[derive(Debug)]
pub struct List {
    v: Vec<i32>,
}

impl List {
    pub fn add(&mut self, e: i32) {
        self.v.push(e);
        self.v.sort()
    }

    pub fn median(&self) -> Option<i32> {
        if self.v.len() == 0 {
            return None;
        }
        if self.v.len() == 1 {
            return Some(self.v[0]);
        }

        if is_even(self.v.len()) {
            let v1 = self.v[self.v.len() / 2];
            let v2 = self.v[(self.v.len() / 2) + 1];
            return Some((v1 + v2) / 2);
        }

        // Not even
        return Some(self.v[(self.v.len() + 1) / 2]);
    }

    pub fn mode(&self) -> Option<i32> {
        if self.v.len() == 0 {
            return None;
        }

        let mut hash_map = HashMap::new();
        let mut mode = 0;
        for i in &(self.v) {
            let count = hash_map.entry(i).or_insert(1);
            *count += 1;
            if *count > mode {
                mode = *count
            }
        }

        return Some(mode);
    }
}

pub fn new_list() -> List {
    List { v: Vec::new() }
}

fn is_even(v: usize) -> bool {
    v % 2 == 0
}
