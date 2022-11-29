use chrono::{DateTime, NaiveDateTime, Utc};
use clap::{Parser, Subcommand};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    account::from_account, clock::Clock, commitment_config::CommitmentConfig, sysvar,
};

#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    ClusterInfo,
}

const SERVER_URL: &str = "https://api.devnet.solana.com";

fn get_cluster_info(client: &RpcClient) {
    let version = client.get_version().unwrap();
    let result = client
        .get_account_with_commitment(&sysvar::clock::id(), CommitmentConfig::finalized())
        .unwrap();

    let (slot, timestamp) = match result.value {
        Some(clock_account) => {
            let clock: Clock = from_account(&clock_account).unwrap();
            (result.context.slot, clock.unix_timestamp)
        }
        None => {
            panic!("Unexpected None");
        }
    };

    
    let datetime = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap(),
        Utc,
    );

    println!("Cluster version: {}", version.solana_core);
    println!(
        "Block: {}, Time: {}",
        slot,
        datetime.format("%Y-%m-%d %H:%M:%S")
    );
}

fn main() {
    let cli = Cli::parse();
    let client = RpcClient::new(SERVER_URL);

    match &cli.command {
        Some(Commands::ClusterInfo) => {
            println!("Get cluster info");
            get_cluster_info(&client)
        }
        None => {}
    }
}
