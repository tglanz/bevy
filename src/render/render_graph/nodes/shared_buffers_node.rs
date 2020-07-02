use crate::render::{
    render_graph::{Node, ResourceSlots},
    renderer::RenderContext, render_resource::SharedBuffers,
};
use legion::prelude::*;

#[derive(Default)]
pub struct SharedBuffersNode;

impl Node for SharedBuffersNode {
    fn update(
        &mut self,
        _world: &World,
        resources: &Resources,
        render_context: &mut dyn RenderContext,
        _input: &ResourceSlots,
        _output: &mut ResourceSlots,
    ) {
        let shared_buffers = resources.get::<SharedBuffers>().unwrap();
        let mut command_queue = shared_buffers.reset_command_queue();
        command_queue.execute(render_context);
    }
}