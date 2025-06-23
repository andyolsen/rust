use app::demo_ndarray;
use app::demo_polars;
use app::demo_plotters;
use app::demo_linfa;

fn main() {
    demo_ndarray::do_demo();
    demo_polars::do_demo();
    demo_plotters::do_demo();
    demo_linfa::do_demo();
}