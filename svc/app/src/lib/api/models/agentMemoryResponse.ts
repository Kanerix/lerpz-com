// @ts-nocheck
import type { AgentMemoryStatus } from './agentMemoryStatus';

/**
 * Persistent agent memory, retained when the agent is deleted.
 */
export interface AgentMemoryResponse {
  /** Name of the agent this memory belongs to, even after the agent is deleted. */
  agent_name: string;
  /**
     * When the memory was created, if known.
     * @nullable
     */
  created_at?: string | null;
  /** Whether the matching agent has this memory mounted. */
  in_use: boolean;
  status: AgentMemoryStatus;
}
