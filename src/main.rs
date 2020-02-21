use rand::prelude::*;
use std::iter;

struct Node {
    vote: bool,
    neighbours: Vec<*const bool>
}

fn main() {
    // generate nodes
    let mut nodes = Vec::new();
    for _ in 0..1000 {
        nodes.push(Node{
            vote: random(),
            neighbours: Vec::new()
        })
    }
    // generate connections
    for i in 0..1000 {
        let mut excluded = vec![i];
        for _ in 0..4 {
            let mut x;
            loop {
                x = thread_rng().gen_range(0, 1000);
                if !excluded.contains(&x) {
                    excluded.push(x);
                    break;
                }
            }
            let mut ptr = nodes[x].vote;
            nodes[i].neighbours.push(&ptr);
            ptr = nodes[i].vote;
            nodes[x].neighbours.push(&ptr);
        }
    }
    println!("{:?}", nodes[0].neighbours);
    // run simulation
    println!("{}", vote_proportions(&nodes));
    one_epoch(&mut nodes);
    println!("{}", vote_proportions(&nodes));
    one_epoch(&mut nodes);
    println!("res: {}", vote_proportions(&nodes));

}

fn one_epoch(nodes: &mut Vec<Node>){
    for mut node in nodes{
        unsafe {
            let n_of_t = node.neighbours.iter().map(|&x| *x)
                .chain(iter::once(node.vote))
                .filter(|&x| x)
                .count() as f32;
            let all = node.neighbours.len() as f32 / 2.0;
            println!("{:?}", node.neighbours.iter().map(|&x| *x));
            println!("{} {}", n_of_t, all);
            node.vote = n_of_t > all;
        }
    }
}

fn vote_proportions(nodes: &Vec<Node>) -> f32 {
    nodes.iter().filter(|x| x.vote).count() as f32 / nodes.len() as f32
}