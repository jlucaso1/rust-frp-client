use aes::*;
use anyhow::Result;
use cfb_mode::{BufDecryptor, BufEncryptor};
use cipher::{KeyInit, KeyIvInit};
use ring::pbkdf2;
use std::num::NonZeroU32;

type Aes128CfbEnc = BufEncryptor<Aes128>;
type Aes128CfbDec = BufDecryptor<Aes128>;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA1;
const DEFAULT_SALT: &str = "frp";

#[derive(Debug, Clone)]
pub struct FrpCoder {
    iv: [u8; 16],
    key: [u8; 16],
    enc: Aes128CfbEnc,
    dec: Aes128CfbDec,
}

impl FrpCoder {
    pub fn new(token: &str, iv: [u8; 16]) -> Self {
        let mut key = [0x00; 16];
        pbkdf2::derive(
            PBKDF2_ALG,
            NonZeroU32::new(64).unwrap(),
            DEFAULT_SALT.as_bytes(),
            token.as_bytes(),
            &mut key,
        );

        Self {
            iv,
            key,
            enc: Aes128CfbEnc::new_from_slices(&key, &iv).unwrap(),
            dec: Aes128CfbDec::new_from_slices(&key, &iv).unwrap(),
        }
    }

    pub fn key(&self) -> &[u8; 16] {
        &self.key
    }

    pub fn iv(&self) -> &[u8; 16] {
        &self.iv
    }

    pub fn encypt(&mut self, buf: &mut Vec<u8>) -> Result<()> {
        let (iv, pos) = self.enc.get_state();
        let cipher = Aes128::new_from_slice(self.key()).unwrap();
        self.enc = Aes128CfbEnc::from_state(cipher, iv, pos);

        self.enc.encrypt(buf);

        Ok(())
    }

    pub fn decrypt(&mut self, buf: &mut Vec<u8>) -> Result<()> {
        let (iv, pos) = self.dec.get_state();
        let cipher = Aes128::new_from_slice(self.key()).unwrap();
        self.dec = Aes128CfbDec::from_state(cipher, iv, pos);

        self.dec.decrypt(buf);

        Ok(())
    }
}
