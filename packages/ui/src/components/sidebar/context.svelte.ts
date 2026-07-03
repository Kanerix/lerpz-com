import { getContext, setContext } from "svelte";

export const SIDEBAR_KEY = Symbol("sidebar");

export type SidebarState = "expanded" | "collapsed";

export class SidebarContext {
    /** Persistent desktop open state, toggled by the user (trigger/rail). */
    open = $state(true);
    /** Mobile off-canvas drawer open state. */
    openMobile = $state(false);
    isMobile = $state(false);
    /** Transient hover-to-expand state (desktop only). */
    hovered = $state(false);

    /** Effective visual state used for content layout. */
    get state(): SidebarState {
        // The mobile drawer always shows full-width content; its visibility is
        // handled separately via `openMobile`.
        if (this.isMobile) return "expanded";
        return this.open || this.hovered ? "expanded" : "collapsed";
    }

    /** Persistent state that drives the reserved layout width (ignores hover). */
    get openState(): SidebarState {
        return this.open ? "expanded" : "collapsed";
    }

    toggleSidebar() {
        if (this.isMobile) {
            this.openMobile = !this.openMobile;
            return;
        }
        this.open = !this.open;
        // A deliberate toggle should win over a lingering hover.
        this.hovered = false;
    }

    setOpen(value: boolean) {
        this.open = value;
    }

    setOpenMobile(value: boolean) {
        this.openMobile = value;
    }

    setHovered(value: boolean) {
        this.hovered = value;
    }
}

export function setSidebarContext(ctx: SidebarContext) {
    setContext(SIDEBAR_KEY, ctx);
}

export function useSidebar(): SidebarContext {
    const ctx = getContext<SidebarContext>(SIDEBAR_KEY);
    if (!ctx)
        throw new Error("useSidebar must be used inside <SidebarProvider>");
    return ctx;
}
