extern crate rand;

use std::vec;

#[allow(unused_variables)]
pub struct File {
    name: String,
    data: Vec<u8>
}

impl File {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            data: Vec::new()
        }
    }

    pub fn new_with_data(name: &str, data: &Vec<u8>) -> Self{
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    pub fn read(&mut self, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }
    
}

#[allow(unused_variables)]
pub fn open(f: &mut File) -> bool {
    true
}

#[allow(unused_variables)]
pub fn close(f: &mut File) -> bool {
    true
}

#[allow(unused_variables)]
pub fn exc() {
    let f3_data = vec![114, 117, 115, 116, 33];
    let mut f3 = File::new_with_data("2.txt", &f3_data);

    let mut buffer = vec![];

    open(&mut f3);
    let f3_length = f3.read(&mut buffer);
    close(&mut f3);

    let text = String::from_utf8_lossy(&buffer);

    println!("{}", text);
}