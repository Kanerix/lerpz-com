// @ts-nocheck

/**
 * A user's preferred colour theme.
 *
 * Maps to the `theme_pref` enum type defined in the database schema. `system`
 * defers to the user's operating-system preference.
 */
export type ThemePref = typeof ThemePref[keyof typeof ThemePref];


export const ThemePref = {
  light: 'light',
  dark: 'dark',
  system: 'system',
} as const;
