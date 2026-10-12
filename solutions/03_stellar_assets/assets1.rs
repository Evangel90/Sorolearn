// assets1.rs
//
// Besides XLM, anyone can issue an asset on Stellar: a stablecoin, a loyalty
// point, a share. A classic asset is identified by two things:
//
//   code    1 to 12 letters/digits, e.g. "USDC"
//   issuer  the G... account that created it
//
// So "USDC issued by Circle" and "USDC issued by Mallory" are different
// assets! To hold an asset, an account first opens a *trustline* to it.
//
// In XDR, codes of 1-4 characters are `CreditAlphanum4` and codes of 5-12
// characters are `CreditAlphanum12`; both are padded with zero bytes. The
// common text form (used by wallets and SEP-11) is "CODE:ISSUER", or
// "native" for XLM.
//
// Make the tests pass.

use soroban_sdk::xdr::{
    AccountId, AlphaNum12, AlphaNum4, Asset, AssetCode12, AssetCode4, PublicKey, Uint256,
};
use stellar_strkey::ed25519;

#[derive(Debug, PartialEq, Eq)]
pub enum AssetError {
    InvalidCode,
    InvalidIssuer,
}

/// Parse "native" or "CODE:ISSUER".
pub fn parse_asset(text: &str) -> Result<Asset, AssetError> {
    if text == "native" {
        return Ok(Asset::Native);
    }
    let (code, issuer) = text.split_once(':').ok_or(AssetError::InvalidIssuer)?;
    let issuer = ed25519::PublicKey::from_string(issuer).map_err(|_| AssetError::InvalidIssuer)?;
    let issuer = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(issuer.0)));

    if code.is_empty() || code.len() > 12 || !code.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(AssetError::InvalidCode);
    }
    let mut padded = [0u8; 12];
    padded[..code.len()].copy_from_slice(code.as_bytes());

    if code.len() <= 4 {
        let asset_code = AssetCode4(padded[..4].try_into().unwrap());
        Ok(Asset::CreditAlphanum4(AlphaNum4 { asset_code, issuer }))
    } else {
        Ok(Asset::CreditAlphanum12(AlphaNum12 { asset_code: AssetCode12(padded), issuer }))
    }
}

/// The text form: "native" or "CODE:ISSUER" (without the zero padding).
pub fn asset_to_string(asset: &Asset) -> String {
    match asset {
        Asset::Native => "native".to_string(),
        Asset::CreditAlphanum4(a) => format!("{}:{}", code_text(&a.asset_code.0), issuer_text(&a.issuer)),
        Asset::CreditAlphanum12(a) => format!("{}:{}", code_text(&a.asset_code.0), issuer_text(&a.issuer)),
    }
}

fn code_text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim_end_matches('\0').to_string()
}

fn issuer_text(issuer: &AccountId) -> String {
    let AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(bytes))) = issuer;
    format!("{}", ed25519::PublicKey(*bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    const ISSUER: &str = "GDVEU3DD4KOFECV66VIHWEZOYX4ZKR3WV27L464SIIPOU2IUI3JCZA57";

    #[test]
    fn parses_native() {
        assert_eq!(parse_asset("native"), Ok(Asset::Native));
    }

    #[test]
    fn short_codes_are_alphanum4() {
        let Ok(Asset::CreditAlphanum4(a)) = parse_asset(&format!("USD:{ISSUER}")) else {
            panic!("expected CreditAlphanum4");
        };
        assert_eq!(&a.asset_code.0, b"USD\0");
    }

    #[test]
    fn long_codes_are_alphanum12() {
        let Ok(Asset::CreditAlphanum12(a)) = parse_asset(&format!("SOROLEARN:{ISSUER}")) else {
            panic!("expected CreditAlphanum12");
        };
        assert_eq!(&a.asset_code.0, b"SOROLEARN\0\0\0");
    }

    #[test]
    fn round_trips_text() {
        for text in ["native".to_string(), format!("USD:{ISSUER}"), format!("USDC:{ISSUER}"), format!("SOROLEARN:{ISSUER}")] {
            assert_eq!(asset_to_string(&parse_asset(&text).unwrap()), text);
        }
    }

    #[test]
    fn rejects_bad_input() {
        assert_eq!(parse_asset(&format!("WAYTOOLONGCODE:{ISSUER}")), Err(AssetError::InvalidCode));
        assert_eq!(parse_asset(&format!("US$:{ISSUER}")), Err(AssetError::InvalidCode));
        assert_eq!(parse_asset(&format!(":{ISSUER}")), Err(AssetError::InvalidCode));
        assert_eq!(parse_asset("USDC:GNOTANADDRESS"), Err(AssetError::InvalidIssuer));
        assert_eq!(parse_asset("USDC"), Err(AssetError::InvalidIssuer));
    }
}
