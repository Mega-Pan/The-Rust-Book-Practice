fn main(){
    let s1 = String::from("hello");
    let s2 = &s1;
    let s3 = &s2;
    println!("s1 = {}, s2 = {}",s1,s2);
    println!("s3 = {}, with len = {}",s3,s3.len());
}
