#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(1)
var texture: texture_2d<f32>;
@group(2) @binding(2)
var splr: sampler;
@group(2) @binding(3)
var<uniform> ix_length_flipx_flipy: vec4<f32>;
@group(2) @binding(4)
var<uniform> rgba: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Unpack stuff
    let ix = ix_length_flipx_flipy[0];
    let length = ix_length_flipx_flipy[1];
    let flipx = ix_length_flipx_flipy[2];
    let flipy = ix_length_flipx_flipy[3];
    let r = rgba[0];
    let g = rgba[1];
    let b = rgba[2];
    let a = rgba[3];

    // Apply ix and flip and sample
    let flipped_in = vec2<f32>((1.0 + flipx * in.uv.x) % 1.0, (1.0 + flipy * in.uv.y) % 1.0);
    let index_lower = (1.0 / length) * (ix + 0);
    let index_upper = (1.0 / length) * (ix + 1);
    let out_uv = vec2<f32>(index_lower + (index_upper - index_lower) * flipped_in.x, flipped_in.y);
    let out_rgba = textureSample(texture, splr, out_uv);

    // Apply color
    return vec4<f32>(out_rgba[0] * r, out_rgba[1] * g, out_rgba[2] * b, out_rgba[3] * a);
}
