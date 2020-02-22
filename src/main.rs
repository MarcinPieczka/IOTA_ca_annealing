use rand::prelude::*;
use std::time::Instant;

struct Node {
    vote: bool,
    neighbours: Vec<i16>
}

fn main() {
    // generate nodes
    let start = Instant::now();
    let mut total = 0;
    let mut failed = 0;

    loop {
        total += 1;
        if !one_voting() {
            failed += 1;
            println!(
                "fail ratio: {}%, {} 1/s",
                100.0 * failed as f32 / total as f32,
                total as f32 / start.elapsed().as_secs_f32()
            )
        }
    }
}

fn one_voting() -> bool {
    let result: bool;
    let mut nodes = gen_arrow_greph();
    let mut i = 0;
    loop {
        i += 1;
        let res = vote_proportions(&nodes);
        one_epoch(&mut nodes);
        if i > 1000 {
            result = false;
            break
        }
        if res == 1.0 || res == 0.0 {
            result = true;
            break;
        }
    }
    result
}

fn gen_arrow_greph() -> Vec<Node>{
    let n = thread_rng().gen_range(1000, 10000);
    let mut nodes = initialize_graph(n);
    // generate connections
    for i in 0..n {
        let mut excluded = vec![i];
        for _ in 0..4 {
            let mut x;
            loop {
                x = thread_rng().gen_range(0, n);
                if !excluded.contains(&x) {
                    excluded.push(x);
                    break;
                }
            }
            nodes[i].neighbours.push(x as i16);
            nodes[x].neighbours.push(i as i16);
        }
    }
    nodes
}

fn initialize_graph(n: usize) -> Vec<Node>{
    let mut nodes = Vec::new();
    for _ in 0..n {
        nodes.push(Node{
            vote: random(),
            neighbours: Vec::new()
        })
    }
    nodes
}

fn one_epoch(nodes: &mut Vec<Node>){
    for i in 0..nodes.len(){
        let n_true = nodes[i].neighbours.iter().map(|&x| nodes[x as usize].vote)
            .filter(|&x| x)
            .count();
        let all = nodes[i].neighbours.len();
        if n_true * 2 == all {
            nodes[i].vote = !nodes[i].vote;
        } else {
            nodes[i].vote = (all + 2) / (n_true + 1) == 1;
        }
    }
}

fn vote_proportions(nodes: &Vec<Node>) -> f32 {
    nodes.iter().filter(|x| x.vote).count() as f32 / nodes.len() as f32
}