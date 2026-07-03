import { getContext, setContext } from "svelte";

export const SIDEBAR_KEY = Symbol("sidebar");

export type SidebarState = "expanded" | "collapsed";

export class SidebarContext {
    /** Persistent open state, toggled by the user (trigger/rail). */
    open = $state(true);
    isMobile = $state(false);
    /** Transient hover-to-expand state (desktop only). */
    hovered = $state(false);

    /** Effective visual state: hovering expands a collapsed sidebar on desktop. */
    get state(): SidebarState {
        return this.open || (this.hovered && !this.isMobile)
            ? "expanded"
            : "collapsed";
    }

    /** Persistent state that drives the reserved layout width (ignores hover). */
    get openState(): SidebarState {
        return this.open ? "expanded" : "collapsed";
    }

    toggleSidebar() {
        this.open = !this.open;
        // A deliberate toggle should win over a lingering hover.
        this.hovered = false;
    }

    setOpen(value: boolean) {
        this.open = value;
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
