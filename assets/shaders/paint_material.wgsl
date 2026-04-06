// #import bevy_pbr::forward_io::VertexOutput
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> coordinates: vec2<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> paint_colour: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var texture_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> mesh_surface_resolution: vec2<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let texel = textureSample(texture, texture_sampler, mesh.uv);

    let point = 1.0 - step(0.02, distance(mesh.uv - vec2(0.5, 0.5), coordinates / mesh_surface_resolution));

	return mix(texel, paint_colour, point);
}
