// audit_auth.rs  -  Review the AI #1
//
// An AI assistant wrote this contract from the prompt:
//   "a points ledger where an admin mints points and users can send
//    points to each other"
//
// It compiles, it calls `require_auth`, and the happy-path tests pass.
// It is also exploitable: the `exploit_` test below reproduces a real attack.
//
// Your job: find the vulnerability and fix the contract so the exploit test
// passes, without breaking the other tests. Don't edit the tests.

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, Address, Env,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PointsError {
    InsufficientBalance = 1,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Balance(Address),
}

#[contract]
pub struct Points;

#[contractimpl]
impl Points {
    pub fn __constructor(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        let balance = Self::balance(env.clone(), to.clone());
        env.storage().persistent().set(&DataKey::Balance(to), &(balance + amount));
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        to.require_auth();

        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            panic_with_error!(&env, PointsError::InsufficientBalance);
        }
        let to_balance = Self::balance(env.clone(), to.clone());
        env.storage().persistent().set(&DataKey::Balance(from), &(from_balance - amount));
        env.storage().persistent().set(&DataKey::Balance(to), &(to_balance + amount));
    }

    pub fn balance(env: Env, who: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(who)).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::{Address as _, MockAuth, MockAuthInvoke};
    use soroban_sdk::IntoVal;

    fn setup() -> (Env, Address, PointsClient<'static>) {
        let env = Env::default();
        let admin = Address::generate(&env);
        let id = env.register(Points, (&admin,));
        let client = PointsClient::new(&env, &id);
        (env, id, client)
    }

    #[test]
    fn happy_path_transfer() {
        let (env, _, points) = setup();
        env.mock_all_auths();
        let (alice, bob) = (Address::generate(&env), Address::generate(&env));

        points.mint(&alice, &100);
        points.transfer(&alice, &bob, &40);

        assert_eq!(points.balance(&alice), 60);
        assert_eq!(points.balance(&bob), 40);
    }

    #[test]
    fn exploit_steal_points() {
        let (env, id, points) = setup();
        let victim = Address::generate(&env);
        let attacker = Address::generate(&env);
        env.mock_all_auths();
        points.mint(&victim, &100);

        // The attacker signs a transfer *from the victim to themselves*.
        // Only the attacker's signature exists; the victim signed nothing.
        env.mock_auths(&[MockAuth {
            address: &attacker,
            invoke: &MockAuthInvoke {
                contract: &id,
                fn_name: "transfer",
                args: (&victim, &attacker, 100_i128).into_val(&env),
                sub_invokes: &[],
            },
        }]);
        let attack = points.try_transfer(&victim, &attacker, &100);

        assert!(attack.is_err(), "the attacker stole the victim's points!");
        assert_eq!(points.balance(&victim), 100);
        assert_eq!(points.balance(&attacker), 0);
    }
}
