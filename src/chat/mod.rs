use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ChatChannel {
    Global = 1 << 0,
    Local = 1 << 1,
    Friends = 1 << 2,
    Group = 1 << 3,
}
