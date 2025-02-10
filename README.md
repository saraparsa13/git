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

## Dependencies

- [Rust](https://www.rust-lang.org/) (1.40+ recommended)
- [flate2](https://crates.io/crates/flate2) - For zlib compression and decompression
- [sha1](https://crates.io/crates/sha1) - For computing SHA1 hashes

## Installation

1. **Clone the repository:**

2. **Build the project using Cargo:**

   ```bash
   cargo build --release
   ```

   *Alternatively, you can run the project directly with Cargo.*

## Usage

Run the project using Cargo with the appropriate command and arguments. Use `cargo run -- <command>` to execute the desired functionality.

### 1. Initialize the Repository

Create the necessary `.git` directory structure.

```bash
cargo run -- init
```

*Output:*

```
Initialized git directory
```

### 2. Create a Blob Object

Store a file as a blob object. This command reads the file, computes its blob hash, compresses the content, and writes it to the Git object store.

```bash
cargo run -- hash-object -w <filename>
```

*Example:*

```bash
cargo run -- hash-object -w README.md
```

*Output:*

```
Stored object: .git/objects/ab/cdef... (path based on SHA1 hash)
<sha1 hash>
```

### 3. Read a Blob Object

Decompress and display the contents of a blob object using its SHA1 hash.

```bash
cargo run -- cat-file -p <object-hash>
```

*Example:*

```bash
cargo run -- cat-file -p ab1234567890abcdef1234567890abcdef123456
```

*Output:*

```
<file content>
```

### 4. Read a Tree Object

Display the details of a tree object, listing each entry’s mode, SHA1 hash, and name.

```bash
cargo run -- ls-tree <tree-hash>
```

*Example:*

```bash
cargo run -- ls-tree ab1234567890abcdef1234567890abcdef123456
```

*Output:*

```
040000 tree <sha> <directory-name>
100644 blob <sha> <file-name>
...
```

Alternatively, to list **only the names** of the entries:

```bash
cargo run -- ls-tree --name-only <tree-hash>
```

*Output:*

```
<entry-name-1>
<entry-name-2>
...
```

## Code Overview

- **Initialization (`init`)**  
  Creates the `.git` directory structure and initializes the HEAD file.

- **Blob Handling**  
  - **Creating a Blob (`hash-object -w`)**  
    Reads a file, prepends the blob header (`blob <size>\0`), computes the SHA1 hash, compresses the blob, and writes it to `.git/objects`.
  - **Reading a Blob (`cat-file -p`)**  
    Decompresses the blob file and prints the stored content.

- **Tree Handling**  
  - **Reading a Tree (`ls-tree`)**  
    Decompresses and parses the tree object, printing each entry's mode, SHA1, and name. The optional `--name-only` flag limits the output to just the entry names.
