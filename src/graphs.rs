use super::{DATA_LENGTH, XAxis, PlotType};


use plotlib::scatter::Scatter;
use plotlib::scatter;
use plotlib::style::{Marker, Point};
use plotlib::view::View;
use plotlib::page::Page;

// generates a graph based upon a boost multiplier, gas multiplier, and as price multiplier
// if x_gas is true, show gas amount as the X-Axis
pub fn score_graph<G, GP, B>(gas: G, gp: GP, boost: B, x_axis: XAxis, plot_type: PlotType)
where
    G: Fn(usize) -> f64,
    GP: Fn(usize) -> f64,
    B: Fn(f64) -> f64
{
    let mut gas_prices = Vec::new();
    let mut gases = Vec::new();
    let mut scores = vec![0.0; DATA_LENGTH];
    for i in 0..DATA_LENGTH {
        gas_prices.push(gp(i));
        gases.push(gas(i));
    }

    // reverse here because calculating from highest gp/gas -> lowest in parity-ethereum
    let last_index = DATA_LENGTH - 1;
    scores[last_index] = gas_prices[last_index];
    for idx in (1..DATA_LENGTH).rev() {
        let prev_idx = idx - 1;
        let bump = ((21_000.0 * gases[prev_idx]) /
            scores[idx]);
        scores[prev_idx] = gas_prices[prev_idx] + (bump / 1000.0);
    }

    for idx in (0..DATA_LENGTH).rev() {
        scores[idx] = boost(scores[idx]);
    }

    let data = match x_axis {
        XAxis::Gas => gases.iter().zip(scores.iter()).map(|(x, y)| (*x, *y)).collect::<Vec<(f64, f64)>>(),
        XAxis::GasPrice => gas_prices.iter().zip(scores.iter()).map(|(x, y)| (*x, *y)).collect::<Vec<(f64, f64)>>(),
    };

    plot(&data, x_axis, plot_type);
}
// X, Y; X = Gas  prices, Y = Score
fn plot(data: &[(f64, f64)], x_axis: XAxis, plot_type: PlotType) {

    let s1 = Scatter::from_vec(&data)
        .style(scatter::Style::new()
            .marker(Marker::Circle)
            .colour("#000000")
            .size(2.0));

    let (range_x, range_y) = plot_type.range();
    println!("{:?}", data);

    let v = View::new()
        .add(&s1)
        .x_range(range_x.0, range_x.1)
        .y_range(range_y.0, range_y.1)
        .x_label(x_axis.name())
        .y_label("Score");

    // println!("{}", Page::single(&v).to_text());
    // A page with a single view is then saved to an SVG file
    Page::single(&v).save(plot_type.file_name());
}
