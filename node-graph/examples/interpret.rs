fn main() -> miette::Result<()> {
    let graph = include_str!("./graph.toml");
    let graph = node_graph::parse_node_graph(graph)?;
    let brain = node_graph::node_graph_to_ast(graph)?;
    for (port, node) in brain.smart_ports() {
        let device = node_graph::evaluate_smart_device(&node);
        println!("{:?}", device);
    }

    Ok(())
}
