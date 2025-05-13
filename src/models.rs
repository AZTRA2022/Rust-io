pub struct Database<'a> {
    pub messages: Vec<&'a Message>,
    pub messages: Vec<T>,
    pub history: Vec<String>,
}

pub struct Message {
    pub sender: String,
    pub message: String,
    pub date: String,
}

pub struct Session {
    pub user: String,
    pub ip_address: String,
    pub messages: Vec<Message>,
}