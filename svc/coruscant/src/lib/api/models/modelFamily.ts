// @ts-nocheck

/**
 * Provider family a model belongs to.
 *
 * Mirrors the `model_family` Postgres enum.
 */
export type ModelFamily = typeof ModelFamily[keyof typeof ModelFamily];


export const ModelFamily = {
  openai: 'openai',
  anthropic: 'anthropic',
  google: 'google',
} as const;
