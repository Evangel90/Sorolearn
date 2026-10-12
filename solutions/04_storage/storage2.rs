// storage2.rs
//
// Storage keys can be any contract type. A common pattern is a `DataKey` enum
// with one variant per kind of entry, where variants can carry data, for
// example the address of the user the entry belongs to.
//
// Store one high score per player in *persistent* storage. Players that never
// played have a score of 0.
//
// (Anyone can set anyone's score here! We'll fix that in the auth exercises.)

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Score(Address),
}

#[contract]
pub struct Leaderboard;

#[contractimpl]
impl Leaderboard {
    pub fn set_score(env: Env, player: Address, score: u32) {
        env.storage().persistent().set(&DataKey::Score(player), &score);
    }

    pub fn score(env: Env, player: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::Score(player))
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn stores_scores_per_player() {
        let env = Env::default();
        let client = LeaderboardClient::new(&env, &env.register(Leaderboard, ()));

        // Generate random addresses to act as players.
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);

        assert_eq!(client.score(&alice), 0);

        client.set_score(&alice, &420);
        client.set_score(&bob, &7);

        assert_eq!(client.score(&alice), 420);
        assert_eq!(client.score(&bob), 7);
    }
}
