use bevy::{
    prelude::*,
    render::{
        render_graph::{Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType},
        render_resource::{
            BindGroupDescriptor, BindGroupEntry, BindingResource, FilterMode, Operations,
            PipelineCache, RenderPassColorAttachment, RenderPassDescriptor, SamplerDescriptor,
            UniformBuffer,
        },
        renderer::{RenderContext, RenderQueue},
        view::{ExtractedView, ViewTarget},
    },
};

use crate::post_processing::{
    pipeline::{CameraPostProcessingPipeline, PostProcessingPipeline},
    PostProcessing,
};

pub struct PostProcessingNode {
    query: QueryState<
        (
            &'static ViewTarget,
            &'static CameraPostProcessingPipeline,
            &'static PostProcessing,
        ),
        With<ExtractedView>,
    >,
}

impl PostProcessingNode {
    pub const IN_VIEW: &'static str = "view";

    pub fn new(world: &mut World) -> Self {
        Self {
            query: QueryState::new(world),
        }
    }
}

impl Node for PostProcessingNode {
    fn input(&self) -> Vec<SlotInfo> {
        vec![SlotInfo::new(PostProcessingNode::IN_VIEW, SlotType::Entity)]
    }

    fn update(&mut self, world: &mut World) {
        self.query.update_archetypes(world);
    }

    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let view_entity = graph.get_input_entity(Self::IN_VIEW)?;
        let pipeline_cache = world.resource::<PipelineCache>();
        let post_processing_pipeline = world.resource::<PostProcessingPipeline>();
        let render_queue = world.resource::<RenderQueue>();

        let (target, pipeline, post_processing) = match self.query.get_manual(world, view_entity) {
            Ok(result) => result,
            Err(_) => return Ok(()),
        };

        let pipeline = pipeline_cache
            .get_render_pipeline(pipeline.pipeline_id)
            .unwrap();

        let post_process = target.post_process_write();
        let source = post_process.source;
        let destination = post_process.destination;

        let device = &render_context.render_device;
        let sampler = device.create_sampler(&SamplerDescriptor {
            mipmap_filter: FilterMode::Linear,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            ..default()
        });

        let mut params_buffer = UniformBuffer::from(post_processing.params());
        params_buffer.write_buffer(device, render_queue);

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &post_processing_pipeline.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(source),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: params_buffer.binding().unwrap(),
                },
            ],
        });

        let pass_descriptor = RenderPassDescriptor {
            label: Some("post_processing_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
        };

        let mut render_pass = render_context
            .command_encoder
            .begin_render_pass(&pass_descriptor);

        render_pass.set_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}
