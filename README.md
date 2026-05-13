# gantry

An event streaming broker, written in Rust.

## Roadmap

### v0.1 — single node, single topic

- [x] append-only log on disk
- [ ] minimal wire protocol over TCP (length-prefixed frames)
- [ ] produce and fetch from an offset
- [ ] CLI for publishing and tailing
- [ ] tracing and structured logging wired in from the start

### v0.2 — a real log

- [ ] segment files with rollover
- [ ] offset index for fast lookups
- [ ] retention by size and by time
- [ ] crash recovery — truncate partial writes, validate on startup
- [ ] configurable fsync policy

### v0.3 — topics, partitions, consumer groups

- [ ] topics and partitions as first-class concepts
- [ ] partition assignment
- [ ] consumer groups with offsets stored in an internal topic
- [ ] group rebalancing

### v0.4 — protocol and clients

- [ ] versioned binary protocol
- [ ] batching on producer and consumer side
- [ ] compression
- [ ] proper client library alongside the CLI
- [ ] benchmark suite to catch regressions

### v0.5 — clustering

- [ ] replication between brokers
- [ ] consensus for cluster metadata
- [ ] producer acks: none / leader / all
- [ ] leader election and failover

### v0.6 — operations

- [ ] admin commands: create, delete, describe, list
- [ ] Prometheus metrics endpoint
- [ ] structured logs suitable for aggregation
- [ ] basic web UI for inspecting topics and groups

### v0.7 — security

- [ ] TLS for client and inter-broker connections
- [ ] authentication
- [ ] ACLs for topics and consumer groups
- [ ] audit logging

### v0.8 — log compaction

- [ ] key-based compaction for compacted topics
- [ ] tombstones and delete semantics
- [ ] compaction scheduling and throttling

### v0.9 — hardening

- [ ] property-based tests for the log and protocol
- [ ] fault injection and chaos tests in CI
- [ ] soak tests under sustained load
- [ ] documented performance characteristics

### v1.0 — stable

- [ ] frozen wire protocol with a documented versioning policy
- [ ] frozen on-disk format with a documented upgrade path
- [ ] complete operator and client documentation
- [ ] tested upgrade and rollback procedures
- [ ] reference deployment guides
- [ ] semver commitment going forward
