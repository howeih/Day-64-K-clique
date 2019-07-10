Day 64: K-clique
<br>
Clique in an undirected graph is a subgraph that is complete. Particularly, if there is a subset of k vertices that are connected to each other, we say that graph contains a k-clique.
<br>
We can find all the 2-cliques by simply enumerating all the edges.
<br>
To find k+1-cliques, we can use the previous results. Compare all the pairs of k-cliques. If the two subgraphs have k-1 vertices in common and graph contains the missing edge, we can form a k+1-clique.

run:
```
fn main() {
    let graph = init_graph(vec![0, 1, 2, 3, 4, 5]);
    graph.k_cliques();
}

```

result:
```

2-cliques: #15 [[0, 1], [0, 2], [0, 3], [0, 4], [0, 5], [1, 2], [1, 3], [1, 4], [1, 5], [2, 3], [2, 4], [2, 5], [3, 4], [3, 5], [4, 5]]
3-cliques: #20 [[0, 1, 3], [0, 1, 2], [0, 4, 5], [1, 2, 4], [2, 3, 4], [0, 2, 4], [0, 1, 5], [0, 1, 4], [1, 3, 4], [1, 2, 5], [1, 2, 3], [0, 2, 5], [0, 3, 4], [1, 4, 5], [0, 2, 3], [3, 4, 5], [2, 3, 5], [1, 3, 5], [2, 4, 5], [0, 3, 5]]
4-cliques: #15 [[0, 1, 2, 4], [0, 1, 3, 5], [0, 2, 4, 5], [1, 2, 3, 5], [0, 1, 2, 3], [0, 3, 4, 5], [0, 2, 3, 5], [0, 1, 2, 5], [1, 2, 4, 5], [0, 1, 3, 4], [2, 3, 4, 5], [1, 3, 4, 5], [1, 2, 3, 4], [0, 1, 4, 5], [0, 2, 3, 4]]
5-cliques: #6 [[0, 1, 2, 4, 5], [0, 1, 2, 3, 5], [0, 1, 3, 4, 5], [0, 1, 2, 3, 4], [1, 2, 3, 4, 5], [0, 2, 3, 4, 5]]
6-cliques: #1 [[0, 1, 2, 3, 4, 5]]

```