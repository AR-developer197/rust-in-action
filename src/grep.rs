use std::{fs::File, io::{BufReader, Read}};

fn find_line(str_and_name: Vec<&str>) {
    let f = File::open(&str_and_name[2]).unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    reader.read_to_string(&mut buffer).unwrap();

    match buffer.find(str_and_name[1]) {
        Some(_) => println!("{:#?}", str_and_name[1]),
        None => println!(""),
    };
}

pub fn grep() {
    

    loop {
        let mut line = String::new();

        std::io::stdin().read_line(&mut line).unwrap();
        let str_and_name: Vec<&str> = line.split_whitespace().collect();        

        if str_and_name[0] == "grep" {
            find_line(str_and_name);
        } else {
            println!("idiot")
        }
    }
}