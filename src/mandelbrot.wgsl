@group(0) @binding(0) var <uniform> params: Params;

const RGB_SCHEME: u32 = 1;
const HSV_SCHEME: u32 = 2;

const EPSILON: f32 = 0.001;
const AXIS_EPSILON: f32 = 0.005;

struct Params {
    center: vec4f, // 2 points
    initial_value: vec4f, // 2 points
    max_iter: u32,
    zoom: f32,
    rgb_green: f32,
    rgb_blue: f32,
    color_scheme: u32,
    hsv_saturation: f32,
    hsv_brightness: f32,
    show_axis: u32,
    escape_threshold: f32,
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
    var z = Complex(params.initial_value.x, params.initial_value.y);
    let l = i32(limit);
    for (var i: i32 = 0; i < l; i++) {
        let z_sqrt = norm_sqr(z);

        if z_sqrt > params.escape_threshold {
            return i;
        }

        z = sum(mul(z, z), c);
    }
    return -1;
}

struct VsOut {
    @builtin(position) position: vec4f,
    @location(0) uv: vec2<f32>
}

fn hsv_rgb(hsv: vec3<f32>) -> vec3<f32> {
    if (hsv.y == 0.0) {
        return vec3<f32>(hsv.z, hsv.z, hsv.z);
    } else {
        var hp: f32 = hsv.x * 6.0;
        if (hp == 6.0) {
            hp = 0.0;
        }
        let hpi: i32 = i32(hp);
        let v1: f32 = hsv.z * (1.0 - hsv.y);
        let v2: f32 = hsv.z * (1.0 - hsv.y * (hp - f32(hpi)));
        let v3: f32 = hsv.z * (1.0 - hsv.y * (1.0 - (hp - f32(hpi))));
        switch (hpi) {
            case 0: {
                return vec3<f32>(hsv.z, v3, v1);
            }
            case 1: {
                return vec3<f32>(v2, hsv.z, v1);
            }
            case 2: {
                return vec3<f32>(v1, hsv.z, v3);
            }
            case 3: {
                return vec3<f32>(v1, v2, hsv.z);
            }
            case 4: {
                return vec3<f32>(v3, v1, hsv.z);
            }
            default: {
                return vec3<f32>(hsv.z, v1, v2);
            }
        }
    }
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
    let scale = params.zoom;
    let x = (in.uv.x  - 0.5) / scale * 3.0;
    let y = (in.uv.y  - 0.5) / scale * 2.0;

    let current_point = Complex(x, y);
    let c = sum(center, current_point);

    if ((params.show_axis & 1) > 0) {
        let scaled_epsilon = EPSILON / scale;
        let scaled_axis_epsilon = AXIS_EPSILON / scale;
        let axis_epsilon = scaled_epsilon * 25;

        if (abs(c.im) >= axis_epsilon && abs(abs(c.im) - abs(floor(c.im))) <= scaled_axis_epsilon && abs(c.re) <= axis_epsilon) {
            return vec4f(255, 255, 255, 0);
        }
        if (abs(c.re) >= axis_epsilon && abs(abs(c.re) - abs(floor(c.re))) <= scaled_axis_epsilon && abs(c.im) <= axis_epsilon) {
            return vec4f(255, 255, 255, 0);
        }
        if (abs(c.re) <= scaled_epsilon || abs(c.im) <= scaled_epsilon) {
            return vec4f(255, 255, 255, 0);
        }
    }

    let time = escape_time(c, params.max_iter);

    if (time == -1) {
        return vec4(0.0, 0.0, 0.0, 1.0);
    }

    if ((params.color_scheme & HSV_SCHEME) > 0) {
        let color = log(f32(time) + 1) / log(f32(params.max_iter) + 1);
        let colors = vec3f(color, params.hsv_saturation, params.hsv_brightness);
        return vec4f(hsv_rgb(colors), 1.0);
    }
    else {
        let color = f32(time) / f32(params.max_iter);
        let colors = vec3f(color, params.rgb_green, params.rgb_blue);
        return vec4f(colors, 1.0);
    }
}