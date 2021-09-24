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

  pub fn inorder_iter(&self) -> BSTInorderIter<T> {
      BSTInorderIter::new(&self)
  }

  fn add_lefts(&self) -> Vec<&Self> {
      let mut v = Vec::new();
      let mut curr = self;
      loop {
          v.push(curr);
          match &curr.left {
              None => break,
              Some(n) => curr = n
          }
      }
      v
  }
}

pub struct BSTInorderIter<'a, T : PartialOrd> {
  queue: Vec<&'a BSTNode<T>>,
}

impl<'a, T : PartialOrd> BSTInorderIter<'a, T> {
  pub fn new(node: &'a BSTNode<T>) -> Self {
      Self { queue: node.add_lefts() }
  }
} 

impl<'a, T : PartialOrd> Iterator for BSTInorderIter<'a, T> {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
      self.queue.pop().map(|curr| {
          match &curr.right {
              None => {},
              Some(node) => self.queue.append(&mut node.add_lefts())
          }
          &curr.val
      })
  }
}