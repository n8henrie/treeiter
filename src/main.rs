#[derive(Debug)]
enum Tree<T> {
    Leaf(Vec<T>),
    Children(Vec<Tree<T>>),
}

impl<T: std::fmt::Debug> Tree<T> {
    fn iter(&self) -> impl Iterator<Item = &T> {
        fn traverse_depth<'b, U: std::fmt::Debug>(
            start: &'b Tree<U>,
            stack: &mut Vec<std::slice::Iter<'b, Tree<U>>>,
        ) -> Option<std::slice::Iter<'b, U>> {
            let mut node = start;
            loop {
                match *node {
                    Tree::Leaf(ref items) => break Some(items.iter()),
                    Tree::Children(ref children) => stack.push(children.iter()),
                }

                // stack is std::slice::Iter<'b, U> where U is the type of the ultimate value
                //
                // stack.last_mut() grabs the last iterator (e.g. an iterator of Tree::Leaf or
                // Tree::Children, not the Leaf or Children itself)
                //
                // unwrap() because last_mut returns an option
                //
                // next() returns the *first* / next Leaf or Children from that iterator
                //
                // On first loop:
                // - stack will be empty initially, but contain an iterator over the root Children
                // after the match statement above.
                // - node will be the full tree, but will be the first child from the stack
                // (`Leaf<Vec<1,2,3,4>`) after the line below
                //
                // On second loop:
                // - stack will still be iterator over the root Children
                // - return an iterator over items from the first Leaf
                node = stack.last_mut().unwrap().next()?;
            }
        }
        let mut stack = Vec::new();
        let mut leaf = traverse_depth(self, &mut stack);

        std::iter::from_fn(move || loop {
            if let Some(next) = leaf.as_mut()?.next() {
                break Some(next);
            }

            // Eventually we'll run out of values from the leaf, at which point we need to try to
            // get something from the stack. Here we put Children on the stack until we find a Leaf
            if let Some(next) = stack.last_mut()?.next() {
                leaf = traverse_depth(next, &mut stack);
            } else {
                // Unless there are no Leafs to be found, having gone all the way down, so we pop() and
                // now the stack.last_mut() will be the deepest remaining iterator
                stack.pop();
            }
        })
    }
}

fn main() {
    let tree = Tree::Children(vec![
        Tree::Leaf(vec![1u32, 2, 3, 4]),
        Tree::Leaf(vec![5, 6, 7, 8]),
        Tree::Children(vec![
            Tree::Leaf(vec![9, 10]),
            Tree::Leaf(vec![11, 12]),
            Tree::Children(vec![Tree::Leaf(vec![13, 14]), Tree::Leaf(vec![15, 16])]),
            Tree::Leaf(vec![17, 18]),
        ]),
        Tree::Leaf(vec![19, 20]),
    ]);
    dbg!(&tree);
    let res: Vec<_> = tree.iter().collect();
    dbg!(&res);
}

#[test]
fn test_1() {
    let tree = Tree::Children(vec![
        Tree::Leaf(vec![1u32, 2, 3, 4]),
        Tree::Leaf(vec![5, 6, 7, 8]),
        Tree::Children(vec![
            Tree::Leaf(vec![9, 10]),
            Tree::Leaf(vec![11, 12]),
            Tree::Children(vec![Tree::Leaf(vec![13, 14]), Tree::Leaf(vec![15, 16])]),
            Tree::Leaf(vec![17, 18]),
        ]),
        Tree::Leaf(vec![19, 20]),
    ]);
    let res: Vec<_> = tree.iter().copied().collect();
    assert_eq!(&res, &(1..=20).collect::<Vec<u32>>());
}
