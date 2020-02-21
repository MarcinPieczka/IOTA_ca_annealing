use rand::prelude::*;
use std::time::Instant;


struct Node {
    vote: bool,
    peers: Vec<*const bool>
}

fn main() {
    // generate nodes
    let start = Instant::now();
    let mut total = 0;
    let mut failed = 0;
    loop {
        total += 1;
        let n = thread_rng().gen_range(1000, 10000);
        let mut nodes = Vec::new();
        for _ in 0..n {
            nodes.push(Node{
                vote: random(),
                peers: Vec::new()
            })
        }
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
                let mut ptr = &nodes[x].vote as *const bool;
                nodes[i].peers.push(ptr);
                ptr = &nodes[i].vote as *const bool;
                nodes[x].peers.push(ptr);
            }
        }
        let mut i = 0;
        loop {
            i += 1;
            let res = vote_proportions(&nodes);
            one_epoch(&mut nodes);
            if i > 1000 {
                failed += 1;
                println!(
                    "fail ratio: {}%, {} 1/s",
                    100.0 * failed as f32 / total as f32,
                    total as f32 / start.elapsed().as_secs_f32()
                )
            }
            if res == 1.0 || res == 0.0 || i > 1000 {
                break;
            }
        }
    }


}

fn one_epoch(nodes: &mut Vec<Node>){
    for i in 0..nodes.len(){
        let n_true: usize;
        unsafe {
            n_true = nodes[i].peers.iter()
                .map(|&x| if *x {1} else {0})
                .sum();
        }
        let all = nodes[i].peers.len();
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