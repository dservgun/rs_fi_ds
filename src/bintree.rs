pub mod bintree {

    use std::fmt::Debug;

    pub enum RotationDirection {
        Left,
        Right,
        NoRotation,
    }

    ///
    /// ```use super::*;
    /// use bintree::*;
    /// #[test]
    /// fn test_basic_sort() {
    ///     let mut t = BinTree::new();
    ///     t.add_sorted(4);
    ///     t.add_sorted(5);
    ///     t.add_sorted(6);
    ///     t.add_sorted(10);
    ///     t.add_sorted(1);
    ///     t.add_sorted(94);
    ///     t.add_sorted(54);
    ///     t.add_sorted(3);
    ///     assert_eq!(t.max_value(), Some(94));
    /// }
    /// '''

    #[derive(Debug)]
    pub struct BinTree<T>(Option<Box<BinData<T>>>);

    #[derive(Debug)]
    pub struct BinData<T> {
        data: T,
        h: i8, // Could be boolean for Red Black.
        left: BinTree<T>,
        right: BinTree<T>,
    }

    impl<T> BinData<T> {
        pub fn rot_left(mut self) -> Box<Self> {
            let mut res = match self.right.0 {
                Some(res) => res,
                None => return Box::new(self),
            };
            self.right = BinTree(res.left.0.take());
            self.right.set_height();
            res.left = BinTree(Some(Box::new(self)));
            res.left.set_height();
            res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
            res
        }

        pub fn rot_right(mut self) -> Box<Self> {
            let mut res = match self.left.0 {
                Some(res) => res,
                None => return Box::new(self),
            };
            self.left = BinTree(res.right.0.take());
            self.left.set_height();
            res.right = BinTree(Some(Box::new(self)));
            res.right.set_height();
            res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
            res
        }

        pub fn max_value(self) -> Option<T> {
            match self.right.0 {
                Some(x) => {
                    return x.max_value();
                }
                None => {
                    return Some(self.data);
                }
            }
        }
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

        pub fn rot_left(&mut self) {
            self.0 = self.0.take().map(|v| v.rot_left());
        }

        pub fn rot_right(&mut self) {
            self.0 = self.0.take().map(|v| v.rot_right());
        }

        pub fn max_value(&mut self) -> Option<T> {
            match self.0.take() {
                Some(v) => v.max_value(),
                None => None,
            }
        }
    }

    impl<T: PartialOrd> BinTree<T> {
        pub fn add_sorted(&mut self, data: T) {
            let rot_dir = match self.0 {
                Some(ref mut bd) => {
                    if data < bd.data {
                        bd.left.add_sorted(data);
                        if bd.left.height() - bd.right.height() > 1 {
                            RotationDirection::Left
                        } else {
                            RotationDirection::NoRotation
                        }
                    } else {
                        bd.right.add_sorted(data);
                        if bd.right.height() - bd.left.height() > 1 {
                            RotationDirection::Right
                        } else {
                            RotationDirection::NoRotation
                        }
                    }
                }
                None => {
                    self.0 = Some(Box::new(BinData {
                        data,
                        h: 0,
                        left: BinTree::new(),
                        right: BinTree::new(),
                    }));
                    RotationDirection::NoRotation
                }
            };
            match rot_dir {
                RotationDirection::Left => self.rot_right(),
                RotationDirection::Right => self.rot_left(),
                RotationDirection::NoRotation => self.set_height(),
            }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use bintree::*;
    #[test]
    fn test_basic_sort() {
        let mut t = BinTree::new();
        t.add_sorted(4);
        t.add_sorted(5);
        t.add_sorted(6);
        t.add_sorted(10);
        t.add_sorted(1);
        t.add_sorted(94);
        t.add_sorted(54);
        t.add_sorted(3);
        assert_eq!(t.max_value(), Some(94));
    }
}
