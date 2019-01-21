use super::{DATA_LENGTH, PlotType, DataSet};

pub enum Algorithms {
    // Issue,
    PR,
    Option1,
    Option2,
}

#[derive(Debug, Clone)]
pub struct AlgOpts<G: Fn(usize) -> f64, GP: Fn(usize) -> f64, B: Fn(f64) -> f64> {
    pub gas: G,
    pub gp: GP,
    pub boost: B,
    pub plot_type: PlotType
}

impl Algorithms {

    pub fn color(&self) -> String {
        match self {
            Algorithms::PR => "#000000".to_string(),
            Algorithms::Option1 => "#3503ff".to_string(),
            Algorithms::Option2 => "#ff2600".to_string(),
        }
    }
}

pub fn pr<G, GP, B>(opts: AlgOpts<G, GP, B>) -> DataSet
where
    G: Fn(usize) -> f64,
    GP: Fn(usize) -> f64,
    B: Fn(f64) -> f64
{
    let mut gas_prices = Vec::new();
    let mut gases = Vec::new();
    let mut scores = vec![0.0; DATA_LENGTH];
    for i in 0..DATA_LENGTH {
        gas_prices.push((opts.gp)(i));
        gases.push((opts.gas)(i));
    }

    // reverse here because calculating from highest gp/gas -> lowest in parity-ethereum
    let last_index = DATA_LENGTH - 1;
    scores[last_index] = gas_prices[last_index];
    for idx in (1..DATA_LENGTH).rev() {
        let prev_idx = idx - 1;
        let bump = ((21_000.0 * scores[idx]) /
            gases[prev_idx]);
        scores[prev_idx] = gas_prices[prev_idx] + (bump / 1000.0);
        scores[idx] = (opts.boost)(scores[prev_idx]);
    }
    scores[0] = (opts.boost)(scores[0]);

    let data = match opts.plot_type {
        PlotType::ConstantGasPrice => gases.iter().zip(scores.iter()).map(|(x, y)| (*x, *y)).collect::<Vec<(f64, f64)>>(),
        PlotType::ConstantGas => gas_prices.iter().zip(scores.iter()).map(|(x, y)| (*x, *y)).collect::<Vec<(f64, f64)>>(),
    };

    DataSet {payload: data, color: Algorithms::color(&Algorithms::PR), plot_type: opts.plot_type.clone() }
}

pub fn option1<G, GP, B>(opts: AlgOpts<G, GP, B>) -> DataSet
where
    G: Fn(usize) -> f64,
    GP: Fn(usize) -> f64,
    B: Fn(f64) -> f64
{
    let mut gas_prices = Vec::new();
    let mut gases = Vec::new();
    let mut scores = vec![0.0; DATA_LENGTH];
    for i in 0..DATA_LENGTH {
        gas_prices.push((opts.gp)(i));
        gases.push((opts.gas)(i));
    }

    // reverse here because calculating from highest gp/gas -> lowest in parity-ethereum
    let last_index = DATA_LENGTH - 1;
    scores[last_index] = gas_prices[last_index];
    for idx in (1..DATA_LENGTH).rev() {
        let prev_idx = idx - 1;
        let bump = ((21_000.0 * scores[idx]) /
            gases[prev_idx]);
        scores[prev_idx] = gas_prices[prev_idx] + (bump / 1000.0);
    }

    for idx in (0..DATA_LENGTH).rev() {
        scores[idx] = (opts.boost)(scores[idx]);
    }

    let data = match opts.plot_type {
        PlotType::ConstantGasPrice => gases.iter().zip(scores.iter()).map(|(x, y)| (*x, *y)).collect::<Vec<(f64, f64)>>(),
        PlotType::ConstantGas => gas_prices.iter().zip(scores.iter()).map(|(x, y)| (*x, *y)).collect::<Vec<(f64, f64)>>(),
    };

    DataSet {payload: data, color: Algorithms::color(&Algorithms::Option1), plot_type: opts.plot_type.clone()}
}

pub fn option2<G, GP, B>(opts: AlgOpts<G, GP, B>) -> DataSet
where
    G: Fn(usize) -> f64,
    GP: Fn(usize) -> f64,
    B: Fn(f64) -> f64
{
    let mut gas_prices = Vec::new();
    let mut gases = Vec::new();
    let mut scores = vec![0.0; DATA_LENGTH];
    for i in 0..DATA_LENGTH {
        gas_prices.push((opts.gp)(i));
        gases.push((opts.gas)(i));
    }

    // reverse here because calculating from highest gp/gas -> lowest in parity-ethereum
    let last_index = DATA_LENGTH - 1;
    scores[last_index] = (opts.boost)(gas_prices[last_index]);
    for idx in (1..DATA_LENGTH).rev() {
        let prev_idx = idx - 1;
        let bump = ((21_000.0 * scores[idx]) /
            gases[prev_idx]);
        scores[prev_idx] = (opts.boost)(gas_prices[prev_idx]) + (bump / 1000.0);
    }

    let data = match opts.plot_type {
        PlotType::ConstantGasPrice => gases.iter().zip(scores.iter()).map(|(x, y)| (*x, *y)).collect::<Vec<(f64, f64)>>(),
        PlotType::ConstantGas => gas_prices.iter().zip(scores.iter()).map(|(x, y)| (*x, *y)).collect::<Vec<(f64, f64)>>(),
    };
    DataSet {payload: data, color: Algorithms::color(&Algorithms::Option2), plot_type: opts.plot_type.clone()}
}

