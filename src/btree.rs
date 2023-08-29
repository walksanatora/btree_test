use std::{collections::HashMap, cmp::Ordering};

use bitvec::vec::BitVec;

#[derive(Debug,PartialEq,Eq,Clone,Hash)]
pub enum Value {
    Value(String),
    EOF,
    None
}

impl Value {
    pub fn is_some(&self) -> bool {
        return self != &Value::None
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        println!("Comparing {:?} {:?}",self,other);
        let a = match (self, other) {
            (Value::Value(me),Value::Value(you)) => you.cmp(me),
            (Value::EOF, Value::None) => Ordering::Less,
            (Value::None, Value::EOF) => Ordering::Greater,
            (Value::Value(_), _) => Ordering::Less,
            (_, Value::Value(_)) => Ordering::Greater,
            _ => Ordering::Equal
        };
        println!("result: {:?}",a);
        a
    }
}

#[derive(Clone,PartialEq,Eq)]
pub struct Node {
    pub count: usize,
    pub val: Value,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"<{:?} {}>",self.val,self.count)
    }
}

pub fn pretty_print_tree(node: &Node, indent: usize, lr: bool) {
    if node.is_leaf() {
        //println!("{:indent$}{} Count: {}, tree", "",lr, node.count, indent = indent);
        pretty_print_tree(node.left.as_ref().unwrap(), indent+1,false);
        pretty_print_tree(node.right.as_ref().unwrap(), indent+1,true);
    } else {
        //println!("{:indent$}{} Count: {}, Value: {:?}", "",lr,node.count, node.val, indent = indent);
    }
}

impl From<(Value,usize)> for Node {
    fn from(value: (Value,usize)) -> Self {
        return Node { count: value.1, val: value.0, left: None, right: None }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return other.count.partial_cmp(&self.count)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        println!("Comparing nodes {:?} {:?}",self,other);
        let a = match other.count.cmp(&self.count) {
            Ordering::Equal => self.val.cmp(&other.val),
            x => x
        };
        println!("res: {:?}",a);
        a
    }
}

impl Node {
    pub fn is_leaf(&self) -> bool {
        return self.left.is_some()
    }
    pub fn join_nodes(self, right: Node) -> Node {
        return Node {
            count: self.count+right.count,
            val: Value::None,
            left: Some(Box::new(self)),
            right: Some(Box::new(right))
        };
    }
    pub fn make_tree(values: &mut Vec<Value>) -> Node {
        if values.last().unwrap_or(&Value::None) != &Value::EOF {
            values.push(Value::EOF);
        };
        let mut counts: HashMap<Value, usize> = HashMap::new();
        for value in values {
            counts.insert(
                value.clone(), 
                1 + counts.get(&value)
                    .unwrap_or(&0)
                );
        };
        let mut hash_vec: Vec<Node> = counts.into_iter().map(|x|x.into()).collect();
        println!("sort {} {:?}",hash_vec.is_sorted(),hash_vec);
        hash_vec.sort();
        println!("port-sort {} {:?}",hash_vec.is_sorted(),hash_vec);

        while hash_vec.len()>1 {
            println!("iter");
            for v in hash_vec.iter() {
                println!("{}: {:?}",v.count,v.val)
            }
            let a: Node = hash_vec.pop().unwrap();
            let b: Node = hash_vec.pop().unwrap();
            let new = a.join_nodes(b);
            let insert_index = match hash_vec.binary_search(&new) {
                Ok(index) => index,      // Element already exists at index
                Err(index) => index,     // Element should be inserted at index
            };
            hash_vec.insert(insert_index, new);
            //hash_vec.sort_by(|b,a| a.count.cmp(&b.count));
        }
        return hash_vec.pop().unwrap()
    }
    
    pub fn generate_encodings(&self) -> HashMap<Value,BitVec> {
        let mut map = HashMap::new();
        if self.is_leaf() {
            self.generate_encodings_impl(BitVec::new(), &mut map);
        };
        return map;
    }
    fn generate_encodings_impl(&self,mut current: BitVec,map:  &mut HashMap<Value,BitVec>) {
        if self.is_leaf() {
            let mut left_copy = current.clone();
            left_copy.push(false);
            current.push(true);
            self.left.as_ref().unwrap().generate_encodings_impl(left_copy, map);
            self.right.as_ref().unwrap().generate_encodings_impl(current, map);
        } else {
            map.insert(self.val.clone(), current);
        }
    }
}

