// @ts-nocheck
import type { ThemePref } from './themePref';

/**
 * The authenticated user's account settings.
 *
 * Backs the settings surface in the dashboard: the appearance (theme) and the
 * notification preferences. Every field is always present; when a user has no
 * stored settings the server responds with the defaults.
 */
export interface UserSettings {
  /** Receive the weekly activity digest. */
  notify_activity_digest: boolean;
  /** Receive product-update notifications (new tools, models and features). */
  notify_product_updates: boolean;
  /** Receive security alerts (new-device sign-ins, role changes). */
  notify_security_alerts: boolean;
  /** Preferred colour theme. */
  theme: ThemePref;
}
