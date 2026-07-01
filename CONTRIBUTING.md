# Contributing

Thanks for wanting to help with `legion-kb-rgb`!

## Ground rules

- **No prebuilt binaries in PRs or issues.** Do not attach, link to, or reference compiled executables from a fork or external host. Releases are only published from this repo's own CI pipeline, built from reviewed source. This protects everyone from supply-chain risk — this tool needs raw USB device access, so a malicious binary here is a serious threat.
- Docs and code PRs are both welcome. Keep them focused — one topic per PR makes review faster.
- If your PR changes build steps or dependencies, explain *why* in the PR description (what broke, what you tested it on).
- Test on real hardware where possible and say what you tested in the PR description.

## Getting started

1. Fork the repo and clone your fork.
2. Follow [`docs/Fedora-Guide.md`](docs/Fedora-Guide.md) or the main README to get a working build.
3. Make your changes on a feature branch.
4. Open a PR against `main` with a clear description of what changed and why.
