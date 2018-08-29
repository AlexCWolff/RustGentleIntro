// struct6.rs

/* It would be annoying to have to rewrite the binary tree for all possible kinds of payload, so here's s generic 'Node' with type parameter 'T'. */

type NodeBox<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    payload: T,
    left: NodeBox<T>,
    right: NodeBox<T>
}

/* The implementation shows how different Rust is from other languages. The fundamental operation on the payload is comparison, so 'T' must be comparable with '<', i.e. it implements 'PartialOrd'. */

// The type parameter must be declared in the 'impl' block with its constraints.
impl <T: PartialOrd> Node<T> {
    fn new(s: T) -> Node<T> {
        Node{payload: s, left: None, right: None}
    }

    fn boxer(node: Node<T>) -> NodeBox<T> {
        Some(Box::new(node))
    }

    fn set_left(&mut self, node: Node<T>) {
        self.left = Self::boxer(node);
    }

    fn set_right(&mut self, node: Node<T>) {
        self.right = Self::boxer(node);
    }

    fn insert(&mut self, data: T) {
        if data < self.payload {
            match self.left {
                Some(ref mut n) => n.insert(data),
                None => self.set_left(Self::new(data)),
            }
        } else {
            match self.right {
                Some(ref mut n) => n.insert(data),
                None => self.set_right(Self::new(data)),
            }
        }
    }
}


fn main() {
    let mut root = Node::new("root".to_string());
    root.insert("one".to_string());
    root.insert("two".to_string());
    root.insert("four".to_string());

    println!("root {:#?}", root);
}

/* Generic structs need their type parameter(s) specified in angle brackets like C++. Rust is usually smart enough to work out that type parameter from context; it knows it has a 'Node<T>', and knows that its 'insert' method is passed 'T'. The first call of 'insert' sets 'T' to 'String'. Any further calls are that aren't 'String' will cause an error. Still, you need to constrain that type appropriately. */