import { getContext, setContext } from "svelte";

export const SIDEBAR_KEY = Symbol("sidebar");

export type SidebarState = "expanded" | "collapsed";

export class SidebarContext {
    open = $state(true);
    isMobile = $state(false);

    get state(): SidebarState {
        return this.open ? "expanded" : "collapsed";
    }

    toggleSidebar() {
        this.open = !this.open;
    }

    setOpen(value: boolean) {
        this.open = value;
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
