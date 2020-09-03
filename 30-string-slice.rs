fn main(){
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    // s will be a &str that contains the first 4 bytes of the string
    // each of these characters was 2 bytes,
    // which means s will be "Зд"
    println!("&hello[0..4] is: {}",s);
    
}