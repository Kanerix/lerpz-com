// @ts-nocheck

export interface AgentResourceLimitsRequest {
  /**
     * CPU limit in millicores. 1000 millicores is one CPU core.
     * @minimum 1
     * @nullable
     */
  cpu_millicores?: number | null;
  /**
     * RAM limit in mebibytes. This is separate from persistent agent memory.
     * @minimum 1
     * @nullable
     */
  memory_mib?: number | null;
}
