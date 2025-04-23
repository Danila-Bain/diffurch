// Data is pulled from https://covid.ourworldindata.org/data/owid-covid-data.json
use plotters::prelude::*;

const OUT_FILE_NAME: &str = "target/tmp/image.png";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    {
        let drawing_area = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();

        let _chart = ChartBuilder::on(&drawing_area)
            .build_cartesian_2d(0..100, 0..100)
            .unwrap();

        drawing_area.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&drawing_area)
            .build_cartesian_2d(0..100, 0..100)
            .unwrap();

        chart
            .draw_series(LineSeries::new((0..=100).map(|x| (x, 100 - x)), &BLACK))
            .unwrap();

        chart.configure_mesh().draw().unwrap();
    }

    let _ = open::that(OUT_FILE_NAME);


    Ok(())
}
#[test]
#[ignore]
fn entry_point() {
    main().unwrap();
    // std::thread::sleep(time::Duration::from_secs(1));
}
