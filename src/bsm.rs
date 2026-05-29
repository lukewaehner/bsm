use statrs::distribution::{Continuous, ContinuousCDF, Normal};

#[derive(Debug)]
pub enum OptionType {
    Call,
    Put,
}

pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub vega: f64,
    pub theta: f64,
    pub rho: f64,
}

fn d1_d2(price: f64, strike: f64, maturity: f64, volatility: f64, rfr: f64) -> (f64, f64) {
    // d1:
    // log_strike^price + (rfr + vol^2 / 2) * maturity
    // all over
    // volatility * sqrt(maturity)
    let d1: f64 = ((price / strike).ln() + (rfr + (volatility.powi(2) / 2.0)) * maturity)
        / (volatility * maturity.sqrt());
    let d2: f64 = d1 - volatility * maturity.sqrt();
    (d1, d2)
}

pub trait Pricing {
    fn price(&self, p: f64, k: f64, t: f64, v: f64, r: f64) -> f64;
    fn greeks(&self, p: f64, k: f64, t: f64, v: f64, r: f64) -> Greeks;
}

impl Pricing for OptionType {
    fn price(&self, p: f64, k: f64, t: f64, v: f64, r: f64) -> f64 {
        let n = Normal::new(0.0, 1.0).unwrap();
        let (d1, d2) = d1_d2(p, k, t, v, r);
        let disc_rate = (-r * t).exp(); // e^-rt

        match self {
            OptionType::Call => p * n.cdf(d1) - k * disc_rate * n.cdf(d2),
            OptionType::Put => k * disc_rate * n.cdf(-d2) - p * n.cdf(-d1),
        }
    }

    fn greeks(&self, p: f64, k: f64, t: f64, v: f64, r: f64) -> Greeks {
        let n = Normal::new(0.0, 1.0).unwrap();
        let (d1, d2) = d1_d2(p, k, t, v, r);
        let disc_rate = (-r * t).exp(); // e^-rt

        let gamma = n.pdf(d1) / (p * v * t.sqrt());
        let vega = p * n.pdf(d1) * t.sqrt();
        // Vega is often per 1% change in vol, so we can
        // divide by 100 later
        match self {
            OptionType::Call => Greeks {
                delta: n.cdf(d1),
                gamma,
                vega,
                theta: ((p * n.pdf(d1) * v) / (2.0 * t.sqrt())) - (r * k * disc_rate * n.cdf(d2)),
                rho: k * t * disc_rate * n.cdf(d2),
            },
            OptionType::Put => Greeks {
                delta: n.cdf(d1) - 1.0,
                gamma,
                vega,
                theta: -((p * n.pdf(d1) * v) / (2.0 * t.sqrt())) + (r * k * disc_rate * n.cdf(-d2)),
                rho: -k * t * disc_rate * n.cdf(-d2),
            },
        }
    }
}

