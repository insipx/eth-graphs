
pub enum Algorithms {
    Issue,
    PR,
    Option1,
    Option2,
}


impl Algorithms {

    fn get<G, GP, B>(&self, gas: G, gp: GP, boost: B, x_axis: XAxis, plot_type: PlotType) {
        match self {
            Algorithms::Issue => ,
            Algorithms::PR => ,
            Algorithms::Option1 => ,
            Algorithms::Option2 => ,
        }
    }
}
