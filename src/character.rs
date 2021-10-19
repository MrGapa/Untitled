use super::{
    raylib::prelude::*,
    animation::Animation,
    misc_funcs::Transform
};

// Create a trait for characters

pub struct Character {
    pub sprite: Texture2D,
    pub animation: Animation,
    speed: f32,
    pub transform:Transform
}

impl Character {
   
}