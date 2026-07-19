export interface AppNotification {
    id: string;
    title: string;
    /** Optional longer description shown under the title. */
    body?: string;
    /** Iconify icon name shown in the leading slot. */
    icon?: string;
    /** Optional link the notification navigates to when clicked. */
    href?: string;
    read: boolean;
    /** Creation time in epoch milliseconds, used for ordering and display. */
    createdAt: number;
}

/** Fields a caller supplies when raising a notification. */
export type NewNotification = Omit<
    AppNotification,
    "id" | "read" | "createdAt"
>;

// DRAFT: seeded with sample notifications so the bell has something to show
// while the backend feed doesn't exist yet. Replace `seed` (and wire `add`) to
// a real source — e.g. a SignalR/websocket stream or polling — when available.
const seed: AppNotification[] = [
    {
        id: "welcome",
        title: "Welcome to Lerpz",
        body: "Explore the AI tools from the sidebar to get started.",
        icon: "fa6-solid:wand-magic-sparkles",
        href: "/ai/chats",
        read: false,
        createdAt: Date.now() - 1000 * 60 * 5,
    },
    {
        id: "new-model",
        title: "New model available",
        body: "GPT-4o is now selectable in chat and agents.",
        icon: "fa6-solid:robot",
        read: false,
        createdAt: Date.now() - 1000 * 60 * 60 * 3,
    },
    {
        id: "maintenance",
        title: "Scheduled maintenance",
        body: "Image generation may be briefly unavailable this weekend.",
        icon: "fa6-solid:screwdriver-wrench",
        read: true,
        createdAt: Date.now() - 1000 * 60 * 60 * 24 * 2,
    },
];

class NotificationStore {
    notifications = $state<AppNotification[]>(seed);

    /** Number of notifications the user hasn't read yet. */
    get unreadCount(): number {
        return this.notifications.filter((n) => !n.read).length;
    }

    get hasUnread(): boolean {
        return this.unreadCount > 0;
    }

    /** Add a notification to the top of the feed and return its id. */
    add(notification: NewNotification): string {
        const id = crypto.randomUUID();
        this.notifications = [
            {
                ...notification,
                id,
                read: false,
                createdAt: Date.now(),
            },
            ...this.notifications,
        ];
        return id;
    }

    markRead(id: string) {
        this.notifications = this.notifications.map((n) =>
            n.id === id ? { ...n, read: true } : n,
        );
    }

    markAllRead() {
        this.notifications = this.notifications.map((n) =>
            n.read ? n : { ...n, read: true },
        );
    }

    remove(id: string) {
        this.notifications = this.notifications.filter((n) => n.id !== id);
    }

    clear() {
        this.notifications = [];
    }
}

export const notificationStore = new NotificationStore();
