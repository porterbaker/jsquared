use std::env;
use std::path::Path;
use std::ffi::OsStr;
//use duckdb::{params, Connection, Result};
use duckdb::Result;

/*
struct Duck {
    id: i32,
    name: String,
}
*/

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

    println!("Parquet path: {:?}", parquet_repo_path);

    Ok(())
}

/*
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE ducks (id INTEGER PRIMARY KEY, name TEXT)",
        [],
    )?;

    let ducks = conn
        .prepare("FROM ducks")?
        .query_map([], |row| {
            Ok(Duck {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;

    for duck in ducks {
        println!("{}) {}", duck.id, duck.name)
    }
    */
