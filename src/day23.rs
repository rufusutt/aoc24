use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn find_triangles<'a>(graph: &'a HashMap<&str, Vec<&'a str>>) -> Vec<(&'a str, &'a str, &'a str)> {
    let mut triangles = Vec::new();
    for (a, neighbors) in graph.iter() {
        for b in neighbors {
            for c in neighbors {
                if a < b && b < c && graph[b].contains(&c) {
                    triangles.push((*a, *b, *c));
                }
            }
        }
    }
    triangles
}

fn bron_kerbosch<'a>(
    graph: &'a HashMap<&str, Vec<&'a str>>,
    r: &mut HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    max_clique: &mut HashSet<&'a str>,
) {
    // Base case: if both sets are empty, we've found a maximal clique
    if p.is_empty() && x.is_empty() {
        if r.len() > max_clique.len() {
            max_clique.clear();
            max_clique.extend(r.iter().copied());
        }
        return;
    }

    // Choose pivot to reduce branches (Tomita et al. optimisation)
    let pivot = p
        .iter()
        .chain(x.iter())
        .max_by_key(|&v| {
            graph
                .get(v)
                .map(|neighbors| {
                    let neighbor_set: HashSet<_> = neighbors.iter().copied().collect();
                    p.intersection(&neighbor_set).count()
                })
                .unwrap_or(0)
        })
        .copied()
        .unwrap_or("");

    let pivot_neighbors: HashSet<&str> = graph
        .get(pivot)
        .map(|neighbors| neighbors.iter().copied().collect())
        .unwrap_or_default();

    let candidates: Vec<_> = p
        .iter()
        .filter(|&v| !pivot_neighbors.contains(v))
        .copied()
        .collect();

    for &v in candidates.iter() {
        let neighbors: HashSet<&str> = graph
            .get(v)
            .map(|n| n.iter().copied().collect())
            .unwrap_or_default();

        r.insert(v);

        let mut new_p: HashSet<&str> = p
            .iter()
            .filter(|&&n| neighbors.contains(n))
            .copied()
            .collect();

        let mut new_x: HashSet<&str> = x
            .iter()
            .filter(|&&n| neighbors.contains(n))
            .copied()
            .collect();

        bron_kerbosch(graph, r, &mut new_p, &mut new_x, max_clique);

        r.remove(v);
        p.remove(v);
        x.insert(v);
    }
}

fn find_maximum_clique<'a>(graph: &'a HashMap<&str, Vec<&'a str>>) -> HashSet<&'a str> {
    let mut max_clique = HashSet::new();
    let mut r = HashSet::new();
    let mut p: HashSet<_> = graph.keys().cloned().collect();
    let mut x = HashSet::new();

    bron_kerbosch(graph, &mut r, &mut p, &mut x, &mut max_clique);

    max_clique
}

pub fn solution(input: &str) {
    let connections: Vec<_> = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect();

    let graph: HashMap<_, _> = connections
        .iter()
        .fold(HashMap::new(), |mut acc, (from, to)| {
            acc.entry(*from).or_insert_with(Vec::new).push(*to);
            acc.entry(*to).or_insert_with(Vec::new).push(*from);
            acc
        });

    let triangles = find_triangles(&graph);

    let part1 = triangles
        .iter()
        .filter(|(a, b, c)| [a, b, c].iter().any(|&&node| node.starts_with("t")))
        .count();
    println!("Part 1: {}", part1);

    let max_clique = find_maximum_clique(&graph);

    let mut nodes: Vec<_> = max_clique.iter().collect();
    nodes.sort_unstable();
    let part2 = nodes.iter().join(",").to_string();
    println!("Part 2: {}", part2);
}
