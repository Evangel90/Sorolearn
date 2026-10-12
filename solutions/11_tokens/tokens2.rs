// tokens2.rs
//
// Classic Stellar assets (like the ones from the assets lessons) are usable
// from Soroban through their *Stellar Asset Contract* (SAC). Same asset, same
// balances: the SAC is just its contract interface.
//
// Issuers of regulated assets (stablecoins, securities) set the
// AUTH_REQUIRED flag on their account. Then nobody can hold the asset until
// the issuer *authorizes* them (on the classic side: authorizes their
// trustline). Through the SAC:
//
//   StellarAssetClient::authorized(&id)            -> bool
//   StellarAssetClient::set_authorized(&id, &bool)    (admin only)
//
// This payroll contract pays employees in a regulated stablecoin. Make it
// refuse, with a clear error, to pay someone the issuer has not approved,
// instead of failing deep inside the token contract.

#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PayrollError {
    EmployeeNotAuthorized = 1,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Token,
}

#[contract]
pub struct Payroll;

#[contractimpl]
impl Payroll {
    pub fn __constructor(env: Env, token: Address) {
        env.storage().instance().set(&DataKey::Token, &token);
    }

    pub fn pay(env: Env, employer: Address, employee: Address, amount: i128) -> Result<(), PayrollError> {
        employer.require_auth();
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();

        if !token::StellarAssetClient::new(&env, &token).authorized(&employee) {
            return Err(PayrollError::EmployeeNotAuthorized);
        }
        token::TokenClient::new(&env, &token).transfer(&employer, &employee, &amount);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::{Address as _, IssuerFlags};

    #[test]
    fn pays_only_authorized_employees() {
        let env = Env::default();
        env.mock_all_auths();

        // A regulated stablecoin: holders must be authorized by the issuer.
        let asset = env.register_stellar_asset_contract_v2(Address::generate(&env));
        asset.issuer().set_flag(IssuerFlags::RequiredFlag);
        let admin = token::StellarAssetClient::new(&env, &asset.address());
        let usd = token::TokenClient::new(&env, &asset.address());

        let employer = Address::generate(&env);
        let employee = Address::generate(&env);
        admin.set_authorized(&employer, &true);
        admin.mint(&employer, &10_000);

        let payroll = PayrollClient::new(&env, &env.register(Payroll, (&asset.address(),)));

        // Not approved by the issuer yet: a clear, typed error.
        assert!(!admin.authorized(&employee));
        assert_eq!(payroll.try_pay(&employer, &employee, &3_000), Err(Ok(PayrollError::EmployeeNotAuthorized)));

        // After KYC, the issuer authorizes the employee.
        admin.set_authorized(&employee, &true);
        payroll.pay(&employer, &employee, &3_000);
        assert_eq!(usd.balance(&employee), 3_000);
        assert_eq!(usd.balance(&employer), 7_000);
    }
}
