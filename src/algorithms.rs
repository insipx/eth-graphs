use super::{PlotType, DataSet};


pub enum Algorithms {
    PR,
    Option1,
    Option2,
    GasPriceOnly,
}
impl Algorithms {
    pub fn color(&self) -> String {
        match self {
            Algorithms::PR => "#ff2600".to_string(), // red
            Algorithms::Option1 => "#3503ff".to_string(),  // blue
            Algorithms::Option2 => "#000000".to_string(), // black
            Algorithms::GasPriceOnly => "#008711".to_string(), // green
        }
    }
}

// closures controlling how much Gas/Gas Price go up, and how much it is boosted by. PlotType is an indicator for whether the
// gas or gas price is being kept constant
#[derive(Debug, Clone)]
pub struct AlgOpts<G: Fn(usize) -> f64, GP: Fn(usize) -> f64, B: Fn(f64) -> f64> {
    pub gas: G,
    pub gp: GP,
    pub boost: B,
    pub plot_type: PlotType,
}

pub fn pr<G, GP, B>(opts: AlgOpts<G, GP, B>, length: usize) -> DataSet
where
    G: Fn(usize) -> f64,
    GP: Fn(usize) -> f64,
    B: Fn(f64) -> f64
{
    let mut scores = vec![0.0; length];
    let (gas, gas_price) = generate_gas_vectors(opts.gas, opts.gp, length);

    // reverse here because calculating from highest gp/gas -> lowest in parity-ethereum
    let last_index = length - 1;
    scores[last_index] = gas_price[last_index];
    for idx in (1..length).rev() {
        let prev_idx = idx - 1;
        let bump = ((21_000.0 * scores[idx]) /
            gas[prev_idx]);
        scores[prev_idx] = gas_price[prev_idx] + (bump / 1000.0);
        scores[idx] = (opts.boost)(scores[prev_idx]);
    }
    scores[0] = (opts.boost)(scores[0]);

    collect_data(opts.plot_type, gas, gas_price, scores, Algorithms::color(&Algorithms::PR))
}

pub fn option1<G, GP, B>(opts: AlgOpts<G, GP, B>, length: usize) -> DataSet
where
    G: Fn(usize) -> f64,
    GP: Fn(usize) -> f64,
    B: Fn(f64) -> f64
{

    let mut scores = vec![0.0; length];
    let (gas, gas_price) = generate_gas_vectors(opts.gas, opts.gp, length);

    // reverse here because calculating from highest gp/gas -> lowest in parity-ethereum
    let last_index = length - 1;
    scores[last_index] = gas_price[last_index];
    for idx in (1..length).rev() {
        let prev_idx = idx - 1;
        let bump = ((21_000.0 * scores[idx]) /
            gas[prev_idx]);
        scores[prev_idx] = gas_price[prev_idx] + (bump / 1000.0);
    }

    for idx in (0..length).rev() {
        scores[idx] = (opts.boost)(scores[idx]);
    }

    collect_data(opts.plot_type, gas, gas_price, scores, Algorithms::color(&Algorithms::Option1))
}

pub fn option2<G, GP, B>(opts: AlgOpts<G, GP, B>, length: usize) -> DataSet
where
    G: Fn(usize) -> f64,
    GP: Fn(usize) -> f64,
    B: Fn(f64) -> f64
{
    let mut scores = vec![0.0; length];
    let (gas, gas_price) = generate_gas_vectors(opts.gas, opts.gp, length);

    // reverse here because calculating from highest gp/gas -> lowest in parity-ethereum
    let last_index = length - 1;
    scores[last_index] = (opts.boost)(gas_price[last_index]);
    for idx in (1..length).rev() {
        let prev_idx = idx - 1;
        let bump = ((21_000.0 * scores[idx]) /
            gas[prev_idx]);
        scores[prev_idx] = (opts.boost)(gas_price[prev_idx]) + (bump / 1000.0);
    }

    collect_data(opts.plot_type, gas, gas_price, scores, Algorithms::color(&Algorithms::Option2))
}


pub fn gas_price_only<G, GP, B>(opts: AlgOpts<G, GP, B>, length: usize) -> DataSet
where
    G: Fn(usize) -> f64,
    GP: Fn(usize) -> f64,
    B: Fn(f64) -> f64
{
    let mut scores = vec![0.0; length];
    let (gas, gas_price) = generate_gas_vectors(opts.gas, opts.gp, length);

    for i in 0..length {
        scores[i] = (opts.boost)(gas_price[i]);
    }

    collect_data(opts.plot_type, gas, gas_price, scores, Algorithms::color(&Algorithms::GasPriceOnly))
}

// HELPERS
fn generate_gas_vectors<G, GP>(gas: G, gas_price: GP, length: usize) -> (Vec<f64>, Vec<f64>)
where
    G: Fn(usize) -> f64,
    GP: Fn(usize) -> f64
{
    let mut gas_prices = Vec::new();
    let mut gases = Vec::new();

    for i in 0..length {
        gas_prices.push(gas_price(i));
        gases.push(gas(i));
    }

    (gases, gas_prices)
}

// Collect data depending on whether gas or gas price constant; this will decide whether gas or gas price is on X-Axis
fn collect_data(plot_type: PlotType, gas: Vec<f64>, gas_price: Vec<f64>, scores: Vec<f64>, color: String) -> DataSet {
    let data = match plot_type {
        PlotType::ConstantGasPrice => gas.iter().zip(scores.iter()).map(|(x, y)| (*x, *y)).collect::<Vec<(f64, f64)>>(),
        PlotType::ConstantGas => gas_price.iter().zip(scores.iter()).map(|(x, y)| (*x, *y)).collect::<Vec<(f64, f64)>>(),
    };
    DataSet {payload: data, color: color, plot_type: plot_type}
}
