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

fn main() {

    let gas_price_multiplier = |i:usize| {
        (i * 1000) as f64
    };

    graphs::plot_gas_price();
    graphs::plot_gas();
}

