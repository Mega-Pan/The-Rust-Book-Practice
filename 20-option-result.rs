enum Option<T> {
    Some(T),
    None,
}
enum Result<T,E>{
    OK(T),
    Err(E),
}