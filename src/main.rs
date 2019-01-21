mod graphs;
mod algorithms;

pub const DATA_LENGTH: usize = 200;

// gas price w/ boost

#[derive(Debug)]
pub struct DataSet {
    payload: Vec<(f64, f64)>,
    plot_type: PlotType,
    color: String,
}

#[derive(Debug, Clone)]
pub enum PlotType {
    ConstantGasPrice,
    ConstantGas,
}

impl PlotType {

    pub fn axis(&self) -> &str {
        match self {
            PlotType::ConstantGasPrice => "Gas",
            PlotType::ConstantGas => "Gas Price",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            PlotType::ConstantGasPrice => "Constant Gas Price",
            PlotType::ConstantGas => "Constant Gas",
        }
    }

    pub fn file_name(&self) -> &str {
        match self {
            PlotType::ConstantGasPrice => "ConstantGasPrice.svg",
            PlotType::ConstantGas => "ConstantGas.svg",
        }
    }

    // returns a range to correspond what x/y axes should be
    pub fn range(&self) -> ((f64, f64), (f64, f64)) {
        match self {
            PlotType::ConstantGasPrice => ((0.0, 200_000.00), (685_000_000.0, 696_000_000.0)),
            PlotType::ConstantGas => ((0.0, 200_000.0), (0.0, 6_500_000_000.0)),
        }
    }
}

fn main() {

    let gas_price_multiplier = |i:usize| {
        (i * 1000) as f64
    };

    graphs::plot_gas_price();
    graphs::plot_gas();
}

