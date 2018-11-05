// *** v1
// enum consists of "variants"
// enum IpAddrKind {
//     V4,
//     V6
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String
// }

// *** v2
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn main() {
    // *** v1
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1")
    // };

    // *** v2
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    if let IpAddr::V4(a, b, c, d) = home {
        println!("ip v4 address is {}.{}.{}.{}", a, b, c, d);
    }

    if let IpAddr::V6(a) = loopback {
        println!("ip v6 address is {}", a);
    }
}
