// @ts-nocheck
import type { ModelFamily } from './modelFamily';
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
  family?: null | ModelFamily;
  /**
     * Portkey provider slug the deployment lives under.
     * @nullable
     */
  provider?: string | null;
  settings?: null | ModelSettings;
}
