use std::{
    collections::{
        hash_map::{Iter, IterMut},
        HashMap,
    },
    net::SocketAddr,
};

use crate::{client::Client, math::Vector2};

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

    pub fn get_nearby_clients(&self, position: &Vector2) -> Vec<&Client> {
        let mut result = Vec::<&Client>::new();
        let level = get_y_level(position.y);

        for other in self.clients.values() {
            let other_level = get_y_level(other.position.y);

            // If the players are within 3 screens from eachother, they are close enough to matchmake
            if (level - other_level).abs() <= 3 {
                result.push(other);
            }
        }

        result
    }
}

#[inline]
fn get_y_level(y: f32) -> i32 {
    const Y_LEVEL_HEIGHT: i32 = 360; // 360 is the (unscaled) height in pixels of the screen at any time

    // y- is up in the game, but we're gonna treat a positive level as up, so we invert the y value
    let y_i32 = (-y.round()) as i32;

    y_i32 / Y_LEVEL_HEIGHT
}
