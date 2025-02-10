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

fn construct_path(hash: &str) -> String {
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
        Ok(decompressed_data[null_index + 1..].to_string())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid blob format: no null byte found",
        ))
    }
}

fn read_blob_file(hash: &str) {
    let blob_path = construct_path(hash);
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

#[derive(Debug)]
struct TreeEntry {
    mode: String,
    sha: String,
    name: String,
}

fn parse_tree_entries(hash: &str) -> io::Result<Vec<TreeEntry>> {
    let tree_path = construct_path(hash);
    if !Path::new(&tree_path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("tree path not found: {}", tree_path),
        ));
    }

    let file = File::open(&tree_path)?;
    let mut decoder = ZlibDecoder::new(file);
    let mut decoded_data: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut decoded_data)?;

    let header_end = decoded_data.iter().position(|&b| b == 0).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid tree object: missing header null byte",
        )
    })? + 1;
    let mut pos = header_end;
    let mut entries = Vec::new();

    while pos < decoded_data.len() {
        let space_pos = decoded_data[pos..]
            .iter()
            .position(|&b| b == b' ')
            .map(|p| p + pos)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid tree object: missing space in entry",
                )
            })?;
        let mode = std::str::from_utf8(&decoded_data[pos..space_pos])
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            .to_string();

        pos = space_pos + 1;

        let null_pos = decoded_data[pos..]
            .iter()
            .position(|&b| b == 0)
            .map(|p| p + pos)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid tree object: missing null byte after filename",
                )
            })?;
        let name = std::str::from_utf8(&decoded_data[pos..null_pos])
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            .to_string();

        pos = null_pos + 1;

        if pos + 20 > decoded_data.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Unexpected end of tree entry data",
            ));
        }
        
        let sha_bytes = &decoded_data[pos..pos + 20];
        let sha = sha_bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        pos += 20;

        entries.push(TreeEntry { mode, sha, name });
    }
    Ok(entries)
}

fn print_tree(hash: &str) -> io::Result<()> {
    let entries = parse_tree_entries(hash)?;
    for entry in entries {
        match entry.mode.as_str() {
            "40000" => println!("040000 tree {} {}", entry.sha, entry.name),
            "100644" => println!("100644 blob {} {}", entry.sha, entry.name),
            _ => println!("{} {} {}", entry.mode, entry.sha, entry.name),
        }
    }
    Ok(())
}

fn print_tree_names(hash: &str) -> io::Result<()> {
    let entries = parse_tree_entries(hash)?;
    for entry in entries {
        println!("{}", entry.name);
    }
    Ok(())
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
        "ls-tree" => {
            if args.len() == 3 {
                if let Err(e) = print_tree(&args[2]) {
                    eprintln!("Error: {}", e);
                }
            } else if args.len() == 4 {
                if args[2] == "--name-only" {
                    if let Err(e) = print_tree_names(&args[3]) {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
        _ => eprintln!("Unknown command: {:?}", args),
    }
}
