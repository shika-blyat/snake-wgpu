use wgpu::*;

use super::{renderer::Renderer, vertex::Vertex};

pub struct Frame<'a> {
    pub renderer: &'a mut Renderer,
    pub frame: SwapChainOutput,
    pub vertices: Vec<Vertex>,
}

impl<'a> Frame<'a> {
    pub fn new(renderer: &'a mut Renderer, frame: SwapChainOutput) -> Self {
        Frame {
            renderer,
            frame,
            vertices: vec![],
        }
    }
    pub async fn render(self) {
        let mut encoder =
            self.renderer
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &self.frame.view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                },
            }],
            depth_stencil_attachment: None,
        });
        let buffer = self.renderer.vertex_buffer.map_write(
            0,
            (self.vertices.len() * std::mem::size_of::<Vertex>()) as u64,
        );
        self.renderer.device.poll(Maintain::Wait);
        let mut buffer = buffer.await.expect("Failed to write in the vertex buffer");
        buffer
            .as_slice()
            .copy_from_slice(bytemuck::cast_slice(&self.vertices));

        render_pass.set_pipeline(&self.renderer.render_pipeline);
        render_pass.set_vertex_buffer(0, &self.renderer.vertex_buffer, 0, 0);
        render_pass.draw(0..self.vertices.len() as u32, 0..1);
        std::mem::drop(render_pass);
        self.renderer.queue.submit(&[encoder.finish()]);
    }
}
