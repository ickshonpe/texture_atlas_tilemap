struct TilestripUniform {
    len: f32,
    atlas_rects: array<vec4<f32>, 1000u>,
}

@group(1) @binding(0)
var color_texture: texture_2d<f32>;

@group(1) @binding(1)
var color_sampler: sampler;

@group(1) @binding(2)
var<uniform> tilestrip_uniform: TilestripUniform;

@fragment
fn fragment(
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>
) -> @location(0) vec4<f32> {
    let ix = tilestrip_uniform.len * uv.x;
    let idx = u32(ix);
    let atlas_rects = tilestrip_uniform.atlas_rects[idx];
    let ruv = vec2<f32>(fract(ix), uv.y);
    return textureSample(color_texture, color_sampler, atlas_rects.xy + ruv * atlas_rects.zw); 
}
