mod config;
mod hand;
mod layout;
mod numerals;
mod svg;

use hand::HandSprite;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub use config::load_face_id;

const RASTER_SIZE: u32 = 512;
const NUMERAL_COUNT: usize = 12;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FaceId {
    RetroRoman,
}

impl FaceId {
    pub fn default() -> Self {
        Self::RetroRoman
    }

    pub fn parse(name: &str) -> Option<Self> {
        match name.to_ascii_lowercase().as_str() {
            "retro-roman" | "retro_roman" | "default" => Some(Self::RetroRoman),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::RetroRoman => "retro-roman",
        }
    }

    pub fn asset_path(self) -> &'static str {
        match self {
            Self::RetroRoman => "retro-roman/face.svg",
        }
    }
}

struct LoadedFace {
    dial: Vec<u8>,
    numerals: [svg::RasterGlyph; NUMERAL_COUNT],
    hour_hand: HandSprite,
    minute_hand: HandSprite,
    second_hand: HandSprite,
    hub: Option<HandSprite>,
}

struct FaceCache {
    active: FaceId,
    faces: HashMap<FaceId, LoadedFace>,
}

static CACHE: OnceLock<Mutex<FaceCache>> = OnceLock::new();

fn cache() -> &'static Mutex<FaceCache> {
    CACHE.get_or_init(|| {
        let active = load_face_id();
        Mutex::new(FaceCache {
            active,
            faces: HashMap::new(),
        })
    })
}

fn load_hand(asset: layout::HandAsset, label: &str) -> Option<HandSprite> {
    HandSprite::load(asset).or_else(|| {
        eprintln!("[faces] missing or invalid hand SVG: {} ({label})", asset.file);
        None
    })
}

fn load_face(id: FaceId) -> Option<LoadedFace> {
    let path = svg::resolve_face_path(id.asset_path());
    let tree = svg::parse_face_svg(&path)?;
    let dial = svg::rasterize_dial(&tree, RASTER_SIZE)?;
    let mut numerals = std::array::from_fn(|_| svg::RasterGlyph {
        pixels: Vec::new(),
        width: 0,
        height: 0,
    });
    for i in 0..NUMERAL_COUNT {
        let node_id = format!("numeral-{i}");
        if let Some(glyph) = svg::rasterize_tree_node(&tree, &node_id, 4) {
            numerals[i] = glyph;
        } else {
            eprintln!("[faces] missing glyph {node_id} in {}", path.display());
        }
    }

    let layout = id.layout();
    let hour_hand = load_hand(layout.hour_hand, "hour")?;
    let minute_hand = load_hand(layout.minute_hand, "minute")?;
    let second_hand = load_hand(layout.second_hand, "second")?;
    let hub = layout.hub.and_then(|h| load_hand(h, "hub"));

    Some(LoadedFace {
        dial,
        numerals,
        hour_hand,
        minute_hand,
        second_hand,
        hub,
    })
}

fn ensure_loaded(cache: &mut FaceCache, id: FaceId) {
    if cache.faces.contains_key(&id) {
        return;
    }
    match load_face(id) {
        Some(face) => {
            cache.faces.insert(id, face);
        }
        None => {
            eprintln!("[faces] failed to load {}", id.asset_path());
        }
    }
}

fn with_face<F>(f: F)
where
    F: FnOnce(&LoadedFace),
{
    let mut cache = cache().lock().expect("face cache");
    let id = cache.active;
    ensure_loaded(&mut cache, id);
    if let Some(face) = cache.faces.get(&id) {
        f(face);
    }
}

pub fn draw_face(
    canvas: &mut Canvas<Window>,
    cx: i32,
    cy: i32,
    diameter: u32,
) {
    let mut cache = cache().lock().expect("face cache");
    let id = cache.active;
    ensure_loaded(&mut cache, id);
    let Some(face) = cache.faces.get(&id) else {
        return;
    };
    let x = cx - (diameter as i32 / 2);
    let y = cy - (diameter as i32 / 2);
    svg::draw_rgba(
        canvas,
        &face.dial,
        RASTER_SIZE,
        RASTER_SIZE,
        x,
        y,
        diameter,
        diameter,
    );
    numerals::draw_positioned(canvas, &face.numerals, id.layout(), cx, cy, diameter);
}

pub fn draw_hour_hand(
    canvas: &mut Canvas<Window>,
    cx: i32,
    cy: i32,
    length: i32,
    angle_deg: f32,
    night: bool,
) {
    with_face(|face| {
        hand::draw_hand_sprite(
            canvas,
            &face.hour_hand,
            cx,
            cy,
            length,
            angle_deg,
            hand::ivory_tint(night),
        );
    });
}

pub fn draw_minute_hand(
    canvas: &mut Canvas<Window>,
    cx: i32,
    cy: i32,
    length: i32,
    angle_deg: f32,
    night: bool,
) {
    with_face(|face| {
        hand::draw_hand_sprite(
            canvas,
            &face.minute_hand,
            cx,
            cy,
            length,
            angle_deg,
            hand::ivory_tint(night),
        );
    });
}

pub fn draw_hub(canvas: &mut Canvas<Window>, cx: i32, cy: i32, night: bool) {
    let mut cache = cache().lock().expect("face cache");
    let id = cache.active;
    ensure_loaded(&mut cache, id);
    let layout = id.layout();
    let Some(face) = cache.faces.get(&id) else {
        return;
    };
    if let Some(hub) = &face.hub {
        hand::draw_hand_sprite(
            canvas,
            hub,
            cx,
            cy,
            layout.hub_screen_diameter,
            0.0,
            hand::ivory_tint(night),
        );
    }
}

pub fn draw_second_hand(
    canvas: &mut Canvas<Window>,
    cx: i32,
    cy: i32,
    length: i32,
    angle_deg: f32,
    night: bool,
) {
    with_face(|face| {
        hand::draw_hand_sprite(
            canvas,
            &face.second_hand,
            cx,
            cy,
            length,
            angle_deg,
            hand::second_tint(night),
        );
    });
}