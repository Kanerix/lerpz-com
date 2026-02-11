# Globally unique identifier (GUID)

Allowed characters are the English alphabet (a–z) and digits (0–9). The GUID
MUST always be lowercase. Special characters in names (such as `ø`, `é`, `ä`, `ß`)
should be normalized to their closest ASCII equivalents (e.g., `ø` → `o`, `é` → `e`)
before constructing the GUID.

Format: `<x><y>.<z><w>@<domain>.<tld>`

Where:

- **x** = first 2–3 letters of the user's first name
- **y** = first 2–3 letters of the user's last name (or middle name, if used as last)
- **z** = the user's department code (2–3 letters)
- **w** = the last 2 digits of the year the user was employed
- **domain** = the company's email domain (letters, digits, and `-` as allowed by normal DNS rules)
- **tld** = top-level domain (e.g., `com`, `org`)

Examples

Kasper Jønsson, Security — 15/11/2022

- [kasjon.sec22@lerpz.com]

Explanation:

- `kas` → first 3 letters of "Kasper"
- `jon` → first 3 letters of "Jønsson" (ø normalized to o)
- `sec` → Security department
- `22` → employed in 2022

Emma May, HR - 22/1/2025

- [emmmay.hr26@lerpz.com]

Explanation:

- `emm` → first 3 letters of “Emma”
- `may` → first 3 letters of “May”
- `hr` → HR department
- `25` → employed in 2025
