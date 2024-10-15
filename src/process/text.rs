use crate::{cli::text_options::TextSignFormat, utils::get_reader};
use anyhow::Ok;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use std::{fs, io::Read};

trait TextSign {
    /// sign the data from reader and return the signature
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>, anyhow::Error>;
}

// trait TextVerify {
//     fn verify<R: Read>(&self, reader: &mut R, sign: &[u8]) -> Result<bool, anyhow::Error>;
// }

struct Blake3Signer {
    key: [u8; 32],
}

impl TextSign for Blake3Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>, anyhow::Error> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        Ok(blake3::keyed_hash(&self.key, &buffer).as_bytes().to_vec())
    }
}

// impl TextVerify for Blake3Signer {
//     fn verify<R: Read>(&self, reader: &mut R, sign: &[u8]) -> Result<bool, anyhow::Error> {
//         let mut buffer = Vec::new();
//         reader.read_to_end(&mut buffer)?;
//         let hash = blake3::hash(&buffer);
//         let hash = hash.as_bytes();
//         Ok(hash == sign)
//     }
// }

// struct Ed25519Signer {
//     key: [u8; 32],
// }

// struct Ed25519Verifier {
//     key: [u8; 32],
// }

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> Result<(), anyhow::Error> {
    let mut reader: Box<dyn Read> = get_reader(input)?;

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let sign = match format {
        TextSignFormat::Blake3 => {
            let key = fs::read(key)?;
            let key = &key[..32];
            let key = key.try_into()?;
            let signer = Blake3Signer { key };
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => todo!(),
    };

    let sign = URL_SAFE_NO_PAD.encode(&sign);
    println!("{}", sign);

    Ok(())
}
