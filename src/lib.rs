use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: MessageBody,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageBody {
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

fn get_id() -> u32 {
    static COUNTER: AtomicU32 = AtomicU32::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub fn handle_echo(message: &Message) -> Message {
    let mut reply = (*message).clone();

    reply.body.in_reply_to = message.body.msg_id;
    reply.src = message.dest.clone();
    reply.dest = message.src.clone();
    reply.body.message_type = String::from("echo_ok");

    return reply;
}

pub fn handle_init(message: &Message) -> Message {
    let mut reply = (*message).clone();

    reply.body.in_reply_to = message.body.msg_id;
    reply.src = message.dest.clone();
    reply.dest = message.src.clone();
    reply.body.message_type = String::from("init_ok");

    return reply;
}

pub fn handle_generate(message: &Message) -> Message {
    let mut reply = (*message).clone();

    reply.src = message.dest.clone();
    reply.dest = message.src.clone();
    reply.body.in_reply_to = message.body.msg_id;
    reply.body.message_type = String::from("generate_ok");
    let id = Uuid::new_v4().to_string();
    reply.body.id = Some(id);

    return reply;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let message = Message {
            src: String::from("admin"),
            dest: String::from("n1"),
            body: MessageBody {
                message_type: String::from("echo"),
                msg_id: Some(10),
                in_reply_to: None,
                echo: None,
                id: None,
            },
        };

        let reply = handle_echo(&message);

        assert_eq!(reply.body.in_reply_to, Some(10));
        assert_eq!(reply.body.msg_id, Some(11));
        assert_eq!(reply.src, String::from("n1"));
        assert_eq!(reply.dest, String::from("admin"));
    }
}
