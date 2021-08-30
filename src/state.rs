use std::{
    collections::{
        hash_map::{Iter, IterMut},
        HashMap,
    },
    net::SocketAddr,
};

use crate::client::Client;

pub struct State {
    clients: HashMap<SocketAddr, Client>,
}

impl State {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub fn add_client(&mut self, address: &SocketAddr, client: Client) -> Option<Client> {
        self.clients.insert(*address, client)
    }

    pub fn remove_client(&mut self, address: &SocketAddr) -> Option<Client> {
        self.clients.remove(&address)
    }

    pub fn get_client(&self, address: &SocketAddr) -> Option<&Client> {
        self.clients.get(address)
    }

    pub fn get_client_mut(&mut self, address: &SocketAddr) -> Option<&mut Client> {
        self.clients.get_mut(address)
    }

    pub fn get_clients_iter(&self) -> Iter<SocketAddr, Client> {
        self.clients.iter()
    }

    pub fn get_clients_iter_mut(&mut self) -> IterMut<SocketAddr, Client> {
        self.clients.iter_mut()
    }
}
