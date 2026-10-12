# Stellar: assets

Anyone can **issue an asset** on Stellar. A classic asset is the pair **code + issuer**: `USDC`
from Circle and `USDC` from someone else are different assets. Accounts opt in to holding an
asset by opening a **trustline**.

- Codes of 1-4 characters are `CreditAlphanum4`; 5-12 characters are `CreditAlphanum12`.
- The common text form is `CODE:ISSUER`, or `native` for XLM ([SEP-11](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0011.md)).
- Issuers can set flags such as **AUTH_REQUIRED** (holders need the issuer's approval) or
  **clawback**. That's how regulated assets work.

Every classic asset also has a **Stellar Asset Contract (SAC)**, so Soroban contracts can use it
like any token (that's what `TokenClient` talked to in `08_tokens`). In `assets2` a contract pays
out a regulated asset that has AUTH_REQUIRED set.

## Regulated assets (assets2)

Classic assets can require issuer approval (**AUTH_REQUIRED**). Through the SAC, check it with
`StellarAssetClient::authorized(&id)`; the issuer (the SAC admin) approves holders with
`set_authorized(&id, &true)`. In tests, turn the flag on with
`asset.issuer().set_flag(IssuerFlags::RequiredFlag)`.

## Further reading

- [Assets](https://developers.stellar.org/docs/learn/fundamentals/stellar-data-structures/assets)
- [Stellar Asset Contract](https://developers.stellar.org/docs/tokens/stellar-asset-contract)
