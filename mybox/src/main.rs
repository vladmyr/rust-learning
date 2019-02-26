use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data)
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let a = 5;
    let b = &a;
    let c = Box::new(a);
    let d = MyBox::new(a);

    assert_eq!(5, a);
    assert_eq!(5, *b);
    assert_eq!(5, *c);
    assert_eq!(5, *d);      // *d = *(y.deref()) is what actually being done

    // Deref cerction
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);
    // (*m): MyBox<String> -> String
    // [..]: String -> str
    // &: str -> &str

    // cleanup with Drop trait
    let e = CustomSmartPointer { data: String::from("my stuff") };
    let f = CustomSmartPointer { data: String::from("other stuff") };
    let g = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointers created.");
    drop(g);
    println!("^ CustomSmartPointer dropped before the end of function scope.");
}
