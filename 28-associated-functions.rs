#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // this is a function, not method 
    // call by '::'
    fn square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            height: size,  
        }
    }
}
fn main(){
    let sq = Rectangle::square(3);
    println!(
        "The area of the rectangle is {} square pixels.",
        sq.area()
    );
}