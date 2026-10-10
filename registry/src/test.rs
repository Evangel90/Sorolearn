extern crate std;

use super::*;
use soroban_sdk::testutils::{Address as _, Events as _};
use soroban_sdk::{Bytes, Event};

const POLL_OK: &[u8] = include_bytes!("../testdata/poll_ok.wasm");
const POLL_TODO: &[u8] = include_bytes!("../testdata/poll_todo.wasm");
const POLL_DOUBLE_VOTE: &[u8] = include_bytes!("../testdata/poll_double_vote.wasm");

fn setup() -> (Env, Address, RegistryClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(Registry, ());
    let client = RegistryClient::new(&env, &id);
    (env, id, client)
}

fn upload(env: &Env, wasm: &[u8]) -> BytesN<32> {
    env.deployer()
        .upload_contract_wasm(Bytes::from_slice(env, wasm))
}

#[test]
fn correct_poll_graduates() {
    let (env, id, registry) = setup();
    let learner = Address::generate(&env);
    let hash = upload(&env, POLL_OK);

    assert_eq!(registry.graduate(&learner, &hash), 1);

    let expected = Graduated {
        learner: learner.clone(),
        wasm_hash: hash.clone(),
        number: 1,
    };
    assert_eq!(
        env.events().all().filter_by_contract(&id),
        [expected.to_xdr(&env, &id)]
    );
    assert!(registry.is_graduate(&learner));
    assert_eq!(registry.count(), 1);
    let record = registry.graduation(&learner).unwrap();
    assert_eq!(record.wasm_hash, hash);
}

#[test]
fn requires_learner_auth() {
    let (env, _, registry) = setup();
    let learner = Address::generate(&env);
    let hash = upload(&env, POLL_OK);

    registry.graduate(&learner, &hash);
    assert_eq!(env.auths()[0].0, learner);
}

#[test]
fn unsolved_poll_fails_exam() {
    let (env, _, registry) = setup();
    let learner = Address::generate(&env);
    let hash = upload(&env, POLL_TODO);

    // `has_voted` is the first call of the exam, and it is still `todo!()`.
    assert_eq!(
        registry.try_graduate(&learner, &hash),
        Err(Ok(RegistryError::WrongHasVoted))
    );
    assert!(!registry.is_graduate(&learner));
}

#[test]
fn double_voting_poll_fails_exam() {
    let (env, _, registry) = setup();
    let learner = Address::generate(&env);
    let hash = upload(&env, POLL_DOUBLE_VOTE);

    assert_eq!(
        registry.try_graduate(&learner, &hash),
        Err(Ok(RegistryError::DoubleVoteAllowed))
    );
}

#[test]
fn cannot_graduate_twice_but_retries_get_a_fresh_exam() {
    let (env, _, registry) = setup();
    let (alice, bob) = (Address::generate(&env), Address::generate(&env));
    let hash = upload(&env, POLL_OK);

    registry.graduate(&alice, &hash);
    assert_eq!(
        registry.try_graduate(&alice, &hash),
        Err(Ok(RegistryError::AlreadyGraduated))
    );
    // Same Wasm, new learner: a fresh instance is deployed, so the exam
    // is not affected by the previous one's votes.
    assert_eq!(registry.graduate(&bob, &hash), 2);
}

#[test]
fn lists_graduates_in_order() {
    let (env, _, registry) = setup();
    let hash = upload(&env, POLL_OK);
    let learners: std::vec::Vec<Address> = (0..3).map(|_| Address::generate(&env)).collect();
    for l in &learners {
        registry.graduate(l, &hash);
    }

    let page = registry.graduates(&1, &10);
    assert_eq!(page.len(), 3);
    assert_eq!(page.get(0).unwrap().learner, learners[0]);
    assert_eq!(page.get(2).unwrap().learner, learners[2]);
    assert_eq!(registry.graduates(&3, &10).len(), 1);
    assert_eq!(registry.graduates(&4, &10).len(), 0);
}
