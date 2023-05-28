# UNTS

Simple Notes management app using Rust and Mysql as a backend for remote storage bulk, also support for offline mode and usage of favorite text editor for editing note (ie vim).

## 🛠️ Development

Make sure you have Mysql and Rust installed

```bash
# Clone the directory & Enter the cloned directory
git clone https://github.com/UnknownRori/unts
cd unts

# Copy .env and setup the database connection url
cp .env .env.example
vim .env

# Install dependency and build the project
# Make sure you already make a database that correspond to migrations directory
cargo build

# To try out the unts
cargo run
```

## 🚀 Usage

## 🌟 Contribution

Feel free to contribute, send pull request or issue and i will take a look
