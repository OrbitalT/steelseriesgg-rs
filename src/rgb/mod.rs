//! RGB lighting control and effects.

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// RGB color representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new color from RGB values.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create color from hex value (0xRRGGBB).
    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    /// Convert to hex value.
    pub const fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// Black (off).
    pub const BLACK: Color = Color::new(0, 0, 0);
    /// White.
    pub const WHITE: Color = Color::new(255, 255, 255);
    /// Red.
    pub const RED: Color = Color::new(255, 0, 0);
    /// Green.
    pub const GREEN: Color = Color::new(0, 255, 0);
    /// Blue.
    pub const BLUE: Color = Color::new(0, 0, 255);
    /// Cyan.
    pub const CYAN: Color = Color::new(0, 255, 255);
    /// Magenta.
    pub const MAGENTA: Color = Color::new(255, 0, 255);
    /// Yellow.
    pub const YELLOW: Color = Color::new(255, 255, 0);
    /// Orange.
    pub const ORANGE: Color = Color::new(255, 128, 0);
    /// Purple.
    pub const PURPLE: Color = Color::new(128, 0, 255);
    /// Pink.
    pub const PINK: Color = Color::new(255, 105, 180);

    /// Blend between two colors.
    pub fn blend(a: Color, b: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color {
            r: (a.r as f32 * (1.0 - t) + b.r as f32 * t) as u8,
            g: (a.g as f32 * (1.0 - t) + b.g as f32 * t) as u8,
            b: (a.b as f32 * (1.0 - t) + b.b as f32 * t) as u8,
        }
    }

    /// Scale brightness (0.0 = black, 1.0 = original).
    pub fn scale(&self, factor: f32) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        Color {
            r: (self.r as f32 * factor) as u8,
            g: (self.g as f32 * factor) as u8,
            b: (self.b as f32 * factor) as u8,
        }
    }

    /// Convert from HSV to RGB.
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Color {
        let h = h % 360.0;
        let s = s.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Color {
            r: ((r + m) * 255.0) as u8,
            g: ((g + m) * 255.0) as u8,
            b: ((b + m) * 255.0) as u8,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

/// RGB lighting effect types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effect {
    /// Static single color.
    Static { color: Color },

    /// Breathing effect (pulse).
    Breathing {
        color: Color,
        speed: f32, // Cycles per second
    },

    /// Color cycle through spectrum.
    Spectrum {
        speed: f32, // Cycles per second
    },

    /// Wave effect across zones.
    Wave {
        colors: Vec<Color>,
        speed: f32,
        direction: WaveDirection,
    },

    /// Reactive effect (responds to key presses).
    Reactive {
        color: Color,
        duration: f32, // Seconds
    },

    /// Gradient between two colors.
    Gradient { start: Color, end: Color },

    /// Custom per-zone colors.
    Custom { colors: Vec<Color> },

    /// Disabled (all LEDs off).
    Off,
}

/// Wave effect direction.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WaveDirection {
    LeftToRight,
    RightToLeft,
    CenterOut,
    OutCenter,
}

impl Default for Effect {
    fn default() -> Self {
        Effect::Static {
            color: Color::WHITE,
        }
    }
}

/// RGB effect engine that computes colors over time.
pub struct EffectEngine {
    effect: Effect,
    start_time: Instant,
    zone_count: usize,
}

impl EffectEngine {
    /// Create a new effect engine.
    pub fn new(effect: Effect, zone_count: usize) -> Self {
        Self {
            effect,
            start_time: Instant::now(),
            zone_count,
        }
    }

    /// Set a new effect.
    pub fn set_effect(&mut self, effect: Effect) {
        self.effect = effect;
        self.start_time = Instant::now();
    }

    /// Get the current effect.
    pub fn effect(&self) -> &Effect {
        &self.effect
    }

    /// Compute current colors for all zones.
    pub fn compute(&self) -> Vec<Color> {
        let elapsed = self.start_time.elapsed().as_secs_f32();

        match &self.effect {
            Effect::Static { color } => vec![*color; self.zone_count],

            Effect::Breathing { color, speed } => {
                // Sine wave for breathing effect
                let t = (elapsed * speed * 2.0 * std::f32::consts::PI).sin();
                let brightness = (t + 1.0) / 2.0; // Normalize to 0-1
                vec![color.scale(brightness); self.zone_count]
            }

            Effect::Spectrum { speed } => {
                // Cycle through hue
                let hue = (elapsed * speed * 360.0) % 360.0;
                let color = Color::from_hsv(hue, 1.0, 1.0);
                vec![color; self.zone_count]
            }

            Effect::Wave {
                colors,
                speed,
                direction,
            } => {
                if colors.is_empty() {
                    return vec![Color::BLACK; self.zone_count];
                }

                let phase = elapsed * speed;
                let mut result = Vec::with_capacity(self.zone_count);

                for i in 0..self.zone_count {
                    let zone_offset = match direction {
                        WaveDirection::LeftToRight => i as f32 / self.zone_count as f32,
                        WaveDirection::RightToLeft => {
                            1.0 - (i as f32 / self.zone_count as f32)
                        }
                        WaveDirection::CenterOut => {
                            let center = self.zone_count as f32 / 2.0;
                            (i as f32 - center).abs() / center
                        }
                        WaveDirection::OutCenter => {
                            let center = self.zone_count as f32 / 2.0;
                            1.0 - (i as f32 - center).abs() / center
                        }
                    };

                    let t = (phase + zone_offset) % 1.0;
                    let color_index = (t * colors.len() as f32) as usize % colors.len();
                    let next_index = (color_index + 1) % colors.len();
                    let blend_t = (t * colors.len() as f32) % 1.0;

                    result.push(Color::blend(colors[color_index], colors[next_index], blend_t));
                }

                result
            }

            Effect::Reactive { color, .. } => {
                // Base state - actual reactivity handled by input events
                vec![color.scale(0.2); self.zone_count]
            }

            Effect::Gradient { start, end } => {
                let mut result = Vec::with_capacity(self.zone_count);
                for i in 0..self.zone_count {
                    let t = i as f32 / (self.zone_count - 1).max(1) as f32;
                    result.push(Color::blend(*start, *end, t));
                }
                result
            }

            Effect::Custom { colors } => {
                let mut result = colors.clone();
                result.resize(self.zone_count, Color::BLACK);
                result
            }

            Effect::Off => vec![Color::BLACK; self.zone_count],
        }
    }

    /// Reset the effect timer.
    pub fn reset(&mut self) {
        self.start_time = Instant::now();
    }
}

/// RGB controller for managing device lighting.
pub struct RgbController {
    engine: EffectEngine,
    brightness: f32,
}

impl RgbController {
    /// Create a new RGB controller.
    pub fn new(zone_count: usize) -> Self {
        Self {
            engine: EffectEngine::new(Effect::default(), zone_count),
            brightness: 1.0,
        }
    }

    /// Set the lighting effect.
    pub fn set_effect(&mut self, effect: Effect) {
        self.engine.set_effect(effect);
    }

    /// Set brightness (0.0 to 1.0).
    pub fn set_brightness(&mut self, brightness: f32) {
        self.brightness = brightness.clamp(0.0, 1.0);
    }

    /// Get current brightness.
    pub fn brightness(&self) -> f32 {
        self.brightness
    }

    /// Compute current colors with brightness applied.
    pub fn compute_colors(&self) -> Vec<Color> {
        self.engine
            .compute()
            .into_iter()
            .map(|c| c.scale(self.brightness))
            .collect()
    }
}
