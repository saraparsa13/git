# Build Your Own Git in Rust

This project is a minimal Git-like implementation written in Rust.

## Features

- **Initialize the .git directory**  
  Set up the necessary directory structure (`.git`, `.git/objects`, `.git/refs`) and create the initial HEAD reference.

- **Create a Blob Object**  
  Reads a file, constructs a blob with a header (`blob <size>\0`), computes its SHA1 hash, compresses the content using zlib, and stores it in the appropriate location inside `.git/objects`.

- **Read a Blob Object**  
  Decompresses a blob object from the Git object store and prints its file content (excluding the header).

- **Read a Tree Object**  
  Decompresses a tree object and parses its entries, listing file modes, SHA1 hashes, and file names. Optionally, you can list only the names of the entries.
