#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct PaintData {
    coordinates: vec2f,
    colour: vec4f,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var texture_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> mesh_surface_resolution: vec2f;

@group(#{MATERIAL_BIND_GROUP}) @binding(100) var<storage, read> paint_data: array<PaintData>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let texel = textureSample(texture, texture_sampler, mesh.uv);
    let mesh_aligned_uv = mesh.uv - vec2f(0.5, 0.5);

    var point = 1.0;
    var paint_colour = texel;

    for (var i = 0; i < i32(arrayLength(&paint_data)); i++) {
        let dist = step(0.02, distance(mesh_aligned_uv, paint_data[i].coordinates / mesh_surface_resolution));

        if (dist == 0.0) {
            point = 1.0 - dist;
            paint_colour = paint_data[i].colour;
        }
    }

	return mix(texel, paint_colour, point);
}
