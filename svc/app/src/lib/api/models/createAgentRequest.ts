// @ts-nocheck
import type { AgentMemoryChoice } from './agentMemoryChoice';
import type { AgentResourceLimitsRequest } from './agentResourceLimitsRequest';

export interface CreateAgentRequest {
  /** Create new persistent memory, reuse this name's existing memory, or run without it. */
  memory: AgentMemoryChoice;
  /**
     * Agent name. Use 1 to 40 lowercase letters, digits or hyphens, starting
     * and ending with a letter or digit.
     * @minLength 1
     * @maxLength 40
     * @pattern ^[a-z0-9]([a-z0-9-]{0,38}[a-z0-9])?$
     */
  name: string;
  resource_limits?: null | AgentResourceLimitsRequest;
}
