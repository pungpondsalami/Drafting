// spatial.rs - หัวใจของการคำนวณพื้นที่และการจัดการ R-Tree

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl BoundingBox {
    pub fn from_points(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self {
            min_x: x1.min(x2),
            min_y: y1.min(y2),
            max_x: x1.max(x2),
            max_y: y1.max(y2),
        }
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min_x <= other.max_x && self.max_x >= other.min_x &&
        self.min_y <= other.max_y && self.max_y >= other.min_y
    }

    // เพิ่มฟังก์ชัน contains สำหรับเช็คการคลิกเมาส์ (Selection)
    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.min_x && x <= self.max_x &&
        y >= self.min_y && y <= self.max_y
    }

    pub fn merge(&self, other: &BoundingBox) -> Self {
        Self {
            min_x: self.min_x.min(other.min_x),
            min_y: self.min_y.min(other.min_y),
            max_x: self.max_x.max(other.max_x),
            max_y: self.max_y.max(other.max_y),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RTreeNode<T> {
    Leaf { bounds: BoundingBox, data: T },
    Internal { bounds: BoundingBox, children: Vec<RTreeNode<T>> },
}

impl<T> RTreeNode<T> {
    pub fn bounds(&self) -> BoundingBox {
        match self {
            RTreeNode::Leaf { bounds, .. } => *bounds,
            RTreeNode::Internal { bounds, .. } => *bounds,
        }
    }
}

#[derive(Debug)]
pub struct SpatialIndex<T> {
    root: Option<RTreeNode<T>>,
    max_children: usize,
}

impl<T: Clone> SpatialIndex<T> {
    pub fn new(max_children: usize) -> Self {
        Self { root: None, max_children }
    }

    // --- ระบบ Insert พร้อม Node Splitting ---
    pub fn insert(&mut self, data: T, bounds: BoundingBox) {
        let new_leaf = RTreeNode::Leaf { bounds, data };

        if let Some(root) = self.root.take() {
            let mut new_nodes = self.insert_recursive(root, new_leaf);

            if new_nodes.len() == 1 {
                self.root = Some(new_nodes.remove(0));
            } else {
                // ถ้าแตกออกมาเป็น 2 โหนด ให้สร้าง Root ใหม่ (ต้นไม้สูงขึ้น)
                let combined_bounds = new_nodes[0].bounds().merge(&new_nodes[1].bounds());
                self.root = Some(RTreeNode::Internal {
                    bounds: combined_bounds,
                    children: new_nodes,
                });
            }
        } else {
            self.root = Some(new_leaf);
        }
    }

    fn insert_recursive(&self, current: RTreeNode<T>, new_node: RTreeNode<T>) -> Vec<RTreeNode<T>> {
        match current {
            RTreeNode::Leaf { .. } => vec![current, new_node],
            RTreeNode::Internal { mut children, .. } => {
                // เลือกกิ่งที่จะใส่ (ในที่นี้เลือกตัวสุดท้ายเพื่อความง่าย)
                let last_child = children.pop().unwrap();
                let mut result_nodes = self.insert_recursive(last_child, new_node);
                children.append(&mut result_nodes);

                if children.len() > self.max_children {
                    self.split_node(children)
                } else {
                    let new_bounds = self.calculate_bounds(&children);
                    vec![RTreeNode::Internal { bounds: new_bounds, children }]
                }
            }
        }
    }

    fn split_node(&self, mut children: Vec<RTreeNode<T>>) -> Vec<RTreeNode<T>> {
        let mid = children.len() / 2;
        let right_children = children.split_off(mid);
        vec![
            RTreeNode::Internal { bounds: self.calculate_bounds(&children), children },
            RTreeNode::Internal { bounds: self.calculate_bounds(&right_children), children: right_children },
        ]
    }

    fn calculate_bounds(&self, children: &[RTreeNode<T>]) -> BoundingBox {
        let mut b = children[0].bounds();
        for child in &children[1..] {
            b = b.merge(&child.bounds());
        }
        b
    }

    // --- ระบบ Query ---
    pub fn query<'a>(&'a self, area: &BoundingBox, results: &mut Vec<&'a T>) {
        if let Some(ref node) = self.root {
            self.query_recursive(node, area, results);
        }
    }

    fn query_recursive<'a>(&'a self, node: &'a RTreeNode<T>, area: &BoundingBox, results: &mut Vec<&'a T>) {
        if node.bounds().intersects(area) {
            match node {
                RTreeNode::Leaf { data, .. } => results.push(data),
                RTreeNode::Internal { children, .. } => {
                    for child in children {
                        self.query_recursive(child, area, results);
                    }
                }
            }
        }
    }

    pub fn query_point<'a>(&'a self, x: f64, y: f64, results: &mut Vec<&'a T>) {
        if let Some(ref node) = self.root {
            self.query_point_recursive(node, x, y, results);
        }
    }

    fn query_point_recursive<'a>(&'a self, node: &'a RTreeNode<T>, x: f64, y: f64, results: &mut Vec<&'a T>) {
        if node.bounds().contains(x, y) {
            match node {
                RTreeNode::Leaf { data, .. } => results.push(data),
                RTreeNode::Internal { children, .. } => {
                    for child in children {
                        self.query_point_recursive(child, x, y, results);
                    }
                }
            }
        }
    }

    pub fn get_root_bounds(&self) -> Option<BoundingBox> {
        self.root.as_ref().map(|node| node.bounds())
    }

    pub fn get_all_elements(&self) -> Vec<(&BoundingBox, &T)> {
        let mut results = Vec::new();
        if let Some(ref node) = self.root {
            self.collect_all_recursive(node, &mut results);
        }
        results
    }

    fn collect_all_recursive<'a>(&self, node: &'a RTreeNode<T>, results: &mut Vec<(&'a BoundingBox, &'a T)>) {
        match node {
            RTreeNode::Leaf { bounds, data } => results.push((bounds, data)),
            RTreeNode::Internal { children, .. } => {
                for child in children { self.collect_all_recursive(child, results); }
            }
        }
    }
}
