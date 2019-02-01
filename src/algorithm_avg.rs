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

pub fn plot(data: Vec<DataSet>, name: String, gtype: GraphType) {

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
        .add(&plots[1]) // Gas Price Only Scoring
        .add(&plots[2])
        .add(&plots[3])
        .x_range(gtype.range().x.0, gtype.range().x.1)
        .y_range(gtype.range().y.0, gtype.range().y.1)
        .y_label("Score")
        .x_label(gtype.axis());
    // A page with a single view is then saved to an SVG file
    Page::single(&v).save(name.as_str());
}

