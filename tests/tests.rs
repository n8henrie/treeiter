#![warn(clippy::pedantic)]
use treeiter::Tree;

#[test]
fn it_works() {
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
    let res: Vec<_> = tree.iter().copied().collect();
    assert_eq!(&res, &(1..=20).collect::<Vec<u32>>());
}
