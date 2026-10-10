// storage3.rs
//
// Storage on Stellar is rented. Every entry has a TTL ("time to live"),
// measured in ledgers (one ledger closes roughly every 5 seconds). When the
// TTL runs out:
//
//   - temporary entries are deleted for good,
//   - persistent entries are archived and must be restored before use.
//
// Contracts extend the TTL of entries they care about with
// `extend_ttl(&key, threshold, extend_to)`: "if the TTL is below `threshold`
// ledgers, extend it to `extend_to` ledgers".
//
// Two things to fix:
//   1. Notes are important: after saving one, extend its TTL using the
//      constants below.
//   2. Cached values are cheap and disposable: they belong in temporary
//      storage, not persistent storage.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol};

/// Roughly one day of ledgers.
pub const TTL_THRESHOLD: u32 = 17_280;
/// Roughly thirty days of ledgers.
pub const TTL_EXTEND_TO: u32 = 518_400;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Note(u32),
    Cache(u32),
}

#[contract]
pub struct Notebook;

#[contractimpl]
impl Notebook {
    pub fn save_note(env: Env, id: u32, note: Symbol) {
        let key = DataKey::Note(id);
        env.storage().persistent().set(&key, &note);
        env.storage()
            .persistent()
            .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
    }

    pub fn note(env: Env, id: u32) -> Option<Symbol> {
        env.storage().persistent().get(&DataKey::Note(id))
    }

    pub fn cache(env: Env, id: u32, value: u32) {
        env.storage().temporary().set(&DataKey::Cache(id), &value);
    }

    pub fn cached(env: Env, id: u32) -> Option<u32> {
        env.storage().temporary().get(&DataKey::Cache(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::symbol_short;
    use soroban_sdk::testutils::storage::Persistent as _;

    #[test]
    fn notes_live_long() {
        let env = Env::default();
        let contract_id = env.register(Notebook, ());
        let client = NotebookClient::new(&env, &contract_id);

        client.save_note(&1, &symbol_short!("gm"));
        assert_eq!(client.note(&1), Some(symbol_short!("gm")));

        // Peek into the contract's storage from the test.
        let ttl = env.as_contract(&contract_id, || {
            env.storage().persistent().get_ttl(&DataKey::Note(1))
        });
        assert_eq!(ttl, TTL_EXTEND_TO);
    }

    #[test]
    fn cache_is_temporary() {
        let env = Env::default();
        let contract_id = env.register(Notebook, ());
        let client = NotebookClient::new(&env, &contract_id);

        client.cache(&7, &99);
        assert_eq!(client.cached(&7), Some(99));

        env.as_contract(&contract_id, || {
            assert!(env.storage().temporary().has(&DataKey::Cache(7)));
            assert!(!env.storage().persistent().has(&DataKey::Cache(7)));
        });
    }
}
