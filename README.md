# commit-miner

Classify Git commit diffs and messages with an LLM via [OpenRouter](https://openrouter.ai). Bug fixes, security fixes/CWEs, and change types.

## Install

Requires Rust/Cargo and Git.

```bash
git clone https://github.com/r0075h3ll/commit-miner.git
cd commit-miner
cargo install --path . --locked
export PATH="$HOME/.cargo/bin:$PATH"
export OPENROUTER_API_KEY='your-key'
```

Get a key at [openrouter.ai/keys](https://openrouter.ai/keys).

## Use

```bash
commit-miner scan . -n 500
commit-miner scan https://github.com/owner/repo -n 500
commit-miner scan . --commit SHA_OR_REF
commit-miner scan . --since 2026-01-01 --until 2026-03-31 -o report.html
commit-miner scan . -n 500 -o report.csv
commit-miner scan . -n 500 --model openai/gpt-5-mini
```

| Argument | Meaning |
| --- | --- |
| `-n, --commits N` | Latest N; default 250, or all matches with dates |
| `--commit SHA_OR_REF` | Exactly one commit; cannot combine with count/dates |
| `--since / --until YYYY-MM-DD` | Inclusive UTC committer dates |
| `-w, --workers N` | Default and maximum 8 |
| `--model ID` | OpenRouter model ID; also settable via `OPENROUTER_MODEL`. Default: `meta-llama/llama-3.3-70b-instruct` |
| `-f, --format html\|csv` | Report format; stdout if no output path |
| `-o, --output PATH` | Save report; infer format from extension |
| `--only security,change_performance` | Filter displayed classifications |
| `--cwe 79,89` | Filter by CWE |
| `--plain` | Disable color and animation |

`commit-miner scan --help` lists all arguments, including the full recommended model list. `commit-miner categories` lists classifications/CWEs. Filters preserve the full saved scan. Managed GitHub clones refresh automatically; local directories use their current HEAD.

### Choosing a model

Any [OpenRouter model](https://openrouter.ai/models) that supports structured JSON outputs works. Recommended:

| Kind | Model ID | Notes |
| --- | --- | --- |
| Open-weight (default) | `meta-llama/llama-3.3-70b-instruct` | Good accuracy/cost balance |
| Open-weight | `meta-llama/llama-4-scout`, `meta-llama/llama-4-maverick` | Larger context, MoE |
| Open-weight | `qwen/qwen-2.5-72b-instruct`, `qwen/qwen3-30b-a3b` | Strong at code |
| Open-weight | `deepseek/deepseek-chat-v3.1`, `mistralai/mistral-small-3.2-24b-instruct` | Cheapest open-weight options |
| Low-cost GPT | `openai/gpt-4o-mini`, `openai/gpt-4.1-mini`, `openai/gpt-4.1-nano` | |
| Low-cost GPT | `openai/gpt-5-mini`, `openai/gpt-5-nano` | Cheapest GPT options |

New scans show estimated cost from reported token usage and known model pricing for the models above. [Calculation and protocol details](docs/PROTOCOL.md#cost).

## Saved scans

```bash
commit-miner list
commit-miner show SCAN_ID --only security
commit-miner export SCAN_ID -o report.html
```

Saved locally (`~/.local/share/commit-miner` on Linux); override with `--data-dir PATH`. Ctrl+C retains completed results.

## Agent skill

```bash
commit-miner install-skill all
# Or: commit-miner install-skill codex|claude|opencode (choose one)
```

Restart your harness. The bundled [SKILL.md](skills/commit-miner/SKILL.md) includes binary installation, arguments, defaults, examples, and saved-result commands. For manual installation, copy `skills/commit-miner/` into your harness’s skills directory.
