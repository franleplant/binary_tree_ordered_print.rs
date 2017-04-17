

#[derive(Debug, Clone)]
struct Node {
    data: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(data: u32) -> Node {
        Node {
            data: data,
            left: None,
            right: None,
        }
    }

    fn left(mut self, node: Box<Node>) -> Node {
        self.left = Some(node);
        self
    }

    fn right(mut self, node: Box<Node>) -> Node {
        self.right = Some(node);
        self
    }

    fn to_box(self) -> Box<Node> {
        Box::new(self)
    }
}

enum NodeOrValue {
    Node(Box<Node>),
    Value(u32),
}

//TODO
//- avoid cloning
fn print_in_order(node: Box<Node>) {
    let mut stack: Vec<NodeOrValue> = vec![NodeOrValue::Node(node)];

    while stack.len() != 0 {
        let top = stack.pop().expect("Poping an empty stack");
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
    let tree = Box::new(Node {
        data: 5,
        left: Some(Box::new(Node {
            data: 3,
            left: Some(Box::new(Node {
                data: 2,
                left: None,
                right: None,
            })),
            right: Some(Box::new(Node {
                data: 4,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(Node {
            data: 7,
            left: Some(Box::new(Node {
                data: 6,
                left: None,
                right: None,
            })),
            right: Some(Box::new(Node {
                data: 8,
                left: None,
                right: None,
            })),
        })),
    });

    let tree2 = Node::new(5)
        .left(
            Node::new(3)
                .left(Node::new(2).to_box())
                .right(Node::new(4).to_box())
                .to_box()
        )
        .right(
            Node::new(7)
                .left(Node::new(6).to_box())
                .right(Node::new(8).to_box())
                .to_box()
        )
        .to_box();

    println!("{:?}", tree);

    print_in_order(tree);

    println!("-----");
    print_in_order(tree2);
}
