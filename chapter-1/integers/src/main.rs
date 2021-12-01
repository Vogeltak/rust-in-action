use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    // Stack-allocated
    let a = 10;
    // Heap-allocated
    let b = Box::new(20);
    // Boxed integer wrapped within a **reference counter**
    let c = Rc::new(Box::new(30));
    // Integer wrapped in an atomic ref. counter and
    // protected by a mutual exclusion lock
    let d = Arc::new(Mutex::new(40));

    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
}
