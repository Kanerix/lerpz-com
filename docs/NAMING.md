# Naming

This document outlines the naming conventions used in the Lerpz.com platform.

## Service Names

Service names are drawn from the **Star Wars Republic** era — the period
covering _The Phantom Menace_, _Attack of the Clones_, _Revenge of the Sith_,
and _The Clone Wars_. Each name was chosen to reflect the role the service plays
in the platform.

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

### `ilum` — Qdrant (Vector Database)

Ilum is the ice planet where kyber crystals are mined — the raw source of
meaning and resonance in the Force. Every Jedi younglings travels there to find
the crystal that will become the heart of their lightsaber. Qdrant is a vector
database: it stores embeddings that carry semantic meaning and enables
similarity search — finding things that _resonate_ with a query rather than
matching it literally. Ilum is where that meaning lives.

### `jocasta` — PostgreSQL (Primary Database)

Jocasta Nu is the Jedi Temple's chief librarian and keeper of the Archives —
the most complete repository of knowledge in the galaxy. She maintains,
catalogues, and guards every record with precision. The primary relational
database is exactly that: the meticulous, authoritative source of truth that
everything else depends on.

### `kessel` — Dragonfly (Cache)

The Kessel Run is the most famous speed record in the galaxy — Han Solo made
it in less than 12 parsecs. Speed is the entire point. A cache layer is fast,
ephemeral, and built for rapid response; it sits at the edge and accelerates
everything around it. Kessel is fast.

### `geonosis` — MinIO (Object Storage)

Geonosis is a vast industrial world dominated by enormous droid foundries —
sprawling factory complexes that manufacture, store, and ship on a massive
scale. Object storage is exactly that: a large, passive place where files,
images, and blobs are held until they are needed. Geonosis holds things.
