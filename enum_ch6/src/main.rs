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
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    // the struct IpAddr has a kind field that is of type IpAddrKind and address
    // home has a kind of V4, and loopback has a kind of V6

    // We can try to do this in a more concise way using just an enum
    enum IpAddrEnum {
        V4(String),
        V6(String),
    }
    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback = IpAddrEnum::V6(String::from("::1"));
    // We can attach data to each variant of the enum directly, so there is no need for
    // the extra struct
    
    // Additionaly, each variant can have different types and amounts of associated data.
    // V4 IP Addresses, will always have 4 numeric components. We can store V4 addresses as
    // 4 u8 values, while keeping V6 as a string
    enum IpAddrEnumVariant {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddrEnumVariant::V4(127, 0, 0, 1);
    let loopback = IpAddrEnumVariant::V6(String::from("::1"));

    // the standard library defines IpAddr as an enum, and each variant has a specific
    // struct associated with it
    struct Ipv4Addr{
        // --snip--
    }
    struct IpV6Addr {
        // --snip --
    }
    enum IpAddrStandard {
        V4(Ipv4Addr),
        V6(IpV6Addr),
    }
    // even though the standard library contains a definithion for IpAddr, we can still
    // create our own definition because we haven't brought the standard library's 
    // definiton into our scope

    // let's check out aonther enum
    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // there are 4 variants here: Quit has no data, Move includes an anonymous struct,
    // Write has a single string, and ChangeColor has three i32 values

    // this is similar to defining different struct definitions, but the variants are
    // grouped under the message type. The following structs could hold the same data
    // as the preceding enum variants:
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct
    // if we used different structs, which each have their own type, we couldn't as 
    // easily define a function to take any of these kinds of messages as we could with
    // the Message enum, as it is a single type

    // just as we can define methods on structs using impl, we can define methods on enums
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

    // THE OPTION ENUM AND ITS ADVANTAGES OVER NULL VALUES
    // from the standard library, the Option enum is used in many places because it encodes
    // the very common scenario in which a value could be something or it could be nothing
    // expressing this concept in terms of the type system means the compiler can check 
    // whether you've handled all the cases you should be handling; this functionality can
    // prevent bugs that are extremely common in other languages

    // Rust doesn't have the null feature.

    // Instead, Rust has an enum that can encode the concept of a value being present or
    // absent. This enum is Option<T> as defined by the standard library:
    enum OptionExample<T> {
        Some(T),
        None,
    }
    // Option<T> is so useful that it's included in the prelude, and doesn't need to be
    // brought into scope. In addition, so are its variants: you can use Some and None
    // directly without the Option:: prefix
    // <T> syntax is a generic type parameter. The Some variant of Option can hold one
    // piece of any data. Examples:
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    // if we use None rather than Some, we need to tell Rust what type of Option<T> we
    // have, because the compiler can't infer the type that the Some variant will hold
    // by looking only at the None value
    // Some means the value is present and held in Some, None means the same as null, so
    // why is this better than null?

    // Because Option<T> and T are differnt types, the compiler won't let us us and
    // Option<T> value as if it were a definitely valid value
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // this will fail because Rust doesn't know how to add the different
    // types together

    // when we have a value of a type like i8, the compiler will ensure that we always have 
    // a valid value. Only when we have an Option<i8> do we have to worry about possibly
    // not having a vlue, and the compiler will make sure we handle that case before
    // using the value.
    // You must convert Option<T> to T before you can perform T operations with it.
    // Values can only be null if 'opt-in'ed by making the type of the value Option<T>, 
    // then when the value is used, None must be handled. Everywhere else can be safely
    // assumed to be not null

    // Option<T> has a large number of methods (https://doc.rust-lang.org/std/option/enum.Option.html)
    // that can be used, including getting T out of a Some variant. Generally, you need to 
    // code what will happen with each variant. The match expression is a control flow
    // ocnstruct that does just this when used with enums: it will run different code
    // depending on which variant of the enum it has, and that code can use the data inside
    // the matching value



}
