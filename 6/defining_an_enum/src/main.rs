enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);

    let loopback = IpAddrKind::V6(String::from("::1"));

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

}
