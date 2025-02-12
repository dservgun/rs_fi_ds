use std::fmt::Debug;
mod bond;
mod interest_rate_swap;
mod tbills;

#[derive(Debug)]
pub struct BinTree<T>(Option<Box<BinData<T>>>);

// height how many nodes are below a node.

#[derive(Debug)]
pub struct BinData<T> {
    data: T,
    h: i8, // Could be boolean for Red Black.
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }
    pub fn height(&self) -> i8 {
        match self.0 {
            Some(ref t) => t.h,
            None => 0,
        }
    }
    pub fn set_height(&mut self) {
        if let Some(ref mut t) = self.0 {
            t.h = 1 + std::cmp::max(t.left.height(), t.right.height());
        }
    }
}

impl<T: PartialOrd> BinTree<T> {
    pub fn add_sorted(&mut self, data: T) {
        match self.0 {
            Some(ref mut bd) => {
                if data < bd.data {
                    bd.left.add_sorted(data);
                } else {
                    bd.right.add_sorted(data);
                }
            }
            None => {
                self.0 = Some(Box::new(BinData {
                    data,
                    h: 0,
                    left: BinTree::new(),
                    right: BinTree::new(),
                }));
            }
        }
        self.set_height();
    }
}

impl<T: Debug> BinTree<T> {
    pub fn print_lfirst(&self, dp: i32) {
        if let Some(ref bd) = self.0 {
            bd.left.print_lfirst(dp + 1);
            let mut spc = String::new();
            for _ in 0..dp {
                spc.push('.');
            }
            println!("{} : {}{:?}", bd.h, spc, bd.data);
            bd.right.print_lfirst(dp + 1);
        }
    }
}

#[allow(dead_code)]
fn main() {
    let mut t = BinTree::new();
    t.add_sorted(4);
    t.add_sorted(3);
    t.add_sorted(5);
    t.add_sorted(1);
    t.add_sorted(6);
    t.add_sorted(10);
    t.add_sorted(54);
    t.add_sorted(94);
    t.print_lfirst(0);
}

fn main_old() {}
