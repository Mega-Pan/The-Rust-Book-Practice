// in C code
// void f(Foo* ptr){
//      if (!ptr){
//          return;    
//      }   
//  ptr->g();
//}

// What could go wrong?
// - 1. Forget the null check
// - 2. ptr could be mutated
// - 3. ptr could be invalit

fn f(ptr: Option<&Foo>){
    match ptr{  
        Some(ptr) => ptr.g(),
        None => {}      
    }
}
