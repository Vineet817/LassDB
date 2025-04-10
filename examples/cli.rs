
use lassdb::{LassDB, config::Config, memory_storage::InMemoryStorage, hybrid_storage::HybridStorage, StorageBackend};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "LassDB CLI")]
struct Args {
    #[arg(long)]
    no_disk: bool,
    #[command(subcommand)]
    command: Command,

}
#[derive(Subcommand)]
enum Command {
    Put {
        key: String,
        value: String,
    },
    Get {
        key: String,
    },
    Delete {
        key: String,
    },
}

fn main() {
    let args = Args::parse();

    let config = Config {
        autosave: false,
        flush_on_exit: true,
        snapshot_path: "snapshot.json".to_string(),
    };

    let storage: Box<dyn StorageBackend> = {
        #[cfg(target_arch = "wasm32")]
        {
            Box::new(InMemoryStorage::new(None))
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            if args.no_disk {
                Box::new(InMemoryStorage::new(None))
            } else {
                Box::new(HybridStorage::new(&config.snapshot_path))
            }
        }
    };

    let mut db = LassDB::with_config(storage, config);

    match args.command {
        Command::Put { key, value } => {
            db.put(key, value);
            println!("Value inserted.");
        }
        Command::Get { key } => {
            match db.get(&key) {
                Some(val) => println!("Value: {}", val),
                None => println!("Key not found."),
            }
        }
        Command::Delete { key } => {
            if db.delete(&key) {
                println!("Key deleted.");
            } else {
                println!("Key not found.");
            }
        }
    }

}
