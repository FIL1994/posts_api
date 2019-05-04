# posts_api

## Setup Database
> sqlite3 must be installed

### 1. Create DB

```bash
./scripts/create_db.sh
```

### 2. Insert Rows into DB

```
./scripts/insert_data.sh 300
```

## Rust

```bash
cd rust
cargo build --release
./cargo/target/release/posts_api
```

## Node

```bash
cd node
yarn install
yarn run start
```
