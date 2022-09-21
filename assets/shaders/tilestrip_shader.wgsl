struct View {
    view_proj: mat4x4<f32>,
    inverse_view_proj: mat4x4<f32>,
    view: mat4x4<f32>,
    inverse_view: mat4x4<f32>,
    projection: mat4x4<f32>,
    inverse_projection: mat4x4<f32>,
    world_position: vec3<f32>,
    viewport: vec4<f32>,
};

struct TileStripUniform {
    len: f32,
    atlas_coords: array<vec4<f32>, 1000u>,
}

@group(0) @binding(0)
var<uniform> view: View;

@group(1) @binding(0)
var color_texture: texture_2d<f32>;

@group(1) @binding(1)
var color_sampler: sampler;

@group(2) @binding(0)
var<uniform> tile_strip_uniform: TileStripUniform;

struct VertexOutput {
    @location(0) uv: vec2<f32>,
    @builtin(position) position: vec4<f32>,
}

@vertex
fn vertex(
    @location(0) vertex_position: vec3<f32>,
    @location(1) vertex_uv: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.uv = vertex_uv;
    out.position = view.view_proj * vec4<f32>(vertex_position, 1.0);
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let ix = tile_strip_uniform.len * in.uv.x;
    let tile_idx = u32(ix);
    let atlas_coords = tile_strip_uniform.atlas_coords[tile_idx];
    let ruv = vec2<f32>(fract(ix), in.uv.y);
    var color = textureSample(color_texture, color_sampler, atlas_coords.xy + ruv * atlas_coords.zw); 
    return color;
}
