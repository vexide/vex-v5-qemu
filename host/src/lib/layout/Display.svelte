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
    import { session } from "~/lib/stores";

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
                    const {
                        CopyBuffer: { buffer, top_left, bottom_right, stride },
                    } = command;

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
                    const { data, size, location, opaque, background } =
                        command.Text;

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
                            metrics.fontBoundingBoxAscent +
                                metrics.fontBoundingBoxDescent,
                        );
                    }

                    ctx.fillStyle = rgb8ToHex(color);
                    ctx.fillText(
                        data,
                        coords.x,
                        coords.y + metrics.fontBoundingBoxAscent,
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

<div class="display" class:running={$session?.running}>
    <header>
        <span class="title">
            {$session?.running ? programName : "Upload Program"}
        </span>
        {#if $session?.running}
            <time>0:00</time>
        {/if}
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
                    class="brain-icon-outline"
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
    {#if !$session?.running}
        <div class="upload-hint">
            <svg width="64" height="76" viewBox="0 0 64 76" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M3 4.5V71.5C3 72.3284 3.67157 73 4.5 73H59.5C60.3284 73 61 72.3284 61 71.5V20.6353C61 20.2292 60.8354 19.8405 60.5438 19.558L43.8901 3.4227C43.6102 3.1516 43.2359 3 42.8463 3H4.5C3.67157 3 3 3.67157 3 4.5Z" stroke="#E4E4E4" stroke-width="5.5"/>
                <path d="M9.76953 40.8203H19.707C21.3633 40.8203 22.6328 41.2305 23.5156 42.0508C24.4062 42.8711 24.8516 43.8867 24.8516 45.0977C24.8516 46.1133 24.5352 46.9844 23.9023 47.7109C23.4805 48.1953 22.8633 48.5781 22.0508 48.8594C23.2852 49.1562 24.1914 49.668 24.7695 50.3945C25.3555 51.1133 25.6484 52.0195 25.6484 53.1133C25.6484 54.0039 25.4414 54.8047 25.0273 55.5156C24.6133 56.2266 24.0469 56.7891 23.3281 57.2031C22.8828 57.4609 22.2109 57.6484 21.3125 57.7656C20.1172 57.9219 19.3242 58 18.9336 58H9.76953V40.8203ZM15.125 47.5586H17.4336C18.2617 47.5586 18.8359 47.418 19.1562 47.1367C19.4844 46.8477 19.6484 46.4336 19.6484 45.8945C19.6484 45.3945 19.4844 45.0039 19.1562 44.7227C18.8359 44.4414 18.2734 44.3008 17.4688 44.3008H15.125V47.5586ZM15.125 54.3086H17.832C18.7461 54.3086 19.3906 54.1484 19.7656 53.8281C20.1406 53.5 20.3281 53.0625 20.3281 52.5156C20.3281 52.0078 20.1406 51.6016 19.7656 51.2969C19.3984 50.9844 18.75 50.8281 17.8203 50.8281H15.125V54.3086ZM28.6484 40.8203H33.9688V58H28.6484V40.8203ZM37.8242 40.8203H42.7812L49.25 50.3242V40.8203H54.2539V58H49.25L42.8164 48.5664V58H37.8242V40.8203Z" fill="#00ACD8"/>
                <path d="M40 3V23.5C40 24.3284 40.6716 25 41.5 25H62.5" stroke="#E4E4E4" stroke-width="5.5"/>
            </svg>
            <p>Click the button on the right to upload a program.</p>
        </div>
    {/if}
</div>

<style>
    .display.running header {
        background-color:  #0099cc;
        color: #000;
    }

    .display:not(.running) .brain-icon-outline {
        stroke: #e6e6e4;
    }

    .display {
        overflow: hidden;
        display: flex;
        flex-direction: column;
        position: relative;
        width: 480px;
        height: 272px;
        user-select: none;
        -webkit-user-select: none;
        cursor: inherit;
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
        background-color: #505050;
        color: #fff;
    }

    .display header .title {
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
        margin-left: auto;
        align-items: flex-end;
    }

    .upload-hint {
        width: 480px;
        height: 240px;
        margin-top: 32px;
        position: absolute;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        z-index: 1;
        gap: 16px;
        color: #fff;
        background-color: #191919;
        font-family: "Noto Sans";
        font-weight: 400;
        font-size: 16px;
        padding: 48px;
    }

    .upload-hint svg {
        flex: 0 0 auto;
    }

    .upload-hint p {
        flex: 1 1 auto;
    }
</style>
