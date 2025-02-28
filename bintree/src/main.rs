use crate::bintree::bintree::BinTree;
mod bintree;
mod bond;
mod interest_rate_swap;
mod pandl;
mod tbills;

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
