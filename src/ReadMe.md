Creates a binary for Windows or Linux that embeddes a command to be executed upon file execution on the target host.

Install Rust on Linux:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Initiate the new Rust project:
cargo new <name of the project> (folder will be created)

Change into the folder with main.rs:
cargo run
cargo build --release (to build a redistributable binary)

