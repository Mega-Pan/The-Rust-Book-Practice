
// Functions can change things
// Passing data into function and changed

fn append_dot(t: &mut String){
    t.push('.');
}
// Don't forget to dereference with primitive types
fn plus_one_by_ref(n: &mut i32){
    *n = *n + 1;
}
fn main(){
    let mut s = String::from("Please add a dot");
    append_dot(&mut s);
    println!("s with dot = {}",s);

    let mut n: i32 = 19;
    plus_one_by_ref(&mut n);
    println!("n = {}", n);
}
