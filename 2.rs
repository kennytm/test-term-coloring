use std::ops::Range;

trait Haystack: Sized  {
    type Hay: Hay + ?Sized;
}

trait Hay {
    type Index: Copy;
}

#[derive(Clone)]
struct Shared<H: Haystack> {
    a: H,
    b: Range<<(H::Hay) as Hay>::Index>,
}

impl<H: Haystack + Clone> Shared<H> {
    fn c(&self) -> Self {
        Self::clone(self)
    }
}

fn main() {}
