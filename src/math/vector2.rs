use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Clone for Vector2 {
    fn clone(&self) -> Self {
        *self
    }
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
