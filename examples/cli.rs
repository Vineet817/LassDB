use lassdb::{put_user, get_user, delete_user};
use lassdb::user::User;
use lassdb::schema::UserV2;
use clap::{Parser, Subcommand};
use std::env;
use lassdb::store::UserStore;

use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "lassdb", version = "0.1", author = "Vineet", about = "Rust-based embedded DB")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Insert a new user
    Put {
        key: String,
        name: String,
        email: String,
    },
    /// Retrieve a user by key
    Get {
        key: String,
    },
    /// Delete a user by key
    Delete {
        key: String,
    },
}

use tracing::{info, error};
use tracing_subscriber;
use anyhow::Result;


fn main() -> anyhow::Result<()> {
    // Setup tracing
    tracing_subscriber::fmt::init();

    // Open persistent DB
    let db = UserStore::new("db")?;

    println!("Welcome to LassDB (Interactive Mode)");
    println!("Commands: put <key> <email> | get <key> | delete <key> | exit");

    loop {
        print!("lassdb> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let args: Vec<String> = input.trim().split_whitespace().map(String::from).collect();

        match args.as_slice() {
            [cmd, key, email] if cmd == "put" => {
                let user = User::V2(UserV2 {
                    id: 1,
                    name: key.clone(),
                    email: email.clone(),
                });
                db.put(key, &user)?;
                info!("Inserted key: {}", key);
                println!("Inserted!");
            }
            [cmd, key] if cmd == "get" => {
                match db.get(key)? {
                    Some(user) => println!("Fetched: {:?}", user),
                    None => println!("Key not found"),
                }
            }
            [cmd, key] if cmd == "delete" => {
                let deleted = db.delete(key)?;
                println!("Deleted: {}", deleted);
            }
            [cmd] if cmd == "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Usage: put <key> <email> | get <key> | delete <key> | exit"),
        }

    }

    Ok(())
}