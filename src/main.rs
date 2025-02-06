use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

fn construct_blob_path(hash: &str) -> String {
    assert!(hash.len() == 40, "Invalid hash length");

    let (dir, file) = hash.split_at(2);
    format!(".git/objects/{}/{}", dir, file)
}

fn decompress_blob(blob_path: &str) -> io::Result<String> {
    let file = File::open(blob_path)?;

    let mut decoder = ZlibDecoder::new(file);
    let mut decompressed_data = String::new();

    decoder.read_to_string(&mut decompressed_data)?;

    if let Some(null_index) = decompressed_data.find('\0') {
        let content = &decompressed_data[null_index + 1..];
        Ok(content.to_string())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid blob format: no null byte found",
        ))
    }
}

fn read_blob_file(hash: &str) {
    let blob_path = construct_blob_path(hash);

    if Path::new(&blob_path).exists() {
        match decompress_blob(&blob_path) {
            Ok(content) => print!("{}", content),
            Err(e) => eprintln!("Error decompressing blob: {}", e),
        }
    } else {
        eprintln!("Blob not found: {}", blob_path);
    }
}

fn create_blob_file(filename: &str) -> io::Result<String> {
    let content = fs::read(filename)?;

    let header = format!("blob {}\0", content.len());
    let mut full_content = Vec::new();
    full_content.extend_from_slice(header.as_bytes());
    full_content.extend_from_slice(&content);

    let hash = Sha1::digest(&full_content);
    let hash_hex = format!("{:x}", hash);

    let mut compressed = Vec::new();
    let mut encoder = ZlibEncoder::new(&mut compressed, Compression::default());
    encoder.write_all(&full_content)?;
    encoder.finish()?;

    let (dir, file) = hash_hex.split_at(2);
    let object_dir = format!(".git/objects/{}", dir);
    let object_path = format!("{}/{}", object_dir, file);

    fs::create_dir_all(&object_dir)?;
    fs::write(&object_path, compressed)?;

    println!("Stored object: {}", object_path);
    Ok(hash_hex)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Missing command");
        return;
    }

    match args[1].as_str() {
        "init" => {
            fs::create_dir(".git").unwrap_or_else(|_| ());
            fs::create_dir(".git/objects").unwrap_or_else(|_| ());
            fs::create_dir(".git/refs").unwrap_or_else(|_| ());
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory");
        }
        "hash-object" if args.len() == 4 && args[2] == "-w" => match create_blob_file(&args[3]) {
            Ok(hash) => println!("{}", hash),
            Err(e) => eprintln!("Error: {}", e),
        },
        "cat-file" if args.len() == 4 && args[2] == "-p" => {
            read_blob_file(&args[3]);
        }
        _ => eprintln!("Unknown command: {:?}", args),
    }
}
