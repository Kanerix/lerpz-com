// @ts-nocheck
import type { ModelSettings } from './modelSettings';

/**
 * Parameters for creating a new model.
 */
export interface CreateModelRequest {
  /** Portkey deployment name used when routing requests. */
  deployment_name: string;
  /**
     * Optional longer description of the model.
     * @nullable
     */
  description?: string | null;
  /** Human-readable name shown in UIs (e.g. `GPT-4o`). */
  display_name: string;
  /** Provider family the model belongs to. */
  family: string;
  /** Input/output modalities the model supports (e.g. `text`, `image`). Defaults to `[]`. */
  modalities?: string[];
  /** Portkey provider slug the deployment lives under. */
  provider: string;
  settings?: null | ModelSettings;
}
