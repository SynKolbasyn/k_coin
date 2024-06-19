use std::env;
use std::path::PathBuf;

use anyhow::Result;

use rand::prelude::ThreadRng;
use rsa::{
    RsaPrivateKey,
    RsaPublicKey,
    pkcs1v15::{
        SigningKey,
        VerifyingKey,
    },
    pkcs8::{
        DecodePrivateKey,
        DecodePublicKey,
        EncodePrivateKey,
        EncodePublicKey,
        LineEnding::LF,
    },
    signature::{
        Keypair,
        RandomizedSigner,
        SignatureEncoding,
        Verifier,
    },
    sha2::Sha512_256,
};


pub struct KeyManager {
    rsa_private_key: RsaPrivateKey,
    signing_key: SigningKey<Sha512_256>,
    verifying_key: VerifyingKey<Sha512_256>,
}


impl KeyManager {
    pub fn new() -> Result<KeyManager> {
        let (rsa_private_key, signing_key, verifying_key): (RsaPrivateKey, SigningKey<Sha512_256>, VerifyingKey<Sha512_256>) = if Self::get_path()?.exists() {
            Self::load_keys()?
        }
        else {
            Self::generate_keys()?
        };
        
        let result: KeyManager = KeyManager {
            rsa_private_key,
            signing_key,
            verifying_key,
        };

        if !Self::get_path()?.exists() {
            result.save_keys()?;
        }
        
        Ok(result)
    }

    pub fn generate_keys() -> Result<(RsaPrivateKey, SigningKey<Sha512_256>, VerifyingKey<Sha512_256>)> {
        let mut rng: ThreadRng = rand::thread_rng();
        let rsa_private_key: RsaPrivateKey = RsaPrivateKey::new(&mut rng, 2048)?;
        let signing_key: SigningKey<Sha512_256> = SigningKey::<Sha512_256>::new(rsa_private_key.clone());
        let verifying_key: VerifyingKey<Sha512_256> = signing_key.verifying_key();

        Ok((rsa_private_key, signing_key, verifying_key))
    }

    pub fn save_keys(&self) -> Result<()> {
        self.rsa_private_key.write_pkcs8_pem_file(Self::get_path()?, LF)?;
        Ok(())
    }

    pub fn load_keys() -> Result<(RsaPrivateKey, SigningKey<Sha512_256>, VerifyingKey<Sha512_256>)> {
        let rsa_private_key: RsaPrivateKey = RsaPrivateKey::read_pkcs8_pem_file(Self::get_path()?)?;
        let signing_key: SigningKey<Sha512_256> = SigningKey::<Sha512_256>::new(rsa_private_key.clone());
        let verifying_key: VerifyingKey<Sha512_256> = signing_key.verifying_key();
        
        Ok((rsa_private_key, signing_key, verifying_key))
    }
    
    fn get_path() -> Result<PathBuf> {
        Ok(env::current_exe()?.parent().unwrap().join("key.pem"))
    }
    
    pub fn get_rsa_private_key(&self) -> RsaPrivateKey {
        self.rsa_private_key.clone()
    }
    
    pub fn get_signing_key(&self) -> SigningKey<Sha512_256> {
        self.signing_key.clone()
    }
    
    pub fn get_verifying_key(&self) -> VerifyingKey<Sha512_256> {
        self.verifying_key.clone()
    }
}
