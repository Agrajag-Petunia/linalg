use std::num::{Primitive, cast};
mod constants;

pub struct Euler<T>
{
    pitch: T,
    yaw: T,
    roll: T
}

impl<T: Primitive> Euler<T>
{
    pub fn new(pitch: T, yaw: T, roll: T) -> Euler<T>
    {
        Euler{pitch: pitch, yaw: yaw, roll: roll}
    }

    pub fn new_empty() -> Euler<T>
    {
        Euler::new( cast(0).unwrap(),
                    cast(0).unwrap(),
                    cast(0).unwrap() )
    }
}