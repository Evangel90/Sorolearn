// quiz1.rs
//
// Quiz time! This one combines storage, custom types, errors and auth.
//
// Build a simple poll:
//
//   vote(voter, option) -> Result<(), PollError>
//     - `voter` must authorize the call.
//     - Each address may vote only once. A second vote returns
//       `PollError::AlreadyVoted`.
//     - Every vote adds one to the tally of `option`.
//
//   votes(option) -> u32
//     - The number of votes `option` received (0 if none).
//
//   has_voted(voter) -> bool
//
// Use persistent storage, with the `DataKey` enum below as keys.

#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, Symbol};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PollError {
    AlreadyVoted = 1,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Set once an address has voted.
    Voted(Address),
    /// Number of votes for an option.
    Tally(Symbol),
}

#[contract]
pub struct Poll;

#[contractimpl]
impl Poll {
    pub fn vote(env: Env, voter: Address, option: Symbol) -> Result<(), PollError> {
        voter.require_auth();

        let storage = env.storage().persistent();
        let voted_key = DataKey::Voted(voter);
        if storage.has(&voted_key) {
            return Err(PollError::AlreadyVoted);
        }
        storage.set(&voted_key, &true);

        let tally_key = DataKey::Tally(option);
        let tally: u32 = storage.get(&tally_key).unwrap_or(0);
        storage.set(&tally_key, &(tally + 1));
        Ok(())
    }

    pub fn votes(env: Env, option: Symbol) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::Tally(option))
            .unwrap_or(0)
    }

    pub fn has_voted(env: Env, voter: Address) -> bool {
        env.storage().persistent().has(&DataKey::Voted(voter))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::symbol_short;
    use soroban_sdk::testutils::Address as _;

    fn setup() -> (Env, PollClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let client = PollClient::new(&env, &env.register(Poll, ()));
        (env, client)
    }

    #[test]
    fn tallies_votes() {
        let (env, poll) = setup();
        let (alice, bob, carol) = (
            Address::generate(&env),
            Address::generate(&env),
            Address::generate(&env),
        );

        poll.vote(&alice, &symbol_short!("pizza"));
        poll.vote(&bob, &symbol_short!("pizza"));
        poll.vote(&carol, &symbol_short!("tacos"));

        assert_eq!(poll.votes(&symbol_short!("pizza")), 2);
        assert_eq!(poll.votes(&symbol_short!("tacos")), 1);
        assert_eq!(poll.votes(&symbol_short!("sushi")), 0);
        assert!(poll.has_voted(&alice));
    }

    #[test]
    fn one_vote_per_address() {
        let (env, poll) = setup();
        let alice = Address::generate(&env);

        poll.vote(&alice, &symbol_short!("pizza"));
        assert_eq!(
            poll.try_vote(&alice, &symbol_short!("tacos")),
            Err(Ok(PollError::AlreadyVoted))
        );
        assert_eq!(poll.votes(&symbol_short!("tacos")), 0);
    }

    #[test]
    fn requires_auth() {
        let (env, poll) = setup();
        let alice = Address::generate(&env);

        poll.vote(&alice, &symbol_short!("pizza"));
        let auths = env.auths();
        assert_eq!(auths.len(), 1);
        assert_eq!(auths[0].0, alice);
    }
}
