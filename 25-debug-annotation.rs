#[derive(Debug)]

struct Rectangle {
    width: i32,
    height: i32,
}

fn main(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!{"recl1 is {:?}", rect1};
}