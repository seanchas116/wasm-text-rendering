use log::info;
use std::{f64, str::FromStr};
use wasm_bindgen::prelude::*;

struct TestOutlineBuilder<'a> {
    context: &'a web_sys::CanvasRenderingContext2d,
    scale: f64,
}

impl<'a> ttf_parser::OutlineBuilder for TestOutlineBuilder<'a> {
    fn move_to(&mut self, x: f32, y: f32) {
        //info!("move_to: {}, {}", x, y);
        self.context
            .move_to(x as f64 * self.scale, y as f64 * self.scale);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        //info!("line_to: {}, {}", x, y);
        self.context
            .line_to(x as f64 * self.scale, y as f64 * self.scale);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        //info!("quad_to: {}, {}, {}, {}", x1, y1, x, y);
        self.context.quadratic_curve_to(
            x1 as f64 * self.scale,
            y1 as f64 * self.scale,
            x as f64 * self.scale,
            y as f64 * self.scale,
        );
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        //info!("curve_to: {}, {}, {}, {}, {}, {}", x1, y1, x2, y2, x, y);
        self.context.bezier_curve_to(
            x1 as f64 * self.scale,
            y1 as f64 * self.scale,
            x2 as f64 * self.scale,
            y2 as f64 * self.scale,
            x as f64 * self.scale,
            y as f64 * self.scale,
        );
    }

    fn close(&mut self) {
        //info!("close");
        self.context.close_path();
    }
}

#[wasm_bindgen]
pub struct TextRenderer {
    context: web_sys::CanvasRenderingContext2d,
    font_face: ttf_parser::Face<'static>,
    rustybuzz_face: rustybuzz::Face<'static>,
}

#[wasm_bindgen]
impl TextRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(context: web_sys::CanvasRenderingContext2d) -> TextRenderer {
        let font_data = include_bytes!("NotoSerifJP-Regular.otf");
        let font_face = ttf_parser::Face::from_slice(font_data, 0).unwrap();
        let rustybuzz_face = rustybuzz::Face::from_slice(font_data, 0).unwrap();
        TextRenderer {
            context,
            font_face,
            rustybuzz_face,
        }
    }

    #[wasm_bindgen(js_name=drawText)]
    pub fn draw_text(
        &self,
        text: &str,
        vertical: bool,
        font_size: f64,
        line_height: f64,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        self.context.save();

        let scale = font_size / self.font_face.units_per_em() as f64;

        if vertical {
            self.context.translate(x + width, y);
        } else {
            self.context.translate(x, y);
        }

        let linebreaks: Vec<(usize, unicode_linebreak::BreakOpportunity)> =
            unicode_linebreak::linebreaks(text).collect();

        let mut pos_x = 0.0;
        let mut pos_y = 0.0;

        let mut last_offset: usize = 0;
        for (offset, break_type) in linebreaks {
            let mut span = &text[last_offset..offset];
            last_offset = offset;
            if span.ends_with("\n") {
                span = &span[..span.len() - 1];
            }

            let mut buffer = rustybuzz::UnicodeBuffer::new();
            buffer.push_str(span);
            if vertical {
                buffer.set_direction(rustybuzz::Direction::TopToBottom);
            }

            let glyph_buffer = rustybuzz::shape(&self.rustybuzz_face, &[], buffer);

            if vertical {
                let span_height = glyph_buffer
                    .glyph_positions()
                    .iter()
                    .map(|info| info.y_advance.abs())
                    .sum::<i32>() as f64
                    * scale;

                if pos_y + span_height > height {
                    pos_x -= line_height;
                    pos_y = 0.0;
                }
            } else {
                let span_width = glyph_buffer
                    .glyph_positions()
                    .iter()
                    .map(|info| info.x_advance)
                    .sum::<i32>() as f64
                    * scale;

                if pos_x + span_width > width {
                    pos_x = 0.0;
                    pos_y += line_height;
                }
            }

            for i in 0..glyph_buffer.len() {
                let glyph = glyph_buffer.glyph_infos()[i];
                let pos = glyph_buffer.glyph_positions()[i];

                let char = span[(glyph.cluster as usize)..].chars().next().unwrap();
                info!("char: {}", char);

                let glyph_id = ttf_parser::GlyphId(glyph.glyph_id as u16);

                let vertical_orientation = unicode_vo::char_orientation(char);

                self.context.save();
                self.context.translate(pos_x, pos_y + font_size);
                self.context.scale(1.0, -1.0);
                if vertical && vertical_orientation == unicode_vo::Orientation::Rotated {
                    self.context.rotate(-std::f64::consts::FRAC_PI_2);
                    self.context.translate(-font_size, 0.0);
                }

                self.context.scale(scale, scale);

                self.context.begin_path();

                self.font_face.outline_glyph(
                    glyph_id,
                    &mut TestOutlineBuilder {
                        context: &self.context,
                        scale: 1.0,
                    },
                );

                self.context.fill();
                self.context.restore();

                if vertical {
                    if vertical_orientation == unicode_vo::Orientation::Rotated {
                        pos_y +=
                            self.font_face.glyph_hor_advance(glyph_id).unwrap_or(0) as f64 * scale;
                    } else {
                        pos_y +=
                            self.font_face.glyph_ver_advance(glyph_id).unwrap_or(0) as f64 * scale;
                    }
                } else {
                    pos_x += self.font_face.glyph_hor_advance(glyph_id).unwrap_or(0) as f64 * scale;
                }

                // if vertical && vertical_orientation == unicode_vo::Orientation::Rotated {
                //     pos_y += pos.x_advance as f64 * scale;
                //     pos_x -= pos.y_advance as f64 * scale;
                // } else {
                //     pos_x += pos.x_advance as f64 * scale;
                //     pos_y -= pos.y_advance as f64 * scale;
                // }
            }

            if break_type == unicode_linebreak::BreakOpportunity::Mandatory {
                if vertical {
                    pos_x -= line_height;
                    pos_y = 0.0;
                } else {
                    pos_x = 0.0;
                    pos_y += line_height;
                }
            }
        }

        self.context.restore();
    }
}
