mod node;
mod pipeline;

use bevy::{
    asset::load_internal_asset,
    core_pipeline::core_3d,
    ecs::query::QueryItem,
    prelude::*,
    reflect::TypeUuid,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_graph::RenderGraph,
        render_resource::{ShaderType, SpecializedRenderPipelines},
        RenderApp, RenderStage,
    },
};

use self::{
    node::PostProcessingNode,
    pipeline::{prepare_post_processing_pipelines, PostProcessingPipeline},
};

const POST_PROCESSING_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 9918465827091940976);
const POST_PROCESSING: &str = "post_processing";

pub struct PostProcessingPlugin;

impl Plugin for PostProcessingPlugin {
    fn build(&self, app: &mut App) {
        // app
        load_internal_asset!(
            app,
            POST_PROCESSING_SHADER_HANDLE,
            "post_processing.wgsl",
            Shader::from_wgsl
        );
        app.add_plugin(ExtractComponentPlugin::<PostProcessing>::default());

        // render app
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .init_resource::<PostProcessingPipeline>()
            .init_resource::<SpecializedRenderPipelines<PostProcessingPipeline>>()
            .add_system_to_stage(RenderStage::Prepare, prepare_post_processing_pipelines);

        let node = PostProcessingNode::new(&mut render_app.world);
        let mut binding = render_app.world.resource_mut::<RenderGraph>();
        let graph = binding.get_sub_graph_mut(core_3d::graph::NAME).unwrap();
        graph.add_node(POST_PROCESSING, node);
        graph
            .add_slot_edge(
                graph.input_node().unwrap().id,
                core_3d::graph::input::VIEW_ENTITY,
                POST_PROCESSING,
                PostProcessingNode::IN_VIEW,
            )
            .unwrap();
        graph
            .add_node_edge(core_3d::graph::node::TONEMAPPING, POST_PROCESSING)
            .unwrap();
        graph
            .add_node_edge(
                POST_PROCESSING,
                core_3d::graph::node::END_MAIN_PASS_POST_PROCESSING,
            )
            .unwrap();
    }
}

#[derive(Component, Clone)]
pub struct PostProcessing {
    pub aberration: f32,
}

impl PostProcessing {
    pub fn new(aberration: f32) -> Self {
        Self { aberration }
    }

    fn params(&self) -> PostProcessingParams {
        self.into()
    }
}

impl ExtractComponent for PostProcessing {
    type Query = &'static Self;
    type Filter = With<Camera>;

    fn extract_component(item: QueryItem<Self::Query>) -> Self {
        item.clone()
    }
}

#[derive(ShaderType)]
struct PostProcessingParams {
    aberration: f32,
    wasm_padding_1: f32,
    wasm_padding_2: f32,
    wasm_padding_3: f32,
}

impl From<&PostProcessing> for PostProcessingParams {
    fn from(post_processing: &PostProcessing) -> Self {
        Self {
            aberration: post_processing.aberration,
            wasm_padding_1: 0.0,
            wasm_padding_2: 0.0,
            wasm_padding_3: 0.0,
        }
    }
}
