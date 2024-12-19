use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::rc::{Rc, Weak};


#[derive(Debug)]
enum ValueOrPair {
    Value(u8),
    Pair([Rc<RefCell<Node>>; 2]),
}

impl fmt::Display for ValueOrPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueOrPair::Value(val) => write!(f, "{}", val),
            ValueOrPair::Pair(pair) => write!(f, "[{},{}]", pair[0].borrow(), pair[1].borrow()),
        }
    }
}

impl ValueOrPair {
    fn is_pair_of_values(&self) -> bool {
        matches!(self, ValueOrPair::Pair([left, right])
                 if matches!(left.borrow().value, ValueOrPair::Value(_)) &&
                    matches!(right.borrow().value, ValueOrPair::Value(_)))
    }
}


#[derive(Debug)]
struct Node {
    value: ValueOrPair,
    parent: Option<Weak<RefCell<Node>>>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Node {
    fn new_value(value: u8) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: ValueOrPair::Value(value),
            parent: None,
        }))
    }

    fn new_pair(left: Rc<RefCell<Node>>,
                right: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: ValueOrPair::Pair([left, right]),
            parent: None,
        }))
    }

    fn set_parent(&mut self, parent: &Rc<RefCell<Node>>) {
        self.parent = Some(Rc::downgrade(parent));
    }

    /*
    fn set_left(parent: &Rc<RefCell<Node>>, child: Rc<RefCell<Node>>) {
        child.borrow_mut().parent = Some(Rc::downgrade(parent));
        parent.borrow_mut().left = Some(child);
    }

    fn set_right(parent: &Rc<RefCell<Node>>, child: Rc<RefCell<Node>>) {
        child.borrow_mut().parent = Some(Rc::downgrade(parent));
        parent.borrow_mut().right = Some(child);
    }

     */
}

fn parse_number(num: &[char], idx: &mut usize) -> Rc<RefCell<Node>> {
    if num[*idx].is_digit(10) {
        let value = num[*idx].to_digit(10).unwrap() as u8;
        *idx += 1;

        Node::new_value(value)
    } else {
        assert_eq!(num[*idx], '[');
        *idx += 1;
        let left = parse_number(num, idx);
        assert_eq!(num[*idx], ',');
        *idx += 1;
        let right = parse_number(num, idx);
        assert_eq!(num[*idx], ']');
        *idx += 1;

        let node = Node::new_pair(left.clone(), right.clone());
        left.borrow_mut().set_parent(&node);
        right.borrow_mut().set_parent(&node);

        node
    }
}


fn find_explode(node: &Rc<RefCell<Node>>, depth: usize) -> Option<Rc<RefCell<Node>>> {
    if depth > 4 {
        let node_borrow = node.borrow();
        if let ValueOrPair::Pair(_) = &node_borrow.value {
            if node_borrow.value.is_pair_of_values() {
                return Some(node.clone());
            }
        }
    }

    let node_borrow = node.borrow();
    if let ValueOrPair::Pair([left, right]) = &node_borrow.value {
        if let Some(found) = find_explode(left, depth + 1) {
            return Some(found);
        }
        if let Some(found) = find_explode(right, depth + 1) {
            return Some(found);
        }
    }

    None
}


fn find_rightmost(node: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
    match &node.borrow().value {
        ValueOrPair::Value(_) => Some(node.clone()),
        ValueOrPair::Pair([_, right]) => find_rightmost(&right),
    }
}

fn find_left(node: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
    let mut left_node = None;

    let mut parent = Some(node.clone());
    loop {
        if let Some(p_node) = parent.clone() {
            if let Some(ref p_weak) = p_node.borrow().parent {
                parent = p_weak.upgrade();
                if let Some(ref p) = parent {
                    if let ValueOrPair::Pair([left, _right]) = &p.borrow().value {
                        println!("left {:?}", left);
                        println!("node {:?}", node);
                        if !Rc::ptr_eq(left, node) {
                            println!("possible left");
                            left_node = Some(left.clone());
                        }
                    }
                }
            }
        }
        if left_node.is_some() {
            break;
        }
        if parent.is_none() {
            break;
        }
    }

    if let Some(left) = left_node {
        return find_rightmost(&left);
    }

    None
}


fn reduce(num: &Rc<RefCell<Node>>) {
    //let mut mut_num = num.clone();

    let pair = find_explode(&num, 1);
    if pair.is_some() {
        let node = pair.unwrap();
        println!("{}", node.clone().borrow());
        let left = find_left(&node);
        if let Some(left) = left {
            println!("{}", left.clone().borrow());
        }
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    //let file_str: String = fs::read_to_string("data/day15/day15.txt")?;

    // let num = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".chars().collect::<Vec<_>>();
    let num = "[[[[[9,8],1],2],3],4]".chars().collect::<Vec<_>>();
    let mut idx = 0;
    let sailfish = parse_number(&num, &mut idx);

    println!("{}", sailfish.borrow());

    reduce(&sailfish);

    Ok(())
}