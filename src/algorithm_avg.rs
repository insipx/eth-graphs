use super::algorithms::{AlgOpts, option1, gas_price_only};
use super::helpers::average;
use super::graphs::GraphType;
use super::{PlotType, DataSet};

use plotlib::scatter::Scatter;
use plotlib::scatter;
use plotlib::style::{Marker, Point};
use plotlib::view::View;
use plotlib::page::Page;

use std::clone::Clone;

pub fn average_dataset<G, GP, B>(opts: AlgOpts<G, GP, B>, fun: fn(AlgOpts<G, GP, B>, usize) -> DataSet) -> DataSet
where
    G: Fn(usize) -> f64 + std::clone::Clone,
    GP: Fn(usize) -> f64 + std::clone::Clone,
    B: Fn(f64) -> f64 + std::clone::Clone,
{
    let mut averages = Vec::new();
    let mut result = None;
    for i in 2..200 {
        let data = fun(opts.clone(), i);
        averages.push((i as f64, average(data.payload.iter().map(|(x, y)| *y).collect::<Vec<f64>>())));
        result = Some(data)
    }
    if let Some(res) = result {
        DataSet {
            color: res.color,
            plot_type: res.plot_type,
            payload: averages
        }
    } else {
        panic!("Empty Result");
    }
}



