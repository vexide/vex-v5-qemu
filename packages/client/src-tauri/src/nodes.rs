use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

use log::info;
use node_graph::{
    evaluate_adi_device, evaluate_smart_device, interpreter::Device, parser::NodeGraph,
};
use serde::Serialize;
use tauri::{Emitter, Manager, State};
use tokio::{sync::Mutex, time::sleep};

use crate::AppState;

#[derive(Default, Debug)]
pub struct InterpreterState {
    pub node_ast: node_graph::ast::Brain,
    pub node_graph: NodeGraph,
    pub interpreter_task: Option<tauri::async_runtime::JoinHandle<()>>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct NodeGraphUpdate {
    pub devices: BTreeMap<String, Device>,
}

#[tauri::command]
pub fn start_node_graph_interpreter(state: State<'_, Mutex<AppState>>, app: tauri::AppHandle) {
    info!("Starting node graph interpreter");
    let mut state = state.blocking_lock();
    state.interpreter.interpreter_task = Some(tauri::async_runtime::spawn(async move {
        let state = app.state::<Mutex<AppState>>();

        let mut time = 0.0;

        loop {
            let start = Instant::now();

            let brain = state.lock().await.interpreter.node_ast.clone();
            let graph = state.lock().await.interpreter.node_graph.clone();

            let mut devices: BTreeMap<String, Device> = BTreeMap::new();

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

            // Evaluate smart ports
            for (node, id) in smart_ports.iter() {
                let device = evaluate_smart_device(node, time);
                devices.insert((*id).to_owned(), device);
            }
            // Evaluate ADI ports
            for (node, id) in adi_ports.iter() {
                let device = evaluate_adi_device(node, time);
                devices.insert((*id).to_owned(), device);
            }
            app.emit("node-graph-update", NodeGraphUpdate { devices })
                .unwrap();
            let elapsed = start.elapsed();
            sleep(Duration::from_millis(10) - elapsed).await;
            time += 0.01;
        }
    }));
}

#[tauri::command]
pub fn stop_node_graph_interpreter(state: State<'_, Mutex<AppState>>) {
    let mut state = state.blocking_lock();
    if let Some(task) = state.interpreter.interpreter_task.take() {
        task.abort();
    }
}

#[tauri::command]
pub fn update_node_graph(state: State<'_, Mutex<AppState>>, opts: NodeGraph) {
    let mut state = state.blocking_lock();
    let brain = node_graph::node_graph_to_ast(&opts).unwrap();
    state.interpreter.node_ast = brain;
    state.interpreter.node_graph = opts;
}
