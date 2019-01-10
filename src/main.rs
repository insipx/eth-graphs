mod graphs;

pub const DATA_LENGTH: usize = 3;

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
    LinearGasPriceNoBoost,
    LinearGasPrice,
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
            PlotType::ConstantGas => "Constant Gas",
            PlotType::LinearGasPrice => "Linear Gas Price",
            PlotType::LinearGasPriceNoBoost => "Linear Gas Price No Boost"
        }
    }

    pub fn file_name(&self) -> &str {
        match self {
            PlotType::ConstantGasPriceNoBoost => "ConstantGasPriceNoBoost.svg",
            PlotType::ConstantGasNoBoost => "ConstantGasNoBoost.svg",
            PlotType::ConstantGasPrice => "ConstantGasPrice.svg",
            PlotType::ConstantGas => "ConstantGas.svg",
            PlotType::LinearGasPrice => "LinearGasPrice.svg",
            PlotType::LinearGasPriceNoBoost => "LinearGasPriceNoBoost.svg",
        }
    }

    // returns a range to correspond what x/y axes should be
    pub fn range(&self) -> ((f64, f64), (f64, f64)) {
        match self {
            PlotType::ConstantGasPriceNoBoost => ((0.0, 1_500_000.00), (20_950.0, 21_060.00)),
            PlotType::ConstantGasNoBoost => ((0.0, 1_00_000_000.0), (0.0, 110_000_000.00)),
            PlotType::ConstantGasPrice => ((0.0, 25_000.00), (19_000.00, 22_000.00)),
            PlotType::ConstantGas => ((0.0, 1_500_000.0), (0.0, 70_000_000.0)),
            PlotType::LinearGasPrice => ((0.0, 22_000.0), (0.0, 600_000_000.0)),
            PlotType::LinearGasPriceNoBoost => ((0.0, 1_0_000.0), (0.0, 420_000.0)),        }
    }
}

fn main() {

    let gas_price_multiplier = |i:usize| {
        (10u64.pow((i * 3) as u32)) as f64
    };
    // constant gas
    graphs::score_graph(|_| 21_000.0, gas_price_multiplier, |score| score, XAxis::GasPrice, PlotType::ConstantGasNoBoost);
    graphs::score_graph(|_| 21_000.0, gas_price_multiplier, |score| ((score as u64) << 15) as f64, XAxis::GasPrice, PlotType::ConstantGas);
    // graphs::score_graph(|i| ((10_000.0) * (10.0 * i as f64)), |_| 21_000.0, |score| score, XAxis::Gas, PlotType::ConstantGasPriceNoBoost); // GOOD
    // graphs::score_graph(|i| ((100.0) * (10.0 * i as f64)), |_| 21_000.0, |score| ((score as u64) << 15) as f64, XAxis::Gas, PlotType::ConstantGasPrice);// GOOD
}

fn gen_original_algorithm_plots() {
    graphs::score_graph(|i| ((100.0) * (10.0 * i as f64)), |_| 1000.0, |score| score, XAxis::Gas, PlotType::ConstantGasPriceNoBoost);
    graphs::score_graph(|_| 1_000_000.0, |i| ((100.0) * (10.0 * i as f64)), |score| score, XAxis::GasPrice, PlotType::ConstantGasNoBoost);
    graphs::score_graph(|i| ((100.0) * (10.0 * i as f64)), |_| 1000.0, |score| ((score as u64) << 15) as f64, XAxis::Gas, PlotType::ConstantGasPrice);
    graphs::score_graph(|_| 1_000_000.0, |i| ((100.0) * (10.0 * i as f64)), |score| ((score as u64) << 15) as f64, XAxis::GasPrice, PlotType::ConstantGas);
}

