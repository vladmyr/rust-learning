use blog::Post;

fn main() {
    // let mut post = Post::new();
    
    // post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content());

    // post.request_review();
    // assert_eq!("", post.content());

    // post.add_text("This text goes to void");
    // assert_eq!("", post.content());

    // post.approve();
    // assert_eq!("I ate a salad for lunch today", post.content());

    // post.add_text("Another text that goes to void");
    // assert_eq!("I ate a salad for lunch today", post.content());

    // post.reject();
    // assert_eq!("", post.content());

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
