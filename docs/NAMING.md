# Naming

This document outlines the naming conventions used in the Lerpz.com platform.

## Service Names

Application service names are drawn from the **Star Wars Republic** era — the
period covering _The Phantom Menace_, _Attack of the Clones_, _Revenge of the
Sith_, and _The Clone Wars_. Each name was chosen to reflect the role the
service plays in the platform. Third-party infrastructure (databases, cache,
storage, etc.) keeps its conventional technology name.

### `coruscant` — Frontend (Next.js)

Coruscant is the gleaming, city-covered centre of the Republic — the face of
the galaxy that everyone sees and interacts with. As the user-facing interface,
`coruscant` is how users see and interact with the platform.

### `kamino` — Backend API (Rust/Axum)

Kamino is the hidden ocean world that nobody knew existed — deleted from the
Jedi Archives, operating in secret, yet producing everything the Republic
depended on. As the backend API that powers the platform, `kamino` is the
hidden engine behind everything `coruscant` presents to the world.

### `artoo` — AI Agent (Rust)

R2-D2 is the most capable and resourceful droid in the galaxy. He acts
autonomously, assesses situations on the fly, interfaces with foreign systems,
and executes missions without being told exactly how. He spins up, reasons over
the problem, takes action, and gets the job done. An AI agent that operates
independently, reasons over data, and returns results maps perfectly onto this
kind of self-directed, tireless capability.
