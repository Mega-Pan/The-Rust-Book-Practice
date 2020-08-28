fn main(){
    let howdy = "Howdy👍"; // 5 long?
    println!("Some str stuff: {} {}", howdy.len(), howdy.is_empty());
    println!("The bytes of howdy👍: {:?}",howdy.as_bytes());
    //string 
    let mut hello = String::from("Hello ");
    hello.push('w');
    hello.push_str("orld");
    println!("string: {}",hello);
    hello.insert(5,',');
    println!("String: {}",hello);
}