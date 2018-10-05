
// the struct is defined in this scope so it can be used by multiple functions
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // USING STRUCTS TO STRUCTURE RELATED DATA
    // a struct or structure is a custom data type used to name and package values
    // into a meaningfull group.

    // DEFINING AND INSTANTIATING STRUCTS
    // structs are similar to tuples; the pieces of a struct can be different types
    // unlike tuples, we name each pieces of data so its clear what the values mean
    // structs are more flexible than tuples: we don't have to rely on the order
    // of the data to specify or access the values
    //  when defining a struct, the struct needs to be names something significant; 
    // the pieces (fields) are then named as well
    // struct User {
    //     username: String,
    //     email: String,
    //     sign_in_count: u64,
    //     active: bool,
    // }
    // to use a struct after its been defined, we create an instance of it by
    // specifying values for each of the fields, but we don't have to do it in the same
    // order. The struct definition is like a template, and instances fill that template
    // with the specific data they need
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // to get a specific value from a struct, we can use dot notation, aka user1.email
    // if we set the instance to mutable, we can change values this way as well
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    // it should be noted that the entire instance must be mutable; Rust doesn't allow for
    // individual fields to be marked as immutable.

    // we can construct a new instance as the last expression in a function to implicitly
    // returns that instance
    let user2 = build_user(String::from("athirdemail@example.com"), String::from("anotheruser345"));
    let user3 = build_user_shorthand(String::from("afourthemail@example.com"), String::from("anotheruser345"));

    // CREATING INSTANCES FROM OTHER INSTANCES WITH STRUCT UPDATE SYNTAX
    // we'll often want to create a new instance of a struct that uses most of an old
    // instance's values but changes some. we can use struct update syntax
    // let's create a new User in user2 without the struct update syntax
    let user2 = User {
        email: String::from("anotheremail@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    // using struct update syntax, we can achieve the same effect with less code
    let user2 = User {
        email: String::from("anotheremail@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    // TUPLE STRUCTS WITHOUT NAMED FIELDS TO CREATE DIFFERENT TYPES
    // you can define structs that look similar to tuples, called tuple structs. Tuple
    // structs have the added meaning the struct name provides, but don't have names 
    // associated with fields; the fields are just typed. This is useful when you want
    // to give a tuple a name to differentiate from other tuples
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // each struct defined is its own type, even if the field types are the same. We can
    // destructure tuple structs with a . followed by the index to access a specific falue

    // UNIT LIKE STRUCTS WITHOUT ANY FIELDS
    // We can define structs without any fields. These are called unit-like structs because
    // they behave similarly to (), the unit type. Unit-like structs are useful when you 
    // want to implement a trait on a type, but don't have any data to store in the type
    // itself.

    // OWNERSHIP OF STRUCT DATA
    // in the User struct definition, we used the owned String type instead of the &str
    // string slice type. We want instances of the struct to own all of its data and for
    // that data to be valid for the life of the struct
    // it's possible to store references of data, but requires the use of lifetimes
    // struct User {
    //     username: &str,
    //     email: &str,
    //     sign_in_count: u64,
    //     active: bool,
    // }
    // this will fail because a lifetime parameter is expected

}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
    // having to repeat the email and username field names is a bit tedious, can we do
    // this better?
}

fn build_user_shorthand (email: String, username: String) -> User {
    User {
        email, // this works because the parameter and name are the same
        username,
        active: true,
        sign_in_count: 1,
    }
}