use crate::{
    cli::{gen_pass_options::GenPassOptions, text_options::TextSignFormat},
    utils::get_reader,
};
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::{fs, io::Read, path::Path};

use super::process_genpass;

trait TextSign {
    /// sign the data from reader and return the signature
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
    fn verify<R: Read>(&self, reader: &mut R, sign: &[u8]) -> Result<bool>;
}

trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        Ok(blake3::keyed_hash(&self.key, &buffer).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify<R: Read>(&self, reader: &mut R, sign: &[u8]) -> Result<bool> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let hash = blake3::keyed_hash(&self.key, &buffer);
        let hash = hash.as_bytes();
        Ok(hash == sign)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let options = GenPassOptions {
            length: 32,
            no_uppercase: false,
            no_lowercase: false,
            no_numbers: false,
            no_symbols: false,
        };
        let password = process_genpass(&options)?;
        Ok(vec![password.as_bytes().to_vec()])
    }
}

struct Ed25519Signer {
    key: SigningKey,
}

impl Ed25519Signer {
    fn new(key: SigningKey) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let key = key.try_into()?;
        let key = SigningKey::from_bytes(key);
        let signer = Ed25519Signer::new(key);
        Ok(signer)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let sign = self.key.sign(&buffer);
        Ok(sign.to_bytes().to_vec())
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        Ok(vec![
            signing_key.to_bytes().to_vec(),
            verifying_key.to_bytes().to_vec(),
        ])
    }
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

impl Ed25519Verifier {
    fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let key = key.try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        Ok(Self::new(key))
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify<R: Read>(&self, reader: &mut R, sign: &[u8]) -> Result<bool> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let sign = Signature::from_bytes(sign.try_into()?);
        Ok(self.key.verify(&buffer, &sign).is_ok())
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let sign = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };
    let sign = URL_SAFE_NO_PAD.encode(&sign);
    Ok(sign)
}

pub fn process_verify(input: &str, key: &str, format: TextSignFormat, sign: &str) -> Result<bool> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let sign = URL_SAFE_NO_PAD.decode(sign)?;
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let verifier = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, &sign)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &sign)?
        }
    };
    Ok(verifier)
}

pub fn process_genkey(format: &TextSignFormat) -> Result<Vec<Vec<u8>>> {
    let keys = match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }?;
    Ok(keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify_blake3() -> anyhow::Result<()> {
        let blake3 = Blake3::load("fixtures/blake3.key")?;
        let mut reader = "hello".as_bytes();
        let sign = blake3.sign(&mut reader)?;
        let mut reader = "hello".as_bytes();
        assert!(blake3.verify(&mut reader, &sign)?);
        Ok(())
    }

    #[test]
    fn test_sign_and_verify_ed25519() -> anyhow::Result<()> {
        let sk = Ed25519Signer::load("fixtures/ed25519.key")?;
        let pk = Ed25519Verifier::load("fixtures/ed25519.pub")?;

        let mut reader = "hello".as_bytes();
        let sign = sk.sign(&mut reader)?;

        let mut reader = "hello".as_bytes();
        assert!(pk.verify(&mut reader, &sign)?);
        Ok(())
    }
}
