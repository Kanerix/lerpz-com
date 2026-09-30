// @ts-nocheck

export type AgentStatus = typeof AgentStatus[keyof typeof AgentStatus];


export const AgentStatus = {
  ready: 'ready',
  not_ready: 'not_ready',
  stopped: 'stopped',
  unknown: 'unknown',
} as const;
