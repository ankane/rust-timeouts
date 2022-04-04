// use unit tests (src)
// instead of integration tests (tests)
// so tests run in parallel

#[macro_use]
mod helpers;

mod std_tcpstream;

mod elasticsearch;
mod postgres;
mod redis;
mod reqwest;
mod sqlx;
mod ureq;
