import { startNodeGraphInterpreter, stopNodeGraphInterpreter, updateNodeGraph } from "~/lib/invoke";
import type { NodeGraph } from "~/lib/nodeGraph";

class Interpreter {
    started: boolean = false;
    constructor() {
        this.started = false;
    }
    async start() {
        console.log("Starting interpreter");
        if (!this.started) {
            this.started = true;
            startNodeGraphInterpreter();
        }
    }
    async update(nodeGraph: NodeGraph) {
        if (this.started) {
            updateNodeGraph(nodeGraph);
        }
    }
    async stop() {
        if (this.started) {
            this.started = false;
            stopNodeGraphInterpreter();
        }
    }
}

export default Interpreter;
