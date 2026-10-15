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
