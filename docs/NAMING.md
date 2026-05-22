# Naming

This document outlines the naming conventions used in the Lerpz.com platform.

## Service Names

Service names are drawn from the **Star Wars Republic** era — the period
covering *The Phantom Menace*, *Attack of the Clones*, *Revenge of the Sith*,
and *The Clone Wars*. Each name was chosen to reflect the role the service plays
in the platform.

### `holonet` — Frontend (Next.js)

The HoloNet is the Republic's galaxy-wide communications and information
network — the interface through which citizens, senators, and soldiers send
messages, receive news, and interact with the wider galaxy. It is the
visible, public-facing layer of the Republic. As the user-facing interface,
`holonet` is how users see and interact with the platform.

### `coruscant` — Backend API (Rust/Axum)

Coruscant is the center of the Republic — the hub through which all governance,
communication, and coordination flows. Every hyperlane leads there; nothing
happens in the galaxy without going through it. As the backend API that powers
the platform, `coruscant` is always at the center and everything else depends
on it.

### `artoo` — AI Agent

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
similarity search — finding things that *resonate* with a query rather than
matching it literally. Ilum is where that meaning lives.

### `kamino` — PostgreSQL (Primary Database)

Kamino is a remote ocean world whose inhabitants are obsessive record-keepers —
they maintain perfect archives of every clone, every template, every genetic
detail. Replication is literally their purpose. The primary relational database
is exactly that: a meticulous, complete, and replicated source of truth.

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
