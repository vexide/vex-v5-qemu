import { spawnQemu, killQemu } from "~/lib/invoke";
import { terminal } from "~/lib/stores";

class Session {
    binary: string;
    paused: boolean = false;
    running: boolean = false;

    constructor(binary: string) {
        this.binary = binary;
    }

    async start() {
        if (!this.running) {
            this.running = true;
            this.paused = false;
            spawnQemu({
                gdb: false,
                qemu: "qemu-system-arm",
                kernel: "../../../kernel/target/armv7a-none-eabi/debug/kernel",
                binary: this.binary,
                qemu_args: [],
            });
        }
    }

    async stop() {
        if (this.running) {
            this.running = false;
            this.paused = false;
            terminal.subscribe((t) => t?.clear());
            killQemu();
        }
    }

    async reset() {
        if (this.running) {
            terminal.subscribe((t) => t?.clear());
            killQemu();
            spawnQemu({
                gdb: false,
                qemu: "qemu-system-arm",
                // temporary
                kernel: "../../../kernel/target/armv7a-none-eabi/debug/kernel",
                binary: this.binary,
                qemu_args: [],
            });
        }
    }
}

export default Session;
