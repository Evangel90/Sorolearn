// types2.rs
//
// Enums can be contract types too. Variants may be plain (`Pending`) or carry
// data (`Square(u32)`).
//
// Make `area` handle every kind of shape.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Shape {
    /// A square with the given side length.
    Square(u32),
    /// A rectangle with the given width and height.
    Rectangle(u32, u32),
    /// A triangle with the given base and height.
    Triangle(u32, u32),
}

#[contract]
pub struct Geometry;

#[contractimpl]
impl Geometry {
    pub fn area(_env: Env, shape: Shape) -> u32 {
        match shape {
            Shape::Square(side) => side * side,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(base, height) => base * height / 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_areas() {
        let env = Env::default();
        let client = GeometryClient::new(&env, &env.register(Geometry, ()));

        assert_eq!(client.area(&Shape::Square(3)), 9);
        assert_eq!(client.area(&Shape::Rectangle(2, 5)), 10);
        assert_eq!(client.area(&Shape::Triangle(4, 6)), 12);
    }
}
