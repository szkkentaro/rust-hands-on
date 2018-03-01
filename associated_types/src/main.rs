// trait Graph<N, E> {
//     fn has_edge(&self, &N, &N) -> bool;
//     fn edges(&self, &N) -> Vec<E>;
//     // etc...
// }
// fn distance<N, E, G: Graph<N, E>(graph: &G, start: &N, end: &N) -> u32 { ... }


struct Edge;
struct Node;
struct MyGraph;

// trait with associated type
trait Graph {
    type N;
    type E;

    fn has_edge(&self, &Self::N, &Self::N) -> bool;
    fn edges(&self, &Self::N) -> Vec<Self::E>;
    // etc...
}

impl Graph for MyGraph {
    type N = Node;
    type E = Edge;

    fn has_edge(&self, _n1: &Node, _n2: &Node) -> bool {
        true
    }
    fn edges(&self, _n: &Node) -> Vec<Edge> {
        Vec::new()
    }
}

fn distance<G> (_graph: &G) -> u32 { 
    10
}

fn main() {
    let g = MyGraph;
    println!("{}", distance(&g));
}
