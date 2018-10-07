fn main() {
    // DEFINING AN ENUM
    // suppose we need to work with IP addresses. There are two standards version 4
    // and version 6. An IP address must be one, but not both. We can enumerate all
    // possible value. (where enum comes from)
    // both v4 and v6 are fundamentally IP addresses, so they should be treated as 
    // the same type when the code is handling situations that apply to any kind of
    // IP address

    // define an IpAddrKind enumeration and list the possible kinds (variants), V4, V6
    enum IpAddrKind {
        V4,
        V6,
    }
    // IpAddrKind is now a custom data type we can use elsewhere in our code

    // ENUM VALUES
    // we can create instances of each of the two variants
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // this is important, because the enum is namespaced under its identifier. Both
    // types are housed under the same type: IpAddrKind, which means we can define a 
    // function that takes any kind of IpAddrKind
    fn route(ip_type: IpAddrKind) {}
    // we can call this with either variant
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // at the moment, we don't have a way to store the actual IP address data; we only
    // know what kind it is. given what we learned about structs, we might try this:
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

}
