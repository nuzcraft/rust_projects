fn main() {
    // this program will calculate the area of a rectangle.
    // the first iteration will use single variables, then we'll refactor to use structs

    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is {} square pixels.", area(width1, height1));
    // width and height are related to each other because together, they describe a
    // rectangle. But the relationship isn't expressed anywhere.

    // let's try grouping width and height together into a tuple type
    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", tuple_area(rect1));
    // this is kinda better, because width and height are grouped, but we don't know 
    // which is which. Plus, in the area function, we need to use an index, which
    // can be confusing

    // lets try restructuring with structs, place a rectangle struct outside the 'main'
    // function to ensure it has the appropriate scope
    let rect1 = Rectangle {width: 30, height: 50};
    println!("The area of the rectangle is {} square pixels.", struct_area(&rect1));
    // we defined a Rectangle struct, and instantiated a new instance with width 30
    // and height 50. The area function takes a single rectangle parameter that is a 
    // reference to an existing Rectangle (so ownership is not passed to the function)
    // it accesses the width and height fields of the Rectangle instance to calculate
    // the area and is very clear

    // ADDING USEFUL FUNCTIONALITY WITH DERIVED TRAITS
    // it would be nice to be able to print an instance of Rectangle to see the 
    // values for the fields
    // println!("rect 1 is {}", rect1); // shocked...this doesn't work
    // by default the {} tells println! to use formatting known as Display: output
    // intended for end user consumption. With primitives, this is simple and Display is
    // set to default. But for structs, its less clear what should actually be printed

    // the error messages tells us to try to use :? or :#? with a format string
    // println!("rect 1 is {:?}", rect1); // this gives an error too
    // but it tells us to add #[derive(Debug)] to our implementation. Let's add it and 
    // try again.
    println!("rect 1 is {:?}", rect1);
    // woo, it worked! lets try to reformat it a bit
    println!("rect 1 is {:#?}", rect1);
    // this puts our fields in a vertical line for an easier time seeing them

    // there are many other traits that can be added to our custom types

    // this area function is nice, but it only works for rectangles, we should tie
    // it closer to the Rectangle struct. In the next setion, we'll make this 
    // an area method of the Rectangle type

}

// note: #[derive(Debug)] was added after line 38
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
