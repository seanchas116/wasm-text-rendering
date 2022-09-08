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

    draw_text(&context, "あのイーハトーヴォのすきとおった風、夏でも底に冷たさをもつ青いそら、うつくしい森で飾られたモリーオ市、郊外のぎらぎらひかる草の波。");
}

struct TestOutlineBuilder<'a> {
    context: &'a web_sys::CanvasRenderingContext2d,
}

impl<'a> ttf_parser::OutlineBuilder for TestOutlineBuilder<'a> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.context.move_to(x as f64, y as f64);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.context.line_to(x as f64, y as f64);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.context
            .quadratic_curve_to(x1 as f64, y1 as f64, x as f64, y as f64);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.context.bezier_curve_to(
            x1 as f64, y1 as f64, x2 as f64, y2 as f64, x as f64, y as f64,
        );
    }

    fn close(&mut self) {
        self.context.close_path();
    }
}

fn draw_text(context: &web_sys::CanvasRenderingContext2d, text: &str) {
    let font_data = include_bytes!("NotoSansJP-Regular.otf");

    let face = ttf_parser::Face::from_slice(font_data, 0).unwrap();
    info!("face: {}", face.number_of_glyphs());

    let mut rustybuzz_face = rustybuzz::Face::from_slice(font_data, 0).unwrap();
    let mut buffer = rustybuzz::UnicodeBuffer::new();
    buffer.push_str(&text);

    let glyph_buffer = rustybuzz::shape(&rustybuzz_face, &[], buffer);

    info!(
        "{}",
        glyph_buffer.serialize(&rustybuzz_face, rustybuzz::SerializeFlags::default())
    );

    context.translate(0.0, 100.0);
    context.scale(0.05, -0.05);

    for i in 0..glyph_buffer.len() {
        let glyph = glyph_buffer.glyph_infos()[i];
        let pos = glyph_buffer.glyph_positions()[i];

        info!("{:?}", pos);

        face.outline_glyph(
            ttf_parser::GlyphId(glyph.glyph_id as u16),
            &mut TestOutlineBuilder { context },
        );

        context.fill();

        context.translate(pos.x_advance as f64, 0.0);
    }
}
