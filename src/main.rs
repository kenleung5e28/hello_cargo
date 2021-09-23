pub struct BSTNode<T> where T: PartialOrd {
    val: T,
    left: Option<Box<BSTNode<T>>>,
    right: Option<Box<BSTNode<T>>>,
}

impl<T: PartialOrd> BSTNode<T> {
    pub fn new(val: T) -> Self {
        Self { val: val, left: None, right: None, }
    }

    pub fn add(&mut self, val: T) {
        if val < self.val {
            match self.left.as_mut() {
                None => self.left = Some(Box::new(Self::new(val))),
                Some(node) => node.add(val)
            }
        } else {
            match self.right.as_mut() {
                None => self.right = Some(Box::new(Self::new(val))),
                Some(node) => node.add(val)
            }
        }
    }
}

pub struct BSTInorderIter<'a, T : PartialOrd> {
    queue: Vec<&'a T>,
}

impl<'a, T : PartialOrd> BSTInorderIter<'a, T> {
    pub fn new() -> Self {
        Self { queue: vec![] }
    }
} 

impl<'a, T : PartialOrd> Iterator for BSTInorderIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        
    } 
}

fn add_lefts<T : PartialOrd>(t: &BSTNode<T>) -> Vec<&T> {
    let mut v = Vec::new();
    let mut curr = t;
    loop {
        v.push(&curr.val);
        match &curr.left {
            None => break,
            Some(n) => curr = n.as_ref(),
        }
    }
    v
}

fn main() {
   let mut x = BSTNode::new(192);
   x.add(33);
   x.add(100);
   x.add(200);
   let v = add_lefts(&x);
   println!("{:?}", v);
   println!("Some element: {}", x.left.map_or(0, |y| y.right.map_or(0, |z| z.val)));
}
