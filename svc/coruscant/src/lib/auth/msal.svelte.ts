import type { AccountInfo, PublicClientApplication } from "@azure/msal-browser";
import { InteractionStatus } from "@azure/msal-browser";
import { initializeMsal } from "./msal-auth.js";
import { loginRequest } from "./msal-config.js";

class MsalStore {
    instance = $state<PublicClientApplication | null>(null);
    accounts = $state<AccountInfo[]>([]);
    inProgress = $state<InteractionStatus>(InteractionStatus.Startup);

    get activeAccount(): AccountInfo | null {
        return this.instance?.getActiveAccount() ?? null;
    }

    get isAuthenticated(): boolean {
        return this.accounts.length > 0;
    }

    async initialize() {
        const inst = await initializeMsal();
        this.instance = inst;
        this.accounts = inst.getAllAccounts();
        this.inProgress = InteractionStatus.None;
    }

    async loginRedirect() {
        await this.instance?.loginRedirect(loginRequest);
    }

    async logoutRedirect() {
        await this.instance?.logoutRedirect();
    }

    async switchAccount() {
        await this.instance?.loginRedirect({
            prompt: "select_account",
            ...loginRequest,
        });
    }
}

export const msalStore = new MsalStore();
