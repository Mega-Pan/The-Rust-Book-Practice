enum Foo {A(i32),B,C}

let foo = match x{
    Foo::A(n) => n, 
    Foo::B => {},
    _ => {}    //anything else
};