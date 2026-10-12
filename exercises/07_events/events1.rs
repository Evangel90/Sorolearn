// events1.rs
//
// Events let off-chain apps (wallets, indexers, explorers) react to what your
// contract does. Declare an event as a struct with `#[contractevent]`:
//
//   - fields marked `#[topic]` become *topics*, which indexers can filter on,
//   - the remaining fields become the event's *data*,
//   - by default, the struct name in snake_case is added as the first topic.
//
// Publish an event with `MyEvent { ... }.publish(&env)`.
//
// The tip jar should emit a `Tipped` event for every tip.

#![no_std]

use soroban_sdk::{contract, contractevent, contractimpl, Address, Env};

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tipped {
    #[topic]
    pub from: Address,
    #[topic]
    pub to: Address,
    pub amount: i128,
}

#[contract]
pub struct TipJar;

#[contractimpl]
impl TipJar {
    pub fn tip(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        // (A real tip jar would move tokens here. See the next topic!)
        // TODO: let the world know about this tip.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Events as _};
    use soroban_sdk::Event;

    #[test]
    fn emits_tipped_event() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(TipJar, ());
        let client = TipJarClient::new(&env, &contract_id);

        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        client.tip(&alice, &bob, &100);

        let expected = Tipped { from: alice, to: bob, amount: 100 };
        assert_eq!(env.events().all(), [expected.to_xdr(&env, &contract_id)]);
    }
}
