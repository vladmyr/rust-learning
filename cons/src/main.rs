use std::rc::Rc;
use std::cell::RefCell;

enum ListBox {
    Cons(i32, Box<ListBox>),
    Nil,
}

enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil
}

#[derive(Debug)]
enum ListRcRefCell {
    Cons(Rc<RefCell<i32>>, Rc<ListRcRefCell>),
    Nil
}

#[derive(Debug)]
enum ListRefCellRc {
    Cons(i32, RefCell<Rc<ListRefCellRc>>),
    Nil,
}

impl ListRefCellRc {
    fn tail(&self) -> Option<&RefCell<Rc<ListRefCellRc>>> {
        match self {
            ListRefCellRc::Cons(_, item) => Some(item),
            ListRefCellRc::Nil => None,
        }
    }
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

    // example of having multiple owners of mutable data with ListRcRefCell
    println!("*** Multiple owners of mutable data");
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(ListRcRefCell::Cons(Rc::clone(&value), Rc::new(ListRcRefCell::Nil)));
    let b = ListRcRefCell::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = ListRcRefCell::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // reference cycle
    println!("*** Reference cycle");
    let a = Rc::new(ListRefCellRc::Cons(5, RefCell::new(Rc::new(ListRefCellRc::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(ListRefCellRc::Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // ERROR: stack overflow
    // println!("a next item = {:?}", a.tail());
}
