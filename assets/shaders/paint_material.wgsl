#import bevy_pbr::{
    mesh_view_bindings::view,
    forward_io::VertexOutput,
    utils::coords_to_viewport_uv,
    view_transformations::position_world_to_clip,
}
// struct VertexOutput {
//     @builtin(position) position: vec4<f32>,
//     @location(2) uv: vec2<f32>,
// }

// struct PaintMaterial {
//     coordinates: vec2<f32>,
//     colour: vec4<f32>,
// }

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> coordinates: vec3<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> paint_colour: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // let clip_position = position_world_to_clip(coordinates);
    // let viewport_uv = coords_to_viewport_uv(coordinates, view.viewport);

    let point_colour = 1.0 - step(0.2, distance(mesh.world_position.xyz, coordinates));

    let colour = paint_colour * vec4(vec3(point_colour), 1.0);

	return colour;
}
