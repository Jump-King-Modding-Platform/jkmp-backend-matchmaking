use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ChatChannel {
    Global,
    Local,
    Friends,
    Group,
}
