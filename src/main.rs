mod graphs;
mod algorithms;
mod algorithm_avg;
mod helpers;
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

    graphs::plot_gas_price();
    graphs::plot_gas();
    graphs::plot_gas_price_avg();
    graphs::plot_gas_avg();
}

