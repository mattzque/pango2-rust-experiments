pub mod freetype {
    use std::{
        ffi::{c_uchar, CStr, CString},
        os::raw::c_long,
    };

    use freetype_sys::{
        FT_Done_Face, FT_Done_Library, FT_Face, FT_Init_FreeType, FT_Library, FT_New_Face, FT_New_Memory_Face,
    };

    #[derive(Debug)]
    pub enum FreetypeError {
        InitError,
        FaceError,
    }

    pub struct Library {
        raw: FT_Library,
    }

    impl Library {
        pub fn init() -> Result<Library, FreetypeError> {
            unsafe {
                let mut raw = std::mem::MaybeUninit::uninit();
                let err = FT_Init_FreeType(raw.as_mut_ptr());
                if err == 0 {
                    Ok(Library {
                        raw: raw.assume_init(),
                    })
                } else {
                    Err(FreetypeError::InitError)
                }
            }
        }

        pub fn raw(&self) -> &FT_Library {
            &self.raw
        }

        pub fn raw_mut(&mut self) -> &mut FT_Library {
            &mut self.raw
        }

        pub fn face_from_file(&self, path: &str, face_index: i32) -> Result<Face, FreetypeError> {
            unsafe {
                let filename = CString::new(path).unwrap();
                let mut raw = std::mem::MaybeUninit::uninit();
                let err = FT_New_Face(
                    self.raw,
                    filename.as_ptr(),
                    face_index as c_long,
                    raw.as_mut_ptr(),
                );
                if err == 0 {
                    Ok(Face {
                        raw: raw.assume_init(),
                    })
                } else {
                    Err(FreetypeError::FaceError)
                }
            }
        }

        pub fn face_from_buffer(&self, buffer: &[u8], face_index: i32) -> Result<Face, FreetypeError> {
            unsafe {
                let mut raw = std::mem::MaybeUninit::uninit();
                let err = FT_New_Memory_Face(
                    self.raw,
                    buffer.as_ptr() as *const c_uchar,
                    buffer.len() as c_long,
                    face_index as c_long,
                    raw.as_mut_ptr(),
                );
                if err == 0 {
                    Ok(Face {
                        raw: raw.assume_init(),
                    })
                } else {
                    Err(FreetypeError::FaceError)
                }
            }
        }
    }

    impl Drop for Library {
        fn drop(&mut self) {
            unsafe {
                FT_Done_Library(self.raw);
            }
        }
    }

    pub struct Face {
        raw: FT_Face,
    }

    impl Face {
        pub fn face_name(&self) -> String {
            unsafe {
                let name = (*self.raw).family_name;
                CStr::from_ptr(name).to_string_lossy().to_string()
            }
        }

        pub fn raw(&self) -> &FT_Face {
            &self.raw
        }

        pub fn raw_mut(&mut self) -> &mut FT_Face {
            &mut self.raw
        }
    }

    impl Drop for Face {
        fn drop(&mut self) {
            unsafe {
                FT_Done_Face(self.raw);
            }
        }
    }
}

pub mod harfbuzz {
    use std::ffi::{c_char, c_uint, CStr};

    use super::freetype;
    use harfbuzz_sys::{
        hb_face_destroy, hb_face_make_immutable, hb_face_t, hb_ft_face_create_referenced,
        hb_language_t, hb_ot_name_get_utf8, hb_ot_var_get_named_instance_count, hb_ot_var_has_data,
        hb_ot_var_named_instance_get_subfamily_name_id,
    };

    pub struct Face {
        raw: *mut hb_face_t,
    }

    impl Face {
        pub fn from_ft(face: &freetype::Face) -> Face {
            unsafe {
                // let face_ptr: &mut freetype_sys::FT_FaceRec = face.raw_mut();
                // let face_ptr2: *mut harfbuzz_sys::FT_FaceRec_ = face_ptr as *mut freetype_sys::FT_FaceRec as *mut harfbuzz_sys::FT_FaceRec_;
                let hb_face = hb_ft_face_create_referenced(*face.raw());
                hb_face_make_immutable(hb_face);
                Face { raw: hb_face }
            }
        }

        pub fn raw(&self) -> &*mut hb_face_t {
            &self.raw
        }

        pub fn is_variable(&self) -> bool {
            unsafe { hb_ot_var_has_data(self.raw) == 1 }
        }

        pub fn get_named_instances(&self) -> Vec<String> {
            unsafe {
                let count = hb_ot_var_get_named_instance_count(self.raw);
                const MAX_NAME_LENGTH: usize = 255;
                let buffer = [0i8; MAX_NAME_LENGTH];
                let mut names = Vec::new();

                let invalid_language: hb_language_t = 0 as hb_language_t;

                for i in 0..count {
                    // hb_ot_name_get_utf8 receives the size of the buffer in bytes and
                    // writes the actual size of the string in bytes to the same variable
                    // it also writes a null byte so we ignore that
                    let mut buffer_length: c_uint = MAX_NAME_LENGTH as c_uint;
                    hb_ot_name_get_utf8(
                        self.raw,
                        hb_ot_var_named_instance_get_subfamily_name_id(self.raw, i),
                        invalid_language,
                        &mut buffer_length,
                        buffer.as_ptr() as *mut c_char,
                    );
                    names.push(
                        CStr::from_ptr(buffer.as_ptr())
                            .to_string_lossy()
                            .to_string(),
                    );
                }

                names
            }
        }
    }

    impl Drop for Face {
        fn drop(&mut self) {
            unsafe {
                hb_face_destroy(self.raw);
            }
        }
    }
}

pub mod pango2 {
    use super::cairo;
    use super::harfbuzz;
    use gobject_sys::{g_object_unref, GObject};
    use pango2_sys::{pango2_font_description_free, pango2_hb_face_new_from_hb_face};
    use std::{
        ffi::{c_int, CString},
        ptr,
    };

    pub struct Pango2HbFace {
        raw: *mut pango2_sys::Pango2HbFace,
    }

    impl Pango2HbFace {
        pub fn from_hb_face(face: &harfbuzz::Face, instance_id: i32) -> Self {
            unsafe {
                let raw = pango2_hb_face_new_from_hb_face(
                    *face.raw(),
                    instance_id as c_int,
                    ptr::null(),
                    ptr::null(),
                );
                Self { raw }
            }
        }
    }

    /* 
    TODO This crashes / segfaults

        impl Drop for Pango2HbFace {
            fn drop(&mut self) {
                unsafe {
                    g_object_unref(self.raw as *mut GObject);
                }
            }
        }
    */

    pub struct Pango2FontMap {
        raw: *mut pango2_sys::Pango2FontMap,
    }

    impl Pango2FontMap {
        pub fn new() -> Self {
            unsafe {
                Pango2FontMap {
                    raw: pango2_sys::pango2_font_map_new(),
                }
            }
        }

        pub fn add_face(&self, face: &Pango2HbFace) {
            unsafe {
                pango2_sys::pango2_font_map_add_face(
                    self.raw,
                    face.raw as *mut pango2_sys::Pango2FontFace,
                );
            }
        }
    }

    impl Default for Pango2FontMap {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Drop for Pango2FontMap {
        fn drop(&mut self) {
            unsafe {
                g_object_unref(self.raw as *mut GObject);
            }
        }
    }

    pub struct Pango2Context {
        raw: *mut pango2_sys::Pango2Context,
    }

    impl Pango2Context {
        pub fn from_font_map(font_map: &Pango2FontMap) -> Self {
            unsafe {
                Pango2Context {
                    raw: pango2_sys::pango2_context_new_with_font_map(font_map.raw),
                }
            }
        }

        pub fn update_cairo_context(&self, cairo_context: &cairo::CairoContext) {
            unsafe {
                pango2_sys::pango2_cairo_update_context(*cairo_context.raw(), self.raw);
            }
        }
    }

    impl Drop for Pango2Context {
        fn drop(&mut self) {
            unsafe {
                g_object_unref(self.raw as *mut GObject);
            }
        }
    }

    pub struct Pango2Layout {
        raw: *mut pango2_sys::Pango2Layout,
    }

    impl Pango2Layout {
        pub fn new(context: &Pango2Context) -> Self {
            unsafe {
                Pango2Layout {
                    raw: pango2_sys::pango2_layout_new(context.raw),
                }
            }
        }

        pub fn set_font_description_string(&self, font_description: &str) {
            let description = FontDescription::from_string(font_description);
            self.set_font_description(&description);
        }

        pub fn set_font_description(&self, font_description: &FontDescription) {
            unsafe {
                pango2_sys::pango2_layout_set_font_description(self.raw, font_description.raw);
            }
        }

        pub fn set_text(&self, text: &str) {
            unsafe {
                let ctext = CString::new(text).unwrap();
                pango2_sys::pango2_layout_set_text(self.raw, ctext.as_ptr(), -1);
            }
        }

        pub fn paint(&self, cairo_context: &cairo::CairoContext) {
            unsafe {
                pango2_sys::pango2_cairo_show_layout(*cairo_context.raw(), self.raw);
            }
        }
    }

    impl Drop for Pango2Layout {
        fn drop(&mut self) {
            unsafe {
                g_object_unref(self.raw as *mut GObject);
            }
        }
    }

    pub struct FontDescription {
        raw: *mut pango2_sys::Pango2FontDescription,
    }

    impl FontDescription {
        pub fn from_string(string: &str) -> Self {
            unsafe {
                let cstring = CString::new(string).unwrap();
                FontDescription {
                    raw: pango2_sys::pango2_font_description_from_string(cstring.as_ptr()),
                }
            }
        }
    }

    impl Drop for FontDescription {
        fn drop(&mut self) {
            unsafe {
                pango2_font_description_free(self.raw);
            }
        }
    }
}

pub mod cairo {
    use std::ffi::{c_double, c_int, CString};

    #[derive(Debug)]
    pub enum CairoError {
        SurfaceCreateError
    }

    pub struct CairoSurface {
        raw: *mut cairo_sys::cairo_surface_t,
    }

    impl CairoSurface {
        pub fn new_image_surface(width: i32, height: i32) -> Result<Self, CairoError> {
            unsafe {
                let raw = cairo_sys::cairo_image_surface_create(
                    cairo_sys::_cairo_format_CAIRO_FORMAT_ARGB32,
                    width as c_int,
                    height as c_int,
                );
                let status = cairo_sys::cairo_surface_status(raw);
                if status == cairo_sys::_cairo_status_CAIRO_STATUS_SUCCESS {
                    Ok(Self { raw })
                } else {
                    Err(CairoError::SurfaceCreateError)
                }
            }
        }

        pub fn write_to_png(&self, path: &str) -> Result<(), CairoError> {
            unsafe {
                let cpath = CString::new(path).unwrap();
                let status = cairo_sys::cairo_surface_write_to_png(self.raw, cpath.as_ptr());
                if status == cairo_sys::_cairo_status_CAIRO_STATUS_SUCCESS {
                    Ok(())
                } else {
                    Err(CairoError::SurfaceCreateError)
                }
            }
        }
    }

    impl Drop for CairoSurface {
        fn drop(&mut self) {
            unsafe {
                cairo_sys::cairo_surface_destroy(self.raw);
            }
        }
    }

    pub struct CairoContext {
        raw: *mut cairo_sys::cairo_t,
    }

    impl CairoContext {
        pub fn create(surface: &CairoSurface) -> Self {
            unsafe {
                Self {
                    raw: cairo_sys::cairo_create(surface.raw),
                }
            }
        }

        pub fn raw(&self) -> &*mut cairo_sys::cairo_t {
            &self.raw
        }

        pub fn set_source_rgb(&self, r: f32, g: f32, b: f32) {
            unsafe {
                cairo_sys::cairo_set_source_rgb(
                    self.raw,
                    r as c_double,
                    g as c_double,
                    b as c_double,
                );
            }
        }

        pub fn set_source_rgba(&self, r: f32, g: f32, b: f32, a: f32) {
            unsafe {
                cairo_sys::cairo_set_source_rgba(
                    self.raw,
                    r as c_double,
                    g as c_double,
                    b as c_double,
                    a as c_double,
                );
            }
        }

        pub fn paint(&self) {
            unsafe {
                cairo_sys::cairo_paint(self.raw);
            }
        }

        pub fn paint_with_alpha(&self, alpha: f32) {
            unsafe {
                cairo_sys::cairo_paint_with_alpha(self.raw, alpha as c_double);
            }
        }
    }

    impl Drop for CairoContext {
        fn drop(&mut self) {
            unsafe {
                cairo_sys::cairo_destroy(self.raw);
            }
        }
    }
}