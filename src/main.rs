#![warn(clippy::pedantic)]
use treeiter::Tree;

fn main() {
    let tree = Tree::Children(vec![
        Tree::Leaf(vec![1u32, 2, 3, 4]),
        Tree::Leaf(vec![5, 6, 7, 8]),
        Tree::Children(vec![
            Tree::Leaf(vec![9, 10]),
            Tree::Leaf(vec![11, 12]),
            Tree::Children(vec![
                Tree::Leaf(vec![13, 14]),
                Tree::Leaf(vec![15, 16]),
            ]),
            Tree::Leaf(vec![17, 18]),
        ]),
        Tree::Leaf(vec![19, 20]),
    ]);
    dbg!(&tree);
    let res: Vec<_> = tree.iter().collect();
    dbg!(&res);
}
