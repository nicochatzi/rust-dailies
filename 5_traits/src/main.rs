// struct/enum define data
// impl/traits define behavior

trait Default: std::cmp::PartialEq {
    fn default() -> Self;
    fn is_default(&self) -> bool
    where
        Self: Sized,
    {
        *self == Self::default()
    }
}

#[derive(Debug, PartialEq)]
struct Message {
    data: Vec<u8>,
}

impl Message {
    fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl Default for Message {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

#[derive(Debug, PartialEq)]
struct Email {
    body: String,
}

impl Email {
    fn new(body: String) -> Self {
        Self { body }
    }
}

impl Default for Email {
    fn default() -> Self {
        Self {
            body: String::new(),
        }
    }
}

trait Sendable: std::fmt::Debug {
    fn send(&self) {
        println!("{:?}", self);
    }
}

impl Sendable for Message {}
impl Sendable for Email {}

fn do_transfer<T: Sendable>(sendable: T) {
    sendable.send();
}

fn do_transfer_2(sendable: impl Sendable) {
    sendable.send();
}

fn do_transfer_3<T>(sendable: T)
where
    T: Sendable,
{
    sendable.send();
}

fn main() {
    Message::default().send();

    let msg = Message::new(vec![1, 2, 3]);
    let email = Email::new(String::from("hello"));
    let sendables: Vec<&dyn Sendable> = vec![&msg, &email];
    for sendable in sendables {
        sendable.send();
    }
}
