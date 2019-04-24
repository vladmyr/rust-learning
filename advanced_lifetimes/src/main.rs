/**
 * Advanced features of lifetimes
 * - lifetime subtyping: ensure that one lifetime outlives another lifetime
 * - Lifetime bounds: specifies a lifetime for a reference to a generic type
 * -  Inference of trait object lifetimes: allows the compiler to infer trait
 * object lifetimes and when they need to be specified
 */

// *** Ensuring one lifetime outlives another with lifetime subtyping
struct Context<'s>(&'s str);

struct Parser<'c, 's: 'c> {
  context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
  fn parse(&self) -> Result<(), &'s str> {
    Err(&self.context.0[1..])
  }
}

fn parse_context(context: Context) -> Result<(), &str> {
  Parser { context: &context }.parse()
}

fn main() {
  
}
