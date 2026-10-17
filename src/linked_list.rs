struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

fn create_node(data: i32) -> Node {
    Node { data, next: None }
}

fn insert_node(head: &mut Option<Box<Node>>, data: i32) -> &Node {
    let mut current = head;
    while let Some(node) = current {
        current = &mut node.next;
    }
    current.insert(Box::new(create_node(data)))
}

fn remove_node(head: &mut Option<Box<Node>>, data: i32) -> bool {
    let mut current = head;

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
