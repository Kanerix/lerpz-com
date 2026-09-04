// @ts-nocheck
import type { ModelSettings } from './modelSettings';

/**
 * Parameters for updating an existing model.
 *
 * Every field is optional; omitted fields are left unchanged.
 */
export interface UpdateModelRequest {
  /**
     * Portkey deployment name used when routing requests.
     * @nullable
     */
  deployment_name?: string | null;
  /**
     * Longer description of the model.
     * @nullable
     */
  description?: string | null;
  /**
     * Human-readable name shown in UIs.
     * @nullable
     */
  display_name?: string | null;
  /**
     * Provider family the model belongs to.
     * @nullable
     */
  family?: string | null;
  /**
     * Input/output modalities the model supports (e.g. `text`, `image`).
     * @nullable
     */
  modalities?: string[] | null;
  /**
     * Portkey provider slug the deployment lives under.
     * @nullable
     */
  provider?: string | null;
  settings?: null | ModelSettings;
}
