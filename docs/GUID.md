# Globally unique identifier (GUID)

Allowed characters are the English alphabet (a–z) and digits (0–9). The GUID
MUST always be lowercase. Special characters in names (such as `ø`, `é`, `ä`, `ß`)
should be normalized to their closest ASCII equivalents (e.g., `ø` → `o`, `é` → `e`)
before constructing the GUID. If there is no mapping in place for the special character,
it should be skipped.

Format: `<x><y>.<w>@<domain>`

Where:

- **x** = first 2–3 letters of the user's first name
- **y** = first 2–3 letters of the user's last name (or middle name)
- **w** = the last 2 digits of the year the user was employed
- **domain** = the company's email domain (letters, digits, and `-` as allowed by normal DNS rules)

Examples

Kasper Jønsson, Security — 15/11/2022

- [kasjon.22@lerpz.com]

Explanation:

- `kas` → first 3 letters of "Kasper"
- `jon` → first 3 letters of "Jønsson" (ø normalized to o)
- `22` → employed in 2022

Emma May, HR - 22/1/2025

- [emmmay.26@lerpz.com]

Explanation:

- `emm` → first 3 letters of “Emma”
- `may` → first 3 letters of “May”
- `25` → employed in 2025
