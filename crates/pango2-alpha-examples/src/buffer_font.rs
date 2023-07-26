use pango2_sys_examples::{freetype, harfbuzz, pango2, cairo};

fn main() {
    let buffer = include_bytes!("../fonts/NotoSerifDisplay/NotoSerifDisplay-VariableFont_wdth,wght.ttf");

    println!("Loading font from buffer ({} bytes)", buffer.len());

    let lib = freetype::Library::init().unwrap();
    let face = lib.face_from_buffer(buffer, 0).unwrap();

    println!("Family name: {}", face.face_name());

    let hb_face = harfbuzz::Face::from_ft(&face);
    println!("Is font face variable? {:?}", hb_face.is_variable());
    println!(
        "Named instances in font: {:?}",
        hb_face.get_named_instances()
    );

    let pango2_face = pango2::Pango2HbFace::from_hb_face(&hb_face, -2);

    let font_map = pango2::Pango2FontMap::new();
    font_map.add_face(&pango2_face);

    let pango_context = pango2::Pango2Context::from_font_map(&font_map);

    let cairo_surface = cairo::CairoSurface::new_image_surface(650, 150).unwrap();
    let cairo_context = cairo::CairoContext::create(&cairo_surface);

    pango_context.update_cairo_context(&cairo_context);

    cairo_context.set_source_rgb(1.0, 1.0, 1.0);
    cairo_context.paint();

    let layout = pango2::Pango2Layout::new(&pango_context);

    layout.set_text("Hola, Pango2!");
    layout.set_font_description_string(format!("{} Regular 64", face.face_name()).as_str());

    cairo_context.set_source_rgb(0.0, 0.0, 1.0);
    layout.paint(&cairo_context);

    let output = "buffer_font_test.png";
    println!("Write rendered text to file: {}", output);
    cairo_surface.write_to_png(output).unwrap();
}