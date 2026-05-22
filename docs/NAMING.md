# Naming

This document outlines the naming conventions used in the Lerpz.com platform.

## Service Names

Service names are drawn from the **Star Wars Republic** era — the period
covering *The Phantom Menace*, *Attack of the Clones*, *Revenge of the Sith*,
and *The Clone Wars*. Each name was chosen to reflect the role the service plays
in the platform.

### `holonet` — Frontend

The HoloNet is the Republic's galaxy-wide communications and information network
— the interface through which citizens, senators, and soldiers send messages,
receive news, and interact with the wider galaxy. It is the visible,
public-facing layer of the Republic. As the user-facing interface, `holonet` is
how users see and interact with the platform.

The Venator-class Star Destroyer was equipped with a powerful HoloNet
transceiver, making it one of the network's key nodes in the field. `holonet`
broadcasts what `venator` powers.

### `venator` — Backend API

The Venator-class Star Destroyer is the backbone of the Republic fleet —
powerful, reliable, and always in the thick of it. Every operation depends on
it. It carries troops, coordinates missions, and keeps everything moving.
Everything else in the platform depends on `venator` in the same way.

### `artoo` — AI Agent

R2-D2 is the most capable and helpful droid in the galaxy. He fixes ships,
carries vital messages, interfaces with foreign systems, and bails everyone
out of impossible situations — autonomously, resourcefully, and without being
asked twice. An AI agent that retrieves information, takes action, and solves
problems maps directly onto this role.

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

### `naboo` — Dragonfly (Cache)

Naboo is the home of the N-1 starfighter — one of the fastest and most elegant
ships in the prequel era. Fast, sleek, and built for rapid response. A cache
layer is fast, peripheral, and ephemeral — it sits at the edge and accelerates
everything around it.

### `geonosis` — MinIO (Object Storage)

Geonosis is a vast industrial world dominated by enormous droid foundries —
sprawling factory complexes that manufacture, store, and ship on a massive
scale. Object storage is exactly that: a large, passive place where files,
images, and blobs are held until they are needed. Geonosis holds things.
