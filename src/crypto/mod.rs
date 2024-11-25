pub(crate) mod chunked;
pub(crate) mod decrypt;
pub(crate) mod encrypt;
pub(crate) mod pbkdf;
pub(crate) mod random_bytes;

#[cfg(test)]
mod tests {
    #[test]
    fn e2e() {
        let password = b"password";
        let salt = b"salt";

        let plaintext = b"Hello";
        let additional_data = b"World";

        let key = crate::crypto::pbkdf::pbkdf::<32>(password, salt);

        let nonce = b"1234567";

        let ciphertext = crate::crypto::encrypt::encrypt(&key, nonce, &[], &plaintext[..])
            .try_collect::<Vec<_>>()
            .expect("unable to encrypt")
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        let decrypted = crate::crypto::decrypt::decrypt(&key, nonce, &[], &ciphertext[..])
            .try_collect::<Vec<_>>()
            .expect("unable to decrypt")
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        assert_eq!(&*decrypted, plaintext);

        let ciphertext =
            crate::crypto::encrypt::encrypt(&key, nonce, additional_data, &plaintext[..])
                .try_collect::<Vec<_>>()
                .expect("unable to encrypt")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

        let decrypted =
            crate::crypto::decrypt::decrypt(&key, nonce, additional_data, &ciphertext[..])
                .try_collect::<Vec<_>>()
                .expect("unable to decrypt")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

        assert_eq!(&*decrypted, plaintext);
    }
}
