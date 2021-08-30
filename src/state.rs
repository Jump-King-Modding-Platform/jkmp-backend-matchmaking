use std::{collections::HashMap, net::SocketAddr};

use crate::client::Client;

pub struct State {
    pub clients: HashMap<SocketAddr, Client>,
}

impl State {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }
}
