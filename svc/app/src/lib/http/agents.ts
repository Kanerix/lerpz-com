import type { CreateQueryOptions } from "@tanstack/svelte-query";
import {
    listAgentMemory,
    listAgents,
    readAgent,
} from "$lib/api/agents/agents.js";
import type {
    AgentMemoryResponse,
    AgentMemoryStatus,
    AgentResponse,
    AgentStatus,
} from "$lib/api/models/index.js";
import { publicEnv } from "$lib/env.js";
import { agentAccountKey, agentsEnabled } from "./agent-mutator.js";

export function agentsQueryKey(...resource: string[]) {
    return [
        "agents",
        publicEnv.PUBLIC_API_URL,
        publicEnv.PUBLIC_ENTRA_ID_SCOPE,
        agentAccountKey(),
        ...resource,
    ] as const;
}

export function agentsQueryOptions(): CreateQueryOptions<AgentResponse[]> {
    return {
        queryKey: agentsQueryKey("list"),
        enabled: agentsEnabled(),
        queryFn: async ({ signal }) => {
            const response = await listAgents({ signal });
            if (response.status !== 200) throw response.data;
            return response.data;
        },
        staleTime: 0,
        gcTime: 0,
        retry: false,
        refetchInterval: 10_000,
        meta: { skipGlobalErrorDialog: true },
    };
}

export function agentQueryOptions(
    name: string,
): CreateQueryOptions<AgentResponse> {
    return {
        queryKey: agentsQueryKey("detail", name),
        enabled: agentsEnabled() && Boolean(name),
        queryFn: async ({ signal }) => {
            if (!/^[a-z0-9](?:[a-z0-9-]{0,38}[a-z0-9])?$/.test(name)) {
                throw new Error("Invalid agent name.");
            }
            const response = await readAgent(name, { signal });
            if (response.status !== 200) throw response.data;
            return response.data;
        },
        staleTime: 0,
        gcTime: 0,
        retry: false,
        refetchInterval: 10_000,
        meta: { skipGlobalErrorDialog: true },
    };
}

export function agentMemoryQueryOptions(): CreateQueryOptions<
    AgentMemoryResponse[]
> {
    return {
        queryKey: agentsQueryKey("memory"),
        enabled: agentsEnabled(),
        queryFn: async ({ signal }) => {
            const response = await listAgentMemory({ signal });
            if (response.status !== 200) throw response.data;
            return response.data;
        },
        staleTime: 0,
        gcTime: 0,
        retry: false,
        refetchInterval: 10_000,
        meta: { skipGlobalErrorDialog: true },
    };
}

export function agentStatusLabel(status: AgentStatus): string {
    const labels: Record<AgentStatus, string> = {
        ready: "Ready",
        not_ready: "Not ready",
        stopped: "Stopped",
        unknown: "Unknown",
    };
    return labels[status];
}

export function memoryStatusLabel(status: AgentMemoryStatus): string {
    const labels: Record<AgentMemoryStatus, string> = {
        pending: "Pending",
        available: "Available",
        unavailable: "Unavailable",
        unknown: "Unknown",
    };
    return labels[status];
}
