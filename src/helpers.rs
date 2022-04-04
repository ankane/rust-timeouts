use std::net::TcpListener;
use std::sync::Once;
use std::thread;

pub use std::time::Duration;

macro_rules! assert_timeout {
    ($code:expr, $exp:expr) => {
        let now = std::time::Instant::now();
        if let Err(error) = $code {
            let s = error.to_string();
            assert!(s.contains($exp), "{}", s);
        } else {
            panic!("No error");
        }
        let elapsed = now.elapsed().as_secs();
        assert!(elapsed >= 1 && elapsed < 2);
    };
}

static START: Once = Once::new();

pub fn setup_read() {
    START.call_once(|| {
        thread::spawn(|| {
            let listener = TcpListener::bind(read_host_and_port()).unwrap();
            for stream in listener.incoming() {
                thread::sleep(Duration::from_secs(3));
                stream.unwrap();
            }
        });
        // give time to start
        thread::sleep(Duration::from_millis(10));
    });
}

pub fn connect_host() -> String {
    "10.255.255.1".to_string()
}

pub fn connect_port() -> i32 {
    80
}

pub fn connect_host_and_port() -> String {
    format!("{}:{}", connect_host(), connect_port())
}

pub fn connect_url() -> String {
    format!("http://{}", connect_host_and_port())
}

pub fn read_host() -> String {
    "127.0.0.1".to_string()
}

pub fn read_port() -> i32 {
    4567
}

pub fn read_host_and_port() -> String {
    format!("{}:{}", read_host(), read_port())
}

pub fn read_url() -> String {
    format!("http://{}", read_host_and_port())
}
