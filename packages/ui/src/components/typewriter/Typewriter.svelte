<script lang="ts">
import type { Snippet } from "svelte";

let {
    text = "",
    animate = true,
    speed = 500,
    catchUp = 20,
    children,
}: {
    text?: string;
    animate?: boolean;
    speed?: number;
    catchUp?: number;
    children: Snippet<[string]>;
} = $props();

let revealed = $state(0);

$effect(() => {
    if (!animate) return;

    let frame = 0;
    let last = performance.now();

    const tick = (now: number) => {
        const dt = Math.min((now - last) / 1000, 0.1);
        last = now;

        const full = text.length;
        if (revealed > full) revealed = full;

        const gap = full - revealed;
        if (gap > 0) {
            const velocity = Math.max(speed, gap * catchUp);
            revealed = Math.min(full, revealed + velocity * dt);
        }

        frame = requestAnimationFrame(tick);
    };

    frame = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(frame);
});

let visible = $derived(animate ? text.slice(0, Math.floor(revealed)) : text);
</script>

{@render children(visible)}
