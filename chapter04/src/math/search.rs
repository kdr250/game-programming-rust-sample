use core::f32;
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
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SquareState {
    Empty,
    X,
    O,
}

#[derive(Debug, Clone)]
pub struct GameState {
    board: [[SquareState; 3]; 3],
}

#[derive(Debug, Clone)]
pub struct GTNode {
    children: Vec<Rc<RefCell<GTNode>>>,
    state: GameState,
}

fn generate_states(root: Rc<RefCell<GTNode>>, x_player: bool) {
    for i in 0..3 {
        for j in 0..3 {
            let mut next = None;
            if root.borrow().state.board[i][j] == SquareState::Empty {
                let mut game_state = root.borrow().state.clone();
                game_state.board[i][j] = if x_player {
                    SquareState::X
                } else {
                    SquareState::O
                };
                let node = GTNode {
                    children: vec![],
                    state: game_state,
                };
                let node_ref = Rc::new(RefCell::new(node));
                root.borrow_mut().children.push(node_ref.clone());
                next = Some((node_ref, !x_player));
            }

            if let Some((next_node, next_x_player)) = next {
                generate_states(next_node, next_x_player);
            }
        }
    }
}

fn get_score(state: &GameState) -> f32 {
    // Are any of the rows the same?
    for i in 0..3 {
        let mut same = true;
        let v = &state.board[i][0];

        for j in 1..3 {
            if state.board[i][j] != *v {
                same = false;
            }
        }

        if same {
            match *v {
                SquareState::X => return 1.0,
                _ => return -1.0,
            };
        }
    }

    // Are any of the columns the same?
    for j in 0..3 {
        let mut same = true;
        let v = &state.board[0][j];

        for i in 1..3 {
            if state.board[i][j] != *v {
                same = false;
            }
        }

        if same {
            match *v {
                SquareState::X => return 1.0,
                _ => return -1.0,
            }
        }
    }

    // What about diagonals?
    let is_diagonals_same = ((state.board[0][0] == state.board[1][1])
        && (state.board[1][1] == state.board[2][2]))
        || ((state.board[2][0] == state.board[1][1]) && (state.board[1][1] == state.board[0][2]));

    if is_diagonals_same {
        match state.board[1][1] {
            SquareState::X => return 1.0,
            _ => return -1.0,
        }
    }

    // We tied
    0.0
}

fn max_player(node: Rc<RefCell<GTNode>>) -> f32 {
    // If this is a leaf, return score
    if node.borrow().children.is_empty() {
        return get_score(&node.borrow().state);
    }

    // Find the subtree with the maximum value
    let mut max_value = f32::NEG_INFINITY;
    for child in &node.borrow().children {
        max_value = max_value.max(min_player(child.clone()));
    }
    max_value
}

fn min_player(node: Rc<RefCell<GTNode>>) -> f32 {
    // If this is a leaf, return score
    if node.borrow().children.is_empty() {
        return get_score(&node.borrow().state);
    }

    // Find the subtree with the minimum value
    let mut min_value = f32::INFINITY;
    for child in &node.borrow().children {
        min_value = min_value.min(max_player(child.clone()));
    }
    min_value
}

pub fn minimax_decide(root: Rc<RefCell<GTNode>>) -> Option<Rc<RefCell<GTNode>>> {
    // Find the subtree with the maximum value, and save the choice
    let mut choice = None;
    let mut max_value = f32::NEG_INFINITY;
    for child in &root.borrow().children {
        let v = min_player(child.clone());
        if v > max_value {
            max_value = v;
            choice = Some(child.clone());
        }
    }
    choice
}

fn alpha_beta_max(node: Rc<RefCell<GTNode>>, mut alpha: f32, beta: f32) -> f32 {
    // If this is a leaf, return score
    if node.borrow().children.is_empty() {
        return get_score(&node.borrow().state);
    }

    // Find the subtree with the maximum value
    let mut max_value = f32::NEG_INFINITY;
    for child in &node.borrow().children {
        max_value = max_value.max(alpha_beta_min(child.clone(), alpha, beta));
        if max_value >= beta {
            return max_value; // Beta prune
        }
        alpha = max_value.max(alpha);
    }
    max_value
}

fn alpha_beta_min(node: Rc<RefCell<GTNode>>, alpha: f32, mut beta: f32) -> f32 {
    // If this is a leaf, return score
    if node.borrow().children.is_empty() {
        return get_score(&node.borrow().state);
    }

    // Find the subtree with the minimum value
    let mut min_value = f32::INFINITY;
    for child in &node.borrow().children {
        min_value = min_value.min(alpha_beta_max(child.clone(), alpha, beta));
        if min_value <= alpha {
            return min_value; // Alpha prune
        }
        beta = min_value.min(beta);
    }
    min_value
}

pub fn alpha_beta_decide(root: Rc<RefCell<GTNode>>) -> Option<Rc<RefCell<GTNode>>> {
    let mut choice = None;
    let mut max_value = f32::NEG_INFINITY;
    let beta = f32::INFINITY;
    for child in &root.borrow().children {
        let v = alpha_beta_min(child.clone(), max_value, beta);
        if v > max_value {
            max_value = v;
            choice = Some(child.clone());
        }
    }
    choice
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::math::search::{a_ster, alpha_beta_decide, AStarMap};

    use super::{
        bfs, gbfs, generate_states, minimax_decide, GBFSMap, GTNode, GameState, Graph, GraphNode,
        NodeToParentMap, SquareState, WeightedEdge, WeightedGraph, WeightedGraphNode,
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

    #[test]
    fn test_minimax() {
        //  O |   | X
        // -----------
        //  X | O | O
        // -----------
        //  X |   |
        let board = [
            [SquareState::O, SquareState::Empty, SquareState::X],
            [SquareState::X, SquareState::O, SquareState::O],
            [SquareState::X, SquareState::Empty, SquareState::Empty],
        ];

        let state = GameState { board };
        let root = GTNode {
            children: vec![],
            state,
        };
        let root_ref = Rc::new(RefCell::new(root));

        generate_states(root_ref.clone(), true);

        let choice = minimax_decide(root_ref).unwrap();
        let actual = &choice.borrow().state.board;

        //  O |   | X
        // -----------
        //  X | O | O
        // -----------
        //  X |   | X
        let expectd = &[
            [SquareState::O, SquareState::Empty, SquareState::X],
            [SquareState::X, SquareState::O, SquareState::O],
            [SquareState::X, SquareState::Empty, SquareState::X],
        ];

        assert_eq!(expectd, actual);
    }

    #[test]
    fn test_alpha_beta_pruning() {
        //  O |   | X
        // -----------
        //  X | O | O
        // -----------
        //  X |   |
        let board = [
            [SquareState::O, SquareState::Empty, SquareState::X],
            [SquareState::X, SquareState::O, SquareState::O],
            [SquareState::X, SquareState::Empty, SquareState::Empty],
        ];

        let state = GameState { board };
        let root = GTNode {
            children: vec![],
            state,
        };
        let root_ref = Rc::new(RefCell::new(root));

        generate_states(root_ref.clone(), true);

        let choice = alpha_beta_decide(root_ref).unwrap();
        let actual = &choice.borrow().state.board;

        //  O |   | X
        // -----------
        //  X | O | O
        // -----------
        //  X |   | X
        let expectd = &[
            [SquareState::O, SquareState::Empty, SquareState::X],
            [SquareState::X, SquareState::O, SquareState::O],
            [SquareState::X, SquareState::Empty, SquareState::X],
        ];

        assert_eq!(expectd, actual);
    }
}
