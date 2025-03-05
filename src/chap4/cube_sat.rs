#[derive(Debug)]
pub enum StatusMessage {
    OK,
}

#[derive(Debug)]
pub struct CubeSat {
    id: u64,
    mailbox: Mailbox
}

#[derive(Debug)]
pub struct Mailbox {
    messages: Vec<Message>
}

pub struct GroundStation;

pub type Message = String;

pub fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("{:#?}: {:#?}", sat_id, StatusMessage::OK);
    sat_id
}

impl GroundStation {
    pub fn send(to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg);
    }
}

impl CubeSat {
    pub fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}


pub fn exc() {
    let mut sat_a = CubeSat { id: 0, mailbox: Mailbox { messages: vec![] } };
    let sat_b = CubeSat { id: 1, mailbox: Mailbox { messages: vec![] } };
    let sat_c = CubeSat { id: 2, mailbox: Mailbox { messages: vec![] } };

    GroundStation::send(&mut sat_a, Message::from("Hello"));
    println!("{:#?}", sat_a.mailbox.messages);

    let msg = sat_a.recv();

    println!("msg: {:#?}", msg)
}