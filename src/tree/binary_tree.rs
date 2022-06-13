use std::collections::VecDeque;

pub struct BinaryTree<T> {
    pub value: T,
    pub left: Option<Box<BinaryTree<T>>>,
    pub right: Option<Box<BinaryTree<T>>>,
}

impl<T: std::fmt::Display> BinaryTree<T> {
    // Constructor
    pub fn new(value: T) -> Self {
        BinaryTree {
            value,
            left: None,
            right: None,
        }
    }

    // Add a new left node to the current node
    pub fn left(mut self, node: BinaryTree<T>) -> Self {
        self.left = Some(Box::new(node));
        return self;
    }

    // Add a new right node to the current node
    pub fn right(mut self, node: BinaryTree<T>) -> Self {
        self.right = Some(Box::new(node));
        return self;
    }

    // Insert a new node and preserve branches lengths
    pub fn insert(&mut self, value: T) {
        let mut queue: VecDeque<&mut BinaryTree<T>> = VecDeque::new();
        queue.push_front(self);

        // Iterate until a new node is inserted
        loop {
            let BinaryTree { 
                ref mut left,
                ref mut right,
                ..
            } = queue.pop_back().unwrap();

            match left {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *left = Some(Box::new(BinaryTree::new(value)));
                    return ;
                }
            }

            match right {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *right = Some(Box::new(BinaryTree::new(value)));
                    return ;
                }
            }
        }
    }

    // pub fn recursive_print(&self, node: BinaryTree<T>, mut space: i32) {
    //     if self.right.is_none() && self.left.is_none() {
    //         return ;
    //     }

    //     space += 5;
    //     self.recursive_print(*node.right.unwrap(), space);

    //     println!();
    //     println!(" {}", self.value);
        
    //     self.recursive_print(*node.left.unwrap(), space);
    // }

    pub fn print(&self) {
        // self.recursive_print(self, 0);

        let mut tree = self.clone();

        let mut next = tree.right.value;
        // next = Some(Some(*next.value));
    }
}

