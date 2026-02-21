use rerun::{
    demo_util::{bounce_lerp, color_spiral},
    external::glam,
};

pub fn visualization_test() {
    const NUM_POINTS: usize = 100;
    const TAU: f32 = 0.5;

    let rec = rerun::RecordingStreamBuilder::new("rerun_example_dna_abacus")
        .connect_grpc().unwrap();

    let (points1, colors1) = color_spiral(NUM_POINTS, 2.0, 0.02, 0.0, 0.1);
    let (points2, colors2) = color_spiral(NUM_POINTS, 2.0, 0.02, TAU * 0.5, 0.1);

    rec.log(
        "dna/structure/left",
        &rerun::Points3D::new(points1.iter().copied())
            .with_colors(colors1)
            .with_radii([0.08]),
    ).unwrap();
    rec.log(
        "dna/structure/right",
        &rerun::Points3D::new(points2.iter().copied())
            .with_colors(colors2)
            .with_radii([0.08]),
    ).unwrap();
}
