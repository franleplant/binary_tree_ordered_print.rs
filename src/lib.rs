use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    pub data: u32,
    pub left: Option<Rc<Node>>,
    pub right: Option<Rc<Node>>,
}

impl Node {
    pub fn new(data: u32) -> Node {
        Node {
            data: data,
            left: None,
            right: None,
        }
    }

    pub fn left(mut self, node: Rc<Node>) -> Node {
        self.left = Some(node);
        self
    }

    pub fn right(mut self, node: Rc<Node>) -> Node {
        self.right = Some(node);
        self
    }

    pub fn to_ptr(self) -> Rc<Node> {
        Rc::new(self)
    }
}

enum NodeOrValue {
    Node(Rc<Node>),
    Value(u32),
}

pub fn print_in_order(node: Rc<Node>) {
    let mut stack: Vec<NodeOrValue> = vec![NodeOrValue::Node(node)];

    while let Some(top) = stack.pop() {
        //println!("Iteration");
        match top {
            NodeOrValue::Node(node) => {
                if let &Some(ref right) = &node.right {
                    stack.push(NodeOrValue::Node(right.clone()));
                }

                stack.push(NodeOrValue::Value(node.data));

                if let &Some(ref left) = &node.left {
                    stack.push(NodeOrValue::Node(left.clone()));
                }

            },
            NodeOrValue::Value(value) => {
                println!("{}", value);
            }
        }
    }

}

#[test]
fn it_works() {
    let tree = Node::new(5)
        .left(
            Node::new(3)
                .left(Node::new(2).to_ptr())
                .right(Node::new(4).to_ptr())
                .to_ptr()
        )
        .right(
            Node::new(7)
                .left(Node::new(6).to_ptr())
                .right(Node::new(8).to_ptr())
                .to_ptr()
        )
        .to_ptr();

    println!("{:?}", tree);
    println!("Ordered lineal print");
    print_in_order(tree);

}
