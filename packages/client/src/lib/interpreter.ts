import { startNodeGraphInterpreter, stopNodeGraphInterpreter, updateNodeGraph } from "~/lib/invoke";

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
    //TODO: actually support updating
    async update() {
        if (this.started) {
            updateNodeGraph();
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
