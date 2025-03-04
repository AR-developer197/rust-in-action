extern crate rand;

use std::vec;

use rand::Rng;

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed
}

#[allow(unused_variables)]
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
    state: FileState
}

impl File {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed
        }
    }

    pub fn new_with_data(name: &str, data: &Vec<u8>) -> Self{
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    pub fn read(&mut self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
    
    #[allow(unused_variables)]
    pub fn open(mut self) -> Result<Self, String> {
        self.state = FileState::Open;

        Ok(self)
    }

    #[allow(unused_variables)]
    pub fn close(mut self) -> Result<Self, String> {
        self.state = FileState::Closed;

        Ok(self)
    }
}

#[allow(unused_variables)]
pub fn exc() {
    let f3_data = vec![114, 117, 115, 116, 33];
    let f3 = File::new_with_data("2.txt", &f3_data);

    let mut buffer = vec![];

    let mut f3 = f3.open().unwrap();
    let f3_length = f3.read(&mut buffer);
    let f3 = f3.close().unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:#?}", f3.state);
}