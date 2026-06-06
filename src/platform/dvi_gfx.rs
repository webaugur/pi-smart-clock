//! Display-list rendering for the Pico DVI Sock (640×480 VGA timing).

use alloc::format;

use pico_dvi_rs::{
    dvi::VERTICAL_REPEAT,
    render::{
        end_display_list, renderlist::RenderlistBuilder, rgb, start_display_list, BW_PALETTE,
        FONT_HEIGHT,
    },
    scanlist::ScanlistBuilder,
    DISPLAY_HEIGHT, DISPLAY_WIDTH,
};

use crate::layout::l;

mod boot_splash_embedded {
    include!(concat!(env!("OUT_DIR"), "/boot_splash_embedded.rs"));
}

type TmdsRgb = [pico_dvi_rs::dvi::tmds::TmdsPair; 3];

fn color24(c: u32) -> TmdsRgb {
    rgb(
        ((c >> 16) & 0xFF) as u8,
        ((c >> 8) & 0xFF) as u8,
        (c & 0xFF) as u8,
    )
}

pub struct DviGfx {
    bg: u32,
}

impl DviGfx {
    pub const fn new() -> Self {
        Self { bg: 0x000000 }
    }

    pub async fn clear(&mut self, color: u32) {
        self.bg = color;
    }

    pub async fn fill_rect(&mut self, _x: i32, _y: i32, _w: i32, _h: i32, color: u32) {
        self.bg = color;
    }

    pub async fn present_clock_frame(&mut self, hour: u32, minute: u32, second: u32) {
        let layout = l();
        let (mut rb, mut sb) = start_display_list();
        let canvas_h = DISPLAY_HEIGHT as i32 - FONT_HEIGHT as i32;
        let bg = color24(self.bg);

        let cx = scale_x(layout.clock_cx);
        let cy = scale_y(layout.clock_cy);
        let outer = scale_len(layout.clock_outer_r);
        let inner = outer - scale_len(12);

        for y in 0..canvas_h {
            stripe_row((&mut rb, &mut sb), y, |sb| {
                fill_scanline_disc(sb, y, cx, cy, outer, color24(0x1a1a2e), bg);
                fill_scanline_disc(sb, y, cx, cy, inner, bg, bg);
            });
        }

        let hour_angle = ((hour % 12) as f32 * 30.0 + minute as f32 * 0.5).to_radians();
        let minute_angle = (minute as f32 * 6.0 + second as f32 * 0.1).to_radians();
        let second_angle = (second as f32 * 6.0).to_radians();

        draw_hand_lines(
            &mut rb,
            &mut sb,
            canvas_h,
            cx,
            cy,
            scale_len(layout.hand_length * 58 / 100),
            hour_angle,
            color24(0xcccccc),
            5,
            bg,
        );
        draw_hand_lines(
            &mut rb,
            &mut sb,
            canvas_h,
            cx,
            cy,
            scale_len(layout.hand_length * 82 / 100),
            minute_angle,
            color24(0xeeeeee),
            3,
            bg,
        );
        draw_hand_lines(
            &mut rb,
            &mut sb,
            canvas_h,
            cx,
            cy,
            scale_len(layout.hand_length),
            second_angle,
            color24(0xff4444),
            2,
            bg,
        );

        for y in 0..canvas_h {
            stripe_row((&mut rb, &mut sb), y, |sb| {
                fill_scanline_disc(sb, y, cx, cy, scale_len(8), color24(0x666666), bg);
            });
        }

        rb.begin_stripe(FONT_HEIGHT);
        let status = format!("{:02}:{:02}:{:02}", hour, minute, second);
        let text_w = rb.text(&status);
        let text_w = text_w + text_w % 2;
        rb.end_stripe();
        sb.begin_stripe(FONT_HEIGHT);
        sb.pal_1bpp(text_w, &BW_PALETTE);
        sb.solid(DISPLAY_WIDTH - text_w, bg);
        sb.end_stripe();

        end_display_list(rb, sb);
    }

    /// Full-screen PNG splash (compile-time raster) with optional status line.
    pub async fn present_splash_frame(&mut self, status: &str) {
        use boot_splash_embedded::{SPLASH_H, SPLASH_RGB, SPLASH_W};

        let (mut rb, mut sb) = start_display_list();
        let bg = color24(self.bg);

        if SPLASH_W > 0 && SPLASH_H > 0 && SPLASH_RGB.len() == (SPLASH_W * SPLASH_H * 3) as usize {
            for y in 0..SPLASH_H {
                let row = &SPLASH_RGB[(y * SPLASH_W * 3) as usize..][..(SPLASH_W * 3) as usize];
                stripe_row((&mut rb, &mut sb), y as i32, |sb| {
                    emit_scanline_rgb(sb, row, SPLASH_W, bg);
                });
            }
        } else {
            let canvas_h = DISPLAY_HEIGHT / VERTICAL_REPEAT as u32 - FONT_HEIGHT;
            rb.begin_stripe(canvas_h);
            rb.end_stripe();
            sb.begin_stripe(canvas_h);
            sb.solid(DISPLAY_WIDTH, color24(0x000814));
            sb.end_stripe();
        }

        rb.begin_stripe(FONT_HEIGHT);
        let s_w = rb.text(status);
        let s_w = s_w + s_w % 2;
        rb.end_stripe();
        sb.begin_stripe(FONT_HEIGHT);
        sb.pal_1bpp(s_w, &BW_PALETTE);
        sb.solid(DISPLAY_WIDTH - s_w, bg);
        sb.end_stripe();

        end_display_list(rb, sb);
    }

    pub async fn present_boot_frame(&mut self, title: &str, subtitle: &str) {
        let (mut rb, mut sb) = start_display_list();
        let canvas_h = DISPLAY_HEIGHT / VERTICAL_REPEAT as u32 - FONT_HEIGHT * 2;
        let bg = color24(0x000814);

        rb.begin_stripe(canvas_h);
        rb.end_stripe();
        sb.begin_stripe(canvas_h);
        sb.solid(DISPLAY_WIDTH, bg);
        sb.end_stripe();

        rb.begin_stripe(FONT_HEIGHT);
        let t_w = rb.text(title);
        let t_w = t_w + t_w % 2;
        rb.end_stripe();
        sb.begin_stripe(FONT_HEIGHT);
        sb.pal_1bpp(t_w, &BW_PALETTE);
        sb.solid(DISPLAY_WIDTH - t_w, bg);
        sb.end_stripe();

        rb.begin_stripe(FONT_HEIGHT);
        let s_w = rb.text(subtitle);
        let s_w = s_w + s_w % 2;
        rb.end_stripe();
        sb.begin_stripe(FONT_HEIGHT);
        sb.pal_1bpp(s_w, &BW_PALETTE);
        sb.solid(DISPLAY_WIDTH - s_w, color24(0x00ffaa));
        sb.end_stripe();

        end_display_list(rb, sb);
    }
}

fn stripe_row(sb_rb: (&mut RenderlistBuilder, &mut ScanlistBuilder), y: i32, draw: impl FnOnce(&mut ScanlistBuilder)) {
    let (rb, sb) = sb_rb;
    let _ = y;
    rb.begin_stripe(1);
    rb.end_stripe();
    sb.begin_stripe(1);
    draw(sb);
    sb.end_stripe();
}

fn fill_scanline_disc(sb: &mut ScanlistBuilder, y: i32, cx: i32, cy: i32, radius: i32, fill: TmdsRgb, bg: TmdsRgb) {
    if radius <= 0 {
        sb.solid(DISPLAY_WIDTH, bg);
        return;
    }
    let dy = y - cy;
    let r2 = radius * radius;
    if dy * dy > r2 {
        sb.solid(DISPLAY_WIDTH, bg);
        return;
    }
    let half = isqrt(r2 - dy * dy);
    let x1 = (cx - half).max(0) as u32;
    let x2 = (cx + half).max(0) as u32;
    if x1 > 0 {
        sb.solid(x1, bg);
    }
    let mid = x2.saturating_sub(x1);
    if mid > 0 {
        sb.solid(mid, fill);
    }
    if x2 < DISPLAY_WIDTH {
        sb.solid(DISPLAY_WIDTH - x2, bg);
    }
}

fn draw_hand_lines(
    rb: &mut RenderlistBuilder,
    sb: &mut ScanlistBuilder,
    canvas_h: i32,
    cx: i32,
    cy: i32,
    len: i32,
    angle: f32,
    color: TmdsRgb,
    thickness: i32,
    bg: TmdsRgb,
) {
    let x2 = cx + libm::roundf(libm::sinf(angle) * len as f32) as i32;
    let y2 = cy - libm::roundf(libm::cosf(angle) * len as f32) as i32;

    for y in 0..canvas_h {
        if y < cy.min(y2) - thickness || y > cy.max(y2) + thickness {
            continue;
        }
        stripe_row((rb, sb), y, |sb| {
            if (y - cy).abs() <= thickness && (y - y2).abs() <= thickness {
                paint_thick_point(sb, cx, color, thickness, bg);
                return;
            }
            if y2 == cy {
                paint_thick_point(sb, cx, color, thickness, bg);
                return;
            }
            let t = (y - cy) as f32 / (y2 - cy) as f32;
            if !(0.0..=1.0).contains(&t) {
                sb.solid(DISPLAY_WIDTH, bg);
                return;
            }
            let x = cx as f32 + t * (x2 - cx) as f32;
            paint_thick_point(sb, libm::roundf(x) as i32, color, thickness, bg);
        });
    }
}

fn paint_thick_point(sb: &mut ScanlistBuilder, cx: i32, color: TmdsRgb, thickness: i32, bg: TmdsRgb) {
    let half = thickness;
    let x1 = (cx - half).max(0) as u32;
    let x2 = (cx + half).max(0) as u32;
    if x1 > 0 {
        sb.solid(x1, bg);
    }
    let mid = x2.saturating_sub(x1).max(1);
    sb.solid(mid, color);
    if x2 < DISPLAY_WIDTH {
        sb.solid(DISPLAY_WIDTH - x2, bg);
    }
}

fn scale_x(x: i32) -> i32 {
    x * DISPLAY_WIDTH as i32 / l().screen_w
}

fn scale_y(y: i32) -> i32 {
    y * (DISPLAY_HEIGHT as i32 - FONT_HEIGHT as i32) / l().screen_h
}

fn scale_len(v: i32) -> i32 {
    v * DISPLAY_WIDTH as i32 / l().screen_w
}

fn emit_scanline_rgb(sb: &mut ScanlistBuilder, row: &[u8], width: u32, bg: TmdsRgb) {
    if row.len() < (width * 3) as usize {
        sb.solid(DISPLAY_WIDTH, bg);
        return;
    }
    let mut x = 0u32;
    while x < width {
        let i = (x * 3) as usize;
        let color = rgb(row[i], row[i + 1], row[i + 2]);
        let mut run = 1u32;
        while x + run < width {
            let j = ((x + run) * 3) as usize;
            if row[j] != row[i] || row[j + 1] != row[i + 1] || row[j + 2] != row[i + 2] {
                break;
            }
            run += 1;
        }
        sb.solid(run, color);
        x += run;
    }
    if x < DISPLAY_WIDTH {
        sb.solid(DISPLAY_WIDTH - x, bg);
    }
}

fn isqrt(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}