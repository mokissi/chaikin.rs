use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

fn chaikin(points: &[Point]) -> Vec<Point> {
    if points.len() <= 2  {
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
   return out;
}
fn points_at_step(points: &[Point], k: usize) -> Vec<Point> {
    let mut pts = points.to_vec();
    for _ in 0..k {
        pts = chaikin(&pts);
    }
    pts
}
#[macroquad::main("Chaikin Demo")]
async fn main() {
    let mut control_points: Vec<Point> = Vec::new();
    let mut current_step: usize = 0;
    let mut playing = false;
    let mut timer: f64 = 0.0;
    let interval = 0.7; 

    loop {
        clear_background(BLACK);

        // --- mouse input ---
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            
            control_points.push(Point { x: mx, y: my });
        }

        // --- keyboard input ---
        if is_key_pressed(KeyCode::Enter) && !control_points.is_empty() {
            playing = true;
            current_step = 0;
            timer = 0.0;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // --- update animation ---
        if playing {
            timer += get_frame_time() as f64;
            if timer >= interval {
                timer = 0.0;
                current_step += 1;
                if current_step > 7 {
                    current_step = 0;
                }
            }
        }

        // --- draw control points ---

        for p in &control_points {
            draw_circle(p.x, p.y, 5.0, RED);
        }


        // --- draw curve/line ---
        if !control_points.is_empty() && playing  {
            let curve_points = match control_points.len() {
                1 | 2 => control_points.clone(),
                _ => points_at_step(&control_points, current_step),
            };

            for w in curve_points.windows(2) {
                let p1 = w[0];
                let p2 = w[1];
                draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, WHITE);
            }
        }

        // --- info text ---
        draw_text(
            &format!("Points: {} | Step: {}", control_points.len(), current_step),
            20.0,
            20.0,
            24.0,
            YELLOW,
        );
        draw_text(
            "Left Click: add point | Enter: play | Esc: quit",
            20.0,
            40.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
