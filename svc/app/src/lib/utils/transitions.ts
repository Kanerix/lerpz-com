import { prefersReducedMotion } from "svelte/motion";
import {
    fade as svelteFade,
    fly as svelteFly,
    slide as svelteSlide,
    type FadeParams,
    type FlyParams,
    type SlideParams,
    type TransitionConfig,
} from "svelte/transition";

/**
 * Reduced-motion-aware wrappers around Svelte's built-in transitions.
 *
 * They read the live `prefers-reduced-motion` media query at the moment the
 * transition runs, so users who opt out of animations get an instant (or
 * movement-free) result while everyone else gets the full effect.
 */

/** Vertical reveal for disclosures. Collapses to an instant swap when reduced. */
export function slide(node: Element, params?: SlideParams): TransitionConfig {
    if (prefersReducedMotion.current) {
        return svelteSlide(node, { ...params, duration: 0 });
    }
    return svelteSlide(node, params);
}

/**
 * Fly in/out. Movement is essential motion, so when the user prefers reduced
 * motion we fall back to a plain opacity fade (keeping any delay for staggering).
 */
export function fly(node: Element, params?: FlyParams): TransitionConfig {
    if (prefersReducedMotion.current) {
        return svelteFade(node, {
            delay: params?.delay,
            duration: params?.duration,
        });
    }
    return svelteFly(node, params);
}

/** Opacity fade. Fades aren't motion, so this runs unchanged either way. */
export function fade(node: Element, params?: FadeParams): TransitionConfig {
    return svelteFade(node, params);
}
