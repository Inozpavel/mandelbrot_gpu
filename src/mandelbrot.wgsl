@group(0) @binding(0) var <uniform> uniforms: Uniforms;

const RGB_SCHEME: u32 = 1;
const HSV_SCHEME: u32 = 2;

const EPSILON: f32 = 0.001;
const AXIS_EPSILON: f32 = 0.005;

const JULIA_FRACTAL_TYPE: u32 = 2;
const MANDELBROT_FRACTAL_TYPE: u32 = 1;

const MIN_DRAW_LINE_MULTIPLIER = 2.0;

struct Uniforms {
    center: vec2f,
    initial_value: vec2f,
    resolution: vec2f,
    max_iter: u32,
    zoom: f32,
    rgb_green: f32,
    rgb_blue: f32,
    color_scheme: u32,
    hsv_saturation: f32,
    hsv_brightness: f32,
    show_axis: u32,
    escape_threshold: f32,
    fractal_type: u32,
    pow: u32
}

struct Complex {
    re: f32,
    im: f32
}

fn norm_sqr(c: Complex) -> f32 {
    return c.re * c.re + c.im * c.im;
}
fn norm_sqrt(c: Complex) -> f32 {
    return sqrt(norm_sqr(c));
}

fn complex_pow(c: Complex, n: u32) -> Complex {
    var r = norm_sqrt(c);
    let pow_degree = f32(n);
    var theta = atan2(c.im, c.re);
    var rn = pow(r, pow_degree);

    return Complex(rn * cos(pow_degree * theta), rn * sin(pow_degree * theta));
}

fn mul(c1: Complex, c2: Complex) -> Complex {
    return Complex(c1.re * c2.re - c1.im * c2.im, c1.re * c2.im + c1.im * c2.re);
}

fn sum(c1: Complex, c2: Complex) -> Complex {
    return Complex(c1.re + c2.re, c1.im + c2.im);
}

fn escape_time(c: Complex, limit: u32) -> i32 {
    let constant = Complex(uniforms.initial_value.x, uniforms.initial_value.y);
    var z: Complex;

    if ((uniforms.fractal_type & JULIA_FRACTAL_TYPE) > 0) {
        z = c;
    } else {
        z = constant;
    }
    let l = i32(limit);
    for (var i: i32 = 0; i < l; i++) {
        let z_sqrt = norm_sqr(z);

        if z_sqrt > uniforms.escape_threshold {
            return i;
        }

        if ((uniforms.fractal_type & JULIA_FRACTAL_TYPE) > 0) {
            z = sum(complex_pow(z, uniforms.pow), constant);
        } else {
            z = sum(complex_pow(z, uniforms.pow), c);
        }
    }
    return -1;
}

struct VsOut {
    @builtin(position) position: vec4f,
    @location(0) uv: vec2<f32>
}

fn hsv2rgb(c: vec3f) -> vec3f {
    var rgb = clamp(
        abs(((vec3f(c.x) * 6.0 + vec3f(0.0, 4.0, 2.0)) % 6.0) - 3.0) - 1.0,
        vec3f(0.0),
        vec3f(1.0)
    );
    rgb = rgb * rgb * (vec3f(3.0) - 2.0 * rgb);
    return c.z * mix(vec3f(1.0), rgb, c.y);
}

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VsOut {
    // Fullscreen triangle. Constructed in a single expression
    let x = f32(i32(index & 1u) * 4 - 1);
    let y = f32(i32(index >> 1u) * 4 - 1);
    return VsOut(vec4f(x, y, 0.0, 1.0), vec2f(x, y) * 0.5 + 0.5);
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4f {
    var uv = in.uv;

    let aspect = uniforms.resolution.x / uniforms.resolution.y;
    let scale = uniforms.zoom;

    let x = (uv.x - 0.5) * aspect / scale;
    let y = (uv.y - 0.5) / scale;

    let center = Complex(uniforms.center.x, uniforms.center.y);

    let current_point = Complex(x, y);
    let c = sum(center, current_point);

    var rgb = vec3f(0.0);
    let axis_color = vec3f(1.0);

    let time = escape_time(c, uniforms.max_iter);

    if (time != -1) {
        if ((uniforms.color_scheme & HSV_SCHEME) > 0) {
            let color = log(f32(time) + 1) / log(f32(uniforms.max_iter) + 1);
            let colors = vec3f(color, uniforms.hsv_saturation, uniforms.hsv_brightness);
            rgb = hsv2rgb(colors);
        }
        else {
            let color = f32(time) / f32(uniforms.max_iter);
            let colors = vec3f(color, uniforms.rgb_green, uniforms.rgb_blue);
            rgb = colors;
        }
    }

    if ((uniforms.show_axis & 1) > 0) {
        let x = c.re;
        let y = c.im;
        let blur_x = fwidth(x) * MIN_DRAW_LINE_MULTIPLIER;
        let blur_y = fwidth(y) * MIN_DRAW_LINE_MULTIPLIER;
        let x_axis_mask = plot(0.0, x, blur_x);
        let y_axis_mask = plot(0.0, y, blur_y);

        let width = fwidth(x) * 15;
        let len = fwidth(y) * 1.5;
        let blur = 0.00001;
        let y_ticks_mask = axis_ticks_mask(x, y, blur_x, width, len);
        let x_ticks_mask = axis_ticks_mask(y, x, blur_y, width, len);
        rgb = mix(rgb, axis_color, x_axis_mask);
        rgb = mix(rgb, axis_color, y_axis_mask);
        rgb = mix(rgb, axis_color, y_ticks_mask);
        rgb = mix(rgb, axis_color, x_ticks_mask);
    }

    return vec4f(rgb, 1.0);
}

fn plot(target_value: f32, current: f32, blur: f32) -> f32 {
    let b = max(blur, fwidth(current) * MIN_DRAW_LINE_MULTIPLIER);
    return saturate(smoothstep(target_value - b, target_value, current) * smoothstep(target_value + b, target_value, current));
}

fn strip_mask(a: f32, b: f32, current: f32, blur: f32) -> f32 {
    let half_width = (b - a) * 0.5;
    let aa = clamp(max(fwidth(current) * MIN_DRAW_LINE_MULTIPLIER, blur), 0.0, half_width * 0.9);
   let result = smoothstep(a - aa, a, current) * smoothstep(b + aa, b, current);

    return saturate(result);
}

fn triangle_mask(coordinades: vec4f, current: vec2f, blur: f32) -> f32 {
    let x = strip_mask(coordinades.x, coordinades.y, current.x, blur);
    let y = strip_mask(coordinades.z, coordinades.w, current.y, blur);
    let result = x * y;
    return saturate(result);
}

fn axis_ticks_mask(current: f32, across: f32, blur: f32, width: f32, len: f32) -> f32 {
    var local = fract(current + 0.5) - 0.5;

    let aa = max(fwidth(current) * MIN_DRAW_LINE_MULTIPLIER, blur);
    let aa_len = len;
    let aa_width = width;

    let width_mask = strip_mask(-aa_width, aa_width, across, aa);
    let len_mask = strip_mask(-aa_len, aa_len, local, aa);

    return width_mask * len_mask;
}