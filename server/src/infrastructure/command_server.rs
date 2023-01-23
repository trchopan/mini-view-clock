use std::collections::HashMap;

use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use tracing::debug;

use crate::domain::View;

/// Command server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message for Command server communications

/// `CommandServer` manages Command rooms and responsible for coordinating Command session.
///
/// Implementation is very naïve.
#[derive(Default)]
pub struct CommandServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rng: ThreadRng,
}

impl CommandServer {
    pub fn new() -> CommandServer {
        CommandServer {
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    /// Send message to all users in the room
    pub fn send_message(&self, message: &str) {
        for (_, addr) in self.sessions.iter() {
            addr.do_send(Message(message.to_owned()));
        }
    }
}

/// Make actor from `CommandServer`
impl Actor for CommandServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// New Command session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for CommandServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        debug!("Ws client connected");

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        // send id back
        id
    }
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for CommandServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) -> Self::Result {
        debug!("Someone disconnected");
        self.sessions.remove(&msg.id);
    }
}

/// Command clients to change view
#[derive(Message)]
#[rtype(result = "()")]
pub struct ChangeView {
    pub view: View,
}

impl Handler<ChangeView> for CommandServer {
    type Result = ();

    fn handle(&mut self, msg: ChangeView, _: &mut Self::Context) -> Self::Result {
        self.send_message(&format!("CmdChangeView {:?}", msg.view));
    }
}
