// Creating Structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle{
    fn area(&self) -> u32 {
        &self.width * &self.height
    }
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        &self.width > &rectangle.width && &self.height > &rectangle.height
    }
} 

impl Rectangle{
    fn square(size : u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size,
        }
    }
} 

fn main() {

    // Instantiating Structs
    let mut rect = Rectangle{width: 10, height: 50};

    // Access Struct elements
    println!("Accessing element 'height': {}", rect.height);

    // Change Struct elements
    rect.height = 60;
    println!("Accessing element 'height' post-change: {}", rect.height);

    // Tuple Structs
    struct Color(u32, u32, u32);
    let black = Color(0, 0, 0);

    // Access smt in a t-struct
    println!("Accessing element t-struct: {}", black.1);

    println!("{:#?}", rect);

    let rect1 = Rectangle{width: 1, ..rect};
    println!("{:#?}", rect1);


    fn new_rect(width: u32, height: u32) -> Rectangle{
        Rectangle{
            width, 
            height,
        }
    }

    let rect2 = new_rect(5, 25);
    println!("{:#?}", rect2);
    println!("Area: {}", rect2.area());

    println!("Does rect2 fit into rect0? {}", rect.can_hold(&rect2));

    let square = Rectangle::square(10);
    println!("{:#?}", square);

}