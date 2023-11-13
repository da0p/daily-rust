fn main() {
    // types implement From and Into to facilitate type conversions
    let s = String::from("hello");
    let addr = std::net::IpAddr::from([127, 0, 0, 1]);
    let one = i16::from(true);
    let bigger = i32::from(123i16);
    println!("{s}, {addr}, {one}, {bigger}");

    let s2: String = "hello".into();
    let addr2: std::net::Ipv4Addr = [127, 0, 0, 1].into();
    let one2: i16 = true.into();
    let bigger2: i32 = 123i16.into();
    println!("{s2}, {addr2}, {one2}, {bigger2}");
}
