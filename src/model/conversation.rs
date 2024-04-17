use serde::Deserialize;
use serde::Serialize;

// The `Conversation` struct represents a conversation that contains a vector of `Message` objects.
// `Serialize` and `Deserialize` are traits provided by the `serde` crate for serializing and deserializing Rust data structures.
// `Clone` is a trait that allows for the duplication of an object.
// `Debug` is a trait that formats the value using the `Debug` formatter.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub messages: Vec<Message>
}

// `impl` is used to define implementations on types. Here, it's used to define methods on the `Conversation` struct.
impl Conversation {
    // `new` is a common convention for a function that returns a new instance of a struct.
    // Here, it returns a new `Conversation` with an empty vector of `Message` objects.
    pub fn new() -> Conversation {
        Conversation {
            messages: Vec::new()
        }
    }
}

// The `Message` struct represents a single message in a conversation.
// It has a `user` field indicating whether the message is from the user (true) or not (false),
// and a `text` field storing the content of the message.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub user: bool,
    pub text: String,
}