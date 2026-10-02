type AccountKey = string | null;

function accountResource(accountKey: AccountKey, resource: string) {
    return ["account", accountKey, resource] as const;
}

export const queryKeys = {
    health: {
        status: () => ["public", "health"] as const,
    },
    agents: {
        all: (accountKey: AccountKey) => accountResource(accountKey, "agents"),
        list: (accountKey: AccountKey) =>
            [...accountResource(accountKey, "agents"), "list"] as const,
        memory: (accountKey: AccountKey) =>
            [...accountResource(accountKey, "agents"), "memory"] as const,
        detail: (accountKey: AccountKey, agent: string) =>
            [
                ...accountResource(accountKey, "agents"),
                "detail",
                agent,
            ] as const,
    },
    chats: {
        all: (accountKey: AccountKey) => accountResource(accountKey, "chats"),
        list: (accountKey: AccountKey) =>
            [...accountResource(accountKey, "chats"), "list"] as const,
        detail: (accountKey: AccountKey, chatId: string) =>
            [
                ...accountResource(accountKey, "chats"),
                "detail",
                chatId,
            ] as const,
    },
    models: {
        all: (accountKey: AccountKey) => accountResource(accountKey, "models"),
        list: (accountKey: AccountKey) =>
            [...accountResource(accountKey, "models"), "list"] as const,
    },
    images: {
        all: (accountKey: AccountKey) => accountResource(accountKey, "images"),
        list: (accountKey: AccountKey, limit: number) =>
            [
                ...accountResource(accountKey, "images"),
                "list",
                { limit },
            ] as const,
        nearby: (
            accountKey: AccountKey,
            imageId: string | undefined,
            limit: number,
        ) =>
            [
                ...accountResource(accountKey, "images"),
                "nearby",
                imageId,
                { limit },
            ] as const,
    },
    videos: {
        all: (accountKey: AccountKey) => accountResource(accountKey, "videos"),
        list: (accountKey: AccountKey, limit: number) =>
            [
                ...accountResource(accountKey, "videos"),
                "list",
                { limit },
            ] as const,
        nearby: (
            accountKey: AccountKey,
            videoId: string | undefined,
            limit: number,
        ) =>
            [
                ...accountResource(accountKey, "videos"),
                "nearby",
                videoId,
                { limit },
            ] as const,
    },
};
