// use unit tests (src)
// instead of integration tests (tests)
// so tests run in parallel

#[macro_use]
mod helpers;

mod std_tcpstream;

mod awc;
mod curl;
mod elasticsearch;
mod hyper;
mod postgres;
mod redis;
mod reqwest;
mod sqlx;
mod ureq;
