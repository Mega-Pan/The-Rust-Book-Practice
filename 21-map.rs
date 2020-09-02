fn add_four(x:i32)->(i32){
    x+4
}

fn maybe_add_four(y: Option<i32> -> Option(i32)){
    match y {
        Some(yy)=>Some(add_four(yy)),
        None => None,
    }
    // euqals
    // y.map(add_four);
    // or,
    // y.map(|x| x+4)
}