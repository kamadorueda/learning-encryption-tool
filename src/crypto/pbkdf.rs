use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

#[cfg(debug_assertions)]
const ROUNDS: u32 = 1;
#[cfg(not(debug_assertions))]
const ROUNDS: u32 = 1000000;

pub(crate) fn pbkdf<const KEY_LEN: usize>(password: &[u8], salt: &[u8]) -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];

    println!("{}", ROUNDS);
    pbkdf2_hmac::<Sha256>(password, salt, ROUNDS, &mut key);

    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pbkdf() {
        let password = b"password";
        let salt = b"salt";

        assert_eq!(pbkdf::<4>(password, salt), [18, 15, 182, 207]);
    }
}
