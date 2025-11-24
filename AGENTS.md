# Project Agents Guide

This document outlines the non-obvious, project-specific aspects of the OpenAI Proxy Carousel that agents should be aware of.

## Configuration

The application uses `figment` to load configuration from `config.toml` (or `config.<ENV>.toml`) and environment variables prefixed with `PROXY_`. Configuration is accessed via a static singleton (`LazyLock`) holding `Arc<RwLock<Config>>` in `ProxyState` ([`src/state.rs`](src/state.rs:4)).

## Key Rotation

The proxy implements automatic key rotation upon receiving a `429 Too Many Requests` status from the upstream API. The key list is managed by `KeyManager` ([`src/key_manager.rs`](src/key_manager.rs:1)).

## Streaming

API responses are proxied using `async_stream::stream!` to handle `text/event-stream` responses.

## Logging

Logging is configured via `log4rs` ([`src/logger.rs`](src/logger.rs:1)) to both stdout (Info) and a file (`log/data.log`).

## Code Style

`rustfmt.toml` enforces `max_width = 100`.