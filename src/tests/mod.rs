use bitcoin::key::{Keypair, Secp256k1};
use bitcoin::secp256k1::PublicKey;
use wasm_bindgen_test::wasm_bindgen_test;

pub mod free_mint_test_integration;
pub mod free_mint_test_unit_wasm;
pub mod std;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_public_key_serialization_and_deserialization() -> anyhow::Result<()> {
        let secp = Secp256k1::new();
        let keypair = Keypair::new(&secp, &mut rand::thread_rng());
        let pk = keypair.public_key();

        let pk_serialized = pk.serialize().to_vec();
        assert_eq!(pk_serialized.len(), 33);

        // need to break down the public key into a u128
        let pk_version = pk_serialized[0] as u128;
        println!("pk_version {}", pk_version);

        let pk_part_1 = u128::from_be_bytes(pk_serialized[1..17].try_into().unwrap());
        let pk_part_2 = u128::from_be_bytes(pk_serialized[17..].try_into().unwrap());

        let mut reconstructed_pk_bytes = vec![];
        reconstructed_pk_bytes.push(*pk_version.to_be_bytes().last().unwrap());
        println!("pk version bytes: {:?}", pk_version.to_be_bytes());
        pk_part_1.to_be_bytes().iter().for_each(|x| reconstructed_pk_bytes.push(*x));
        pk_part_2.to_be_bytes().iter().for_each(|x| reconstructed_pk_bytes.push(*x));

        assert_eq!(reconstructed_pk_bytes.len(), 33);
        assert_eq!(pk_serialized, reconstructed_pk_bytes, "error with reconstruction");
        let reconstructed_pk = PublicKey::from_slice(&reconstructed_pk_bytes.as_slice()).expect("should work");
        assert_eq!(reconstructed_pk, pk);

        Ok(())
    }
}