use std::env;
use std::path::Path;
use std::ffi::OsStr;
use std::fs;
use std::io;
use duckdb::{Config, Connection, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct DbConfig {
    access_mode: String,
    threads: String,
}

#[derive(Deserialize)]
struct 

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <file_name.yaml> <path/*.parquet>", args[0]);
        return Ok(());
    }

    let config_file_path = Path::new(&args[1]);
    let parquet_repo_path = Path::new(&args[2]);

    match config_file_path.extension() {
        Some(ext) if ext == OsStr::new("yaml") => {
            println!("Valid config file: {:?}", config_file_path);
        }
        _ => {
            eprintln!("Error: File must be a YAML file.");
            return Ok(());
        }
    }

    let file = fs::File::open(config_file_path).expect("Failed to open config file");
    let reader = io::BufReader::new(file);
    let db_config: DbConfig = serde_yaml::from_reader(reader).expect("Failed to parse .yaml");

    let mut config = Config::default();
    config.with("access_mode", db_config.access_mode)?;
    config.with("threads", db_config.threads)?;


    let conn = Connection::open(&config.database.path)?;

    conn.execute(&format!("PRAGMA threads={}", config.engine.threads), [])?;
    conn.execute(&format!(
        "PRAGMA memory_limit='{}'",
        config.engine.memory_limit
    ), [])?;



    println!("DuckDB configured and connection opened successfully.");
    Ok(())
}