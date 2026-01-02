# IFB Architecture

## Core loop (baseline)

- **In-process LibAFL** with `StdFuzzer::fuzz_loop`.
- **Coverage-guided** feedback via a shared map observer.
- **Mutations** use `HavocScheduledMutator` plus a structure-aware URL mutator.
- **Default mode is no-LLM** and optimized for throughput.

## Target modes

- **libcurl (default):** URL parsing path via the in-process harness.
- **dummy target:** set `IFB_TARGET=dummy` for a fast local smoke target.

## LLM mode (opt-in seed injection)

When built with `--features llm`, an optional background seed generator is available.

- **Not in the hot loop:** an async worker generates seeds only on stagnation.
- **Trigger:** no new coverage for `--llm-stagnation-secs`.
- **Validation:** seeds are sanitized and only kept if they add coverage/solutions.
- **Memory:** only successful seeds are kept in a bounded history.
- **Failure handling:** timeouts and retries are capped, and failure falls back to baseline.
