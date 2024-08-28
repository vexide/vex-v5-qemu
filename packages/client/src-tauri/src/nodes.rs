use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

use log::info;
use node_graph::{
    evaluate, interpreter::{Device, InterpreterOutput, InterpreterContext}, parser::NodeGraph
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
            let mut ctx = InterpreterContext::from_brain(time, &brain);

            let mut devices: BTreeMap<String, Device> = BTreeMap::new();

            // Evaluate all nodes in the graph by starting at the root nodes
            for node in ctx.root_nodes() {
                evaluate(&node, &mut ctx);
            }

            for (id, value) in ctx.evaluated_nodes() {
                if let InterpreterOutput::Device(device) = value {
                    devices.insert(id.clone(), *device);
                }
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
