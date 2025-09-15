use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

fn chaikin(points: &[Point]) -> Vec<Point> {
    if points.len() <= 2 {
        return points.to_vec(); 
    }

    let mut out = vec![points[0]]; 

    for i in 0..points.len() - 1 {
        let p = points[i];
        let q = points[i + 1];

        let q1 = Point {
            x: 0.75 * p.x + 0.25 * q.x,
            y: 0.75 * p.y + 0.25 * q.y,
        };
        let q2 = Point {
            x: 0.25 * p.x + 0.75 * q.x,
            y: 0.25 * p.y + 0.75 * q.y,
        };

        out.push(q1);
        out.push(q2);
    }

    out.push(points[points.len() - 1]); 
    out
}
