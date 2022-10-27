use std::{fmt::{Debug, Formatter, Result}, collections::LinkedList};

pub struct Kiwi<T: Clone + Debug> {
    queue: LinkedList<T>
}

impl<T: Clone + Debug> Debug for Kiwi<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Kiwi")
         .field("queue", &self.queue)
         .finish()
    }
}

impl<T: Clone + Debug> Kiwi<T> {
    pub fn new() -> Self {
        Self {
            queue: LinkedList::new()
        }
    }

    pub fn enkiwi(&mut self, item: T) -> () {
        self.queue.push_back(item);
    }

    pub fn dekiwi(&mut self) -> () {
        self.queue.pop_front();
    }

    pub fn peak(&mut self) -> Option<&T> {
        self.queue.front()
    }
}

fn main() {
    let mut x: Kiwi<i32> = Kiwi::new();

    x.enkiwi(1);
    x.enkiwi(2);
    x.enkiwi(3);
    x.dekiwi();

    println!("{:?}", x.peak());
}
