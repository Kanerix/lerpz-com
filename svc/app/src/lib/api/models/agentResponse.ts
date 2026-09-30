// @ts-nocheck
import type { AgentStatus } from './agentStatus';

/**
 * The public view of a private agent.
 */
export interface AgentResponse {
  /** @nullable */
  created_at?: string | null;
  /** Whether the agent uses persistent memory. */
  memory: boolean;
  name: string;
  status: AgentStatus;
  /**
     * Authenticated application URL, when available. This does not guarantee readiness.
     * @nullable
     */
  url?: string | null;
}
