use std::{fs::File, io::BufWriter, path::Path, sync::Arc};

use anyhow::{anyhow, Result};
use baby_shark::{io::write_to_file, voxel::prelude::MeshToVolume};

use crate::{
    models::{Point, SupportSettings, SurfaceGraph},
    stages::{
        support_structure_refinement::{
            evaluation::logic::genome_to_graph_descriptor, ContactNode, SupportNode, SupportNodeId,
            SupportStructureGene,
        },
        visualization::visualize_final_supports,
        MeshExportedState, Pipeline, PipelineBehaviourTrait, SupportStructureRefinedState,
    },
    support::shape_generation::{Circle, ShapeFactory, Sphere, TruncatedCone},
};

pub struct ExportingStage {}

impl ExportingStage {
    fn add_support_structure(
        builder: &mut ShapeFactory,
        s: &SupportStructureGene,
        settings: &SupportSettings,
    ) {
        Self::add_cones(builder, s, settings);
        Self::add_beams(builder, s, settings);
    }

    fn add_cones(builder: &mut ShapeFactory, s: &SupportStructureGene, settings: &SupportSettings) {
        let point_to_pos = |x: &SupportNodeId| s.nodes[x].get_position();
        s.nodes
            .values()
            .flat_map(|x| match x {
                SupportNode::Contact(x) => Some(x),
                _ => None,
            })
            .for_each(|x: &ContactNode| {
                let cone_top = x.position;
                let radius = x.radius.max(settings.min_cone_radius);
                x.leans_on.iter().map(point_to_pos).for_each(|cone_base| {
                    Self::add_cone(builder, cone_base, cone_top, radius, settings)
                });
            })
    }

    fn add_beams(builder: &mut ShapeFactory, s: &SupportStructureGene, settings: &SupportSettings) {
        let descriptor = genome_to_graph_descriptor(s);
        let is_contact = |x: SupportNodeId| s.nodes[&x].is_contact();
        let point_to_pos = |x: SupportNodeId| s.nodes[&x].get_position();
        let point_to_rad = |x: SupportNodeId| s.nodes[&x].radius();
        descriptor
            .edges
            .iter()
            .filter(|x| !is_contact(*x.0))
            .flat_map(|(node, adjacent)| {
                adjacent
                    .iter()
                    .filter(|adj| **adj < *node)
                    .filter(|adj| !is_contact(**adj))
                    .map(|adj| (node, adj))
                    .collect::<Vec<_>>()
            })
            .map(|(a, b)| {
                (
                    point_to_pos(*a),
                    point_to_pos(*b),
                    point_to_rad(*a),
                    point_to_rad(*b),
                )
            })
            .for_each(|(a, b, ra, rb)| Self::add_beam(builder, a, b, ra, rb, settings));
    }

    fn add_beam(
        builder: &mut ShapeFactory,
        a: Point,
        b: Point,
        ra: f32,
        rb: f32,
        settings: &SupportSettings,
    ) {
        let (point_bottom, point_top, r_bottom, r_top) = if a.z < b.z {
            (a, b, ra, rb)
        } else {
            (b, a, rb, ra)
        };

        let versor = (point_top - point_bottom).as_versor();

        let bottom_too_close_to_ground = point_bottom.z <= r_bottom;
        let top_too_close_to_ground = point_top.z <= r_top;

        let circle_bottom = Circle::new(
            point_bottom,
            r_bottom,
            if bottom_too_close_to_ground {
                Point::UPWARD
            } else {
                versor
            },
        );
        let circle_top = Circle::new(
            point_top,
            r_top,
            if top_too_close_to_ground {
                Point::UPWARD
            } else {
                versor
            },
        );

        let cone = TruncatedCone::new(circle_bottom, circle_top, 10e9, 10e9);
        builder.add_positive_shape(cone);

        if !top_too_close_to_ground {
            let sphere = Sphere::new(point_top, r_top);
            builder.add_positive_shape(sphere);
        }
        if !bottom_too_close_to_ground {
            let sphere = Sphere::new(point_bottom, r_bottom);
            builder.add_positive_shape(sphere);
        }

        if point_bottom.z <= 0. {
            let circle_bottom =
                Circle::new(point_bottom, settings.base_cylinder_radius, Point::UPWARD);
            let circle_top = Circle::new(
                point_bottom + Point::UPWARD.to_scaled(settings.base_cylinder_height),
                settings.base_cylinder_radius,
                Point::UPWARD,
            );
            let cone = TruncatedCone::new(circle_bottom, circle_top, 10e9, 10e9);
            builder.add_positive_shape(cone);
        }
    }

    fn add_cone(
        builder: &mut ShapeFactory,
        cone_base: Point,
        cone_top: Point,
        radius_top: f32,
        settings: &SupportSettings,
    ) {
        let versor = (cone_top - cone_base).as_versor();
        let base_radius = settings.beam_radius;

        let bottom_versor = if cone_base.z < base_radius {
            Point::UPWARD
        } else {
            versor
        };
        let bottom = Circle::new(cone_base, base_radius, bottom_versor);
        let top = Circle::new(cone_top, radius_top, Point::UPWARD);
        let cone = TruncatedCone::new(
            bottom,
            top,
            settings.cone_thickness,
            settings.min_cone_thickness_for_hole,
        );
        builder.add_positive_shape(cone);
    }
}

impl ExportingStage {
    pub fn execute<TB>(
        input: Pipeline<SupportStructureRefinedState, TB>,
    ) -> Result<Pipeline<MeshExportedState, TB>>
    where
        TB: PipelineBehaviourTrait,
    {
        let mut builder = ShapeFactory::new();
        let settings = &input.state.settings;
        let support_settings = &settings.support_settings;

        input
            .state
            .support_structures
            .iter()
            .for_each(|s| Self::add_support_structure(&mut builder, s, support_settings));

        // cutting out the mesh we are printing from the area of supports
        let volume = MeshToVolume::default()
            .with_voxel_size(support_settings.voxel_size)
            .convert(&input.state.graph.mesh.original)
            .ok_or(anyhow!("fail in creation of volume"))?;
        let volume = volume.offset(support_settings.support_detachment);
        builder.add_negative_volume(volume);

        let mesh = builder.build(settings)?;
        let path = &settings.io_settings.output_file_path;

        let mesh_rc = Arc::new(mesh.clone().into());
        let graph = SurfaceGraph::new(&mesh_rc);
        visualize_final_supports(&graph, &input.state.graph)?;

        write_to_file(&mesh, Path::new(path))
            .map_err(|e| anyhow!("unable to export file: {e:?}"))?;

        let path = &settings.io_settings.output_json_path;
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &input.state.support_structures)?;

        Ok(Pipeline::from_state(MeshExportedState {}))
    }
}
