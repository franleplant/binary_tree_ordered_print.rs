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




fn calc_middle(n: usize) -> usize {
    let middle = if n % 2 == 0 {
        n as f32 / 2 as f32
    } else {
        (n as f32 / 2 as f32).trunc() + 1 as f32
    };

    middle as usize
}


impl From<Vec<u32>> for Node {
    fn from(v: Vec<u32>) -> Node {
        let len = v.len();
        if len == 1 {
            return Node::new(v[0])
        }

        if len == 2 {
            return Node::new(v[1])
                .left(Node::new(v[0]).to_ptr())
        }

        let middle = calc_middle(len);

        //println!("left {:?}, middle {}, right {:?}",v[..(middle-1)].to_vec(), v[middle - 1], v[middle..].to_vec());
        //TODO avoid copying data?
        let left = Node::from(v[..(middle-1)].to_vec());
        let right = Node::from(v[middle..].to_vec());

        Node::new(v[middle - 1])
            .left(left.to_ptr())
            .right(right.to_ptr())
    }

}

//TODO make it accept references instead of Rc
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

pub fn print_tree(node: &Node) {
    print_tree_r(node, 0);
}

//TODO try a horizontal print
fn print_tree_r(node: &Node, level: usize) {
    if level == 0 {
        println!("{}", node.data);
        if let Some(ref right) = node.right {
            print_tree_r(right, level + 1);
        }

        if let Some(ref left) = node.left {
            print_tree_r(left, level + 1);
        }
        return;
    }

    let prev_level = level - 1;
    let prev_indent = prev_level * 4;
    let indent = 4;
    //println!("level {}, indent {}, prev_level: {}, prev_indent: {}", level, indent, prev_level, prev_indent);
    println!("{:>prev_indent$}|{:->indent$}", "", node.data, prev_indent=prev_indent, indent=indent);

    if let Some(ref right) = node.right {
        print_tree_r(right, level + 1);
    }

    if let Some(ref left) = node.left {
        print_tree_r(left, level + 1);
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


    let cases = vec![
        vec![2],
        vec![2,3],
        vec![2,3,4],
        vec![2,3,4,5],
        vec![2,3,4,5,6,7,8,9,10,11,12],
        (1..100).collect(),
    ];

    for (i, v) in cases.iter().enumerate() {
        println!("TREE {}, {:?}", i, v);
        let tree = Node::from(v.clone());
        print_tree(&tree);
        println!("Ordered lineal print");
        //print_in_order(tree);
    }
}
