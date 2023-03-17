use aes_gcm::{
    aead::{Aead, KeyInit, Payload, AeadCore, OsRng, generic_array::typenum::U16},
    aes::Aes256,
    AesGcm,
    Nonce,
    Tag,
};
use rand::distributions::{Alphanumeric, DistString};
use String;
use crate::utils::base64;
use crate::error::Result;


pub fn encrypt(text: &String, secret: &String) -> Encryption {
    //let nonce = Alphanumeric.sample_string(&mut rand::thread_rng(), 12);
   // let nonce = Nonce::from_slice(nonce.as_bytes();
    let nonce = AesGcm::<Aes256, U16>::generate_nonce(&mut OsRng);
//    let tag = Aes256Gcm::generate_tag(&mut OsRng);
//    let secret = Aes256Gcm::new_from_slice(&secret.as_bytes()).unwrap();
    let tag = Alphanumeric.sample_string(&mut OsRng, 16);
    let tag = Tag::from_slice(&tag.as_bytes());
    let payload = Payload {
        msg: text.as_bytes(),
        aad: tag
    };
    let cipher: AesGcm<Aes256, U16> = AesGcm::new_from_slice(format!("{:0>len$.len$}", secret, len = 32).as_bytes()).unwrap();
    let text = cipher.encrypt(&nonce, payload).expect("Failed to encrypt the text");
    
    Encryption {
        text: base64::encode(text),
        tag: base64::encode(tag.to_vec()),
        nonce: base64::encode(nonce.to_vec()),
    }
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

    /*
    let payload = Payload {
        msg: &ciphertext,
        aad: &tag
    };
*/
    let cipher: AesGcm<Aes256, U16> = AesGcm::new_from_slice(format!("{:0>len$.len$}", secret, len = 32).as_bytes())?;

    let result = cipher.decrypt(nonce, ciphertext.as_ref())?;
    let result = String::from_utf8(result).map_err(crate::error::utf8)?;

    Ok(result)

    /*
    //(format!("{:0>len$.len$}", secret, len = 32)
    match AesGcm::new_from_slice(format!("{:0>len$.len$}", secret, len = 32).as_bytes()) {
        Ok(cipher) => {
            let res = cipher.decrypt(nonce, ciphertext.as_ref())?;
            Ok(String::from_utf8(res).map_err(crate::error::utf8)?)
        },
        Err(err) => Err(err)?
    }
    */
}
