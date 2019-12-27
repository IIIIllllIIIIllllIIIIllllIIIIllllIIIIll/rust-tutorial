#[derive(Debug)]
pub struct BST {
    head: Link,
}

type Link = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    elem: i32,
    left: Link,
    right: Link
}

impl BST {
    pub fn new() -> Self {
        BST { head: None }
    }
    pub fn insert(&mut self, elem: i32) -> bool {
        insert(&mut self.head, elem)
    }
    pub fn search(&self, elem: i32) -> bool {
        search(&self.head, elem)
    }
}


fn insert(link: &mut Link, elem: i32) -> bool {
    match link {
        None => {
            *link = Some(Box::new(Node {
                elem: elem,
                left: None,
                right: None
            }));  
            true          
        },
        Some(node) => {
            if elem < node.elem {
                insert(&mut node.left, elem)
            } else if node.elem < elem {
                insert(&mut node.right, elem)
            } else {
                false
            }
        }
    }
}
fn search(link: &Link, elem: i32) -> bool {
    match link {
        None => false,
        Some(node) => {
            if elem < node.elem {
                search(&node.left, elem)
            } else if node.elem < elem {
                search(&node.right, elem)
            } else {
                true
            }
        }
    }
}


#[cfg(test)]
mod bst_test {
    use super::BST;
    #[test]
    fn basics() {
        let mut bst = BST::new();
        assert_eq!(true, bst.insert(3));
        assert_eq!(true, bst.insert(2));
        assert_eq!(false, bst.insert(2));
        assert_eq!(true, bst.search(2));
        assert_eq!(true, bst.insert(4));
        assert_eq!(true, bst.insert(6));
        assert_eq!(true, bst.search(6));
        println!("bst={:#?}", bst);
    }
}