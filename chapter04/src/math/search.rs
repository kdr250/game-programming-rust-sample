use std::rc::Rc;

pub struct GraphNode {
    adjacent: Vec<Rc<GraphNode>>,
}

pub struct Graph {
    nodes: Vec<Rc<GraphNode>>,
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
