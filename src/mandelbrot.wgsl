@group(0) @binding(0) var <uniform> params: Params;

struct Params {
    max_iter: u32,
    center: vec2f
}

struct Complex {
    re: f32,
    im: f32
}

fn norm_sqr(c: Complex) -> f32 {
    return c.re * c.re + c.im * c.im;
}

fn mul(c1: Complex, c2: Complex) -> Complex {
    return Complex(c1.re * c2.re - c1.im * c2.im, c1.re * c2.im + c1.im * c2.re);
}

fn sum(c1: Complex, c2: Complex) -> Complex {
    return Complex(c1.re + c2.re, c1.im + c2.im);
}

fn escape_time(c: Complex, limit: u32) -> i32 {
    var z = Complex(0.0, 0.0);
    let l = i32(limit);
    for (var i: i32 = 0; i < l; i++) {
        let z_sqrt = norm_sqr(z);

        if z_sqrt > 4.0 {
            return i;
        }

        z = sum(mul(z, z), c);
    }
    return 255;
}

struct VsOut {
    @builtin(position) position: vec4f,
    @location(0) uv: vec2<f32>
}

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VsOut {
    var pos = array<vec2f, 6>(
        vec2f(-1.0, -1.0),
        vec2f(-1.0,  1.0),
        vec2f(1.0,  1.0),
        vec2f(1.0,  1.0),
        vec2f(1.0,  -1.0),
        vec2f(-1.0,  -1.0)
    );
    var out: VsOut;
    let position = pos[index];

    out.position = vec4f(position, 0.0, 1.0);
    out.uv = (position + vec2f(1.0, 1.0)) * 0.5; // normalization in [0..=1]
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4f {
    let center = Complex(params.center.x, params.center.y);
 let scale = 2.0;
    let x = (in.uv.x - 0.5) * scale * 3.0; // расширяем по x
    let y = (in.uv.y - 0.5) * scale * 2.0; // расширяем по y
    let current_point = Complex(x, y);
    let c = sum(center, current_point);

    let time = escape_time(c, params.max_iter);
    let col = f32(time) / f32(params.max_iter);
    return vec4f(col, col, col, 1.0);
}