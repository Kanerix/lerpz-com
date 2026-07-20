// @ts-nocheck
import type { ThemePref } from './themePref';

/**
 * Partial update for a user's account settings.
 *
 * Every field is optional; omitted fields keep their current value (or fall
 * back to the default when the user has no stored settings yet).
 */
export interface UpdateSettingsRequest {
  /**
     * Receive the weekly activity digest.
     * @nullable
     */
  notify_activity_digest?: boolean | null;
  /**
     * Receive product-update notifications.
     * @nullable
     */
  notify_product_updates?: boolean | null;
  /**
     * Receive security alerts.
     * @nullable
     */
  notify_security_alerts?: boolean | null;
  theme?: null | ThemePref;
}
