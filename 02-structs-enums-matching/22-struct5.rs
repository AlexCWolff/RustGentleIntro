// struct5.rs

/* A powerful technique is a struct that contain references to itself. 

// The basic building block of a binary tree, expressed in C
struct Node {
    const char *payload;
    struct Node *left;
    struct Node *right;
};
    
You can not do this by directly including 'Node' fields because then the size of 'Node' depends on the size of 'Node'. Instead we use pointers to Node structs, since the size of a pointer is always known. If 'left' isn't 'NULL', the Node will have a left pointing to another node, and so on indefinitely.

Rust does not do NULL, so instead we use 'Option', but you cannot just put a 'Node' in that 'Option' because we don't know the size of 'Node'. For this we use Box since it contains an allocated pointer to the data and always has a fixed size. */

// The Rust equivalent, using 'type' to create an alias
type NodeBox = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    payload: String,
    left: NodeBox,
    right: NodeBox
}
// No need for forward declarations.

// A test program
impl Node {
    fn new(s: &str) -> Node {
        Node{payload: s.to_string(), left: None, right: None}
    }

    fn boxer(node: Node) -> NodeBox {
        Some(Box::new(node))
    }

    fn set_left(&mut self, node: Node) {
        self.left = Self::boxer(node);
    }

    fn set_right(&mut self, node: Node) {
        self.right = Self::boxer(node);
    }

}

fn main() {
    let mut root = Node::new("root");
    root.set_left(Node::new("left"));
    root.set_right(Node::new("right"));

    println!("arr {:#?}", root);
}

/* The strings that are less than other strings get put down the left side, otherwise the right side. The output is pretty thanks to "{:#?}" ('#' means 'extended').

root Node {
    payload: "root",
    left: Some(
        Node {
            payload: "left",
            left: None,
            right: None
        }
    ),
    right: Some(
        Node {
            payload: "right",
            left: None,
            right: None
        }
    )
}
*/

/* When 'root' is dropped, all fields are dropped; if the "branches" of the tree are dropped they drop their fields, and so on. 'Box::new' may be the closest you will get to a 'new' keyword, but we have no need for 'delete' or 'free'. */

// A method which inserts nodes in lexical (i.e. alphabetical) order
fn insert(&mut self, data: &str) {
    // Compare the new data to the current node to see if it's less
    if data < &self.payload {
        // Try to insert on the left
        // Note the match
        match self.left {
            // If the 'Option' is 'Some', pull out a mutable reference to the box and apply 'insert'
            Some(ref mut n) => n.insert(data),
            // If the 'Option' is 'None' there is no node on the left, so 'set_left'
            None => self.set_left(Self::new(data)),
        }
    } else { 
        // Otherwise, try to insert on the right
        match self.right {
            Some(ref mut n) => n.insert(data),
            None => self.set_right(Self::new(data)),
        }
    }
}

...
fn main() {
    let mut root = Node::new("root");
    root.insert("one");
    root.insert("two");
    root.insert("four");

    println!("root {:#?}", root);
}

/* 'Box' is a smart pointer; no 'unboxing' was needed to call 'Node' methods. Here's the output tree:

root Node {
    payload: "root",
    left: Some(
        Node {
            payload: "one",
            left: Some(
                Node {
                    payload: "four",
                    left: None,
                    right: None
                }
            ),
            right: None
        }
    ),
    right: Some(
        Node {
            payload: "two",
            left: None,
            right: None
        }
    )
}
 */

/* We can we're visiting the strings in order with in-order traversal; we visit the left, do something on the node, and then visit the right. */

fn visit(&self) {
    // Note the reappearance of 'ref', 'if let' uses the same rules as 'match'
    if let Some(ref left) = self.left {
        left.visit();
    }
    println!("'{}'", self.payload);
    if let Some(ref right) = self.right {
        right.visit();
    }
}
...
...
root.visit();
// 'four'
// 'one'
// 'root'
// 'two'