# Rust Timeouts

An unresponsive service can be worse than a down one. It can tie up your entire system if not handled properly. **All network requests should have a timeout.**

Here’s how to add timeouts for popular Rust crates. **[All have been tested](src)**. The default is no timeout, unless otherwise specified. Enjoy!

[![Build Status](https://github.com/ankane/rust-timeouts/workflows/build/badge.svg?branch=master)](https://github.com/ankane/rust-timeouts/actions)

## Contents

Standard library

- [TcpStream](#tcpstream)

Crates

- [elasticsearch](#elasticsearch)
- [postgres](#postgres)
- [redis](#redis)
- [reqwest](#reqwest)
- [ureq](#ureq)

## Standard Library

### TcpStream

```rust
let stream = std::net::TcpStream::connect_timeout(&addr, Duration::from_secs(1))?;
stream.set_read_timeout(Some(Duration::from_secs(1)))?;
stream.set_write_timeout(Some(Duration::from_secs(1)))?;
```

## Crates

### elasticsearch

```rust
let transport = elasticsearch::http::transport::TransportBuilder::new(conn_pool)
    .timeout(Duration::from_secs(1))
    .build()?;
```

### postgres

```rust
let client = postgres::Config::new()
    .connect_timeout(Duration::from_secs(1))
    .connect(postgres::NoTls)?;
```

### redis

```rust
let mut con = client.get_connection_with_timeout(Duration::from_secs(1))?;
con.set_read_timeout(Some(Duration::from_secs(1)))?;
con.set_write_timeout(Some(Duration::from_secs(1)))?;
```

### reqwest

```rust
let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(1))
    .build()?;
```

or

```rust
let resp = client.get(url)
    .timeout(Duration::from_secs(1))
    .send()
    .await?;
```

### ureq

```rust
let agent = ureq::AgentBuilder::new()
    .timeout_connect(Duration::from_secs(1))
    .timeout_read(Duration::from_secs(1))
    .timeout_write(Duration::from_secs(1))
    .build();
```

## Don’t see a library you use?

[Let us know](https://github.com/ankane/rust-timeouts/issues/new). Even better, [create a pull request](https://github.com/ankane/rust-timeouts/pulls) for it.

## Running the Tests

```sh
git clone https://github.com/ankane/rust-timeouts.git
cd rust-timeouts
```

To run all tests, use:

```sh
cargo test
```

To run individual tests, use:

```sh
cargo test redis
```
