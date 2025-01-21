use crate::species::Species;

pub fn rgba_to_species(r: u8, g: u8, b: u8, a: u8) -> u8 {
    // 透明时返回 Empty
    if a < 250 {
        return Species::Empty as u8;
    }

    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;

    let min = r.min(g).min(b);
    let max = r.max(g).max(b);
    let delta = max - min;

    let mut h: f64;
    let s: f64;
    let l = (min + max) / 2.0;

    if max == min {
        h = 0.0;
    } else if r == max {
        h = (g - b) / delta;
    } else if g == max {
        h = 2.0 + (b - r) / delta;
    } else {
        h = 4.0 + (r - g) / delta;
    }

    h = (h * 60.0).min(360.0);
    if h < 0.0 { h += 360.0; }

    let s = if max == min {
        0.0
    } else if l <= 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };

    // 判断灰度颜色并返回相应物种
    if s < 0.05 {
        if l < 0.05 {
            return Species::Wall as u8;
        }
        if l > 0.95 {
            return Species::Empty as u8;
        }
        if l > 0.5 {
            return Species::Sand as u8;
        }
    }

    // 对非灰度颜色进行物种分类
    let hue_index = ((h + 25.7) / 360.0 * 7.0).floor() as usize;
    let lightness_index = (l * 4.0 - 0.25).floor() as usize;

    let colors_to_species: Vec<Vec<Option<Species>>> = vec![
        vec![Some(Species::Fire), Some(Species::Lava), Some(Species::Rocket)], // Red
        vec![Some(Species::Wood), None, Some(Species::Gas)], // Yellow
        vec![Some(Species::Plant), Some(Species::Dust), Some(Species::Acid)], // Green
        vec![Some(Species::Plant), Some(Species::Dust), Some(Species::Acid)], // Green2
        vec![Some(Species::Water), Some(Species::Ice), Some(Species::Stone)], // Blue
        vec![Some(Species::Oil), Some(Species::Seed), Some(Species::Fungus)], // Purple
        vec![Some(Species::Cloner), Some(Species::Mite), None], // Violet
    ];

    if let Some(species) = colors_to_species.get(hue_index).and_then(|x| x.get(lightness_index)) {
        return species.map_or(Species::Empty as u8, |s| s as u8);
    }

    Species::Empty as u8
}