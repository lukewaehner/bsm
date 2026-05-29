# Black-Scholes Model for Options Pricing in Rust

A CLI tool that prices European call and put options using the Black-Scholes model and computes the associated Greeks.

## Usage

```
bsm -p <PRICE> -t <TIME_TO_MATURITY> -v <VOLATILITY> [OPTIONS]
```

### Arguments

| Flag | Long                 | Description                                             | Default  |
| ---- | -------------------- | ------------------------------------------------------- | -------- |
| `-p` | `--price`            | Underlying asset price                                  | required |
| `-k` | `--strike`           | Strike price                                            | `100.0`  |
| `-t` | `--time-to-maturity` | Time to expiry in years (e.g. `0.5` = 6 months)         | required |
| `-v` | `--volatility`       | Implied volatility as a decimal (e.g. `0.2` = 20%)      | required |
| `-r` | `--risk-free-rate`   | Risk-free interest rate as a decimal (e.g. `0.04` = 4%) | `0.04`   |

### Example

Price an at-the-money call and put with 6 months to expiry, 20% vol, and a 4% risk-free rate:

```
bsm -p 100 -k 100 -t 0.5 -v 0.2
```

Output:

```
price: 100
Call price: 5.8284
Call greeks: delta: 0.5398, gamma: 0.0281, vega: 0.1993, theta: -0.0183, rho: 0.2330
Put price: 3.8443
Put greeks: delta: -0.4602, gamma: 0.0281, vega: 0.1993, theta: -0.0128, rho: -0.2148
```

Greeks are reported in standard per-unit form:

- **Vega** — per 1% change in volatility
- **Theta** — per calendar day

## Build

```
cargo build --release
```

The binary is placed at `target/release/bsm`.

## Dependencies

- [`clap`](https://crates.io/crates/clap) — argument parsing
- [`statrs`](https://crates.io/crates/statrs) — normal distribution CDF/PDF for BSM calculations
