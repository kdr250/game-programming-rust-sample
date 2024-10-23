use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering},
};

static ID: AtomicU32 = AtomicU32::new(0);

pub fn generate_id() -> u32 {
    let id = ID.load(Ordering::SeqCst);
    ID.fetch_add(1, Ordering::SeqCst);
    id
}

#[derive(Debug, PartialEq, Eq)]
pub struct GraphNode {
    id: u32,
    x: i32,
    y: i32,
    adjacent: Vec<Rc<RefCell<GraphNode>>>,
}

impl GraphNode {
    pub fn new(x: i32, y: i32) -> GraphNode {
        Self {
            id: generate_id(),
            x,
            y,
            adjacent: vec![],
        }
    }

    pub fn to_string(&self) -> String {
        format!("[{},{}]", self.x, self.y)
    }
}

pub struct Graph {
    nodes: Vec<Rc<RefCell<GraphNode>>>,
}

pub struct WeightedEdge {
    from: Rc<WeightedGraphNode>,
    to: Rc<WeightedGraphNode>,
    weight: f32,
}

pub struct WeightedGraphNode {
    edges: Vec<WeightedEdge>,
}

pub struct WeightedGraph {
    nodes: Vec<Rc<WeightedGraphNode>>,
}

type NodeToParentMap = HashMap<u32, Rc<RefCell<GraphNode>>>;

pub fn bfs(
    start: Rc<RefCell<GraphNode>>,
    goal: Rc<RefCell<GraphNode>>,
    out_map: &mut NodeToParentMap,
) -> bool {
    let mut path_found = false;
    let mut q = VecDeque::new();
    q.push_back(start.clone());

    while !q.is_empty() {
        let current = q.pop_front().unwrap();
        if current == goal {
            path_found = true;
            break;
        }

        for node in current.borrow().adjacent.clone() {
            let parent = out_map.get(&node.borrow().id);
            if let None = parent {
                if node != start {
                    out_map.insert(node.borrow().id, current.clone());
                    q.push_back(node);
                }
            }
        }
    }

    path_found
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::{bfs, Graph, GraphNode, NodeToParentMap};

    #[test]
    fn test_bfs() {
        let mut g = Graph { nodes: vec![] };

        for i in 0..5 {
            for j in 0..5 {
                let node = Rc::new(RefCell::new(GraphNode::new(i, j)));
                g.nodes.push(node);
            }
        }

        for i in 0..5_usize {
            for j in 0..5_usize {
                let node = g.nodes[i * 5 + j].clone();
                if i > 0 {
                    let ad = g.nodes[(i - 1) * 5 + j].clone();
                    node.borrow_mut().adjacent.push(ad);
                }
                if i < 4 {
                    let ad = g.nodes[(i + 1) * 5 + j].clone();
                    node.borrow_mut().adjacent.push(ad);
                }
                if j > 0 {
                    let ad = g.nodes[i * 5 + j - 1].clone();
                    node.borrow_mut().adjacent.push(ad);
                }
                if j < 4 {
                    let ad = g.nodes[i * 5 + j + 1].clone();
                    node.borrow_mut().adjacent.push(ad);
                }
            }
        }

        let mut map = NodeToParentMap::new();
        let found = bfs(g.nodes[9].clone(), g.nodes[0].clone(), &mut map);

        assert!(found, "BFS not found...");

        let mut answer = String::new();

        answer += &g.nodes[0].borrow().to_string();

        let mut path = map[&g.nodes[0].borrow().id].clone();
        while path != g.nodes[9] {
            answer += &path.borrow().to_string();
            let id = path.borrow().id;
            path = map[&id].clone();
        }

        answer += &path.borrow().to_string();

        assert_eq!(answer.as_str(), "[0,0][0,1][0,2][0,3][0,4][1,4]");
    }
}
