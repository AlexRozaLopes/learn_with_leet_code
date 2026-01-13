struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn reverse_list(head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut prev: Option<Box<Node>> = None;
    let mut curr = head;

    while let Some(mut node) = curr {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        curr = next;
    }

    prev
}
