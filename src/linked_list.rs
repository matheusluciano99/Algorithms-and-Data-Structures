struct LinkedList{
    head: Option<Box<Node>>
}

trait LinkedListOps {
    fn create_node(data: i32) -> Node;
    fn insert_node(&mut self, data: i32) -> &Node;
    fn remove_node(&mut self, data: i32) -> bool;
}

struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl LinkedListOps for LinkedList {
    fn create_node(data: i32) -> Node {
        Node { data, next: None }
    }

    fn insert_node(&mut self, data: i32) -> &Node {
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        current.insert(Box::new(Self::create_node(data)))
    }

    fn remove_node(&mut self, data: i32) -> bool {
        let mut current = &mut self.head;

        loop {
            match current {
                Some(node) if node.data == data => {
                    *current = node.next.take();
                    return true;
                }
                Some(node) => {
                    current = &mut node.next;
                }
                None => return false
            }
        }
    }

}
