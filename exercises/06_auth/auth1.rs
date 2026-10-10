// auth1.rs
//
// On a blockchain anyone can call any contract function with any arguments.
// If a function acts on behalf of a user, the contract must check that the
// user really signed off on it.
//
// In Soroban that is one line: `user.require_auth()`. The host verifies the
// signatures (or the calling contract) and aborts the call if `user` did not
// authorize this exact invocation.
//
// Right now anybody can change anybody's nickname. Fix that.

#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct Nicknames;

#[contractimpl]
impl Nicknames {
    pub fn set_nickname(env: Env, user: Address, nickname: Symbol) {
        env.storage().persistent().set(&user, &nickname);
    }

    pub fn nickname(env: Env, user: Address) -> Option<Symbol> {
        env.storage().persistent().get(&user)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation};
    use soroban_sdk::{symbol_short, IntoVal};

    #[test]
    fn requires_user_auth() {
        let env = Env::default();
        let contract_id = env.register(Nicknames, ());
        let client = NicknamesClient::new(&env, &contract_id);
        let user = Address::generate(&env);

        // In tests, pretend every `require_auth` call is satisfied.
        env.mock_all_auths();

        client.set_nickname(&user, &symbol_short!("satoshi"));

        // `env.auths()` lists which addresses were asked to authorize what
        // during the most recent contract call.
        assert_eq!(
            env.auths(),
            std::vec![(
                user.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        contract_id.clone(),
                        Symbol::new(&env, "set_nickname"),
                        (user.clone(), symbol_short!("satoshi")).into_val(&env),
                    )),
                    sub_invocations: std::vec![],
                }
            )]
        );
        assert_eq!(client.nickname(&user), Some(symbol_short!("satoshi")));
    }

    #[test]
    fn rejects_unauthorized_calls() {
        let env = Env::default();
        let client = NicknamesClient::new(&env, &env.register(Nicknames, ()));
        let user = Address::generate(&env);

        // No mocked auth this time: nobody signed anything.
        assert!(client.try_set_nickname(&user, &symbol_short!("hacker")).is_err());
        assert_eq!(client.nickname(&user), None);
    }
}
