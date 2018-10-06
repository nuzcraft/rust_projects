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
}
