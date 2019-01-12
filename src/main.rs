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
            PlotType::ConstantGasPriceNoBoost => ((0.0, 21_000_000.00), (20_900.0, 21_200.00)),
            PlotType::ConstantGasNoBoost => ((0.0, 200_000.0), (0.0, 200_000.00)),
            PlotType::ConstantGasPrice => ((0.0, 200_000.00), (685_000_000.0, 696_000_000.0)),
            PlotType::ConstantGas => ((0.0, 200_000.0), (0.0, 6_500_000_000.0)),
            PlotType::LinearGasPrice => ((0.0, 22_000.0), (0.0, 600_000_000.0)),
            PlotType::LinearGasPriceNoBoost => ((0.0, 1_0_000.0), (0.0, 420_000.0)),        }
    }
}

fn main() {

    let gas_price_multiplier = |i:usize| {
        (i * 1000) as f64
    };
    // constant gas
    graphs::score_graph(|_| 21_000.0, gas_price_multiplier, |score| score, XAxis::GasPrice, PlotType::ConstantGasNoBoost);
    graphs::score_graph(|_| 21_000.0, gas_price_multiplier, |score| ((score as u64) << 15) as f64, XAxis::GasPrice, PlotType::ConstantGas);
    graphs::score_graph(|i| ((10_000.0) * (10.0 * i as f64)), |_| 21_000.0, |score| score, XAxis::Gas, PlotType::ConstantGasPriceNoBoost); // GOOD
    graphs::score_graph(|i| ((100.0) * (10.0 * i as f64)), |_| 21_000.0, |score| ((score as u64) << 15) as f64, XAxis::Gas, PlotType::ConstantGasPrice);// GOOD
}

fn gen_original_algorithm_plots() {
    graphs::score_graph(|i| ((100.0) * (10.0 * i as f64)), |_| 1000.0, |score| score, XAxis::Gas, PlotType::ConstantGasPriceNoBoost);
    graphs::score_graph(|_| 1_000_000.0, |i| ((100.0) * (10.0 * i as f64)), |score| score, XAxis::GasPrice, PlotType::ConstantGasNoBoost);
    graphs::score_graph(|i| ((100.0) * (10.0 * i as f64)), |_| 1000.0, |score| ((score as u64) << 15) as f64, XAxis::Gas, PlotType::ConstantGasPrice);
    graphs::score_graph(|_| 1_000_000.0, |i| ((100.0) * (10.0 * i as f64)), |score| ((score as u64) << 15) as f64, XAxis::GasPrice, PlotType::ConstantGas);
}

