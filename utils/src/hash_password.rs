use sha3::{Digest, Sha3_256};

/*
fn generate_salt() -> String {
    let mut rng = thread_rng();
    let salt: [u8; 16] = rng.gen(); // Gera um salt de 16 bytes
    hex::encode(salt) // Converte o salt em uma string hexadecimal
}

fn hash_password_with_salt(password: &str, salt: &str) -> String {
    let salted_password = format!("{}{}", salt, password);
    let mut hasher = Sha3_256::new();
    hasher.update(salted_password);
    let result = hasher.finalize();
    format!("{:x}", result)
}

 */

pub fn hash_password(password: &String) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(password);
    let result = hasher.finalize();
    format!("{:x}", result)
}
