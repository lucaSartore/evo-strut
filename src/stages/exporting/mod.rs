use std::path::Path;

use anyhow::{Result, anyhow};
use baby_shark::io::write_to_file;

use crate::{
    models::{Point, SupportSettings}, stages::{
        MeshExportedState, Pipeline, PipelineBehaviourTrait, SupportStructureRefinedState, support_structure_refinement::{ContactNode, SupportNode, SupportNodeId, SupportStructureGene, evaluation::logic::genome_to_graph_descriptor}
    }, support::shape_generation::{Circle, ShapeFactory, Sphere, TruncatedCone}
};


pub struct ExportingStage{}

impl ExportingStage {
    fn add_support_structure(builder: &mut ShapeFactory, s: &SupportStructureGene, settings: &SupportSettings) {
        Self::add_cones(builder, s, settings);
        Self::add_beams(builder, s, settings);
    }

    fn add_cones(builder: &mut ShapeFactory, s: &SupportStructureGene, settings: &SupportSettings) {
        let point_to_pos = |x: &SupportNodeId| s.nodes[x].get_position();
        s.nodes
            .values()
            .flat_map(|x| match x {
                SupportNode::Contact(x) => Some(x),
                _ => None
            })
            .for_each(|x: &ContactNode| {
                let cone_top = x.position;
                let radius = x.radius;
                x.leans_on
                    .iter()
                    .map(point_to_pos)
                    .for_each(|cone_base| {
                        Self::add_cone(builder, cone_base, cone_top, radius, settings)
                    });
            })
    }

    fn add_beams(builder: &mut ShapeFactory, s: &SupportStructureGene, settings: &SupportSettings) {
        let descriptor = genome_to_graph_descriptor(s);
        let is_contact = |x: SupportNodeId| s.nodes[&x].is_contact();
        let point_to_pos = |x: SupportNodeId| s.nodes[&x].get_position();
        descriptor
            .edges
            .iter()
            .filter(|x| !is_contact(*x.0))
            .flat_map(|(node, adjacent)|{
                adjacent
                    .iter()
                    .filter(|adj| **adj < *node)
                    .filter(|adj| !is_contact(**adj))
                    .map(|adj| (node, adj))
                    .collect::<Vec<_>>()
            })
            .map(|(a, b)| (point_to_pos(*a), point_to_pos(*b)))
            .for_each(|(a, b)| Self::add_beam(builder, a, b, settings));
    }

    fn add_beam(builder: &mut ShapeFactory, a: Point, b: Point, settings: &SupportSettings) {
        let versor = (a - b).as_versor();
        let radius = settings.beam_radius;
        let bottom = Circle::new(a, radius, versor);
        let top = Circle::new(b, radius, versor);
        let cone = TruncatedCone::new(bottom, top);
        let sphere_a = Sphere::new(a, radius);
        let sphere_b = Sphere::new(b, radius);
        builder.add_positive_shape(cone);
        builder.add_positive_shape(sphere_a);
        builder.add_positive_shape(sphere_b);
    }

    fn add_cone(builder: &mut ShapeFactory, cone_base: Point, cone_top: Point, radius_top: f32, settings: &SupportSettings ) {
        let versor = (cone_top - cone_base).as_versor();
        let base_radius = settings.beam_radius;

        let bottom = Circle::new(cone_base, base_radius, versor);
        let top = Circle::new(cone_top, radius_top, Point::UPWARD);
        let cone = TruncatedCone::new(bottom, top);
        builder.add_positive_shape(cone);

        let bottom = Circle::new(cone_base, base_radius - settings.cones_width, versor);
        let top = Circle::new(cone_top, 1e-4, Point::UPWARD);
        let cone = TruncatedCone::new(bottom, top);
        builder.add_negative_shape(cone);
    }
}


impl ExportingStage
{
    pub fn execute<TB>(
        input: Pipeline<SupportStructureRefinedState, TB>,
    ) -> Result<Pipeline<MeshExportedState, TB>>
    where TB: PipelineBehaviourTrait,
    {
        let mut builder = ShapeFactory::new();
        let settings = &input.state.settings;
        let support_settings = &settings.support_settings;

        input.state.support_structures
            .iter()
            .for_each(|s| Self::add_support_structure(&mut builder, s, support_settings));

        let mesh = builder.build(settings)?;
        let path = &settings.io_settings.output_file_path;

        write_to_file(&mesh, Path::new(path))
            .map_err(|e| anyhow!("unable to export file: {e:?}"))?;

        Ok(Pipeline::from_state(MeshExportedState {  }))
    }
}
