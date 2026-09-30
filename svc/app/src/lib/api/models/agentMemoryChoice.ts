// @ts-nocheck

export type AgentMemoryChoice = typeof AgentMemoryChoice[keyof typeof AgentMemoryChoice];


export const AgentMemoryChoice = {
  new: 'new',
  existing: 'existing',
  none: 'none',
} as const;
