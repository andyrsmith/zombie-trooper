use bevy::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}


#[derive(Component)]
pub struct Movement {
    pub last_movement: Direction,
}
