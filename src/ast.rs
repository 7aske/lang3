struct Node {
    kind: NodeKind,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}