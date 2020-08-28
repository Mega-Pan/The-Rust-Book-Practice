#![allow(unused_variables)]
#![allow(unused_parens)]

fn main(){
    // expression and statements
    let a = 3 + 7;
    let b = (3 + 7);
    let c = {3 + 7};
    let y = {
        let mut x = 3;
        x = x * 2;
        // the  is no ; as it is an expression
        x + 1
    };
    println!("The value of y is: {}", y);
}