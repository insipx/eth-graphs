use super::{PlotType, DataSet};
use super::algorithms::{AlgOpts, pr, option1, option2, gas_price_only};
use super::algorithm_avg::{self, average_dataset};

use plotlib::scatter::Scatter;
use plotlib::scatter;
use plotlib::style::{Marker, Point};
use plotlib::view::View;
use plotlib::page::Page;


// enum controlling the Ranges and graph types
pub enum GraphType {
    Gas,
    GasPrice,
    GasAvg,
    GasPriceAvg,
}

pub struct Range(pub f64, pub f64);

pub struct Axes {
    pub x: Range,
    pub y: Range
}

impl GraphType {

    pub fn axis(&self) -> String {
        match *self {
            GraphType::Gas => "Gas".to_string(),
            GraphType::GasPrice => "Gas Price".to_string(),
            GraphType::GasAvg | GraphType::GasPriceAvg => "Number Consecutive Transactions".to_string(),
        }
    }

    pub fn range(&self) -> Axes {
        match *self {
            GraphType::Gas => {
                Axes {
                    x: Range(21_000.0, 250_000.0),
                    y: Range (1_000_000.0, 1_001_000.0)
                }
            },
            GraphType::GasPrice => {
                Axes {
                    x: Range (0.0, 200_000.0),
                    y: Range (0.0, 225_000.0)
                }
            },
            GraphType::GasAvg => {
                Axes {
                    x: Range (0.0, 200.0),
                    // y: Range(32_765_000_000.0, 32_805_000_000.0)
                    y: Range(1_000_000.0, 1_001_000.0)
                }
            },
            GraphType::GasPriceAvg => {
                Axes {
                    x: Range (0.0, 200.0),
                    y: Range(1_000_000.0, 1_001_000.0)
                }
            }
        }
    }
}


// X, Y; X = Gas  Y = Score
fn plot(data: Vec<DataSet>, name: String, gtype: GraphType) {
    let mut plots = Vec::new();

    // options for each data set
    for d in data.iter() {
        let s = Scatter::from_vec(d.payload.as_slice())
            .style(scatter::Style::new()
                  .colour(d.color.as_str())
                  .size(2.0));
        plots.push(s);
    }

    let v = View::new()
        .add(&plots[0])
        .add(&plots[1])
        .add(&plots[2])
        .add(&plots[3]) // Gas Price Only Scoring
        .x_range(gtype.range().x.0, gtype.range().x.1)
        .y_range(gtype.range().y.0, gtype.range().y.1)
        .y_label("Score")
        .x_label(gtype.axis());
    // A page with a single view is then saved to an SVG file
    Page::single(&v).save(name.as_str());
}


// Plots of all the different graph schemes. Parameters are decided through the closures.
// Parameter to the closure for gas/gas price is commonly the index of an iterator (0..length) in order to generate linear gas/gas prices of increasing size
pub fn plot_gas_price() {

    let opts = AlgOpts {
        gas: |_| 21_000.0,
        gp: |i: usize| (i * 1000) as f64,
        boost: |score| score,
        plot_type: PlotType::ConstantGas,
    };
    let mut data_set = Vec::new();
    data_set.push(pr(opts.clone(), 200));
    data_set.push(option1(opts.clone(), 200));
    data_set.push(option2(opts.clone(), 200));
    data_set.push(gas_price_only(opts, 200));

    plot(data_set, "GasPrice.svg".into(), GraphType::GasPrice);
}

pub fn plot_gas() {

    let opts = AlgOpts {
        gas: |i| 21_000.0 + ((100.0) * (10.0 * i as f64)),
        gp: |_| 1_000_000.0,
        boost: |score| score,
        plot_type: PlotType::ConstantGasPrice,
    };

    let mut data_set = Vec::new();
    data_set.push(pr(opts.clone(), 200));
    data_set.push(option1(opts.clone(), 200));
    data_set.push(option2(opts.clone(), 200));
    data_set.push(gas_price_only(opts, 200));
    plot(data_set, "Gas.svg".into(), GraphType::Gas);
}

pub fn plot_gas_price_avg() {

    let opts = AlgOpts {
        gas: |_| 21_000.0,
        gp: |i| 1_000_000.0,
        boost: |score| score,
        plot_type: PlotType::ConstantGas,
    };
    let mut data_set = Vec::new();
    data_set.push(average_dataset(opts.clone(), pr));
    data_set.push(average_dataset(opts.clone(), option1));
    data_set.push(average_dataset(opts.clone(), option2));
    data_set.push(average_dataset(opts, gas_price_only));

    plot(data_set, "GasPriceAvg.svg".into(), GraphType::GasPriceAvg);
}

pub fn plot_gas_avg() {

    let opts = AlgOpts {
        gas: |_| 21_000.0,
        gp: |_| 1_000_000.0,
        boost: |score| score,
        plot_type: PlotType::ConstantGasPrice,
    };

    let mut data_set = Vec::new();

    data_set.push(average_dataset(opts.clone(), gas_price_only));
    data_set.push(average_dataset(opts.clone(), pr));
    data_set.push(average_dataset(opts.clone(), option1));
    data_set.push(average_dataset(opts.clone(), option2));
    plot(data_set, "GasAvg.svg".into(), GraphType::GasAvg);
}

