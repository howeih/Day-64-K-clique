use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug)]
struct Graph {
    vertices: Vec<u32>,
    edges: RefCell<HashMap<u32, HashSet<u32>>>,
}

impl Graph {
    fn add_edge(&self, v1: u32, v2: u32) {
        let mut edges = self.edges.borrow_mut();
        let edge_v1 = edges.entry(v1).or_insert(HashSet::<u32>::new());
        edge_v1.insert(v2);
        let edge_v2 = edges.entry(v2).or_insert(HashSet::<u32>::new());
        edge_v2.insert(v1);
    }

    fn clique_combinations(&self, cliques: &Vec<Vec<u32>>) -> Vec<(Vec<u32>, Vec<u32>)> {
        let mut combinations = Vec::<(Vec<u32>, Vec<u32>)>::new();
        let cliques_len = cliques.len();
        for i in 0..cliques_len - 1 {
            for j in i + 1..cliques_len {
                combinations.push((cliques[i].clone(), cliques[j].clone()));
            }
        }
        combinations
    }

    fn init_two_cliques(&self) -> Vec<Vec<u32>> {
        let mut cliques = Vec::<Vec<u32>>::new();
        for i in 0..self.vertices.len() - 1 {
            for j in i + 1..self.vertices.len() {
                let vi = self.vertices[i];
                let vj = self.vertices[j];
                cliques.push(vec![vi, vj]);
            }
        }
        cliques
    }

    fn has_edge(&self, w: &HashSet<u32>) -> bool {
        let mut w_iter = w.iter();
        let w0 = w_iter.next().unwrap();
        let w1 = w_iter.next().unwrap();
        if self.edges.borrow().get(w0).unwrap().contains(w1) {
            return true;
        }
        false
    }

    fn k_cliques(&self) {
        let mut cliques = self.init_two_cliques();
        let mut cliques_len = cliques.len();
        let mut k = 2;

        while cliques_len > 0 {
            println!("{}-cliques: #{} {:?}", k, cliques.len(), cliques);
            let mut cliques_1 = HashSet::<Vec<u32>>::new();
            for (u, v) in self.clique_combinations(&cliques) {
                let u_s: HashSet<u32> = HashSet::from_iter(u.into_iter());
                let v_s: HashSet<u32> = HashSet::from_iter(v.into_iter());
                let w = &u_s ^ &v_s;
                if w.len() == 2 && self.has_edge(&w) {
                    let mut clique = (&u_s | &w).iter().cloned().collect::<Vec<u32>>();
                    clique.sort();
                    cliques_1.insert(clique);
                }
            }
            cliques = Vec::from_iter(cliques_1.iter().cloned());
            cliques_1.clear();
            cliques_len = cliques.len();
            k += 1;
        }
    }

    fn new(vertices: Vec<u32>) -> Self {
        Graph {
            vertices,
            edges: RefCell::new(HashMap::<u32, HashSet<u32>>::new()),
        }
    }
}

fn init_graph(vertices: Vec<u32>) -> Graph {
    let graph = Graph::new(vertices.clone());
    for i in 0..vertices.len() - 1 {
        for j in i + 1..vertices.len() {
            let vi = vertices[i];
            let vj = vertices[j];
            graph.add_edge(vi, vj);
        }
    }
    graph
}

fn main() {
    let graph = init_graph(vec![0, 1, 2, 3, 4, 5]);
    graph.k_cliques();
}
