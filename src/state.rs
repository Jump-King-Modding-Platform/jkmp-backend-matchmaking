use std::{
    collections::{
        hash_map::{Iter, IterMut},
        HashMap,
    },
    hash::Hash,
    net::SocketAddr,
};

use crate::{client::Client, math::Vector2};

pub struct State {
    clients: HashMap<SocketAddr, Client>,
    matchmaking_map: HashMap<MatchmakingOptions, Vec<SocketAddr>>,
    client_matchmaking_map: HashMap<SocketAddr, MatchmakingOptions>,
}

impl State {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            matchmaking_map: HashMap::new(),
            client_matchmaking_map: HashMap::new(),
        }
    }

    pub fn add_client(
        &mut self,
        address: &SocketAddr,
        client: Client,
        matchmaking_options: MatchmakingOptions,
    ) -> Option<Client> {
        match self.clients.insert(*address, client) {
            Some(client) => Some(client),
            None => {
                self.set_matchmaking_options(address, Some(matchmaking_options));
                None
            }
        }
    }

    pub fn remove_client(&mut self, address: &SocketAddr) -> Option<Client> {
        match self.clients.remove(address) {
            Some(client) => {
                self.set_matchmaking_options(address, None);
                Some(client)
            }
            None => None,
        }
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

    pub fn get_clients_in_group(
        &self,
        matchmaking_options: &MatchmakingOptions,
    ) -> impl Iterator<Item = &Client> {
        let group = self.matchmaking_map.get(matchmaking_options).unwrap(); // The client is always guaranteed to be in a group
        let iter = group
            .iter()
            .map(move |addr| self.clients.get(addr).unwrap());

        iter
    }

    pub fn get_nearby_clients(
        &self,
        position: &Vector2,
        matchmaking_options: &MatchmakingOptions,
    ) -> Vec<&Client> {
        let mut result = Vec::<&Client>::new();
        let level = get_y_level(position.y);
        let clients = self.get_clients_in_group(matchmaking_options);

        for other in clients {
            let other_level = get_y_level(other.position.y);

            // If the players are within 3 screens from eachother, they are close enough to matchmake
            if (level - other_level).abs() <= 3 {
                result.push(other);
            }
        }

        result
    }

    pub fn get_matchmaking_options(&self, address: &SocketAddr) -> &MatchmakingOptions {
        self.client_matchmaking_map.get(address).unwrap()
    }

    pub fn set_matchmaking_options(
        &mut self,
        address: &SocketAddr,
        matchmaking_options: Option<MatchmakingOptions>,
    ) {
        let previous_options = self.client_matchmaking_map.get(address);

        // Return early if old and new options are identical
        if previous_options == matchmaking_options.as_ref() {
            return;
        }

        // Remove from previous group if it exists
        if let Some(previous_options) = previous_options {
            let new_group_len: usize;

            {
                let group = self.matchmaking_map.get_mut(previous_options).unwrap();
                group.retain(|addr| addr != address);
                new_group_len = group.len();
            }

            if new_group_len == 0 {
                self.matchmaking_map.remove(previous_options);
            }

            println!(
                "Removed {} from group. Members in group is now {}",
                address, new_group_len
            );
        }

        match matchmaking_options {
            Some(matchmaking_options) => {
                // Add to new group if new matchmaking options is set
                let group = self
                    .matchmaking_map
                    .entry(matchmaking_options.clone())
                    .or_insert_with(Vec::<SocketAddr>::new);

                group.push(*address);

                println!(
                    "Added {} to group. Members in group is now {}",
                    address,
                    group.len()
                );

                // Update client_matchmaking_map
                self.client_matchmaking_map
                    .insert(*address, matchmaking_options);
            }
            None => {
                // Remove address from client_matchmaking_map if new matchmaking options aren't set
                self.client_matchmaking_map.remove(address);
            }
        }
    }
}

#[inline]
fn get_y_level(y: f32) -> i32 {
    const Y_LEVEL_HEIGHT: i32 = 360; // 360 is the (unscaled) height in pixels of the screen at any time

    // y- is up in the game, but we're gonna treat a positive level as up, so we invert the y value
    let y_i32 = (-y.round()) as i32;

    y_i32 / Y_LEVEL_HEIGHT
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct MatchmakingOptions {
    pub password: Option<String>,
    pub level_name: String,
}

impl MatchmakingOptions {
    pub fn new(password: Option<String>, level_name: String) -> Self {
        Self {
            password,
            level_name,
        }
    }
}
