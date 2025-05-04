fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut plot = pgfplots::axis::plot::Plot2D::new();

    plot.coordinates = ndarray::logspace(10., -5., 0., 400)
        .into_iter()
        .map(|t: f64| (t, (1. / t).sin()).into())
        .collect::<Vec<_>>();

    pgfplots::Picture::from(plot).show_pdf(pgfplots::Engine::PdfLatex)?;

    Ok(())
}
