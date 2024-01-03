pub struct Chat<'a> {
    messages: Vec<Message<'a>>
}

pub struct Message<'a> {
    message: &'a str
}