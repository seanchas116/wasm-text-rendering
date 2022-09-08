use log::info;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_console_logger::DEFAULT_LOGGER;

#[wasm_bindgen]
pub fn start() {
    log::set_logger(&DEFAULT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    info!("{canvas:?}");

    draw_text(&context, "あのイーハトーヴォのすきとおった風、夏でも底に冷たさをもつ青いそら、\nうつくしい森で飾られたモリーオ市、郊外のぎらぎらひかる草の波。\nLorem ipsum dolor sit amet, consectetur adipiscing elit,\nsed do eiusmod tempor incididunt ut labore et dolore magna aliqua.", 48.0);
}

struct TestOutlineBuilder<'a> {
    context: &'a web_sys::CanvasRenderingContext2d,
    scale: f64,
}

impl<'a> ttf_parser::OutlineBuilder for TestOutlineBuilder<'a> {
    fn move_to(&mut self, x: f32, y: f32) {
        info!("move_to: {}, {}", x, y);
        self.context
            .move_to(x as f64 * self.scale, y as f64 * self.scale);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        info!("line_to: {}, {}", x, y);
        self.context
            .line_to(x as f64 * self.scale, y as f64 * self.scale);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        info!("quad_to: {}, {}, {}, {}", x1, y1, x, y);
        self.context.quadratic_curve_to(
            x1 as f64 * self.scale,
            y1 as f64 * self.scale,
            x as f64 * self.scale,
            y as f64 * self.scale,
        );
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        info!("curve_to: {}, {}, {}, {}, {}, {}", x1, y1, x2, y2, x, y);
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
        info!("close");
        self.context.close_path();
    }
}

fn draw_text(context: &web_sys::CanvasRenderingContext2d, text: &str, size: f64) {
    let font_data = include_bytes!("NotoSansJP-Regular.otf");

    let face = ttf_parser::Face::from_slice(font_data, 0).unwrap();
    info!("units per em: {}", face.units_per_em());

    let scale = size / face.units_per_em() as f64;

    let rustybuzz_face = rustybuzz::Face::from_slice(font_data, 0).unwrap();

    context.translate(20.0, 20.0);
    context.translate(0.0, size);

    for (line_num, line) in text.lines().enumerate() {
        let mut buffer = rustybuzz::UnicodeBuffer::new();
        buffer.push_str(&line);
        let glyph_buffer = rustybuzz::shape(&rustybuzz_face, &[], buffer);

        context.save();
        context.scale(1.0, -1.0);

        for i in 0..glyph_buffer.len() {
            let glyph = glyph_buffer.glyph_infos()[i];
            let pos = glyph_buffer.glyph_positions()[i];

            info!("{:?}", pos);

            context.begin_path();

            face.outline_glyph(
                ttf_parser::GlyphId(glyph.glyph_id as u16),
                &mut TestOutlineBuilder { context, scale },
            );

            context.fill();

            context.translate(pos.x_advance as f64 * scale, 0.0);
        }

        context.restore();
        context.translate(0.0, size * 1.5);
    }
}
