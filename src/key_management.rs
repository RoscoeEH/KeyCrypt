use std::error::Error;

use crate::crypto::*;
use crate::utils::*;

pub fn generate_user_keypair(
    dk_path: String,
    ek_path: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (ek, dk) = kem_key_gen()?;

    // Get encryption needs
    let salt = get_salt();
    let kek = get_kek(&salt)?;
    let nonce = get_nonce();
    let protected_key = encrypt(&dk, &kek, &nonce)?;

    overwrite_key_file(ek_path, &ek)?;

    let mut dk_data = Vec::<u8>::new();
    dk_data.extend_from_slice("KEY".as_bytes());
    dk_data.extend_from_slice(&protected_key);
    dk_data.extend_from_slice(&nonce);
    dk_data.extend_from_slice(&salt);
    overwrite_key_file(dk_path, &dk_data)?;

    Ok(())
}
