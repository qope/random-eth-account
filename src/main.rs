use clap::Parser;
use ethers::{
    core::rand::Rng as _,
    signers::{LocalWallet, Signer},
    utils::{hex::encode, to_checksum},
};
use rand_chacha::{rand_core::SeedableRng as _, ChaCha20Rng};
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of accounts to generate
    #[arg(short, long, default_value_t = 1)]
    count: usize,

    /// Save output to a file
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut output = String::new();
    let mut chacha_rng = ChaCha20Rng::from_entropy();

    for _ in 0..args.count {
        let private_key = chacha_rng.gen::<[u8; 32]>();
        let private_key_str = encode(&private_key);
        let wallet: LocalWallet = private_key_str.parse().unwrap();
        let address = wallet.address();

        let mut account_info = format!(
            "Address: {}\nPrivate Key: {}\n",
            to_checksum(&address, None),
            private_key_str
        );

        account_info.push('\n');
        output.push_str(&account_info);
        print!("{}", account_info);
    }

    if let Some(file_name) = args.output {
        let mut file = File::create(file_name)?;
        file.write_all(output.as_bytes())?;
    }

    Ok(())
}
