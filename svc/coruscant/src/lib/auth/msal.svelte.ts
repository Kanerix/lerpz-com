import type {
    AccountInfo,
    EventMessage,
    PublicClientApplication,
} from "@azure/msal-browser";
import {
    EventMessageUtils,
    EventType,
    InteractionStatus,
} from "@azure/msal-browser";
import { initializeMsal } from "./msal-auth.js";
import { loginRequest } from "./msal-config.js";

class MsalStore {
    instance = $state<PublicClientApplication | null>(null);
    accounts = $state<AccountInfo[]>([]);
    activeAccount = $state<AccountInfo | null>(null);
    inProgress = $state<InteractionStatus>(InteractionStatus.Startup);

    #initialized = false;

    get isAuthenticated(): boolean {
        return this.accounts.length > 0;
    }

    async initialize() {
        if (this.#initialized) return;
        this.#initialized = true;

        const inst = await initializeMsal();
        this.instance = inst;

        inst.addEventCallback((message: EventMessage) => {
            // Promote the account from a successful interactive/silent flow to
            // the active account so the rest of the app stays in sync.
            if (
                (message.eventType === EventType.LOGIN_SUCCESS || message.eventType === EventType.ACQUIRE_TOKEN_SUCCESS) &&
                message.payload && "account" in message.payload && message.payload.account
            ) {
                inst.setActiveAccount(message.payload.account);
            }

            // Let MSAL derive the canonical interaction status for this event.
            const status = EventMessageUtils.getInteractionStatusFromEvent(
                message,
                this.inProgress,
            );
            if (status !== null) {
                this.inProgress = status;
            }

            this.#sync();
        });

        this.#sync();
        this.inProgress = InteractionStatus.None;
    }

    #sync() {
        if (!this.instance) return;
        this.accounts = this.instance.getAllAccounts();
        this.activeAccount = this.instance.getActiveAccount();
    }

    async loginRedirect() {
        await this.instance?.loginRedirect(loginRequest);
    }

    async logoutRedirect() {
        await this.instance?.logoutRedirect();
    }

    async switchAccount() {
        await this.instance?.loginRedirect({
            ...loginRequest,
            prompt: "select_account",
        });
    }
}

export const msalStore = new MsalStore();
