# Build Your Own Git in Rust

This project is a minimal Git-like implementation written in Rust.

## Features

- **Initialization (`init`)**  
  Creates the `.git` directory structure and initializes the HEAD file.

  ```sh
    cargo run init
  ```

- **Blob Handling**  
  - **Creating a Blob (`hash-object -w`)**  
    Reads a file, prepends the blob header (`blob <size>\0`), computes the SHA1 hash, compresses the blob, and writes it to `.git/objects`.

    ```sh
      echo "hello world" > test.txt
      cargo run hash-object -w test.txt
    ```

  - **Reading a Blob (`cat-file -p`)**  
    Decompresses the blob file and prints the stored content.

    ```sh
      echo "hello world" > test.txt
      git hash-object -w test.txt
      cargo run cat-file -p <hash-object>
    ```

- **Tree Handling**  
  - **Reading a Tree (`ls-tree`)**  
    Decompresses and parses the tree object, printing each entry's mode, SHA1, and name. The optional `--name-only` flag limits the output to just the entry names.

    ```sh
      git write-tree
      cargo run ls-tree <hash-object>
    ```
