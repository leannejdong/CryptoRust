extern crate sha2;
// use crypto::digest::Digest;
// use crypto::sha2::Sha256;
use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};
// use near_sdk::{env};

fn cal_hash(input:&'static [u8; 11]){
    let mut sha = Sha256::new();
    sha.update(input);
    //println!("{}", sha.result_str());
    let result = sha.finalize();
    format!("{:x}", result);
}

#[cfg(test)]
mod tests{
    use super::*;
    // use near_sdk::test_utils::{get_logs, VMContextBuilder};
    // use near_sdk::{testing_env, AccountId};

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
}