// types1.rs
//
// You can pass your own structs in and out of contracts, as long as Soroban
// knows how to encode them. The `#[contracttype]` attribute generates that
// encoding (and adds the type to the contract's public interface).
//
// Make `Profile` usable as a contract argument and return value.

#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Profile {
    pub name: Symbol,
    pub level: u32,
}

#[contract]
pub struct Game;

#[contractimpl]
impl Game {
    /// Returns the profile with its level increased by one.
    pub fn level_up(_env: Env, mut profile: Profile) -> Profile {
        profile.level += 1;
        profile
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::symbol_short;

    #[test]
    fn levels_up() {
        let env = Env::default();
        let client = GameClient::new(&env, &env.register(Game, ()));

        let profile = Profile { name: symbol_short!("alice"), level: 1 };
        let upgraded = client.level_up(&profile);

        assert_eq!(upgraded, Profile { name: symbol_short!("alice"), level: 2 });
    }
}
