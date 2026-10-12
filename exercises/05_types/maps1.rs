// maps1.rs
//
// `soroban_sdk::Map<K, V>` is an ordered key-value map that lives in the host.
// Useful methods:
//
//   map.get(key)        -> Option<V>
//   map.set(key, value)
//   map.contains_key(key)
//   map.remove(key)
//
// Finish `count` so it returns how many times each word appears.

#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Map, Symbol, Vec};

#[contract]
pub struct WordCounter;

#[contractimpl]
impl WordCounter {
    pub fn count(env: Env, words: Vec<Symbol>) -> Map<Symbol, u32> {
        let mut counts = Map::new(&env);
        for word in words.iter() {
            // TODO: increase the count for `word` by one.
            // A word we have not seen yet has no entry in the map.
        }
        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{symbol_short, vec};

    #[test]
    fn counts_words() {
        let env = Env::default();
        let client = WordCounterClient::new(&env, &env.register(WordCounter, ()));

        let xlm = symbol_short!("xlm");
        let usdc = symbol_short!("usdc");
        let counts = client.count(&vec![&env, xlm.clone(), usdc.clone(), xlm.clone()]);

        assert_eq!(counts.get(xlm), Some(2));
        assert_eq!(counts.get(usdc), Some(1));
        assert_eq!(counts.get(symbol_short!("btc")), None);
        assert_eq!(counts.len(), 2);
    }
}
