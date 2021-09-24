mod bst;
use bst::BSTNode;

fn main() {
   let mut x = BSTNode::new(192);
   x.add(33);
   x.add(100);
   x.add(200);
   for value in x.inorder_iter() {
       println!("{}", value);
   }
}
