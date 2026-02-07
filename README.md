# MusicFrog Despicable Infiltrator

Desktop manager for mihomo-based proxy configurations with an Admin Web UI for profiles, network settings, core control (stable update/download/switch), rules, runtime diagnostics (connections/logs/traffic/memory/ip with live stream and auto refresh controls), proxy delay testing (single + batch), and advanced config editors for `rule-providers`, `proxy-providers`, and `sniffer`, plus grouped tray shortcuts for proxy/config pages, language/theme preferences, and live update notifications for tray and core changes. The Android companion app provides VPN/TUN controls, per-app routing with single FFI-backed state, profile edit/import/subscription management, runtime connection management (filter + disconnect), extended DNS/TUN advanced fields (`fallback-filter`, `stack`, `auto-detect-interface`), DNS/Fake-IP/Rules management, and WebDAV sync.

## Tech Stack & Libraries

- Backend: Tauri, Axum, SQLx, Reqwest, Tokio, Serde
- Frontend: Vue 3, Vite, Tailwind CSS

## AI Codex

This project is fully developed and maintained by AI assistants.

- **Google Gemini 2.5 / 3.0 Flash/Pro**: Core logic, system integration, and feature planning.
- **Anthropic Claude 4.5 Sonnet**: Frontend UI/UX design and component refactoring.
- **OpenAI Codex**: Code completion, routine refactorings, and documentation upkeep.

## Documentation Links

- [USAGE_SPEC.md](USAGE_SPEC.md) - Detailed feature descriptions and usage guide (Bilingual).
