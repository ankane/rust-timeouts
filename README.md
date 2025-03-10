# Rust Timeouts

An unresponsive service can be worse than a down one. It can tie up your entire system if not handled properly. **All network requests should have a timeout.**

Here’s how to add timeouts for popular Rust crates. **[All have been tested](src)**. The default is no timeout, unless otherwise specified. Enjoy!

Also available for [Ruby](https://github.com/ankane/the-ultimate-guide-to-ruby-timeouts), [Python](https://github.com/ankane/python-timeouts), [Node](https://github.com/ankane/node-timeouts), [Go](https://github.com/ankane/go-timeouts), and [PHP](https://github.com/ankane/php-timeouts)

[![Build Status](https://github.com/ankane/rust-timeouts/actions/workflows/build.yml/badge.svg)](https://github.com/ankane/rust-timeouts/actions)

## Contents

Standard library

- [TcpStream](#tcpstream)

Crates

- [awc](#awc)
- [curl](#curl)
- [elasticsearch](#elasticsearch)
- [hyper](#hyper)
- [postgres](#postgres)
- [redis](#redis)
- [reqwest](#reqwest)
- [sqlx](#sqlx)
- [ureq](#ureq)

## Standard Library

### TcpStream

```rust
let mut stream = std::net::TcpStream::connect_timeout(&addr, Duration::from_secs(1))?;
stream.set_read_timeout(Some(Duration::from_secs(1)))?;
stream.set_write_timeout(Some(Duration::from_secs(1)))?;
```

Returns `std::io::Error`

## Crates

### awc

```rust
let client = awc::Client::builder()
    .timeout(Duration::from_secs(1))
    .finish();
```

Returns `awc::error::SendRequestError`

### curl

```rust
let mut easy = curl::easy::Easy::new();
easy.connect_timeout(Duration::from_secs(1))?;
easy.timeout(Duration::from_secs(1))?;
```

Returns `curl::Error`

### elasticsearch

```rust
let transport = elasticsearch::http::transport::TransportBuilder::new(conn_pool)
    .timeout(Duration::from_secs(1))
    .build()?;
```

Returns `elasticsearch::Error`

### hyper

```rust
tokio::time::timeout(Duration::from_secs(1), client.get(uri)).await?;
```

Returns `tokio::time::error::Elapsed`

### postgres

```rust
let client = postgres::Config::new()
    .connect_timeout(Duration::from_secs(1))
    .connect(postgres::NoTls)?;
```

Returns `postgres::Error`

### redis

```rust
let mut con = client.get_connection_with_timeout(Duration::from_secs(1))?;
con.set_read_timeout(Some(Duration::from_secs(1)))?;
con.set_write_timeout(Some(Duration::from_secs(1)))?;
```

Returns `redis::RedisError`

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

Returns `reqwest::Error`

### sqlx

```rust
let pool = sqlx::postgres::PgPoolOptions::new()
    .acquire_timeout(Duration::from_secs(1))
    .connect(uri)
    .await?;
```

Returns `sqlx::Error`

### ureq

```rust
let config = ureq::Agent::config_builder()
    .timeout_global(Some(Duration::from_secs(1)))
    .build();
```

Returns `ureq::Error`

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
