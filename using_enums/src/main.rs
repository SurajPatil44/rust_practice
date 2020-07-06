enum IPAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IPAddr::V4(String::from("127.0.0.1"));
    let loopback = IPAddr::V6(String::from("::1"));
}
