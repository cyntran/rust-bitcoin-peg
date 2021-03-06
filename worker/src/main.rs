use nomic_client::{Client as PegClient, ClientError as PegClientError};
use nomic_work::work;
use rand::random;
use sha2::{Digest, Sha256};
use std::env;

const MIN_WORK: u64 = 1 << 20;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        generate();
        return;
    }
    if args[1] == "recover" {
        recover()
    }
}

fn generate() {
    println!("Running work program");
    let mut rpc = PegClient::new("localhost:26657").unwrap();
    let pub_key_bytes = rpc
        .tendermint_rpc
        .status()
        .expect("Unable to connect to tendermint RPC")
        .validator_info
        .pub_key
        .as_bytes();

    let mut nonce = random::<u64>();
    loop {
        let work_value = try_nonce(&pub_key_bytes, nonce);
        if work_value >= MIN_WORK {
            rpc.submit_work_proof(&pub_key_bytes.to_vec(), nonce);
        }
        nonce += 1;
    }
}

fn recover() {
    unimplemented!();
    let mut rpc = PegClient::new("localhost:26657").unwrap();
    let pub_key_bytes = rpc
        .tendermint_rpc
        .status()
        .expect("Unable to connect to tendermint RPC")
        .validator_info
        .pub_key
        .as_bytes();
}

fn try_nonce(pub_key_bytes: &[u8], nonce: u64) -> u64 {
    let mut hasher = Sha256::new();
    hasher.input(pub_key_bytes);
    let nonce_bytes = nonce.to_be_bytes();
    hasher.input(&nonce_bytes);
    let hash = hasher.result().to_vec();

    work(&hash)
}
