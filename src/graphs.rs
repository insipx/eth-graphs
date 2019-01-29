use super::{DATA_LENGTH, PlotType, DataSet};
use super::algorithms::{AlgOpts, pr, option1, option2, gas_price_only};

use plotlib::scatter::Scatter;
use plotlib::scatter;
use plotlib::style::{Marker, Point};
use plotlib::view::View;
use plotlib::page::Page;

pub enum GraphType {
    Gas,
    GasPrice,
}

pub struct Range(f64, f64);

pub struct Axes {
    pub x: Range,
    pub y: Range
}

impl GraphType {

    pub fn axis(&self) -> String {
        match *self {
            GraphType::Gas => "Gas".to_string(),
            GraphType::GasPrice => "Gas Price".to_string(),
        }
    }

    pub fn range(&self) -> Axes {
        match *self {
            GraphType::Gas => {
                Axes {
                    x: Range(21_000.0, 250_000.0),
                    y: Range (688_100_000.0, 688_900_000.0)
                }
            },
            GraphType::GasPrice => {
                Axes {
                    x: Range (0.0, 225_000.0),
                    y: Range (0.0, 6_500_000_000.0)
                }
            }
        }
    }
}


// X, Y; X = Gas  prices, Y = Score
fn plot(data: Vec<DataSet>, name: String, gtype: GraphType) {

    let mut plots = Vec::new();
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



pub fn plot_gas_price() {

    let opts = AlgOpts {
        gas: |_| 21_000.0,
        gp: |i: usize| (i * 1000) as f64,
        boost: |score| ((score as u64) << 15) as f64,
        plot_type: PlotType::ConstantGas
    };
    let mut data_set = Vec::new();
    data_set.push(pr(opts.clone()));
    data_set.push(option1(opts.clone()));
    data_set.push(option2(opts.clone()));
    data_set.push(gas_price_only(opts));

    plot(data_set, "Gas_Price.svg".into(), GraphType::GasPrice);
}

pub fn plot_gas() {

    let opts = AlgOpts {
        gas: |i| 21_000.0 + ((100.0) * (10.0 * i as f64)),
        gp: |_| 21_000.0,
        boost: |score| ((score as u64) << 15) as f64,
        plot_type: PlotType::ConstantGasPrice
    };

    let mut data_set = Vec::new();
    data_set.push(pr(opts.clone()));
    data_set.push(option1(opts.clone()));
    data_set.push(option2(opts.clone()));
    data_set.push(gas_price_only(opts));
    // graphs::score_graph(|i| ((100.0) * (10.0 * i as f64)), |_| 21_000.0, |score| ((score as u64) << 15) as f64, XAxis::Gas, PlotType::ConstantGasPrice);// GOOD
    plot(data_set, "Gas.svg".into(), GraphType::Gas);
}
