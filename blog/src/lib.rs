pub struct Post {
  // state: Option<Box<dyn State>>,
  content: String,
}

pub struct DraftPost {
  content: String,
}

impl Post {
  pub fn new() -> DraftPost {
    DraftPost {
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    &self.content
  }

  // pub fn new() -> Post {
  //   Post {
  //     state: Some(Box::new(Draft {})),
  //     content: String::new(),
  //   }
  // }

  // pub fn add_text(&mut self, text: &str) {
  //   self.state.as_ref().unwrap().add_text(&mut self.content, &text);
  // }

  // pub fn content(&self) -> &str {
  //   self.state.as_ref().unwrap().content(&self)
  // }

  // pub fn request_review(&mut self) {
  //   if let Some(s) = self.state.take() {
  //     self.state = Some(s.request_review())
  //   }
  // }

  // pub fn approve(&mut self) {
  //   if let Some(s) = self.state.take() {
  //     self.state = Some(s.approve())
  //   }
  // }

  // pub fn reject(&mut self) {
  //   if let Some(s) = self.state.take() {
  //     self.state = Some(s.reject())
  //   }
  // }
}

impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  pub fn request_review(self) -> PendingReviewPost {
    PendingReviewPost {
      content: self.content,
    }
  }
}

pub struct PendingReviewPost {
  content: String,
}

impl PendingReviewPost {
  pub fn approve(self) -> Post {
    Post {
      content: self.content
    }
  }
}

trait State {
  // NOTE: self: Box<Self> means the method is only valid when called on a Box
  // holding the type
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn reject(self: Box<Self>) -> Box<dyn State>;
  fn content<'a>(&self, _post: &'a Post) -> &'a str {
    ""
  }
  fn add_text(&self, _content: &mut String, _text: &str) {}
}

struct Draft {}

impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {})
  }

  fn reject(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn add_text(&self, content: &mut String, text: &str) {
    content.push_str(text);
  }
}

struct PendingReview {}

impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn reject(self: Box<Self>) -> Box<dyn State> {
    Box::new(Draft {})
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    Box::new(Published {})
  }
}

struct Published {}

impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn reject(self: Box<Self>) -> Box<dyn State> {
    Box::new(Draft {})
  }

  fn content<'a>(&self, post: &'a Post) -> &'a str {
    &post.content
  }
}
