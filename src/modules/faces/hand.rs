use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn draw_second_hand(
    canvas: &mut Canvas<Window>,
    cx: i32,
    cy: i32,
    length: i32,
    angle_deg: f32,
    night: bool,
) {
    let rad = angle_deg.to_radians();
    let tip_x = cx as f32 + rad.sin() * length as f32;
    let tip_y = cy as f32 - rad.cos() * length as f32;

    let color = if night {
        Color::RGB(255, 120, 64)
    } else {
        Color::RGB(255, 34, 34)
    };
    thick_line(canvas, cx, cy, tip_x.round() as i32, tip_y.round() as i32, 3, color);

    let back = length as f32 * 0.18;
    let tail_x = cx as f32 - rad.sin() * back;
    let tail_y = cy as f32 + rad.cos() * back;
    thick_line(
        canvas,
        cx,
        cy,
        tail_x.round() as i32,
        tail_y.round() as i32,
        2,
        color,
    );

    draw_arrow_tip(canvas, tip_x, tip_y, angle_deg, 10.0, color);
    fill_circle(canvas, cx, cy, 5, Color::RGB(0, 0, 0));
    stroke_circle(canvas, cx, cy, 5, 2, Color::RGB(232, 228, 216));
}

fn draw_arrow_tip(
    canvas: &mut Canvas<Window>,
    tip_x: f32,
    tip_y: f32,
    angle_deg: f32,
    size: f32,
    color: Color,
) {
    let rad = angle_deg.to_radians();
    let left = (
        tip_x - rad.cos() * size * 0.55 - rad.sin() * size * 0.35,
        tip_y - rad.sin() * size * 0.55 + rad.cos() * size * 0.35,
    );
    let right = (
        tip_x + rad.cos() * size * 0.55 - rad.sin() * size * 0.35,
        tip_y + rad.sin() * size * 0.55 + rad.cos() * size * 0.35,
    );
    fill_triangle(canvas, tip_x, tip_y, left.0, left.1, right.0, right.1, color);
}

fn thick_line(
    canvas: &mut Canvas<Window>,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    thick: i32,
    color: Color,
) {
    canvas.set_draw_color(color);
    let steps = ((x2 - x1).abs() + (y2 - y1).abs()).max(1);
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let x = x1 as f32 + t * (x2 - x1) as f32;
        let y = y1 as f32 + t * (y2 - y1) as f32;
        let _ = canvas.fill_rect(Rect::new(
            x.round() as i32 - thick / 2,
            y.round() as i32 - thick / 2,
            thick as u32,
            thick as u32,
        ));
    }
}

fn fill_triangle(
    canvas: &mut Canvas<Window>,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    color: Color,
) {
    canvas.set_draw_color(color);
    let min_y = y1.min(y2).min(y3).floor() as i32;
    let max_y = y1.max(y2).max(y3).ceil() as i32;
    for y in min_y..=max_y {
        let mut xs = Vec::new();
        for (ax, ay, bx, by) in [(x1, y1, x2, y2), (x2, y2, x3, y3), (x3, y3, x1, y1)] {
            if (ay <= y as f32 && (y as f32) < by) || (by <= y as f32 && (y as f32) < ay) {
                let t = (y as f32 - ay) / (by - ay);
                xs.push(ax + t * (bx - ax));
            }
        }
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mut i = 0;
        while i + 1 < xs.len() {
            let x_start = xs[i].round() as i32;
            let x_end = xs[i + 1].round() as i32;
            let w = (x_end - x_start).max(1) as u32;
            let _ = canvas.fill_rect(Rect::new(x_start, y, w, 1));
            i += 2;
        }
    }
}

fn fill_circle(canvas: &mut Canvas<Window>, cx: i32, cy: i32, radius: i32, color: Color) {
    if radius <= 0 {
        return;
    }
    canvas.set_draw_color(color);
    for dy in -radius..=radius {
        let dx = (((radius * radius - dy * dy) as f32).sqrt()) as i32;
        let w = (dx * 2).max(1) as u32;
        let _ = canvas.fill_rect(Rect::new(cx - dx, cy + dy, w, 1));
    }
}

fn stroke_circle(canvas: &mut Canvas<Window>, cx: i32, cy: i32, radius: i32, thick: i32, color: Color) {
    for r in (radius - thick).max(0)..=radius {
        fill_circle(canvas, cx, cy, r, color);
    }
}