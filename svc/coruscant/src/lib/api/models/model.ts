// @ts-nocheck
import type { ModelSettings } from './modelSettings';

/**
 * An AI model that can be routed to via Portkey.
 */
export interface Model {
  /** Timestamp of when the model was created. */
  created_at: string;
  /** Portkey deployment name used when routing requests. */
  deployment_name: string;
  /**
     * Optional longer description of the model.
     * @nullable
     */
  description?: string | null;
  /** Human-readable name shown in UIs. */
  display_name: string;
  /** Provider family the model belongs to. */
  family: string;
  /** Unique model identifier. */
  id: string;
  /** Input/output modalities the model supports. */
  modalities: string[];
  /** Portkey provider slug the deployment lives under. */
  provider: string;
  /** Provider/runtime settings as a JSON object. */
  settings: ModelSettings;
  /** Timestamp of the last update. */
  updated_at: string;
}
