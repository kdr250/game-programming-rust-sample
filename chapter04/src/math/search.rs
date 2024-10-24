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

static WEIGHTED_ID: AtomicU32 = AtomicU32::new(0);

pub fn generate_weighted_id() -> u32 {
    let id = WEIGHTED_ID.load(Ordering::SeqCst);
    WEIGHTED_ID.fetch_add(1, Ordering::SeqCst);
    id
}

#[derive(Debug, PartialEq)]
pub struct WeightedEdge {
    from: Rc<RefCell<WeightedGraphNode>>,
    to: Rc<RefCell<WeightedGraphNode>>,
    weight: f32,
}

impl WeightedEdge {
    pub fn new(
        from: Rc<RefCell<WeightedGraphNode>>,
        to: Rc<RefCell<WeightedGraphNode>>,
        weight: f32,
    ) -> Self {
        Self { from, to, weight }
    }
}

#[derive(Debug, PartialEq)]
pub struct WeightedGraphNode {
    id: u32,
    edges: Vec<Rc<RefCell<WeightedEdge>>>,
}

impl WeightedGraphNode {
    pub fn new() -> Self {
        Self {
            id: generate_weighted_id(),
            edges: vec![],
        }
    }
}

pub struct WeightedGraph {
    nodes: Vec<Rc<RefCell<WeightedGraphNode>>>,
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
            if parent.is_none() && node != start {
                out_map.insert(node.borrow().id, current.clone());
                q.push_back(node);
            }
        }
    }

    path_found
}

#[derive(Debug)]
pub struct GBFSScratch {
    parent_edge: Option<Rc<RefCell<WeightedEdge>>>,
    heuristic: f32,
    in_open_set: bool,
    in_closed_set: bool,
}

impl GBFSScratch {
    pub fn new() -> Self {
        Self {
            parent_edge: None,
            heuristic: 0.0,
            in_open_set: false,
            in_closed_set: false,
        }
    }
}

type GBFSMap = HashMap<u32, Rc<RefCell<GBFSScratch>>>;

fn compute_heuristic(
    _a: &Rc<RefCell<WeightedGraphNode>>,
    _b: &Rc<RefCell<WeightedGraphNode>>,
) -> f32 {
    0.0
}

pub fn gbfs(
    start: Rc<RefCell<WeightedGraphNode>>,
    goal: Rc<RefCell<WeightedGraphNode>>,
    out_map: &mut GBFSMap,
) -> bool {
    let mut open_set = vec![];

    let mut current = start;
    let mut scratch = GBFSScratch::new();
    scratch.in_closed_set = true;
    let scratch_ref = Rc::new(RefCell::new(scratch));
    out_map.insert(current.borrow().id, scratch_ref);

    let mut is_first = true;
    while is_first || current.borrow().id != goal.borrow().id {
        for edge in current.borrow().edges.clone() {
            let id = edge.borrow().to.borrow().id;
            let data = out_map
                .entry(id)
                .or_insert_with(|| Rc::new(RefCell::new(GBFSScratch::new())))
                .clone();
            let mut borrowed_data = data.borrow_mut();
            if !borrowed_data.in_closed_set {
                borrowed_data.parent_edge = Some(edge.clone());
                if !borrowed_data.in_open_set {
                    borrowed_data.heuristic = compute_heuristic(&edge.borrow().to, &goal);
                    borrowed_data.in_open_set = true;
                    open_set.push(edge.borrow().to.clone());
                }
            }
        }

        if open_set.is_empty() {
            break;
        }

        let cloned_open_set = open_set.clone();
        let result = cloned_open_set
            .into_iter()
            .min_by(|a, b| {
                let a_heuristic = out_map
                    .get(&a.borrow().id)
                    .and_then(|scratch| Some(scratch.borrow().heuristic))
                    .unwrap_or(0.0);
                let b_heuristic = out_map
                    .get(&b.borrow().id)
                    .and_then(|scratch| Some(scratch.borrow().heuristic))
                    .unwrap_or(0.0);
                a_heuristic.partial_cmp(&b_heuristic).unwrap()
            })
            .unwrap();

        current = result.clone();
        open_set.retain(|node| node.borrow().id != result.borrow().id);
        let current_id = current.borrow().id;
        let update_scratch = out_map
            .entry(current_id)
            .or_insert_with(|| Rc::new(RefCell::new(GBFSScratch::new())))
            .clone();
        update_scratch.borrow_mut().in_open_set = false;
        update_scratch.borrow_mut().in_closed_set = true;
        is_first = false;
    }

    let found = current.borrow().id == goal.borrow().id;

    found
}

pub struct AStartScratch {
    parent_edge: Option<Rc<RefCell<WeightedEdge>>>,
    heuristic: f32,
    actual_from_start: f32,
    in_open_set: bool,
    in_closed_set: bool,
}

impl AStartScratch {
    pub fn new() -> Self {
        Self {
            parent_edge: None,
            heuristic: 0.0,
            actual_from_start: 0.0,
            in_open_set: false,
            in_closed_set: false,
        }
    }
}

type AStarMap = HashMap<u32, Rc<RefCell<AStartScratch>>>;

pub fn a_ster(
    start: Rc<RefCell<WeightedGraphNode>>,
    goal: Rc<RefCell<WeightedGraphNode>>,
    out_map: &mut AStarMap,
) -> bool {
    let mut open_set = vec![];

    let mut current = start;
    let mut scratch = AStartScratch::new();
    scratch.in_closed_set = true;
    out_map.insert(current.borrow().id, Rc::new(RefCell::new(scratch)));

    let mut is_first = true;
    while is_first || current.borrow().id != goal.borrow().id {
        for edge in current.borrow().edges.clone() {
            let neighbor = edge.borrow().to.clone();
            let data = out_map
                .entry(neighbor.borrow().id)
                .or_insert_with(|| Rc::new(RefCell::new(AStartScratch::new())))
                .clone();
            let mut borrowed_data = data.borrow_mut();
            if !borrowed_data.in_closed_set {
                borrowed_data.parent_edge = Some(edge.clone());
                if !borrowed_data.in_open_set {
                    borrowed_data.heuristic = compute_heuristic(&neighbor, &goal);
                    borrowed_data.actual_from_start =
                        out_map[&current.borrow().id].borrow().actual_from_start
                            + edge.borrow().weight;
                    borrowed_data.in_open_set = true;
                    open_set.push(neighbor);
                } else {
                    // Compute what new actual cost is if current becomes parent
                    let new_g = out_map[&current.borrow().id].borrow().actual_from_start
                        + edge.borrow().weight;
                    if new_g < borrowed_data.actual_from_start {
                        borrowed_data.parent_edge = Some(edge.clone());
                        borrowed_data.actual_from_start = new_g;
                    }
                }
            }
        }

        if open_set.is_empty() {
            break;
        }

        let cloned_open_set = open_set.clone();
        let result = cloned_open_set
            .into_iter()
            .min_by(|a, b| {
                let a_value = out_map
                    .get(&a.borrow().id)
                    .and_then(|s| Some(s.borrow().heuristic + s.borrow().actual_from_start))
                    .unwrap_or(0.0);
                let b_value = out_map
                    .get(&b.borrow().id)
                    .and_then(|s| Some(s.borrow().heuristic + s.borrow().actual_from_start))
                    .unwrap_or(0.0);
                a_value.partial_cmp(&b_value).unwrap()
            })
            .unwrap();

        current = result.clone();
        open_set.retain(|node| node.borrow().id != result.borrow().id);
        let current_id = current.borrow().id;
        let update_scratch = out_map
            .entry(current_id)
            .or_insert_with(|| Rc::new(RefCell::new(AStartScratch::new())))
            .clone();
        update_scratch.borrow_mut().in_open_set = false;
        update_scratch.borrow_mut().in_closed_set = true;
        is_first = false;
    }

    let found = current.borrow().id == goal.borrow().id;

    found
}

//================
// tick-takc-toe
//================
pub enum SquareState {
    Empty,
    X,
    O,
}

pub struct GameState {
    board: [[SquareState; 3]; 3],
}

pub struct GTNode {
    children: Vec<GTNode>,
    state: GameState,
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::math::search::{a_ster, AStarMap};

    use super::{
        bfs, gbfs, GBFSMap, Graph, GraphNode, NodeToParentMap, WeightedEdge, WeightedGraph,
        WeightedGraphNode,
    };

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
        let found = bfs(g.nodes[0].clone(), g.nodes[9].clone(), &mut map);

        assert!(found, "BFS not found...");

        let mut answers = vec![];

        answers.push(g.nodes[9].borrow().to_string());

        let mut path = map[&g.nodes[9].borrow().id].clone();
        while path != g.nodes[0] {
            answers.push(path.borrow().to_string());
            let id = path.borrow().id;
            path = map[&id].clone();
        }

        answers.push(path.borrow().to_string());

        let actual = answers.into_iter().rev().collect::<Vec<String>>().join("");

        assert_eq!("[0,0][1,0][1,1][1,2][1,3][1,4]", actual);
    }

    #[test]
    fn test_gbfs() {
        let mut g = WeightedGraph { nodes: vec![] };

        for _ in 0..5 {
            for _ in 0..5 {
                let node = Rc::new(RefCell::new(WeightedGraphNode::new()));
                g.nodes.push(node);
            }
        }

        for i in 0..5_usize {
            for j in 0..5_usize {
                let node = g.nodes[i * 5 + j].clone();
                if i > 0 {
                    let from = node.clone();
                    let to = g.nodes[(i - 1) * 5 + j].clone();
                    let e = WeightedEdge::new(from, to, 1.0);
                    node.borrow_mut().edges.push(Rc::new(RefCell::new(e)));
                }
                if i < 4 {
                    let from = node.clone();
                    let to = g.nodes[(i + 1) * 5 + j].clone();
                    let e = WeightedEdge::new(from, to, 1.0);
                    node.borrow_mut().edges.push(Rc::new(RefCell::new(e)));
                }
                if j > 0 {
                    let from = node.clone();
                    let to = g.nodes[i * 5 + j - 1].clone();
                    let e = WeightedEdge::new(from, to, 1.0);
                    node.borrow_mut().edges.push(Rc::new(RefCell::new(e)));
                }
                if j < 4 {
                    let from = node.clone();
                    let to = g.nodes[i * 5 + j + 1].clone();
                    let e = WeightedEdge::new(from, to, 1.0);
                    node.borrow_mut().edges.push(Rc::new(RefCell::new(e)));
                }
            }
        }

        let mut map = GBFSMap::new();
        let found = gbfs(g.nodes[0].clone(), g.nodes[9].clone(), &mut map);

        assert!(found, "GBFS not found...");
    }

    #[test]
    fn test_a_star() {
        let mut g = WeightedGraph { nodes: vec![] };

        for _ in 0..5 {
            for _ in 0..5 {
                let node = Rc::new(RefCell::new(WeightedGraphNode::new()));
                g.nodes.push(node);
            }
        }

        for i in 0..5_usize {
            for j in 0..5_usize {
                let node = g.nodes[i * 5 + j].clone();
                if i > 0 {
                    let from = node.clone();
                    let to = g.nodes[(i - 1) * 5 + j].clone();
                    let e = WeightedEdge::new(from, to, 1.0);
                    node.borrow_mut().edges.push(Rc::new(RefCell::new(e)));
                }
                if i < 4 {
                    let from = node.clone();
                    let to = g.nodes[(i + 1) * 5 + j].clone();
                    let e = WeightedEdge::new(from, to, 1.0);
                    node.borrow_mut().edges.push(Rc::new(RefCell::new(e)));
                }
                if j > 0 {
                    let from = node.clone();
                    let to = g.nodes[i * 5 + j - 1].clone();
                    let e = WeightedEdge::new(from, to, 1.0);
                    node.borrow_mut().edges.push(Rc::new(RefCell::new(e)));
                }
                if j < 4 {
                    let from = node.clone();
                    let to = g.nodes[i * 5 + j + 1].clone();
                    let e = WeightedEdge::new(from, to, 1.0);
                    node.borrow_mut().edges.push(Rc::new(RefCell::new(e)));
                }
            }
        }

        let mut map = AStarMap::new();
        let found = a_ster(g.nodes[0].clone(), g.nodes[9].clone(), &mut map);

        assert!(found, "AStar not found...");
    }
}
