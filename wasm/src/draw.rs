use log::info;
use std::f64;

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

pub fn draw_text(context: &web_sys::CanvasRenderingContext2d, text: &str, size: f64) {
    let font_data = include_bytes!("NotoSansJP-Regular.otf");

    let face = ttf_parser::Face::from_slice(font_data, 0).unwrap();

    let scale = size / face.units_per_em() as f64;

    let rustybuzz_face = rustybuzz::Face::from_slice(font_data, 0).unwrap();

    context.translate(20.0, 20.0);
    context.translate(0.0, size);

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
        info!("span: {:?}", span);

        let mut buffer = rustybuzz::UnicodeBuffer::new();
        buffer.push_str(span);
        let glyph_buffer = rustybuzz::shape(&rustybuzz_face, &[], buffer);

        for i in 0..glyph_buffer.len() {
            let glyph = glyph_buffer.glyph_infos()[i];
            let pos = glyph_buffer.glyph_positions()[i];

            context.save();
            context.translate(pos_x, pos_y);
            context.scale(1.0, -1.0);

            context.begin_path();

            face.outline_glyph(
                ttf_parser::GlyphId(glyph.glyph_id as u16),
                &mut TestOutlineBuilder { context, scale },
            );

            context.fill();
            context.restore();

            pos_x += pos.x_advance as f64 * scale;
        }

        if (break_type == unicode_linebreak::BreakOpportunity::Mandatory) {
            pos_x = 0.0;
            pos_y += size * 1.5;
        }
    }
}
