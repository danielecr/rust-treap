use std::rc::Rc;
use std::cell::RefCell;

use std::fmt;

//trait PartialEq+fmt::Display: PartialEq+fmt::Display {}
type NodeContent<T> = (T, u8);
type NodeRef<T> = Rc<RefCell<TreapNode<T>>>;

//#[derive(PartialEq)]
type NodePointer<T> = Option<NodeRef<T>>;

struct TreapNode<T: PartialEq+fmt::Display> {
    value: T,
    priority: u8,
    left: NodePointer<T>,
    right: NodePointer<T>,
    parent: NodePointer<T>,
}

impl<T: PartialEq+fmt::Display + std::fmt::Debug> fmt::Debug for TreapNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parent = match &self.parent {
            Some(v) => v.borrow().value.to_string(),
            _ => format!("<NULL>")
        };
        f.debug_struct("TreapNode")
        .field("parent", &parent)
        .field("value", &self.value)
        .field("priority", &self.priority)
        .field("left", &self.left)
        .field("right", &self.right)
        .finish()
    }
}

impl<T: PartialEq+fmt::Display> TreapNode<T> {
    pub fn new((value, priority): NodeContent<T>) -> Self {
        TreapNode{value, priority, left: None, right: None, parent: None}
    }
    
    pub fn new_ref(content: NodeContent<T>) -> NodeRef<T> {
        Rc::new(RefCell::new(Self::new(content)))
    }
}
#[derive(Debug)]
struct Treap<T: PartialEq+fmt::Display + std::cmp::PartialOrd> {
    root: NodePointer<T>
}


use std::collections::VecDeque;

impl<T: PartialEq+fmt::Display + std::cmp::PartialOrd + Copy> Treap<T> {
    fn insert_in_bst(base_node: NodeRef<T>, new_come: NodeRef<T>) {
        // here the idea is to push each visited node during the new_come insertion
        // then pop each to balance the new_come node based on its weight

        // Another promising idea here is to split each sub-TREAP when the visited node
        // has priority smaller than new_come, pushing the splitted subTREAP into the Queue
        //let mut queue: VecDeque<NodeRef<T>> = VecDeque::new();
        //queue.push_front(Rc::clone(&base_node));
        let mut b = base_node.borrow_mut();
        if new_come.borrow().value > b.value {
            match &b.right {
                None => {
                    b.right = Some(Rc::clone(&new_come));
                    new_come.borrow_mut().parent = Some(Rc::clone(&base_node));
                    println!("its parent is {} and it is a right child",b.value);
                },
                Some(right) => Self::insert_in_bst(Rc::clone(&right), new_come)
            }
        } else {
            //let mut b = base_node.borrow_mut();
            match &b.left {
                None => {
                    b.left = Some(Rc::clone(&new_come));
                    new_come.borrow_mut().parent = Some(Rc::clone(&base_node));
                    println!("its parent is {} and it is a left child",b.value);
                },
                Some(left) => Self::insert_in_bst(Rc::clone(&left), new_come)
            }
        }
    }
    fn balance(new_come: NodeRef<T>) -> bool {
        let mut n = new_come.borrow_mut();
        if let Some(p) = &n.parent {
            //let mut parent = Rc::clone(&p).borrow_mut();
            let p1 = Rc::clone(p);
            let mut parent = p1.borrow_mut();
            if parent.priority < n.priority {
                //return None;
                println!("it should be moved {}",n.value);
                if parent.value >= n.value { 
                    //println!("it is left: {} > {}", parent.value, n.value);
                    println!("it is a left child to turn up-right: {} > {}", parent.value, n.value);
                    parent.left = match &n.right {
                        Some(right_de) => Some(Rc::clone(&right_de)),// left descendant
                        None => None
                    };
                    // ... and right child of the left-child point to the (old)parent
                    n.right = Some(Rc::clone(p));
                } else {
                    println!("it is a right child to turn up-left: {} < {}", parent.value, n.value);
                    // right child of the parent point to the left descendant of the right-child ...
                    parent.right = match &n.left {
                        Some(left_de) => Some(Rc::clone(&left_de)),// left descendant
                        None => None
                    };
                    // ... and left child of the right-child point to the (old)parent
                    n.left = Some(Rc::clone(p));
                }
                // then the grand-parent need to point the node turned
                if let Some(grandparent) = &parent.parent {
                    let mut gp = grandparent.borrow_mut();
                    if gp.value >= n.value { // attach to the left
                        if gp.left.is_none() {
                            panic!("WAS NONE left: ERROR!");
                        }
                        println!("TURN_HAPPENS_in_BRANCH: left");
                        gp.left = Some(Rc::clone(&new_come));
                    } else {
                        if gp.right.is_none() {
                            panic!("WAS NONE right: ERROR! {} ?? {}", gp.value, n.value);
                        }
                        println!("TURN_HAPPENS_in_BRANCH: right");
                        gp.right = Some(Rc::clone(&new_come));
                    }
                    n.parent = Some(Rc::clone(grandparent));
                } else { // the parent as no ancestor, i.e. the parent was root
                    println!("parent has no ancestor");
                    n.parent = None;
                };
                // detach old-parent from grandparent
                parent.parent = Some(Rc::clone(&new_come));
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    pub fn insert(&mut self, tup: &NodeContent<T>) {
        println!("insert {}", tup.0);
        let tn = TreapNode::new_ref(*tup);
        match &self.root {
            None => self.root = Some(tn),
            Some(n) => {
                Self::insert_in_bst(Rc::clone(&n), Rc::clone(&tn));
                println!("BAL::START for {} p:{}",tup.0, tup.1);
                loop {
                    let b = Self::balance(Rc::clone(&tn));
                    if b {
                        if tn.borrow().parent.is_none() {
                            self.root = Some(Rc::clone(&tn));
                            println!("root is changed! <-{} proot: {}",tup.0, tup.1);
                            break;
                        }
                        println!("has moved but not to the top");
                        // has moved but top not reached
                    } else {
                        // no move, givup
                        println!("BAL::END: no move for {} p: {}", tup.0, tup.1);
                        break;
                    }
                }
            }
        }
    }
}

fn main() {
    let enter_values: Vec<NodeContent<char>> = vec![
    ('S', 1),
    ('D', 2),
    ('A', 6),
    ('E', 0),
    ('S', 2),
    ('C', 4),
    ('B', 8),
    ];
    let mut t = Treap {
        root: None
    };
    for tup in enter_values.iter() {
        println!("{} .... {}", tup.0, tup.1);
        t.insert(tup);
        //println!("{:?}", t);
        
    }
    println!("{:?} {:?}",enter_values, t);
}
