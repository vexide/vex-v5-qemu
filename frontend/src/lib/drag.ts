export default function drag(node: HTMLElement, callback: (event: PointerEvent) => void) {
    const pointerdown = (event: PointerEvent) => {
        if (
            (event.pointerType === 'mouse' && event.button === 2) ||
            (event.pointerType !== 'mouse' && !event.isPrimary)
        )
            return;

        node.setPointerCapture(event.pointerId);

        event.preventDefault();

        const onpointerup = (event: PointerEvent) => {
            node.setPointerCapture(event.pointerId);

            window.removeEventListener('pointermove', callback, false);
            window.removeEventListener('pointerup', onpointerup, false);
        };

        window.addEventListener('pointermove', callback, false);
        window.addEventListener('pointerup', onpointerup, false);
    };

    node.addEventListener('pointerdown', pointerdown, { capture: true, passive: false });

    return {
        destroy() {
            node.removeEventListener('pointerdown', pointerdown);
        }
    };
}
