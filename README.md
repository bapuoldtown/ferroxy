# ferroxy
Rust Based Envoy Logic

# ferroxy

A reverse proxy built from scratch in Rust, one small milestone at a time.

This is a learning project: instead of reaching for an existing proxy, I'm
building one incrementally to understand how tools like Envoy, HAProxy, and
nginx actually work under the hood. Every stage is small, runnable, and
committed on its own, so the git history reads as a step-by-step story.

## Status

Working **Layer 4 (TCP) reverse proxy**. It accepts client connections and
relays raw bytes to a backend in both directions. It does not yet understand
HTTP — at this layer it simply moves bytes, which means it works for any
TCP-based protocol.

## What it does today

- Listens on a TCP port for incoming clients.
- For each client, opens its own connection to a single backend.
- Copies bytes in both directions until either side closes the connection.
- Survives an unreachable backend without crashing (logs and moves on).

## How it works
