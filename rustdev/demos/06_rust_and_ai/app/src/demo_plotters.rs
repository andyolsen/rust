use ndarray::prelude::*;
use plotters::prelude::*;

pub fn do_demo() {
    let _ = create_image_from_my_data();
    println!("Done - see my_image.png")
}

fn create_image_from_my_data() -> Result<(), Box<dyn std::error::Error>> {

    println!("\ncreate_image_from_my_data");
    println!("--------------------------------------");

    let y = array!(0, 1, 19, 76, 45, 34, 42, 30, 5, 77, 54);

    let points = y.iter()
        .enumerate()
        .map(|(i, &y)| (i as i32, y))
        .collect::<Vec<(i32, i32)>>();

    let root = BitMapBackend::new("my_image.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Andy's Cool Line Plot", ("courier new", 50).into_font().style(FontStyle::Bold).color(&RED))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..10, 0..100)?;

    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(points, &BLUE))?;

    Ok(())
}