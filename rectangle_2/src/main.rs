// METHOD SYNTAX
// methods are similar to functions except that they're defined
// within the context of a struct (or enum, or trait object)
// the first parameter is always self, which represents the
// instance of the struct the method is being called on

// DEFINING METHODS
// the following section references the previous project, rectangle
// lets change the area function (that takes Rectangle as a parameter) into an
// area method defined on the Rectangle struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50};
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    // to define the area function within the context of Rectangle, we start an impl
    // block (implementation) and move the area function within the scope of the 
    // implementation. The parameter is self. then in fn main(), when we want to call
    // for the area, we use method syntax (rect1.area()) in calling the funtions
    // without any parameters.

    // in the signatures for area, we use &self instead of rectangle: &Rectangle bc
    // rust knows that self is Rectangle due to the implemntation. methods can take 
    // ownership of self, borrow self immutably (like here), or borrow self mutably 

    // we chose &self bc we want to read the data in the struct, not write to it. If
    // we wanted to change the instance, we'd use &mut self. Using just self
    // (passing ownership) is rare and usually used when the method transforms self
    // into something else, and you want to prevent the caller from using the original
    // instance. 

    // METHODS WITH MORE PARAMETERS
    // lets try implementing a second method, one that will return true if a second
    // rectangle can fit inside the given instance (self)
    let rect1 = Rectangle {width: 30, height: 50};
    let rect2 = Rectangle {width: 10, height: 40};
    let rect3 = Rectangle {width: 60, height: 45};

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    // lets add an additional method to the implementation above that checks that the
    // width and height of self are larger than the other rectangle

    // ASSOCIATED FUNCTIONS
    // in impl blocks, we're allowed to define functions that don't take self as a
    // parameter. These are called associated functions. ex String::from
    // associated functions are often used for constructors that will return
    // a new instance of the struct. for example, taking a single imput to create a
    // square rectangle (instead of two identical inputs)
    // here, we place fn square into the impl block
    let sq = Rectangle::square(3);

    // multiple impl blocks are allowed for each struct; each method could have its own
    // impl. This is mostly used with generic types and traits

    // SUMMARY
    // structs let you create custom types that are meaningful to your domain
    // you can keep associated pieces of data connected and named
    // methods let you specify the behavior of your structs
    // associated functions let you namespace functionality that is particular
    // to your struct without having an instance available
}
