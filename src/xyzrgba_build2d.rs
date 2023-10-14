use crate::xyzrgba::*;
use minvect::*;

pub fn put_triangle(buf: &mut Vec<XYZRGBA>, a: Vec2, b: Vec2, c: Vec2, col: Vec4, depth: f32) {
    buf.push(XYZRGBA {
        xyz: vec3(a.x, a.y, depth),
        rgba: col,
    });
    buf.push(XYZRGBA {
        xyz: vec3(b.x, b.y, depth),
        rgba: col,
    });
    buf.push(XYZRGBA {
        xyz: vec3(c.x, c.y, depth),
        rgba: col,
    });
}

pub fn put_quad(buf: &mut Vec<XYZRGBA>, a: Vec2, b: Vec2, c: Vec2, d: Vec2, col: Vec4, depth: f32) {
    put_triangle(buf, a, b, c, col, depth);
    put_triangle(buf, a, c, d, col, depth);
}

pub fn put_poly(buf: &mut Vec<XYZRGBA>, c: Vec2, r: f32, n: usize, phase: f32, col: Vec4, depth: f32) {
    let dtheta = (2.0 * PI) / n as f32;
    for i in 1..n {
        let theta = phase + dtheta * i as f32;
        let p1 = c + r*vec2(theta.cos(), theta.sin());
        let theta = theta - dtheta;
        let p2 = c + r*vec2(theta.cos(), theta.sin());
        put_triangle(buf, c, p1, p2, col, depth);
    }
} 