# Security Policy

## Supported Versions

This repository is maintained for learning and experimentation. It is not intended for production use or security-critical applications.

| Version | Supported |
| ------- | --------- |
| `main`  | ✅         |

## Reporting Vulnerabilities

If you discover a security issue (e.g. unsafe Rust usage, panics from malformed CLI input, or logic bugs):

1. Open a GitHub Issue with `[SECURITY]` in the title.
2. Provide clear reproduction steps or example input that triggers the issue.
3. Explain the impact — what could go wrong, and under what conditions.

You do **not** need to email or privately disclose. This is a transparent, educational project.

## Out of Scope

* Vulnerabilities from unimplemented features (e.g. future `--inputs` CLI parsing)
* Crashes from deliberately malformed test code
* Performance inefficiencies that don't compromise correctness or safety

## Crate Usage

This project avoids unsafe blocks and uses a minimal dependency set (e.g. `clap` for CLI parsing). If a future dependency introduces a known CVE, feel free to open a PR or issue referencing the affected crate.

---

If you're unsure whether something qualifies as a vulnerability, err on the side of opening an issue — better safe than silent.
