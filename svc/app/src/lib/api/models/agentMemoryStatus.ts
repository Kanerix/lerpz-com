// @ts-nocheck

/**
 * Availability of an agent's persistent memory.
 */
export type AgentMemoryStatus = typeof AgentMemoryStatus[keyof typeof AgentMemoryStatus];


export const AgentMemoryStatus = {
  pending: 'pending',
  available: 'available',
  unavailable: 'unavailable',
  unknown: 'unknown',
} as const;
