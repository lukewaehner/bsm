use core::prelude;

use clap::Parser;
mod bsm;
use bsm::{OptionType, Pricing};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Assumed price of the underlying asset
    #[arg(short = 'p', long)]
    price: f64,

    // Strike price of the option
    #[arg(short = 'k', long, default_value_t = 100.0)]
    strike: f64,

    // Time to maturity of the option in years
    #[arg(short = 't', long)]
    time_to_maturity: f64,

    // Assumed market volatility
    #[arg(short = 'v', long)]
    volatility: f64,

    // Risk-free interest rate as a decimal (e.g., 0.04 for 4%, default value is 4%)
    #[arg(short = 'r', long, default_value_t = 0.04)]
    risk_free_rate: f64,
}

fn main() {
    let args = Args::parse();

    println!("price: {}", args.price);
    for opt_type in [OptionType::Call, OptionType::Put] {
        let price = opt_type.price(
            args.price,
            args.strike,
            args.time_to_maturity,
            args.volatility,
            args.risk_free_rate,
        );
        let greeks = opt_type.greeks(
            args.price,
            args.strike,
            args.time_to_maturity,
            args.volatility,
            args.risk_free_rate,
        );

        println!("{:?} price: {:.4}", opt_type, price);

        println!(
            "{:?} greeks: delta: {:.4}, gamma: {:.4}, vega: {:.4}, theta: {:.4}, rho: {:.4}",
            opt_type,
            greeks.delta,
            greeks.gamma,
            greeks.vega / 100.0,
            greeks.theta / 365.0,
            greeks.rho / 100.0
        );
    }
}
