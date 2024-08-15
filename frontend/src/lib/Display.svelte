<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";

    import {
        type DisplayClearPayload,
        type DisplayDrawPayload,
        type DisplayRenderMode,
        type DisplayScrollPayload,
        type Point2,
        type Rect,
        type Shape,
        type TextLocation,
    } from "~/lib/payload";

    export let programName = "";

    let canvas: HTMLCanvasElement | undefined;
    let ctx: CanvasRenderingContext2D | undefined;

    let unlistenDisplayDraw: UnlistenFn | undefined;
    let unlistenDisplayClear: UnlistenFn | undefined;
    let unlistenDisplayScroll: UnlistenFn | undefined;
    let unlistenDisplayRender: UnlistenFn | undefined;
    let unlistenDisplaySetRenderMode: UnlistenFn | undefined;

    onMount(async () => {
        if (!canvas) throw new Error("canvas not found");
        ctx = canvas.getContext("2d") ?? undefined;
        if (!ctx) throw new Error("ctx not found");

        // Force the canvas off a subpixel so it isn't really blurry.
        // ctx.translate(0.5, 0.5);

        // Default background color.
        ctx.fillStyle = "#000000";
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        unlistenDisplayDraw = await listen<DisplayDrawPayload>(
            "display_draw",
            (event) => {
                if (!ctx) return;
                const { command, clip_region, color } = event.payload;

                ctx.save();
                ctx.clip(rectToPath(clip_region));

                if ("Fill" in command) {
                    const path = shapeToPath(command.Fill);
                    ctx.fillStyle = rgb8ToHex(color);
                    ctx.fill(path);
                } else if ("Stroke" in command) {
                    const path = shapeToPath(command.Stroke);
                    ctx.strokeStyle = rgb8ToHex(color);
                    ctx.lineWidth = 0;
                    ctx.stroke(path);
                } else if ("CopyBuffer" in command) {
                    const { CopyBuffer: { buffer, top_left, bottom_right, stride } } = command;

                    const data = new ImageData(
                        new Uint8ClampedArray(
                            new Uint32Array(buffer).buffer,
                            0,
                            buffer.length * 4,
                        ),
                        bottom_right.x - top_left.x,
                        bottom_right.y - top_left.y,
                    );
                    console.log(data);
                    ctx.putImageData(data, top_left.x, top_left.y);
                } else if ("Text" in command) {
                    const {
                        data,
                        size,
                        location,
                        opaque,
                        background,
                    } = command.Text;

                    ctx.font = "13px Noto Mono";
                    ctx.letterSpacing = "1.5%";

                    const coords = textLocationToPoint(location);
                    const metrics = ctx.measureText(data);

                    if (opaque) {
                        ctx.fillStyle = rgb8ToHex(background);
                        ctx.fillRect(
                            coords.x,
                            coords.y,
                            metrics.width,
                            metrics.fontBoundingBoxAscent + metrics.fontBoundingBoxDescent
                        );
                    }

                    ctx.fillStyle = rgb8ToHex(color);
                    ctx.fillText(
                        data,
                        coords.x,
                        coords.y + metrics.fontBoundingBoxAscent
                    );
                }
            },
        );

        unlistenDisplayClear = await listen<DisplayClearPayload>(
            "display_clear",
            (event) => {
                if (!ctx || !canvas) return;

                ctx.save();
                ctx.clip(rectToPath(event.payload.clip_region));

                ctx.fillStyle = rgb8ToHex(event.payload.color);
                ctx.fillRect(0, 0, canvas.width, canvas.height);
            },
        );

        unlistenDisplayScroll = await listen<DisplayScrollPayload>(
            "display_scroll",
            (event) => {
                console.log("display_scroll: ", event.payload);
            },
        );

        unlistenDisplayRender = await listen<void>(
            "display_render",
            (event) => {
                console.log("display_render: ", event.payload);
            },
        );

        unlistenDisplaySetRenderMode = await listen<DisplayRenderMode>(
            "display_set_render_mode",
            (event) => {
                console.log("display_set_render_mode: ", event.payload);
            },
        );
    });

    onDestroy(() => {
        unlistenDisplayDraw?.();
        unlistenDisplayClear?.();
        unlistenDisplayScroll?.();
        unlistenDisplayRender?.();
        unlistenDisplaySetRenderMode?.();
    });

    function rgb8ToHex(rgb: number) {
        return `#${rgb.toString(16).padStart(6, "0")}`;
    }

    function shapeToPath(shape: Shape) {
        const path = new Path2D();
        if ("Rectangle" in shape) {
            const rect = shape.Rectangle;
            path.rect(
                rect.top_left.x,
                rect.top_left.y,
                rect.bottom_right.x - rect.top_left.x,
                rect.bottom_right.y - rect.top_left.y,
            );
        } else if ("Circle" in shape) {
            const circle = shape.Circle;
            path.arc(circle.center.x, circle.center.y, 0, 0, 2 * Math.PI);
        } else if ("Line" in shape) {
            const line = shape.Line;
            path.moveTo(line.start.x, line.start.y);
            path.lineTo(line.end.x, line.end.y);
        }
        return path;
    }

    function rectToPath(rect: Rect) {
        const path = new Path2D();

        path.rect(
            rect.top_left.x,
            rect.top_left.y,
            rect.bottom_right.x - rect.top_left.x,
            rect.bottom_right.y - rect.top_left.y,
        );

        return path;
    }

    function textLocationToPoint(loc: TextLocation): Point2<number> {
        if ("Line" in loc) {
            return {
                x: 0,
                y: 34 + 20 * loc.Line,
            };
        } else {
            return loc.Coordinates;
        }
    }

    export function clear() {
        if (!ctx || !canvas) return;

        ctx.fillStyle = "#000000";
        ctx.fillRect(0, 0, canvas.width, canvas.height);
    }
</script>

<div class="display">
    <header>
        <span class="program-name">{programName}</span>
        <time>0:00</time>
        <div class="header-icons">
            <svg
                class="brain-icon"
                width="31"
                height="30"
                viewBox="0 0 31 30"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path
                    d="M10.5 29.5V21.5H22.5V29.5H10.5Z"
                    fill="#93C83F"
                    stroke="black"
                />
                <rect x="23" y="24" width="2" height="3" fill="black" />
                <rect
                    x="1"
                    y="1"
                    width="29"
                    height="18"
                    rx="2"
                    fill="#333333"
                    stroke="#191919"
                    stroke-width="2"
                />
                <rect
                    x="3"
                    y="4"
                    width="22"
                    height="11"
                    rx="1"
                    fill="#F2F2F2"
                />
                <rect x="26" y="9" width="2" height="2" fill="#E1E1E1" />
                <path
                    d="M12.647 8.63331C12.507 8.34993 12.4211 7.97671 12.4211 7.57238C12.4211 6.71535 12.8157 6 13.2963 6H16.3356C16.4979 6 16.6316 6.23845 16.6316 6.52873C16.6316 6.81902 16.4979 7.05747 16.3356 7.05747H13.2963C13.134 7.05747 13.0099 7.29592 13.0099 7.57238C13.0099 7.84539 13.1244 8.11148 13.2963 8.11148H16.3356C16.4979 8.11148 16.6316 8.34648 16.6316 8.63331C16.6316 8.92014 16.4979 9.15513 16.3356 9.15513H13.2963C13.1244 9.15513 13.0099 9.42123 13.0099 9.69423C13.0099 9.96724 13.134 10.2057 13.2963 10.2057H16.3356C16.4979 10.2057 16.6316 10.4441 16.6316 10.7344C16.6316 11.0247 16.4979 11.2632 16.3356 11.2632H13.2963C12.8157 11.2632 12.4211 10.5444 12.4211 9.69078C12.4211 9.28991 12.507 8.91668 12.647 8.63331Z"
                    fill="#D9272E"
                />
                <path
                    d="M22.1971 9.16069C22.1803 9.16069 22.167 9.16627 22.1536 9.17186C22.1536 9.17186 20.4878 10.1407 20.4042 10.1854C20.3239 10.1379 18.648 9.17186 18.648 9.17186C18.6347 9.16348 18.6213 9.16069 18.6046 9.16069H17.0625C17.0291 9.16069 17.0023 9.17744 16.9923 9.20536C16.9889 9.21095 16.9889 9.21653 16.9889 9.22491C16.9889 9.24445 17.0023 9.264 17.0224 9.27517C17.0224 9.27517 19.3538 10.5931 19.5244 10.688C19.3538 10.7829 16.8117 12.2041 16.8117 12.2041C16.7916 12.2153 16.7782 12.2348 16.7782 12.2544C16.7782 12.2599 16.7782 12.2655 16.7816 12.2711C16.7916 12.2962 16.8183 12.3158 16.8518 12.3158H18.4306C18.4474 12.3158 18.4607 12.3102 18.4741 12.3046C18.4741 12.3046 20.3272 11.2324 20.4108 11.1878C20.4911 11.2352 22.3309 12.3046 22.3309 12.3046C22.3442 12.313 22.3576 12.3158 22.3743 12.3158H23.9264C23.9599 12.3158 23.9866 12.299 23.9967 12.2711C24 12.2655 24 12.2599 24 12.2544C24 12.2348 23.9866 12.2153 23.9666 12.2041C23.9666 12.2041 21.4411 10.7829 21.2705 10.688C21.4378 10.5931 23.7792 9.27517 23.7792 9.27517C23.7993 9.264 23.8127 9.24445 23.8127 9.22491C23.8127 9.21932 23.8127 9.21374 23.8093 9.20816C23.7993 9.18303 23.7725 9.16348 23.7391 9.16348H22.1971V9.16069ZM10.0649 9.16069C10.0414 9.16069 10.0214 9.16907 10.008 9.18303C10.008 9.18303 7.807 11.4307 7.69662 11.5424C7.58958 11.4279 5.42538 9.18303 5.42538 9.18303C5.412 9.16907 5.39194 9.16069 5.36852 9.16069H4.07402C4.04391 9.16069 4.0205 9.17465 4.00712 9.19699C3.99374 9.21932 4.00043 9.24445 4.01715 9.26121L7.00756 12.2935C7.02094 12.3074 7.041 12.3158 7.06442 12.3158H8.32547C8.34554 12.3158 8.36896 12.3074 8.38234 12.2935L11.4162 9.26121C11.4363 9.24166 11.4396 9.21653 11.4263 9.1942C11.4129 9.17186 11.3895 9.1579 11.3594 9.1579H10.0649V9.16069Z"
                    fill="#333333"
                />
            </svg>
        </div>
    </header>
    <canvas width="480" height="272" bind:this={canvas}></canvas>
</div>

<style>
    .display {
        contain: strict;
        display: flex;
        flex-direction: column;
        position: relative;
        width: 480px;
        height: 272px;
        user-select: none;
        -webkit-user-select: none;
        cursor: default;
    }

    .display header {
        display: flex;
        align-items: center;
        position: absolute;
        width: 100%;
        height: 32px;
        padding: 0 8px;
        font-family: "Noto Sans";
        font-size: 22px;
        font-weight: 400;
        background-color: #0099cc;
        color: #000000;
    }

    .display header .program-name {
        flex: 1 1 auto;
    }

    .display header time {
        position: absolute;
        right: 177px;
        font-family: "Noto Mono";
    }

    .header-icons {
        flex: 0 0 auto;
        height: 100%;
        display: flex;
        align-items: flex-end;
    }
</style>
