# Da Vinci ATR / member roster — Rust

Small library and CLI that sanity-check **Coverage** JSON shaped like **ATR / member roster** rows before attribution files leave your payer ETL: beneficiary link, attribution **period**, and member **identifier**.

Synthetic fixtures only. Not ONC certification, not a substitute for the HL7 FHIR Validator or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 21** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [Da Vinci member attribution / ATR](http://hl7.org/fhir/us/davinci-atr/) (roster context)

## What it checks

- `resourceType` is **Coverage**
- **beneficiary.reference** present
- **period.start** for attribution window
- Warning if **identifier** is empty

| Problem | What breaks |
|--------|-------------|
| **Missing beneficiary** | Roster row cannot attach to a member |
| **No period.start** | Attribution window ambiguous in claims |
| **No member id** | File-to-FHIR reconcile fails silently |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-atr-roster -- examples/valid-roster-coverage.json
cargo run --bin validate-atr-roster -- examples/invalid-roster-coverage.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or input.

## Sample output

```bash
cargo run --bin validate-atr-roster -- examples/valid-roster-coverage.json
```

**Pass** — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-roster-coverage.json`) — exit code `1`:

```json
[
  {
    "severity": "error",
    "path": "beneficiary",
    "rule_id": "..."
  }
]
```

## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken fixtures for tests and demos

## GitHub topics

Paste in repository **About** (also: `fhir`, `hl7-fhir`, `healthcare-interoperability`, `rust`):

`da-vinci, member-attribution, roster, coverage, payer-fhir, fhir, hl7-fhir, healthcare-interoperability, rust, fhir-r4`

## Inferno vs this repo

Da Vinci attribution suites exercise live exchange. This repo is **roster row shape** in your mapper.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
