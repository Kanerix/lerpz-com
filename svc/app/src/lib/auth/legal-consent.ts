import { browser } from "$app/environment";

/**
 * Persists whether the user has agreed to the legal policies (Privacy Policy,
 * Terms of Service, Cookie Policy) shown on the login screen. Once accepted the
 * value is remembered in the browser so the user is never asked again.
 */

export const LEGAL_CONSENT_STORAGE_KEY = "lerpz:legal-consent";

export function loadLegalConsent(): boolean {
    if (!browser) return false;
    try {
        return localStorage.getItem(LEGAL_CONSENT_STORAGE_KEY) === "true";
    } catch {
        return false;
    }
}

export function storeLegalConsent(accepted: boolean): void {
    if (!browser) return;
    try {
        if (accepted) localStorage.setItem(LEGAL_CONSENT_STORAGE_KEY, "true");
        else localStorage.removeItem(LEGAL_CONSENT_STORAGE_KEY);
    } catch {
        // Ignore write failures (e.g. storage disabled or quota exceeded).
    }
}
