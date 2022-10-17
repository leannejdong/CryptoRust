extern crate sha2;
// use crypto::digest::Digest;
// use crypto::sha2::Sha256;
use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};
use hmac::{Hmac, Mac};
// use near_sdk::{env};
pub mod salt;
use crate::salt::hash;

fn cal_hash(input:&'static [u8; 11]){
    let mut sha = Sha256::new();
    sha.update(input);
    //println!("{}", sha.result_str());
    let result = sha.finalize();
    format!("{:x}", result);
}

fn cal_hmac(input:&'static [u8]){
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(b"my secret and secure key")
    .expect("HMAC can take key of any size");
mac.update(b"input message");
// `result` has type `CtOutput` which is a thin wrapper around array of
// bytes for providing constant time equality check
    let result = mac.finalize();
    //println!("{}", result);

}

#[cfg(test)]
mod tests{
    use super::*;
    // use near_sdk::test_utils::{get_logs, VMContextBuilder};
    // use near_sdk::{testing_env, AccountId};

    #[test]
    fn test_salt(){
        let result = hash("Hello!");
        println!("Let's debug:{:?}", result);
    }
    #[test]
    fn test_cal_hash(){
            let result = cal_hash(b"hello world");
        // // Using a unit test to rapidly debug and iterate
        // // let debug_solution = "near nomicon ref finance";
        // // let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        // // let debug_hash_string = hex::encode(debug_hash_bytes);
        // // println!("Let's debug: {:?}", debug_hash_string);
        // assert_eq!(result, "Expected the correct answer to return true.");
            println!("Let's debug:{:?}", result);

    }
    #[test]
    fn test_cal_hmac(){
        let result = cal_hmac(b"hello world");
        println!("Let's debug:{:?}", result);
    }

    #[test]
    #[should_panic]
    fn test_sha256(){
        // let result = cal_hash("hello world");
        // //let result = cal_hash(b"hello world");
        // //assert_eq!(result, "")
        // // Using a unit test to rapidly debug and iterate
        // // let debug_solution = "near nomicon ref finance";
        // // let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        // // let debug_hash_string = hex::encode(debug_hash_bytes);
        // // println!("Let's debug: {:?}", debug_hash_string);
        // assert_eq!(result, "Expected the correct answer to return true.");
        // //println!("Let's debug:{:?}", result);

    //     let mut hasher = Sha256::new();

    // // write input message
    // hasher.update(b"hello world");

    // read hash digest and consume hasher
 //   let result = cal_hash(b"hello world");
    //let final_result = result.finalize();


    // same for Sha512
    let mut hasher = Sha256::new();
    hasher.update(b"hello world");
    let result = hasher.finalize();

    assert_eq!(result[..], hex!("
        309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f
        989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f
    ")[..]);

    }
    #[test]
    //#[should_panic]
    fn test_hmac(){
        // Create alias for HMAC-SHA256
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(b"my secret and secure key")
        .expect("HMAC can take key of any size");
    mac.update(b"input message");

    // `result` has type `CtOutput` which is a thin wrapper around array of
    // bytes for providing constant time equality check
    let result = mac.finalize();
    // To get underlying array use `into_bytes`, but be careful, since
    // incorrect use of the code value may permit timing attacks which defeats
    // the security provided by the `CtOutput`
    let code_bytes = result.into_bytes();
    let expected = hex!("
        97d2a569059bbcd8ead4444ff99071f4
        c01d005bcefe0d3567e1be628e5fdcd9
    ");
    assert_eq!(code_bytes[..], expected[..]);
    }
}