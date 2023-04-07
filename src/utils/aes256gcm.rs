use crate::error::Result;
use crate::utils::base64;
use aes_gcm::{
    aead::{generic_array::typenum::U16, Aead, AeadCore, KeyInit, OsRng},
    aes::Aes256,
    AesGcm, Nonce,
};

pub fn encrypt(text: &str, secret: &str) -> Result<Encryption> {
    let nonce = AesGcm::<Aes256, U16>::generate_nonce(&mut OsRng);
    let cipher: AesGcm<Aes256, U16> =
        AesGcm::new_from_slice(format!("{:0>len$.len$}", secret, len = 32).as_bytes())?;
    let ciphertext = cipher.encrypt(&nonce, text.as_bytes())?;

    // The authentication tag is automatically appended by cipher.encrypt but we need to pass it to
    // infisical. So here we extract it as a slice of the ciphertext.
    let tag = &ciphertext[text.len()..];

    Ok(Encryption {
        text: base64::encode(&ciphertext[..text.len()]),
        tag: base64::encode(tag),
        nonce: base64::encode(nonce.as_slice()),
    })
}

#[derive(Debug)]
pub struct Encryption {
    pub text: String,
    pub tag: String,
    pub nonce: String,
}

pub fn decrypt(text: &str, nonce: &str, tag: &str, secret: &str) -> Result<String> {
    let nonce = base64::decode(nonce);
    let tag = base64::decode(tag);
    let text = base64::decode(text);
    let nonce = Nonce::<U16>::from_slice(&nonce);

    let mut ciphertext = text.clone();
    ciphertext.extend(&tag);

    let cipher: AesGcm<Aes256, U16> =
        AesGcm::new_from_slice(format!("{:0>len$.len$}", secret, len = 32).as_bytes())?;

    let result = cipher.decrypt(nonce, ciphertext.as_ref())?;
    let result = String::from_utf8(result).map_err(crate::error::utf8)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_is_successful() {
        let val = "encrypt this text, please.";
        let secret = "secretencryptionkeyienhtenh.,hHArstnitenaritn";

        let encrypted_val = encrypt(&val, &secret).unwrap();
        let decrypted_val = decrypt(
            &encrypted_val.text,
            &encrypted_val.nonce,
            &encrypted_val.tag,
            &secret,
        )
        .unwrap();

        assert_eq!(decrypted_val, val, "Encryption did not provide the expected result upon decryption.\nResult: {}\nExpected: {}", decrypted_val, val);
    }
}
