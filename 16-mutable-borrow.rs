fn main(){
    // mutable borrow
    let mut s1 = String::from("hello");
    let s2 = &mut s1;   // mutable borrow occures here
    s2.pust('!');
    println!("s2 = {}", s2);
    println!("s1 = {}", s1);

    s2.push('?');   // Wont Compile
}
