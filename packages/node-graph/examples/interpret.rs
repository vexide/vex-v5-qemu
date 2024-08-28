use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use node_graph::{evaluate_adi_device, evaluate_smart_device, node_graph_to_ast, parse_node_graph};

pub fn main() -> miette::Result<()> {
    let graph = include_str!("../examples/graph.toml");
    let graph = parse_node_graph(graph)?;
    let brain = node_graph_to_ast(&graph)?;

    let smart_ports = brain
        .smart_ports()
        .into_iter()
        .zip(graph.brain.smart_ports())
        .map(|((_, node), (_, id))| (node, id))
        .collect::<Vec<_>>();
    let adi_ports = brain
        .adi_ports()
        .into_iter()
        .zip(graph.brain.adi_ports())
        .map(|((_, node), (_, id))| (node, id))
        .collect::<Vec<_>>();

    println!("{smart_ports:?} {adi_ports:?}");

    let mut time = 0.0;
    loop {
        let start = Instant::now();
        // Evaluate smart ports
        for (node, id) in smart_ports.iter() {
            let device = evaluate_smart_device(node, time);
            println!("{id}: {device:.2?}");
        }
        // Evaluate ADI ports
        for (node, id) in adi_ports.iter() {
            let device = evaluate_adi_device(node, time);
            println!("{id}: {device:.2?}");
        }
        let elapsed = start.elapsed();
        sleep(Duration::from_millis(10) - elapsed);
        time += 0.01;
    }
}
