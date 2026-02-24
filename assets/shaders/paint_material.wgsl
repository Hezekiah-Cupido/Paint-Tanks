#import bevy_pbr::forward_io::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> coordinates: vec3<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> paint_colour: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // let point_colour = 1.0 - step(0.2, distance(mesh.world_position.xyz, coordinates));
    // let point_colour = 1.0 - step(0.2, distance(mesh.position.xy, vec2(0.5)));

    // let colour = paint_colour * vec4(point_colour);

	return paint_colour;
}
