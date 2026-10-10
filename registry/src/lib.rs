//! Sorolearn graduation registry.
//!
//! A learner graduates by uploading the Wasm of their `quiz1` poll contract
//! to the network and calling `graduate`. The registry then deploys a fresh
//! copy of that Wasm, examines it with real cross-contract calls, and, if it
//! behaves to spec, records the learner as a graduate.
//!
//! Completion is therefore verified on-chain rather than self-reported.

#![no_std]

use soroban_sdk::{
    contract, contractclient, contracterror, contractevent, contractimpl, contracttype,
    symbol_short, Address, BytesN, ContractExecutable, Env, Symbol, Vec,
};

/// Keep graduation records for at least ~180 days after the last touch.
const DAY_IN_LEDGERS: u32 = 17_280;
const RECORD_TTL: u32 = 180 * DAY_IN_LEDGERS;
const RECORD_TTL_THRESHOLD: u32 = 30 * DAY_IN_LEDGERS;
const MAX_PAGE: u32 = 50;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum RegistryError {
    AlreadyGraduated = 1,
    /// The first vote was rejected or the contract trapped.
    VoteFailed = 10,
    /// The same address could vote twice.
    DoubleVoteAllowed = 11,
    /// A second vote did not return `AlreadyVoted` (error #1).
    WrongErrorCode = 12,
    /// `votes` returned the wrong tally.
    WrongTally = 13,
    /// `has_voted` returned the wrong answer.
    WrongHasVoted = 14,
}

/// The interface of the quiz1 poll contract, used to examine submissions.
#[contractclient(name = "PollClient")]
pub trait PollInterface {
    fn vote(env: Env, voter: Address, option: Symbol) -> Result<(), PollError>;
    fn votes(env: Env, option: Symbol) -> u32;
    fn has_voted(env: Env, voter: Address) -> bool;
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PollError {
    AlreadyVoted = 1,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Graduation {
    pub learner: Address,
    /// Wasm hash of the contract that passed the exam.
    pub wasm_hash: BytesN<32>,
    /// The fresh instance the registry deployed and examined.
    pub exam_contract: Address,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Number of graduates (instance).
    Count,
    /// Number of exams started, used to derive unique deploy salts (instance).
    Exams,
    /// learner -> Graduation (persistent).
    Graduate(Address),
    /// index -> learner, for listing (persistent).
    Index(u32),
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Graduated {
    #[topic]
    pub learner: Address,
    pub wasm_hash: BytesN<32>,
    pub number: u32,
}

#[contract]
pub struct Registry;

#[contractimpl]
impl Registry {
    /// Examine the poll contract uploaded as `wasm_hash` and, if it passes,
    /// record `learner` as a Sorolearn graduate.
    pub fn graduate(
        env: Env,
        learner: Address,
        wasm_hash: BytesN<32>,
    ) -> Result<u32, RegistryError> {
        learner.require_auth();
        bump_instance(&env);

        let record_key = DataKey::Graduate(learner.clone());
        if env.storage().persistent().has(&record_key) {
            return Err(RegistryError::AlreadyGraduated);
        }

        let exam_contract = deploy_fresh(&env, &wasm_hash);
        examine(&env, &exam_contract)?;

        let number: u32 = env.storage().instance().get(&DataKey::Count).unwrap_or(0) + 1;
        env.storage().instance().set(&DataKey::Count, &number);

        let record = Graduation {
            learner: learner.clone(),
            wasm_hash: wasm_hash.clone(),
            exam_contract,
            timestamp: env.ledger().timestamp(),
        };
        set_persistent(&env, &record_key, &record);
        set_persistent(&env, &DataKey::Index(number), &learner);

        Graduated {
            learner,
            wasm_hash,
            number,
        }
        .publish(&env);
        Ok(number)
    }

    pub fn is_graduate(env: Env, learner: Address) -> bool {
        env.storage().persistent().has(&DataKey::Graduate(learner))
    }

    pub fn graduation(env: Env, learner: Address) -> Option<Graduation> {
        env.storage().persistent().get(&DataKey::Graduate(learner))
    }

    pub fn count(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Count).unwrap_or(0)
    }

    /// Graduations numbered `start..start+limit` (numbers start at 1). At most 50 per call.
    pub fn graduates(env: Env, start: u32, limit: u32) -> Vec<Graduation> {
        let count = Self::count(env.clone());
        let mut out = Vec::new(&env);
        let first = start.max(1);
        let end = first
            .saturating_add(limit.min(MAX_PAGE))
            .min(count.saturating_add(1));
        for i in first..end {
            if let Some(learner) = env
                .storage()
                .persistent()
                .get::<_, Address>(&DataKey::Index(i))
            {
                if let Some(g) = env.storage().persistent().get(&DataKey::Graduate(learner)) {
                    out.push_back(g);
                }
            }
        }
        out
    }
}

/// Deploy a brand-new instance of the submission, so every exam starts from
/// empty storage no matter how many times a learner retries.
fn deploy_fresh(env: &Env, wasm_hash: &BytesN<32>) -> Address {
    let exams: u32 = env.storage().instance().get(&DataKey::Exams).unwrap_or(0);
    env.storage().instance().set(&DataKey::Exams, &(exams + 1));

    let mut seed = soroban_sdk::Bytes::from_array(env, &exams.to_be_bytes());
    seed.append(&wasm_hash.clone().into());
    let salt: BytesN<32> = env.crypto().sha256(&seed).into();

    // Fails the whole transaction if `wasm_hash` was never uploaded, or if the
    // contract's constructor requires arguments.
    env.deployer()
        .with_current_contract(salt)
        .deploy_contract(ContractExecutable::Wasm(wasm_hash.clone()), ())
}

/// The exam: the registry votes as itself (a contract authorizes its own
/// direct calls), then checks the poll's rules.
fn examine(env: &Env, exam_contract: &Address) -> Result<(), RegistryError> {
    let poll = PollClient::new(env, exam_contract);
    let me = env.current_contract_address();
    let pizza = symbol_short!("pizza");
    let tacos = symbol_short!("tacos");

    if poll.try_has_voted(&me) != Ok(Ok(false)) {
        return Err(RegistryError::WrongHasVoted);
    }
    match poll.try_vote(&me, &pizza) {
        Ok(Ok(())) => {}
        _ => return Err(RegistryError::VoteFailed),
    }
    match poll.try_vote(&me, &tacos) {
        Ok(_) => return Err(RegistryError::DoubleVoteAllowed),
        Err(Ok(PollError::AlreadyVoted)) => {}
        Err(_) => return Err(RegistryError::WrongErrorCode),
    }
    if poll.try_votes(&pizza) != Ok(Ok(1)) || poll.try_votes(&tacos) != Ok(Ok(0)) {
        return Err(RegistryError::WrongTally);
    }
    if poll.try_has_voted(&me) != Ok(Ok(true)) {
        return Err(RegistryError::WrongHasVoted);
    }
    Ok(())
}

fn set_persistent<V: soroban_sdk::IntoVal<Env, soroban_sdk::Val>>(
    env: &Env,
    key: &DataKey,
    value: &V,
) {
    env.storage().persistent().set(key, value);
    env.storage()
        .persistent()
        .extend_ttl(key, RECORD_TTL_THRESHOLD, RECORD_TTL);
}

fn bump_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(RECORD_TTL_THRESHOLD, RECORD_TTL);
}

#[cfg(test)]
mod test;
