mod graphs;

pub const DATA_LENGTH: usize = 200;

pub enum XAxis {
    Gas,
    GasPrice
}

impl XAxis {
    pub fn name(&self) -> &str {
        match self {
            XAxis::Gas => "Gas",
            XAxis::GasPrice => "Gas Price"
        }
    }
}

pub enum PlotType {
    ConstantGasPriceNoBoost,
    ConstantGasNoBoost,
    ConstantGasPrice,
    ConstantGas,
}

impl PlotType {
    pub fn name(&self) -> &str {
        match self {
            PlotType::ConstantGasPriceNoBoost => "Constant Gas Price No Boost",
            PlotType::ConstantGasNoBoost => "Constant Gas No Boost",
            PlotType::ConstantGasPrice => "Constant Gas Price",
            PlotType::ConstantGas => "Constant Gas"
        }
    }

    pub fn file_name(&self) -> &str {
        match self {
            PlotType::ConstantGasPriceNoBoost => "ConstantGasPriceNoBoost.svg",
            PlotType::ConstantGasNoBoost => "ConstantGasNoBoost.svg",
            PlotType::ConstantGasPrice => "ConstantGasPrice.svg",
            PlotType::ConstantGas => "ConstantGas.svg"
        }
    }

    // returns a range to correspond what x/y axes should be
    pub fn range(&self) -> ((f64, f64), (f64, f64)) {
        match self {
            PlotType::ConstantGasPriceNoBoost => ((0.0, 25_000.00), (0.0, 60_000_000_000.00)),
            PlotType::ConstantGasNoBoost => ((0.0, 210_000.00), (0.0, 1_000_000.00)),
            PlotType::ConstantGasPrice => ((0.0, 25_000.00), (0.0, 150_000_000_000.00)),
            PlotType::ConstantGas => ((0.0, 210_000.0), (0.0, 8_000_000.0)),
        }
    }
}

fn main() {
    // constant gas
    graphs::score_graph(1_000_000.0, |i| ((100.0) * (10.0 * i as f64)), |_| 1000.0, |score| score, XAxis::Gas, PlotType::ConstantGasPriceNoBoost);
    graphs::score_graph(1_000_000.0, |_| 1_000_000.0, |i| ((100.0) * (10.0 * i as f64)), |score| score, XAxis::GasPrice, PlotType::ConstantGasNoBoost);
    graphs::score_graph(1_000_000.0, |i| ((100.0) * (10.0 * i as f64)), |_| 1000.0, |score| ((score as u64) << 15) as f64, XAxis::Gas, PlotType::ConstantGasPrice);
    graphs::score_graph(1_000_000.0, |_| 1_000_000.0, |i| ((100.0) * (10.0 * i as f64)), |score| ((score as u64) << 15) as f64, XAxis::GasPrice, PlotType::ConstantGas);
}

pub fn algo<F>(score: f64, gas: f64, gp: f64, boost: &F) -> f64
where
    F: Fn(f64) -> f64
{
    let bump = ((21_000.00 * score) /
               gas);
    let score = gp + (bump / 1000.00);
    score
}
