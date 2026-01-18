#[derive(Debug, Clone)]
pub enum SplitDir {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
pub enum Node {
    Leaf { id: u64 },
    Split {
        dir: SplitDir,
        a: Box<Node>,
        b: Box<Node>,
    },
}

#[derive(Debug, Clone)]
pub struct LayoutTree {
    root: Node,
    next_id: u64,
    active: u64,
}

impl LayoutTree {
    pub fn new() -> Self {
        Self {
            root: Node::Leaf { id: 1 },
            next_id: 2,
            active: 1,
        }
    }

    pub fn split_active(&mut self, dir: SplitDir) -> u64 {
        let new_id = self.next_id;
        self.next_id += 1;
        let mut replaced = false;
        self.root = Self::split_node(self.root.clone(), self.active, new_id, dir, &mut replaced);
        if replaced {
            self.active = new_id;
        }
        new_id
    }

    fn split_node(
        node: Node,
        target: u64,
        new_id: u64,
        dir: SplitDir,
        replaced: &mut bool,
    ) -> Node {
        match node {
            Node::Leaf { id } if id == target => {
                *replaced = true;
                Node::Split {
                    dir,
                    a: Box::new(Node::Leaf { id }),
                    b: Box::new(Node::Leaf { id: new_id }),
                }
            }
            Node::Leaf { .. } => node,
            Node::Split { dir, a, b } => Node::Split {
                dir,
                a: Box::new(Self::split_node(*a, target, new_id, dir.clone(), replaced)),
                b: Box::new(Self::split_node(*b, target, new_id, dir.clone(), replaced)),
            },
        }
    }

    pub fn focus(&mut self, id: u64) -> bool {
        if self.contains(&self.root, id) {
            self.active = id;
            true
        } else {
            false
        }
    }

    pub fn describe(&self) -> String {
        format!("active={} tree={}", self.active, self.describe_node(&self.root))
    }

    fn describe_node(&self, node: &Node) -> String {
        match node {
            Node::Leaf { id } => format!("L{}", id),
            Node::Split { dir, a, b } => {
                let tag = match dir {
                    SplitDir::Horizontal => "H",
                    SplitDir::Vertical => "V",
                };
                format!("{}({},{})", tag, self.describe_node(a), self.describe_node(b))
            }
        }
    }

    fn contains(&self, node: &Node, id: u64) -> bool {
        match node {
            Node::Leaf { id: leaf_id } => *leaf_id == id,
            Node::Split { a, b, .. } => self.contains(a, id) || self.contains(b, id),
        }
    }
}
