use std::rc::Rc;

enum ListBox {
    Cons(i32, Box<ListBox>),
    Nil,
}

enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil
}

fn main() {
    let list = ListBox::Cons(1, 
        Box::new(ListBox::Cons(2, 
            Box::new(ListBox::Cons(3, 
                Box::new(ListBox::Nil)
            ))
        ))
    );

    // attempt to use Box to share data
    let a = ListBox::Cons(5, Box::new(ListBox::Cons(10, Box::new(ListBox::Nil))));
    let b = ListBox::Cons(3, Box::new(a));      // Error: a moved here
    // let c = Cons(4, Box::new(a));   // Error: used a moved value "a"

    // using Rc to share data
    let a = Rc::new(ListRc::Cons(5, Rc::new(ListRc::Cons(10, Rc::new(ListRc::Nil)))));
    let b = ListRc::Cons(3, Rc::clone(&a));     // reference count is 2
    let c = ListRc::Cons(3, Rc::clone(&a));     // reference count is 3
    // NOTE: Rc::clone does not create a deep copies of passed data references

    // reference counting demo
    let a = Rc::new(ListRc::Cons(5, Rc::new(ListRc::Cons(10, Rc::new(ListRc::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = ListRc::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = ListRc::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
