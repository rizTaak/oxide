pub use self::enumerations::*;
pub use self::functions::*;
pub use self::types::*;

use std::os::raw::c_void;

#[derive(Copy, Clone)]
struct FnPtr {
    ptr: *const c_void,
    is_loaded: bool,
}

#[allow(dead_code)]
impl FnPtr {
    fn new(ptr: *const c_void) -> FnPtr {
        if !ptr.is_null() {
            FnPtr {
                ptr,
                is_loaded: true,
            }
        } else {
            FnPtr {
                ptr: FnPtr::not_initialized as *const c_void,
                is_loaded: false,
            }
        }
    }

    fn set_ptr(&mut self, ptr: *const c_void) {
        *self = Self::new(ptr);
    }

    fn aliased(&mut self, other: &FnPtr) {
        if !self.is_loaded && other.is_loaded {
            *self = *other;
        }
    }

    #[inline(never)]
    fn not_initialized() -> ! {
        panic!("gl: function not initialized")
    }
}

unsafe impl Sync for FnPtr {}
unsafe impl Send for FnPtr {}

pub mod types {
    #![allow(dead_code, non_snake_case, non_camel_case_types)]

    use std::os::raw;

    pub type GLvoid = raw::c_void;

    pub type GLbyte = raw::c_char;
    pub type GLubyte = raw::c_uchar;
    pub type GLchar = raw::c_char;
    pub type GLboolean = raw::c_uchar;

    pub type GLshort = raw::c_short;
    pub type GLushort = raw::c_ushort;

    pub type GLint = raw::c_int;
    pub type GLuint = raw::c_uint;
    pub type GLint64 = i64;
    pub type GLuint64 = u64;

    pub type GLintptr = isize;
    pub type GLsizeiptr = isize;
    pub type GLintptrARB = isize;
    pub type GLsizeiptrARB = isize;
    pub type GLint64EXT = i64;
    pub type GLuint64EXT = u64;

    pub type GLsizei = GLint;
    pub type GLclampx = raw::c_int;
    pub type GLfixed = GLint;
    pub type GLhalf = raw::c_ushort;
    pub type GLhalfNV = raw::c_ushort;
    pub type GLhalfARB = raw::c_ushort;

    pub type GLenum = raw::c_uint;
    pub type GLbitfield = raw::c_uint;

    pub type GLfloat = raw::c_float;
    pub type GLdouble = raw::c_double;
    pub type GLclampf = raw::c_float;
    pub type GLclampd = raw::c_double;

    pub type GLcharARB = raw::c_char;

    #[cfg(target_os = "macos")]
    pub type GLhandleARB = *const raw::c_void;
    #[cfg(not(target_os = "macos"))]
    pub type GLhandleARB = raw::c_uint;

    pub enum __GLsync {}

    pub type GLsync = *const __GLsync;

    pub enum _cl_context {}

    pub enum _cl_event {}

    pub type GLvdpauSurfaceNV = GLintptr;
    pub type GLeglClientBufferEXT = *const raw::c_void;
    pub type GLeglImageOES = *const raw::c_void;

    pub type GLDEBUGPROC = extern "system" fn(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut raw::c_void,
    );
    pub type GLDEBUGPROCARB = extern "system" fn(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut raw::c_void,
    );
    pub type GLDEBUGPROCKHR = extern "system" fn(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut GLvoid,
    );
    pub type GLDEBUGPROCAMD = extern "system" fn(
        id: GLuint,
        category: GLenum,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut GLvoid,
    );
    pub type GLVULKANPROCNV = extern "system" fn();
}

pub mod enumerations {
    #![allow(dead_code, non_upper_case_globals, unused_imports)]

    use super::types::*;
    use std::os::raw::*;

    pub const ACTIVE_ATTRIBUTES: c_uint = 0x8B89;
    pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: c_uint = 0x8B8A;
    pub const ACTIVE_TEXTURE: c_uint = 0x84E0;
    pub const ACTIVE_UNIFORMS: c_uint = 0x8B86;
    pub const ACTIVE_UNIFORM_BLOCKS: c_uint = 0x8A36;
    pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: c_uint = 0x8A35;
    pub const ACTIVE_UNIFORM_MAX_LENGTH: c_uint = 0x8B87;
    pub const ALIASED_LINE_WIDTH_RANGE: c_uint = 0x846E;
    pub const ALPHA: c_uint = 0x1906;
    pub const ALREADY_SIGNALED: c_uint = 0x911A;
    pub const ALWAYS: c_uint = 0x0207;
    pub const AND: c_uint = 0x1501;
    pub const AND_INVERTED: c_uint = 0x1504;
    pub const AND_REVERSE: c_uint = 0x1502;
    pub const ANY_SAMPLES_PASSED: c_uint = 0x8C2F;
    pub const ARRAY_BUFFER: c_uint = 0x8892;
    pub const ARRAY_BUFFER_BINDING: c_uint = 0x8894;
    pub const ATTACHED_SHADERS: c_uint = 0x8B85;
    pub const BACK: c_uint = 0x0405;
    pub const BACK_LEFT: c_uint = 0x0402;
    pub const BACK_RIGHT: c_uint = 0x0403;
    pub const BGR: c_uint = 0x80E0;
    pub const BGRA: c_uint = 0x80E1;
    pub const BGRA_INTEGER: c_uint = 0x8D9B;
    pub const BGR_INTEGER: c_uint = 0x8D9A;
    pub const BLEND: c_uint = 0x0BE2;
    pub const BLEND_COLOR: c_uint = 0x8005;
    pub const BLEND_DST: c_uint = 0x0BE0;
    pub const BLEND_DST_ALPHA: c_uint = 0x80CA;
    pub const BLEND_DST_RGB: c_uint = 0x80C8;
    pub const BLEND_EQUATION: c_uint = 0x8009;
    pub const BLEND_EQUATION_ALPHA: c_uint = 0x883D;
    pub const BLEND_EQUATION_RGB: c_uint = 0x8009;
    pub const BLEND_SRC: c_uint = 0x0BE1;
    pub const BLEND_SRC_ALPHA: c_uint = 0x80CB;
    pub const BLEND_SRC_RGB: c_uint = 0x80C9;
    pub const BLUE: c_uint = 0x1905;
    pub const BLUE_INTEGER: c_uint = 0x8D96;
    pub const BOOL: c_uint = 0x8B56;
    pub const BOOL_VEC2: c_uint = 0x8B57;
    pub const BOOL_VEC3: c_uint = 0x8B58;
    pub const BOOL_VEC4: c_uint = 0x8B59;
    pub const BUFFER_ACCESS: c_uint = 0x88BB;
    pub const BUFFER_ACCESS_FLAGS: c_uint = 0x911F;
    pub const BUFFER_MAPPED: c_uint = 0x88BC;
    pub const BUFFER_MAP_LENGTH: c_uint = 0x9120;
    pub const BUFFER_MAP_OFFSET: c_uint = 0x9121;
    pub const BUFFER_MAP_POINTER: c_uint = 0x88BD;
    pub const BUFFER_SIZE: c_uint = 0x8764;
    pub const BUFFER_USAGE: c_uint = 0x8765;
    pub const BYTE: c_uint = 0x1400;
    pub const CCW: c_uint = 0x0901;
    pub const CLAMP_READ_COLOR: c_uint = 0x891C;
    pub const CLAMP_TO_BORDER: c_uint = 0x812D;
    pub const CLAMP_TO_EDGE: c_uint = 0x812F;
    pub const CLEAR: c_uint = 0x1500;
    pub const CLIP_DISTANCE0: c_uint = 0x3000;
    pub const CLIP_DISTANCE1: c_uint = 0x3001;
    pub const CLIP_DISTANCE2: c_uint = 0x3002;
    pub const CLIP_DISTANCE3: c_uint = 0x3003;
    pub const CLIP_DISTANCE4: c_uint = 0x3004;
    pub const CLIP_DISTANCE5: c_uint = 0x3005;
    pub const CLIP_DISTANCE6: c_uint = 0x3006;
    pub const CLIP_DISTANCE7: c_uint = 0x3007;
    pub const COLOR: c_uint = 0x1800;
    pub const COLOR_ATTACHMENT0: c_uint = 0x8CE0;
    pub const COLOR_ATTACHMENT1: c_uint = 0x8CE1;
    pub const COLOR_ATTACHMENT10: c_uint = 0x8CEA;
    pub const COLOR_ATTACHMENT11: c_uint = 0x8CEB;
    pub const COLOR_ATTACHMENT12: c_uint = 0x8CEC;
    pub const COLOR_ATTACHMENT13: c_uint = 0x8CED;
    pub const COLOR_ATTACHMENT14: c_uint = 0x8CEE;
    pub const COLOR_ATTACHMENT15: c_uint = 0x8CEF;
    pub const COLOR_ATTACHMENT16: c_uint = 0x8CF0;
    pub const COLOR_ATTACHMENT17: c_uint = 0x8CF1;
    pub const COLOR_ATTACHMENT18: c_uint = 0x8CF2;
    pub const COLOR_ATTACHMENT19: c_uint = 0x8CF3;
    pub const COLOR_ATTACHMENT2: c_uint = 0x8CE2;
    pub const COLOR_ATTACHMENT20: c_uint = 0x8CF4;
    pub const COLOR_ATTACHMENT21: c_uint = 0x8CF5;
    pub const COLOR_ATTACHMENT22: c_uint = 0x8CF6;
    pub const COLOR_ATTACHMENT23: c_uint = 0x8CF7;
    pub const COLOR_ATTACHMENT24: c_uint = 0x8CF8;
    pub const COLOR_ATTACHMENT25: c_uint = 0x8CF9;
    pub const COLOR_ATTACHMENT26: c_uint = 0x8CFA;
    pub const COLOR_ATTACHMENT27: c_uint = 0x8CFB;
    pub const COLOR_ATTACHMENT28: c_uint = 0x8CFC;
    pub const COLOR_ATTACHMENT29: c_uint = 0x8CFD;
    pub const COLOR_ATTACHMENT3: c_uint = 0x8CE3;
    pub const COLOR_ATTACHMENT30: c_uint = 0x8CFE;
    pub const COLOR_ATTACHMENT31: c_uint = 0x8CFF;
    pub const COLOR_ATTACHMENT4: c_uint = 0x8CE4;
    pub const COLOR_ATTACHMENT5: c_uint = 0x8CE5;
    pub const COLOR_ATTACHMENT6: c_uint = 0x8CE6;
    pub const COLOR_ATTACHMENT7: c_uint = 0x8CE7;
    pub const COLOR_ATTACHMENT8: c_uint = 0x8CE8;
    pub const COLOR_ATTACHMENT9: c_uint = 0x8CE9;
    pub const COLOR_BUFFER_BIT: c_uint = 0x00004000;
    pub const COLOR_CLEAR_VALUE: c_uint = 0x0C22;
    pub const COLOR_LOGIC_OP: c_uint = 0x0BF2;
    pub const COLOR_WRITEMASK: c_uint = 0x0C23;
    pub const COMPARE_REF_TO_TEXTURE: c_uint = 0x884E;
    pub const COMPILE_STATUS: c_uint = 0x8B81;
    pub const COMPRESSED_RED: c_uint = 0x8225;
    pub const COMPRESSED_RED_RGTC1: c_uint = 0x8DBB;
    pub const COMPRESSED_RG: c_uint = 0x8226;
    pub const COMPRESSED_RGB: c_uint = 0x84ED;
    pub const COMPRESSED_RGBA: c_uint = 0x84EE;
    pub const COMPRESSED_RG_RGTC2: c_uint = 0x8DBD;
    pub const COMPRESSED_SIGNED_RED_RGTC1: c_uint = 0x8DBC;
    pub const COMPRESSED_SIGNED_RG_RGTC2: c_uint = 0x8DBE;
    pub const COMPRESSED_SRGB: c_uint = 0x8C48;
    pub const COMPRESSED_SRGB_ALPHA: c_uint = 0x8C49;
    pub const COMPRESSED_TEXTURE_FORMATS: c_uint = 0x86A3;
    pub const CONDITION_SATISFIED: c_uint = 0x911C;
    pub const CONSTANT_ALPHA: c_uint = 0x8003;
    pub const CONSTANT_COLOR: c_uint = 0x8001;
    pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: c_uint = 0x00000002;
    pub const CONTEXT_CORE_PROFILE_BIT: c_uint = 0x00000001;
    pub const CONTEXT_FLAGS: c_uint = 0x821E;
    pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: c_uint = 0x00000001;
    pub const CONTEXT_PROFILE_MASK: c_uint = 0x9126;
    pub const COPY: c_uint = 0x1503;
    pub const COPY_INVERTED: c_uint = 0x150C;
    pub const COPY_READ_BUFFER: c_uint = 0x8F36;
    pub const COPY_WRITE_BUFFER: c_uint = 0x8F37;
    pub const CULL_FACE: c_uint = 0x0B44;
    pub const CULL_FACE_MODE: c_uint = 0x0B45;
    pub const CURRENT_PROGRAM: c_uint = 0x8B8D;
    pub const CURRENT_QUERY: c_uint = 0x8865;
    pub const CURRENT_VERTEX_ATTRIB: c_uint = 0x8626;
    pub const CW: c_uint = 0x0900;
    pub const DECR: c_uint = 0x1E03;
    pub const DECR_WRAP: c_uint = 0x8508;
    pub const DELETE_STATUS: c_uint = 0x8B80;
    pub const DEPTH: c_uint = 0x1801;
    pub const DEPTH24_STENCIL8: c_uint = 0x88F0;
    pub const DEPTH32F_STENCIL8: c_uint = 0x8CAD;
    pub const DEPTH_ATTACHMENT: c_uint = 0x8D00;
    pub const DEPTH_BUFFER_BIT: c_uint = 0x00000100;
    pub const DEPTH_CLAMP: c_uint = 0x864F;
    pub const DEPTH_CLEAR_VALUE: c_uint = 0x0B73;
    pub const DEPTH_COMPONENT: c_uint = 0x1902;
    pub const DEPTH_COMPONENT16: c_uint = 0x81A5;
    pub const DEPTH_COMPONENT24: c_uint = 0x81A6;
    pub const DEPTH_COMPONENT32: c_uint = 0x81A7;
    pub const DEPTH_COMPONENT32F: c_uint = 0x8CAC;
    pub const DEPTH_FUNC: c_uint = 0x0B74;
    pub const DEPTH_RANGE: c_uint = 0x0B70;
    pub const DEPTH_STENCIL: c_uint = 0x84F9;
    pub const DEPTH_STENCIL_ATTACHMENT: c_uint = 0x821A;
    pub const DEPTH_TEST: c_uint = 0x0B71;
    pub const DEPTH_WRITEMASK: c_uint = 0x0B72;
    pub const DITHER: c_uint = 0x0BD0;
    pub const DONT_CARE: c_uint = 0x1100;
    pub const DOUBLE: c_uint = 0x140A;
    pub const DOUBLEBUFFER: c_uint = 0x0C32;
    pub const DRAW_BUFFER: c_uint = 0x0C01;
    pub const DRAW_BUFFER0: c_uint = 0x8825;
    pub const DRAW_BUFFER1: c_uint = 0x8826;
    pub const DRAW_BUFFER10: c_uint = 0x882F;
    pub const DRAW_BUFFER11: c_uint = 0x8830;
    pub const DRAW_BUFFER12: c_uint = 0x8831;
    pub const DRAW_BUFFER13: c_uint = 0x8832;
    pub const DRAW_BUFFER14: c_uint = 0x8833;
    pub const DRAW_BUFFER15: c_uint = 0x8834;
    pub const DRAW_BUFFER2: c_uint = 0x8827;
    pub const DRAW_BUFFER3: c_uint = 0x8828;
    pub const DRAW_BUFFER4: c_uint = 0x8829;
    pub const DRAW_BUFFER5: c_uint = 0x882A;
    pub const DRAW_BUFFER6: c_uint = 0x882B;
    pub const DRAW_BUFFER7: c_uint = 0x882C;
    pub const DRAW_BUFFER8: c_uint = 0x882D;
    pub const DRAW_BUFFER9: c_uint = 0x882E;
    pub const DRAW_FRAMEBUFFER: c_uint = 0x8CA9;
    pub const DRAW_FRAMEBUFFER_BINDING: c_uint = 0x8CA6;
    pub const DST_ALPHA: c_uint = 0x0304;
    pub const DST_COLOR: c_uint = 0x0306;
    pub const DYNAMIC_COPY: c_uint = 0x88EA;
    pub const DYNAMIC_DRAW: c_uint = 0x88E8;
    pub const DYNAMIC_READ: c_uint = 0x88E9;
    pub const ELEMENT_ARRAY_BUFFER: c_uint = 0x8893;
    pub const ELEMENT_ARRAY_BUFFER_BINDING: c_uint = 0x8895;
    pub const EQUAL: c_uint = 0x0202;
    pub const EQUIV: c_uint = 0x1509;
    pub const EXTENSIONS: c_uint = 0x1F03;
    pub const FALSE: c_uchar = 0;
    pub const FASTEST: c_uint = 0x1101;
    pub const FILL: c_uint = 0x1B02;
    pub const FIRST_VERTEX_CONVENTION: c_uint = 0x8E4D;
    pub const FIXED_ONLY: c_uint = 0x891D;
    pub const FLOAT: c_uint = 0x1406;
    pub const FLOAT_32_UNSIGNED_INT_24_8_REV: c_uint = 0x8DAD;
    pub const FLOAT_MAT2: c_uint = 0x8B5A;
    pub const FLOAT_MAT2x3: c_uint = 0x8B65;
    pub const FLOAT_MAT2x4: c_uint = 0x8B66;
    pub const FLOAT_MAT3: c_uint = 0x8B5B;
    pub const FLOAT_MAT3x2: c_uint = 0x8B67;
    pub const FLOAT_MAT3x4: c_uint = 0x8B68;
    pub const FLOAT_MAT4: c_uint = 0x8B5C;
    pub const FLOAT_MAT4x2: c_uint = 0x8B69;
    pub const FLOAT_MAT4x3: c_uint = 0x8B6A;
    pub const FLOAT_VEC2: c_uint = 0x8B50;
    pub const FLOAT_VEC3: c_uint = 0x8B51;
    pub const FLOAT_VEC4: c_uint = 0x8B52;
    pub const FRAGMENT_SHADER: c_uint = 0x8B30;
    pub const FRAGMENT_SHADER_DERIVATIVE_HINT: c_uint = 0x8B8B;
    pub const FRAMEBUFFER: c_uint = 0x8D40;
    pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: c_uint = 0x8215;
    pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: c_uint = 0x8214;
    pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: c_uint = 0x8210;
    pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: c_uint = 0x8211;
    pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: c_uint = 0x8216;
    pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: c_uint = 0x8213;
    pub const FRAMEBUFFER_ATTACHMENT_LAYERED: c_uint = 0x8DA7;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: c_uint = 0x8CD1;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: c_uint = 0x8CD0;
    pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: c_uint = 0x8212;
    pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: c_uint = 0x8217;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: c_uint = 0x8CD3;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: c_uint = 0x8CD4;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: c_uint = 0x8CD2;
    pub const FRAMEBUFFER_BINDING: c_uint = 0x8CA6;
    pub const FRAMEBUFFER_COMPLETE: c_uint = 0x8CD5;
    pub const FRAMEBUFFER_DEFAULT: c_uint = 0x8218;
    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: c_uint = 0x8CD6;
    pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: c_uint = 0x8CDB;
    pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: c_uint = 0x8DA8;
    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: c_uint = 0x8CD7;
    pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: c_uint = 0x8D56;
    pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: c_uint = 0x8CDC;
    pub const FRAMEBUFFER_SRGB: c_uint = 0x8DB9;
    pub const FRAMEBUFFER_UNDEFINED: c_uint = 0x8219;
    pub const FRAMEBUFFER_UNSUPPORTED: c_uint = 0x8CDD;
    pub const FRONT: c_uint = 0x0404;
    pub const FRONT_AND_BACK: c_uint = 0x0408;
    pub const FRONT_FACE: c_uint = 0x0B46;
    pub const FRONT_LEFT: c_uint = 0x0400;
    pub const FRONT_RIGHT: c_uint = 0x0401;
    pub const FUNC_ADD: c_uint = 0x8006;
    pub const FUNC_REVERSE_SUBTRACT: c_uint = 0x800B;
    pub const FUNC_SUBTRACT: c_uint = 0x800A;
    pub const GEOMETRY_INPUT_TYPE: c_uint = 0x8917;
    pub const GEOMETRY_OUTPUT_TYPE: c_uint = 0x8918;
    pub const GEOMETRY_SHADER: c_uint = 0x8DD9;
    pub const GEOMETRY_VERTICES_OUT: c_uint = 0x8916;
    pub const GEQUAL: c_uint = 0x0206;
    pub const GREATER: c_uint = 0x0204;
    pub const GREEN: c_uint = 0x1904;
    pub const GREEN_INTEGER: c_uint = 0x8D95;
    pub const HALF_FLOAT: c_uint = 0x140B;
    pub const INCR: c_uint = 0x1E02;
    pub const INCR_WRAP: c_uint = 0x8507;
    pub const INFO_LOG_LENGTH: c_uint = 0x8B84;
    pub const INT: c_uint = 0x1404;
    pub const INTERLEAVED_ATTRIBS: c_uint = 0x8C8C;
    pub const INT_2_10_10_10_REV: c_uint = 0x8D9F;
    pub const INT_SAMPLER_1D: c_uint = 0x8DC9;
    pub const INT_SAMPLER_1D_ARRAY: c_uint = 0x8DCE;
    pub const INT_SAMPLER_2D: c_uint = 0x8DCA;
    pub const INT_SAMPLER_2D_ARRAY: c_uint = 0x8DCF;
    pub const INT_SAMPLER_2D_MULTISAMPLE: c_uint = 0x9109;
    pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: c_uint = 0x910C;
    pub const INT_SAMPLER_2D_RECT: c_uint = 0x8DCD;
    pub const INT_SAMPLER_3D: c_uint = 0x8DCB;
    pub const INT_SAMPLER_BUFFER: c_uint = 0x8DD0;
    pub const INT_SAMPLER_CUBE: c_uint = 0x8DCC;
    pub const INT_VEC2: c_uint = 0x8B53;
    pub const INT_VEC3: c_uint = 0x8B54;
    pub const INT_VEC4: c_uint = 0x8B55;
    pub const INVALID_ENUM: c_uint = 0x0500;
    pub const INVALID_FRAMEBUFFER_OPERATION: c_uint = 0x0506;
    pub const INVALID_INDEX: c_uint = 0xFFFFFFFF;
    pub const INVALID_OPERATION: c_uint = 0x0502;
    pub const INVALID_VALUE: c_uint = 0x0501;
    pub const INVERT: c_uint = 0x150A;
    pub const KEEP: c_uint = 0x1E00;
    pub const LAST_VERTEX_CONVENTION: c_uint = 0x8E4E;
    pub const LEFT: c_uint = 0x0406;
    pub const LEQUAL: c_uint = 0x0203;
    pub const LESS: c_uint = 0x0201;
    pub const LINE: c_uint = 0x1B01;
    pub const LINEAR: c_uint = 0x2601;
    pub const LINEAR_MIPMAP_LINEAR: c_uint = 0x2703;
    pub const LINEAR_MIPMAP_NEAREST: c_uint = 0x2701;
    pub const LINES: c_uint = 0x0001;
    pub const LINES_ADJACENCY: c_uint = 0x000A;
    pub const LINE_LOOP: c_uint = 0x0002;
    pub const LINE_SMOOTH: c_uint = 0x0B20;
    pub const LINE_SMOOTH_HINT: c_uint = 0x0C52;
    pub const LINE_STRIP: c_uint = 0x0003;
    pub const LINE_STRIP_ADJACENCY: c_uint = 0x000B;
    pub const LINE_WIDTH: c_uint = 0x0B21;
    pub const LINE_WIDTH_GRANULARITY: c_uint = 0x0B23;
    pub const LINE_WIDTH_RANGE: c_uint = 0x0B22;
    pub const LINK_STATUS: c_uint = 0x8B82;
    pub const LOGIC_OP_MODE: c_uint = 0x0BF0;
    pub const LOWER_LEFT: c_uint = 0x8CA1;
    pub const MAJOR_VERSION: c_uint = 0x821B;
    pub const MAP_FLUSH_EXPLICIT_BIT: c_uint = 0x0010;
    pub const MAP_INVALIDATE_BUFFER_BIT: c_uint = 0x0008;
    pub const MAP_INVALIDATE_RANGE_BIT: c_uint = 0x0004;
    pub const MAP_READ_BIT: c_uint = 0x0001;
    pub const MAP_UNSYNCHRONIZED_BIT: c_uint = 0x0020;
    pub const MAP_WRITE_BIT: c_uint = 0x0002;
    pub const MAX: c_uint = 0x8008;
    pub const MAX_3D_TEXTURE_SIZE: c_uint = 0x8073;
    pub const MAX_ARRAY_TEXTURE_LAYERS: c_uint = 0x88FF;
    pub const MAX_CLIP_DISTANCES: c_uint = 0x0D32;
    pub const MAX_COLOR_ATTACHMENTS: c_uint = 0x8CDF;
    pub const MAX_COLOR_TEXTURE_SAMPLES: c_uint = 0x910E;
    pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: c_uint = 0x8A33;
    pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: c_uint = 0x8A32;
    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: c_uint = 0x8B4D;
    pub const MAX_COMBINED_UNIFORM_BLOCKS: c_uint = 0x8A2E;
    pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: c_uint = 0x8A31;
    pub const MAX_CUBE_MAP_TEXTURE_SIZE: c_uint = 0x851C;
    pub const MAX_DEPTH_TEXTURE_SAMPLES: c_uint = 0x910F;
    pub const MAX_DRAW_BUFFERS: c_uint = 0x8824;
    pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: c_uint = 0x88FC;
    pub const MAX_ELEMENTS_INDICES: c_uint = 0x80E9;
    pub const MAX_ELEMENTS_VERTICES: c_uint = 0x80E8;
    pub const MAX_FRAGMENT_INPUT_COMPONENTS: c_uint = 0x9125;
    pub const MAX_FRAGMENT_UNIFORM_BLOCKS: c_uint = 0x8A2D;
    pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: c_uint = 0x8B49;
    pub const MAX_GEOMETRY_INPUT_COMPONENTS: c_uint = 0x9123;
    pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: c_uint = 0x9124;
    pub const MAX_GEOMETRY_OUTPUT_VERTICES: c_uint = 0x8DE0;
    pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: c_uint = 0x8C29;
    pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: c_uint = 0x8DE1;
    pub const MAX_GEOMETRY_UNIFORM_BLOCKS: c_uint = 0x8A2C;
    pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: c_uint = 0x8DDF;
    pub const MAX_INTEGER_SAMPLES: c_uint = 0x9110;
    pub const MAX_PROGRAM_TEXEL_OFFSET: c_uint = 0x8905;
    pub const MAX_RECTANGLE_TEXTURE_SIZE: c_uint = 0x84F8;
    pub const MAX_RENDERBUFFER_SIZE: c_uint = 0x84E8;
    pub const MAX_SAMPLES: c_uint = 0x8D57;
    pub const MAX_SAMPLE_MASK_WORDS: c_uint = 0x8E59;
    pub const MAX_SERVER_WAIT_TIMEOUT: c_uint = 0x9111;
    pub const MAX_TEXTURE_BUFFER_SIZE: c_uint = 0x8C2B;
    pub const MAX_TEXTURE_IMAGE_UNITS: c_uint = 0x8872;
    pub const MAX_TEXTURE_LOD_BIAS: c_uint = 0x84FD;
    pub const MAX_TEXTURE_SIZE: c_uint = 0x0D33;
    pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: c_uint = 0x8C8A;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: c_uint = 0x8C8B;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: c_uint = 0x8C80;
    pub const MAX_UNIFORM_BLOCK_SIZE: c_uint = 0x8A30;
    pub const MAX_UNIFORM_BUFFER_BINDINGS: c_uint = 0x8A2F;
    pub const MAX_VARYING_COMPONENTS: c_uint = 0x8B4B;
    pub const MAX_VARYING_FLOATS: c_uint = 0x8B4B;
    pub const MAX_VERTEX_ATTRIBS: c_uint = 0x8869;
    pub const MAX_VERTEX_OUTPUT_COMPONENTS: c_uint = 0x9122;
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: c_uint = 0x8B4C;
    pub const MAX_VERTEX_UNIFORM_BLOCKS: c_uint = 0x8A2B;
    pub const MAX_VERTEX_UNIFORM_COMPONENTS: c_uint = 0x8B4A;
    pub const MAX_VIEWPORT_DIMS: c_uint = 0x0D3A;
    pub const MIN: c_uint = 0x8007;
    pub const MINOR_VERSION: c_uint = 0x821C;
    pub const MIN_PROGRAM_TEXEL_OFFSET: c_uint = 0x8904;
    pub const MIRRORED_REPEAT: c_uint = 0x8370;
    pub const MULTISAMPLE: c_uint = 0x809D;
    pub const NAND: c_uint = 0x150E;
    pub const NEAREST: c_uint = 0x2600;
    pub const NEAREST_MIPMAP_LINEAR: c_uint = 0x2702;
    pub const NEAREST_MIPMAP_NEAREST: c_uint = 0x2700;
    pub const NEVER: c_uint = 0x0200;
    pub const NICEST: c_uint = 0x1102;
    pub const NONE: c_uint = 0;
    pub const NOOP: c_uint = 0x1505;
    pub const NOR: c_uint = 0x1508;
    pub const NOTEQUAL: c_uint = 0x0205;
    pub const NO_ERROR: c_uint = 0;
    pub const NUM_COMPRESSED_TEXTURE_FORMATS: c_uint = 0x86A2;
    pub const NUM_EXTENSIONS: c_uint = 0x821D;
    pub const OBJECT_TYPE: c_uint = 0x9112;
    pub const ONE: c_uint = 1;
    pub const ONE_MINUS_CONSTANT_ALPHA: c_uint = 0x8004;
    pub const ONE_MINUS_CONSTANT_COLOR: c_uint = 0x8002;
    pub const ONE_MINUS_DST_ALPHA: c_uint = 0x0305;
    pub const ONE_MINUS_DST_COLOR: c_uint = 0x0307;
    pub const ONE_MINUS_SRC1_ALPHA: c_uint = 0x88FB;
    pub const ONE_MINUS_SRC1_COLOR: c_uint = 0x88FA;
    pub const ONE_MINUS_SRC_ALPHA: c_uint = 0x0303;
    pub const ONE_MINUS_SRC_COLOR: c_uint = 0x0301;
    pub const OR: c_uint = 0x1507;
    pub const OR_INVERTED: c_uint = 0x150D;
    pub const OR_REVERSE: c_uint = 0x150B;
    pub const OUT_OF_MEMORY: c_uint = 0x0505;
    pub const PACK_ALIGNMENT: c_uint = 0x0D05;
    pub const PACK_IMAGE_HEIGHT: c_uint = 0x806C;
    pub const PACK_LSB_FIRST: c_uint = 0x0D01;
    pub const PACK_ROW_LENGTH: c_uint = 0x0D02;
    pub const PACK_SKIP_IMAGES: c_uint = 0x806B;
    pub const PACK_SKIP_PIXELS: c_uint = 0x0D04;
    pub const PACK_SKIP_ROWS: c_uint = 0x0D03;
    pub const PACK_SWAP_BYTES: c_uint = 0x0D00;
    pub const PIXEL_PACK_BUFFER: c_uint = 0x88EB;
    pub const PIXEL_PACK_BUFFER_BINDING: c_uint = 0x88ED;
    pub const PIXEL_UNPACK_BUFFER: c_uint = 0x88EC;
    pub const PIXEL_UNPACK_BUFFER_BINDING: c_uint = 0x88EF;
    pub const POINT: c_uint = 0x1B00;
    pub const POINTS: c_uint = 0x0000;
    pub const POINT_FADE_THRESHOLD_SIZE: c_uint = 0x8128;
    pub const POINT_SIZE: c_uint = 0x0B11;
    pub const POINT_SIZE_GRANULARITY: c_uint = 0x0B13;
    pub const POINT_SIZE_RANGE: c_uint = 0x0B12;
    pub const POINT_SPRITE_COORD_ORIGIN: c_uint = 0x8CA0;
    pub const POLYGON_MODE: c_uint = 0x0B40;
    pub const POLYGON_OFFSET_FACTOR: c_uint = 0x8038;
    pub const POLYGON_OFFSET_FILL: c_uint = 0x8037;
    pub const POLYGON_OFFSET_LINE: c_uint = 0x2A02;
    pub const POLYGON_OFFSET_POINT: c_uint = 0x2A01;
    pub const POLYGON_OFFSET_UNITS: c_uint = 0x2A00;
    pub const POLYGON_SMOOTH: c_uint = 0x0B41;
    pub const POLYGON_SMOOTH_HINT: c_uint = 0x0C53;
    pub const PRIMITIVES_GENERATED: c_uint = 0x8C87;
    pub const PRIMITIVE_RESTART: c_uint = 0x8F9D;
    pub const PRIMITIVE_RESTART_INDEX: c_uint = 0x8F9E;
    pub const PROGRAM_POINT_SIZE: c_uint = 0x8642;
    pub const PROVOKING_VERTEX: c_uint = 0x8E4F;
    pub const PROXY_TEXTURE_1D: c_uint = 0x8063;
    pub const PROXY_TEXTURE_1D_ARRAY: c_uint = 0x8C19;
    pub const PROXY_TEXTURE_2D: c_uint = 0x8064;
    pub const PROXY_TEXTURE_2D_ARRAY: c_uint = 0x8C1B;
    pub const PROXY_TEXTURE_2D_MULTISAMPLE: c_uint = 0x9101;
    pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: c_uint = 0x9103;
    pub const PROXY_TEXTURE_3D: c_uint = 0x8070;
    pub const PROXY_TEXTURE_CUBE_MAP: c_uint = 0x851B;
    pub const PROXY_TEXTURE_RECTANGLE: c_uint = 0x84F7;
    pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: c_uint = 0x8E4C;
    pub const QUERY_BY_REGION_NO_WAIT: c_uint = 0x8E16;
    pub const QUERY_BY_REGION_WAIT: c_uint = 0x8E15;
    pub const QUERY_COUNTER_BITS: c_uint = 0x8864;
    pub const QUERY_NO_WAIT: c_uint = 0x8E14;
    pub const QUERY_RESULT: c_uint = 0x8866;
    pub const QUERY_RESULT_AVAILABLE: c_uint = 0x8867;
    pub const QUERY_WAIT: c_uint = 0x8E13;
    pub const R11F_G11F_B10F: c_uint = 0x8C3A;
    pub const R16: c_uint = 0x822A;
    pub const R16F: c_uint = 0x822D;
    pub const R16I: c_uint = 0x8233;
    pub const R16UI: c_uint = 0x8234;
    pub const R16_SNORM: c_uint = 0x8F98;
    pub const R32F: c_uint = 0x822E;
    pub const R32I: c_uint = 0x8235;
    pub const R32UI: c_uint = 0x8236;
    pub const R3_G3_B2: c_uint = 0x2A10;
    pub const R8: c_uint = 0x8229;
    pub const R8I: c_uint = 0x8231;
    pub const R8UI: c_uint = 0x8232;
    pub const R8_SNORM: c_uint = 0x8F94;
    pub const RASTERIZER_DISCARD: c_uint = 0x8C89;
    pub const READ_BUFFER: c_uint = 0x0C02;
    pub const READ_FRAMEBUFFER: c_uint = 0x8CA8;
    pub const READ_FRAMEBUFFER_BINDING: c_uint = 0x8CAA;
    pub const READ_ONLY: c_uint = 0x88B8;
    pub const READ_WRITE: c_uint = 0x88BA;
    pub const RED: c_uint = 0x1903;
    pub const RED_INTEGER: c_uint = 0x8D94;
    pub const RENDERBUFFER: c_uint = 0x8D41;
    pub const RENDERBUFFER_ALPHA_SIZE: c_uint = 0x8D53;
    pub const RENDERBUFFER_BINDING: c_uint = 0x8CA7;
    pub const RENDERBUFFER_BLUE_SIZE: c_uint = 0x8D52;
    pub const RENDERBUFFER_DEPTH_SIZE: c_uint = 0x8D54;
    pub const RENDERBUFFER_GREEN_SIZE: c_uint = 0x8D51;
    pub const RENDERBUFFER_HEIGHT: c_uint = 0x8D43;
    pub const RENDERBUFFER_INTERNAL_FORMAT: c_uint = 0x8D44;
    pub const RENDERBUFFER_RED_SIZE: c_uint = 0x8D50;
    pub const RENDERBUFFER_SAMPLES: c_uint = 0x8CAB;
    pub const RENDERBUFFER_STENCIL_SIZE: c_uint = 0x8D55;
    pub const RENDERBUFFER_WIDTH: c_uint = 0x8D42;
    pub const RENDERER: c_uint = 0x1F01;
    pub const REPEAT: c_uint = 0x2901;
    pub const REPLACE: c_uint = 0x1E01;
    pub const RG: c_uint = 0x8227;
    pub const RG16: c_uint = 0x822C;
    pub const RG16F: c_uint = 0x822F;
    pub const RG16I: c_uint = 0x8239;
    pub const RG16UI: c_uint = 0x823A;
    pub const RG16_SNORM: c_uint = 0x8F99;
    pub const RG32F: c_uint = 0x8230;
    pub const RG32I: c_uint = 0x823B;
    pub const RG32UI: c_uint = 0x823C;
    pub const RG8: c_uint = 0x822B;
    pub const RG8I: c_uint = 0x8237;
    pub const RG8UI: c_uint = 0x8238;
    pub const RG8_SNORM: c_uint = 0x8F95;
    pub const RGB: c_uint = 0x1907;
    pub const RGB10: c_uint = 0x8052;
    pub const RGB10_A2: c_uint = 0x8059;
    pub const RGB10_A2UI: c_uint = 0x906F;
    pub const RGB12: c_uint = 0x8053;
    pub const RGB16: c_uint = 0x8054;
    pub const RGB16F: c_uint = 0x881B;
    pub const RGB16I: c_uint = 0x8D89;
    pub const RGB16UI: c_uint = 0x8D77;
    pub const RGB16_SNORM: c_uint = 0x8F9A;
    pub const RGB32F: c_uint = 0x8815;
    pub const RGB32I: c_uint = 0x8D83;
    pub const RGB32UI: c_uint = 0x8D71;
    pub const RGB4: c_uint = 0x804F;
    pub const RGB5: c_uint = 0x8050;
    pub const RGB5_A1: c_uint = 0x8057;
    pub const RGB8: c_uint = 0x8051;
    pub const RGB8I: c_uint = 0x8D8F;
    pub const RGB8UI: c_uint = 0x8D7D;
    pub const RGB8_SNORM: c_uint = 0x8F96;
    pub const RGB9_E5: c_uint = 0x8C3D;
    pub const RGBA: c_uint = 0x1908;
    pub const RGBA12: c_uint = 0x805A;
    pub const RGBA16: c_uint = 0x805B;
    pub const RGBA16F: c_uint = 0x881A;
    pub const RGBA16I: c_uint = 0x8D88;
    pub const RGBA16UI: c_uint = 0x8D76;
    pub const RGBA16_SNORM: c_uint = 0x8F9B;
    pub const RGBA2: c_uint = 0x8055;
    pub const RGBA32F: c_uint = 0x8814;
    pub const RGBA32I: c_uint = 0x8D82;
    pub const RGBA32UI: c_uint = 0x8D70;
    pub const RGBA4: c_uint = 0x8056;
    pub const RGBA8: c_uint = 0x8058;
    pub const RGBA8I: c_uint = 0x8D8E;
    pub const RGBA8UI: c_uint = 0x8D7C;
    pub const RGBA8_SNORM: c_uint = 0x8F97;
    pub const RGBA_INTEGER: c_uint = 0x8D99;
    pub const RGB_INTEGER: c_uint = 0x8D98;
    pub const RG_INTEGER: c_uint = 0x8228;
    pub const RIGHT: c_uint = 0x0407;
    pub const SAMPLER_1D: c_uint = 0x8B5D;
    pub const SAMPLER_1D_ARRAY: c_uint = 0x8DC0;
    pub const SAMPLER_1D_ARRAY_SHADOW: c_uint = 0x8DC3;
    pub const SAMPLER_1D_SHADOW: c_uint = 0x8B61;
    pub const SAMPLER_2D: c_uint = 0x8B5E;
    pub const SAMPLER_2D_ARRAY: c_uint = 0x8DC1;
    pub const SAMPLER_2D_ARRAY_SHADOW: c_uint = 0x8DC4;
    pub const SAMPLER_2D_MULTISAMPLE: c_uint = 0x9108;
    pub const SAMPLER_2D_MULTISAMPLE_ARRAY: c_uint = 0x910B;
    pub const SAMPLER_2D_RECT: c_uint = 0x8B63;
    pub const SAMPLER_2D_RECT_SHADOW: c_uint = 0x8B64;
    pub const SAMPLER_2D_SHADOW: c_uint = 0x8B62;
    pub const SAMPLER_3D: c_uint = 0x8B5F;
    pub const SAMPLER_BINDING: c_uint = 0x8919;
    pub const SAMPLER_BUFFER: c_uint = 0x8DC2;
    pub const SAMPLER_CUBE: c_uint = 0x8B60;
    pub const SAMPLER_CUBE_SHADOW: c_uint = 0x8DC5;
    pub const SAMPLES: c_uint = 0x80A9;
    pub const SAMPLES_PASSED: c_uint = 0x8914;
    pub const SAMPLE_ALPHA_TO_COVERAGE: c_uint = 0x809E;
    pub const SAMPLE_ALPHA_TO_ONE: c_uint = 0x809F;
    pub const SAMPLE_BUFFERS: c_uint = 0x80A8;
    pub const SAMPLE_COVERAGE: c_uint = 0x80A0;
    pub const SAMPLE_COVERAGE_INVERT: c_uint = 0x80AB;
    pub const SAMPLE_COVERAGE_VALUE: c_uint = 0x80AA;
    pub const SAMPLE_MASK: c_uint = 0x8E51;
    pub const SAMPLE_MASK_VALUE: c_uint = 0x8E52;
    pub const SAMPLE_POSITION: c_uint = 0x8E50;
    pub const SCISSOR_BOX: c_uint = 0x0C10;
    pub const SCISSOR_TEST: c_uint = 0x0C11;
    pub const SEPARATE_ATTRIBS: c_uint = 0x8C8D;
    pub const SET: c_uint = 0x150F;
    pub const SHADER_SOURCE_LENGTH: c_uint = 0x8B88;
    pub const SHADER_TYPE: c_uint = 0x8B4F;
    pub const SHADING_LANGUAGE_VERSION: c_uint = 0x8B8C;
    pub const SHORT: c_uint = 0x1402;
    pub const SIGNALED: c_uint = 0x9119;
    pub const SIGNED_NORMALIZED: c_uint = 0x8F9C;
    pub const SMOOTH_LINE_WIDTH_GRANULARITY: c_uint = 0x0B23;
    pub const SMOOTH_LINE_WIDTH_RANGE: c_uint = 0x0B22;
    pub const SMOOTH_POINT_SIZE_GRANULARITY: c_uint = 0x0B13;
    pub const SMOOTH_POINT_SIZE_RANGE: c_uint = 0x0B12;
    pub const SRC1_ALPHA: c_uint = 0x8589;
    pub const SRC1_COLOR: c_uint = 0x88F9;
    pub const SRC_ALPHA: c_uint = 0x0302;
    pub const SRC_ALPHA_SATURATE: c_uint = 0x0308;
    pub const SRC_COLOR: c_uint = 0x0300;
    pub const SRGB: c_uint = 0x8C40;
    pub const SRGB8: c_uint = 0x8C41;
    pub const SRGB8_ALPHA8: c_uint = 0x8C43;
    pub const SRGB_ALPHA: c_uint = 0x8C42;
    pub const STATIC_COPY: c_uint = 0x88E6;
    pub const STATIC_DRAW: c_uint = 0x88E4;
    pub const STATIC_READ: c_uint = 0x88E5;
    pub const STENCIL: c_uint = 0x1802;
    pub const STENCIL_ATTACHMENT: c_uint = 0x8D20;
    pub const STENCIL_BACK_FAIL: c_uint = 0x8801;
    pub const STENCIL_BACK_FUNC: c_uint = 0x8800;
    pub const STENCIL_BACK_PASS_DEPTH_FAIL: c_uint = 0x8802;
    pub const STENCIL_BACK_PASS_DEPTH_PASS: c_uint = 0x8803;
    pub const STENCIL_BACK_REF: c_uint = 0x8CA3;
    pub const STENCIL_BACK_VALUE_MASK: c_uint = 0x8CA4;
    pub const STENCIL_BACK_WRITEMASK: c_uint = 0x8CA5;
    pub const STENCIL_BUFFER_BIT: c_uint = 0x00000400;
    pub const STENCIL_CLEAR_VALUE: c_uint = 0x0B91;
    pub const STENCIL_FAIL: c_uint = 0x0B94;
    pub const STENCIL_FUNC: c_uint = 0x0B92;
    pub const STENCIL_INDEX: c_uint = 0x1901;
    pub const STENCIL_INDEX1: c_uint = 0x8D46;
    pub const STENCIL_INDEX16: c_uint = 0x8D49;
    pub const STENCIL_INDEX4: c_uint = 0x8D47;
    pub const STENCIL_INDEX8: c_uint = 0x8D48;
    pub const STENCIL_PASS_DEPTH_FAIL: c_uint = 0x0B95;
    pub const STENCIL_PASS_DEPTH_PASS: c_uint = 0x0B96;
    pub const STENCIL_REF: c_uint = 0x0B97;
    pub const STENCIL_TEST: c_uint = 0x0B90;
    pub const STENCIL_VALUE_MASK: c_uint = 0x0B93;
    pub const STENCIL_WRITEMASK: c_uint = 0x0B98;
    pub const STEREO: c_uint = 0x0C33;
    pub const STREAM_COPY: c_uint = 0x88E2;
    pub const STREAM_DRAW: c_uint = 0x88E0;
    pub const STREAM_READ: c_uint = 0x88E1;
    pub const SUBPIXEL_BITS: c_uint = 0x0D50;
    pub const SYNC_CONDITION: c_uint = 0x9113;
    pub const SYNC_FENCE: c_uint = 0x9116;
    pub const SYNC_FLAGS: c_uint = 0x9115;
    pub const SYNC_FLUSH_COMMANDS_BIT: c_uint = 0x00000001;
    pub const SYNC_GPU_COMMANDS_COMPLETE: c_uint = 0x9117;
    pub const SYNC_STATUS: c_uint = 0x9114;
    pub const TEXTURE: c_uint = 0x1702;
    pub const TEXTURE0: c_uint = 0x84C0;
    pub const TEXTURE1: c_uint = 0x84C1;
    pub const TEXTURE10: c_uint = 0x84CA;
    pub const TEXTURE11: c_uint = 0x84CB;
    pub const TEXTURE12: c_uint = 0x84CC;
    pub const TEXTURE13: c_uint = 0x84CD;
    pub const TEXTURE14: c_uint = 0x84CE;
    pub const TEXTURE15: c_uint = 0x84CF;
    pub const TEXTURE16: c_uint = 0x84D0;
    pub const TEXTURE17: c_uint = 0x84D1;
    pub const TEXTURE18: c_uint = 0x84D2;
    pub const TEXTURE19: c_uint = 0x84D3;
    pub const TEXTURE2: c_uint = 0x84C2;
    pub const TEXTURE20: c_uint = 0x84D4;
    pub const TEXTURE21: c_uint = 0x84D5;
    pub const TEXTURE22: c_uint = 0x84D6;
    pub const TEXTURE23: c_uint = 0x84D7;
    pub const TEXTURE24: c_uint = 0x84D8;
    pub const TEXTURE25: c_uint = 0x84D9;
    pub const TEXTURE26: c_uint = 0x84DA;
    pub const TEXTURE27: c_uint = 0x84DB;
    pub const TEXTURE28: c_uint = 0x84DC;
    pub const TEXTURE29: c_uint = 0x84DD;
    pub const TEXTURE3: c_uint = 0x84C3;
    pub const TEXTURE30: c_uint = 0x84DE;
    pub const TEXTURE31: c_uint = 0x84DF;
    pub const TEXTURE4: c_uint = 0x84C4;
    pub const TEXTURE5: c_uint = 0x84C5;
    pub const TEXTURE6: c_uint = 0x84C6;
    pub const TEXTURE7: c_uint = 0x84C7;
    pub const TEXTURE8: c_uint = 0x84C8;
    pub const TEXTURE9: c_uint = 0x84C9;
    pub const TEXTURE_1D: c_uint = 0x0DE0;
    pub const TEXTURE_1D_ARRAY: c_uint = 0x8C18;
    pub const TEXTURE_2D: c_uint = 0x0DE1;
    pub const TEXTURE_2D_ARRAY: c_uint = 0x8C1A;
    pub const TEXTURE_2D_MULTISAMPLE: c_uint = 0x9100;
    pub const TEXTURE_2D_MULTISAMPLE_ARRAY: c_uint = 0x9102;
    pub const TEXTURE_3D: c_uint = 0x806F;
    pub const TEXTURE_ALPHA_SIZE: c_uint = 0x805F;
    pub const TEXTURE_ALPHA_TYPE: c_uint = 0x8C13;
    pub const TEXTURE_BASE_LEVEL: c_uint = 0x813C;
    pub const TEXTURE_BINDING_1D: c_uint = 0x8068;
    pub const TEXTURE_BINDING_1D_ARRAY: c_uint = 0x8C1C;
    pub const TEXTURE_BINDING_2D: c_uint = 0x8069;
    pub const TEXTURE_BINDING_2D_ARRAY: c_uint = 0x8C1D;
    pub const TEXTURE_BINDING_2D_MULTISAMPLE: c_uint = 0x9104;
    pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: c_uint = 0x9105;
    pub const TEXTURE_BINDING_3D: c_uint = 0x806A;
    pub const TEXTURE_BINDING_BUFFER: c_uint = 0x8C2C;
    pub const TEXTURE_BINDING_CUBE_MAP: c_uint = 0x8514;
    pub const TEXTURE_BINDING_RECTANGLE: c_uint = 0x84F6;
    pub const TEXTURE_BLUE_SIZE: c_uint = 0x805E;
    pub const TEXTURE_BLUE_TYPE: c_uint = 0x8C12;
    pub const TEXTURE_BORDER_COLOR: c_uint = 0x1004;
    pub const TEXTURE_BUFFER: c_uint = 0x8C2A;
    pub const TEXTURE_BUFFER_DATA_STORE_BINDING: c_uint = 0x8C2D;
    pub const TEXTURE_COMPARE_FUNC: c_uint = 0x884D;
    pub const TEXTURE_COMPARE_MODE: c_uint = 0x884C;
    pub const TEXTURE_COMPRESSED: c_uint = 0x86A1;
    pub const TEXTURE_COMPRESSED_IMAGE_SIZE: c_uint = 0x86A0;
    pub const TEXTURE_COMPRESSION_HINT: c_uint = 0x84EF;
    pub const TEXTURE_CUBE_MAP: c_uint = 0x8513;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X: c_uint = 0x8516;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: c_uint = 0x8518;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: c_uint = 0x851A;
    pub const TEXTURE_CUBE_MAP_POSITIVE_X: c_uint = 0x8515;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y: c_uint = 0x8517;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z: c_uint = 0x8519;
    pub const TEXTURE_CUBE_MAP_SEAMLESS: c_uint = 0x884F;
    pub const TEXTURE_DEPTH: c_uint = 0x8071;
    pub const TEXTURE_DEPTH_SIZE: c_uint = 0x884A;
    pub const TEXTURE_DEPTH_TYPE: c_uint = 0x8C16;
    pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: c_uint = 0x9107;
    pub const TEXTURE_GREEN_SIZE: c_uint = 0x805D;
    pub const TEXTURE_GREEN_TYPE: c_uint = 0x8C11;
    pub const TEXTURE_HEIGHT: c_uint = 0x1001;
    pub const TEXTURE_INTERNAL_FORMAT: c_uint = 0x1003;
    pub const TEXTURE_LOD_BIAS: c_uint = 0x8501;
    pub const TEXTURE_MAG_FILTER: c_uint = 0x2800;
    pub const TEXTURE_MAX_LEVEL: c_uint = 0x813D;
    pub const TEXTURE_MAX_LOD: c_uint = 0x813B;
    pub const TEXTURE_MIN_FILTER: c_uint = 0x2801;
    pub const TEXTURE_MIN_LOD: c_uint = 0x813A;
    pub const TEXTURE_RECTANGLE: c_uint = 0x84F5;
    pub const TEXTURE_RED_SIZE: c_uint = 0x805C;
    pub const TEXTURE_RED_TYPE: c_uint = 0x8C10;
    pub const TEXTURE_SAMPLES: c_uint = 0x9106;
    pub const TEXTURE_SHARED_SIZE: c_uint = 0x8C3F;
    pub const TEXTURE_STENCIL_SIZE: c_uint = 0x88F1;
    pub const TEXTURE_SWIZZLE_A: c_uint = 0x8E45;
    pub const TEXTURE_SWIZZLE_B: c_uint = 0x8E44;
    pub const TEXTURE_SWIZZLE_G: c_uint = 0x8E43;
    pub const TEXTURE_SWIZZLE_R: c_uint = 0x8E42;
    pub const TEXTURE_SWIZZLE_RGBA: c_uint = 0x8E46;
    pub const TEXTURE_WIDTH: c_uint = 0x1000;
    pub const TEXTURE_WRAP_R: c_uint = 0x8072;
    pub const TEXTURE_WRAP_S: c_uint = 0x2802;
    pub const TEXTURE_WRAP_T: c_uint = 0x2803;
    pub const TIMEOUT_EXPIRED: c_uint = 0x911B;
    pub const TIMEOUT_IGNORED: u64 = 0xFFFFFFFFFFFFFFFF;
    pub const TIMESTAMP: c_uint = 0x8E28;
    pub const TIME_ELAPSED: c_uint = 0x88BF;
    pub const TRANSFORM_FEEDBACK_BUFFER: c_uint = 0x8C8E;
    pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: c_uint = 0x8C8F;
    pub const TRANSFORM_FEEDBACK_BUFFER_MODE: c_uint = 0x8C7F;
    pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: c_uint = 0x8C85;
    pub const TRANSFORM_FEEDBACK_BUFFER_START: c_uint = 0x8C84;
    pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: c_uint = 0x8C88;
    pub const TRANSFORM_FEEDBACK_VARYINGS: c_uint = 0x8C83;
    pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: c_uint = 0x8C76;
    pub const TRIANGLES: c_uint = 0x0004;
    pub const TRIANGLES_ADJACENCY: c_uint = 0x000C;
    pub const TRIANGLE_FAN: c_uint = 0x0006;
    pub const TRIANGLE_STRIP: c_uint = 0x0005;
    pub const TRIANGLE_STRIP_ADJACENCY: c_uint = 0x000D;
    pub const TRUE: c_uchar = 1;
    pub const UNIFORM_ARRAY_STRIDE: c_uint = 0x8A3C;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: c_uint = 0x8A42;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: c_uint = 0x8A43;
    pub const UNIFORM_BLOCK_BINDING: c_uint = 0x8A3F;
    pub const UNIFORM_BLOCK_DATA_SIZE: c_uint = 0x8A40;
    pub const UNIFORM_BLOCK_INDEX: c_uint = 0x8A3A;
    pub const UNIFORM_BLOCK_NAME_LENGTH: c_uint = 0x8A41;
    pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: c_uint = 0x8A46;
    pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: c_uint = 0x8A45;
    pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: c_uint = 0x8A44;
    pub const UNIFORM_BUFFER: c_uint = 0x8A11;
    pub const UNIFORM_BUFFER_BINDING: c_uint = 0x8A28;
    pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: c_uint = 0x8A34;
    pub const UNIFORM_BUFFER_SIZE: c_uint = 0x8A2A;
    pub const UNIFORM_BUFFER_START: c_uint = 0x8A29;
    pub const UNIFORM_IS_ROW_MAJOR: c_uint = 0x8A3E;
    pub const UNIFORM_MATRIX_STRIDE: c_uint = 0x8A3D;
    pub const UNIFORM_NAME_LENGTH: c_uint = 0x8A39;
    pub const UNIFORM_OFFSET: c_uint = 0x8A3B;
    pub const UNIFORM_SIZE: c_uint = 0x8A38;
    pub const UNIFORM_TYPE: c_uint = 0x8A37;
    pub const UNPACK_ALIGNMENT: c_uint = 0x0CF5;
    pub const UNPACK_IMAGE_HEIGHT: c_uint = 0x806E;
    pub const UNPACK_LSB_FIRST: c_uint = 0x0CF1;
    pub const UNPACK_ROW_LENGTH: c_uint = 0x0CF2;
    pub const UNPACK_SKIP_IMAGES: c_uint = 0x806D;
    pub const UNPACK_SKIP_PIXELS: c_uint = 0x0CF4;
    pub const UNPACK_SKIP_ROWS: c_uint = 0x0CF3;
    pub const UNPACK_SWAP_BYTES: c_uint = 0x0CF0;
    pub const UNSIGNALED: c_uint = 0x9118;
    pub const UNSIGNED_BYTE: c_uint = 0x1401;
    pub const UNSIGNED_BYTE_2_3_3_REV: c_uint = 0x8362;
    pub const UNSIGNED_BYTE_3_3_2: c_uint = 0x8032;
    pub const UNSIGNED_INT: c_uint = 0x1405;
    pub const UNSIGNED_INT_10F_11F_11F_REV: c_uint = 0x8C3B;
    pub const UNSIGNED_INT_10_10_10_2: c_uint = 0x8036;
    pub const UNSIGNED_INT_24_8: c_uint = 0x84FA;
    pub const UNSIGNED_INT_2_10_10_10_REV: c_uint = 0x8368;
    pub const UNSIGNED_INT_5_9_9_9_REV: c_uint = 0x8C3E;
    pub const UNSIGNED_INT_8_8_8_8: c_uint = 0x8035;
    pub const UNSIGNED_INT_8_8_8_8_REV: c_uint = 0x8367;
    pub const UNSIGNED_INT_SAMPLER_1D: c_uint = 0x8DD1;
    pub const UNSIGNED_INT_SAMPLER_1D_ARRAY: c_uint = 0x8DD6;
    pub const UNSIGNED_INT_SAMPLER_2D: c_uint = 0x8DD2;
    pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: c_uint = 0x8DD7;
    pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: c_uint = 0x910A;
    pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: c_uint = 0x910D;
    pub const UNSIGNED_INT_SAMPLER_2D_RECT: c_uint = 0x8DD5;
    pub const UNSIGNED_INT_SAMPLER_3D: c_uint = 0x8DD3;
    pub const UNSIGNED_INT_SAMPLER_BUFFER: c_uint = 0x8DD8;
    pub const UNSIGNED_INT_SAMPLER_CUBE: c_uint = 0x8DD4;
    pub const UNSIGNED_INT_VEC2: c_uint = 0x8DC6;
    pub const UNSIGNED_INT_VEC3: c_uint = 0x8DC7;
    pub const UNSIGNED_INT_VEC4: c_uint = 0x8DC8;
    pub const UNSIGNED_NORMALIZED: c_uint = 0x8C17;
    pub const UNSIGNED_SHORT: c_uint = 0x1403;
    pub const UNSIGNED_SHORT_1_5_5_5_REV: c_uint = 0x8366;
    pub const UNSIGNED_SHORT_4_4_4_4: c_uint = 0x8033;
    pub const UNSIGNED_SHORT_4_4_4_4_REV: c_uint = 0x8365;
    pub const UNSIGNED_SHORT_5_5_5_1: c_uint = 0x8034;
    pub const UNSIGNED_SHORT_5_6_5: c_uint = 0x8363;
    pub const UNSIGNED_SHORT_5_6_5_REV: c_uint = 0x8364;
    pub const UPPER_LEFT: c_uint = 0x8CA2;
    pub const VALIDATE_STATUS: c_uint = 0x8B83;
    pub const VENDOR: c_uint = 0x1F00;
    pub const VERSION: c_uint = 0x1F02;
    pub const VERTEX_ARRAY_BINDING: c_uint = 0x85B5;
    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: c_uint = 0x889F;
    pub const VERTEX_ATTRIB_ARRAY_DIVISOR: c_uint = 0x88FE;
    pub const VERTEX_ATTRIB_ARRAY_ENABLED: c_uint = 0x8622;
    pub const VERTEX_ATTRIB_ARRAY_INTEGER: c_uint = 0x88FD;
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: c_uint = 0x886A;
    pub const VERTEX_ATTRIB_ARRAY_POINTER: c_uint = 0x8645;
    pub const VERTEX_ATTRIB_ARRAY_SIZE: c_uint = 0x8623;
    pub const VERTEX_ATTRIB_ARRAY_STRIDE: c_uint = 0x8624;
    pub const VERTEX_ATTRIB_ARRAY_TYPE: c_uint = 0x8625;
    pub const VERTEX_PROGRAM_POINT_SIZE: c_uint = 0x8642;
    pub const VERTEX_SHADER: c_uint = 0x8B31;
    pub const VIEWPORT: c_uint = 0x0BA2;
    pub const WAIT_FAILED: c_uint = 0x911D;
    pub const WRITE_ONLY: c_uint = 0x88B9;
    pub const XOR: c_uint = 0x1506;
    pub const ZERO: c_uint = 0;
}

pub mod functions {
    #![allow(non_snake_case, unused_variables, dead_code, unused_imports)]

    use super::types::*;
    use super::*;
    use std::mem::transmute;
    use std::os::raw::*;

    macro_rules! func {
        ($fun:ident, $ret:ty, $($name:ident: $typ:ty),*) => {
            #[inline] pub unsafe fn $fun($($name: $typ),*) -> $ret {
                transmute::<_, extern "system" fn($($typ),*) -> $ret>(storage::$fun.ptr)($($name),*)
            }
        }
    }

    func!(ActiveTexture, (), texture: GLenum);
    func!(AttachShader, (), program: GLuint, shader: GLuint);
    func!(BeginConditionalRender, (), id: GLuint, mode: GLenum);
    func!(BeginQuery, (), target: GLenum, id: GLuint);
    func!(BeginTransformFeedback, (), primitiveMode: GLenum);
    func!(
        BindAttribLocation,
        (),
        program: GLuint,
        index: GLuint,
        name: *const GLchar
    );
    func!(BindBuffer, (), target: GLenum, buffer: GLuint);
    func!(
        BindBufferBase,
        (),
        target: GLenum,
        index: GLuint,
        buffer: GLuint
    );
    func!(
        BindBufferRange,
        (),
        target: GLenum,
        index: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr
    );
    func!(
        BindFragDataLocation,
        (),
        program: GLuint,
        color: GLuint,
        name: *const GLchar
    );
    func!(
        BindFragDataLocationIndexed,
        (),
        program: GLuint,
        colorNumber: GLuint,
        index: GLuint,
        name: *const GLchar
    );
    func!(BindFramebuffer, (), target: GLenum, framebuffer: GLuint);
    func!(BindRenderbuffer, (), target: GLenum, renderbuffer: GLuint);
    func!(BindSampler, (), unit: GLuint, sampler: GLuint);
    func!(BindTexture, (), target: GLenum, texture: GLuint);
    func!(BindVertexArray, (), array: GLuint);
    func!(
        BlendColor,
        (),
        red: GLfloat,
        green: GLfloat,
        blue: GLfloat,
        alpha: GLfloat
    );
    func!(BlendEquation, (), mode: GLenum);
    func!(
        BlendEquationSeparate,
        (),
        modeRGB: GLenum,
        modeAlpha: GLenum
    );
    func!(BlendFunc, (), sfactor: GLenum, dfactor: GLenum);
    func!(
        BlendFuncSeparate,
        (),
        sfactorRGB: GLenum,
        dfactorRGB: GLenum,
        sfactorAlpha: GLenum,
        dfactorAlpha: GLenum
    );
    func!(
        BlitFramebuffer,
        (),
        srcX0: GLint,
        srcY0: GLint,
        srcX1: GLint,
        srcY1: GLint,
        dstX0: GLint,
        dstY0: GLint,
        dstX1: GLint,
        dstY1: GLint,
        mask: GLbitfield,
        filter: GLenum
    );
    func!(
        BufferData,
        (),
        target: GLenum,
        size: GLsizeiptr,
        data: *const c_void,
        usage: GLenum
    );
    func!(
        BufferSubData,
        (),
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const c_void
    );
    func!(CheckFramebufferStatus, GLenum, target: GLenum);
    func!(ClampColor, (), target: GLenum, clamp: GLenum);
    func!(Clear, (), mask: GLbitfield);
    func!(
        ClearBufferfi,
        (),
        buffer: GLenum,
        drawbuffer: GLint,
        depth: GLfloat,
        stencil: GLint
    );
    func!(
        ClearBufferfv,
        (),
        buffer: GLenum,
        drawbuffer: GLint,
        value: *const GLfloat
    );
    func!(
        ClearBufferiv,
        (),
        buffer: GLenum,
        drawbuffer: GLint,
        value: *const GLint
    );
    func!(
        ClearBufferuiv,
        (),
        buffer: GLenum,
        drawbuffer: GLint,
        value: *const GLuint
    );
    func!(
        ClearColor,
        (),
        red: GLfloat,
        green: GLfloat,
        blue: GLfloat,
        alpha: GLfloat
    );
    func!(ClearDepth, (), depth: GLdouble);
    func!(ClearStencil, (), s: GLint);
    func!(
        ClientWaitSync,
        GLenum,
        sync: GLsync,
        flags: GLbitfield,
        timeout: GLuint64
    );
    func!(
        ColorMask,
        (),
        red: GLboolean,
        green: GLboolean,
        blue: GLboolean,
        alpha: GLboolean
    );
    func!(
        ColorMaski,
        (),
        index: GLuint,
        r: GLboolean,
        g: GLboolean,
        b: GLboolean,
        a: GLboolean
    );
    func!(CompileShader, (), shader: GLuint);
    func!(
        CompressedTexImage1D,
        (),
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const c_void
    );
    func!(
        CompressedTexImage2D,
        (),
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const c_void
    );
    func!(
        CompressedTexImage3D,
        (),
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const c_void
    );
    func!(
        CompressedTexSubImage1D,
        (),
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const c_void
    );
    func!(
        CompressedTexSubImage2D,
        (),
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const c_void
    );
    func!(
        CompressedTexSubImage3D,
        (),
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const c_void
    );
    func!(
        CopyBufferSubData,
        (),
        readTarget: GLenum,
        writeTarget: GLenum,
        readOffset: GLintptr,
        writeOffset: GLintptr,
        size: GLsizeiptr
    );
    func!(
        CopyTexImage1D,
        (),
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        border: GLint
    );
    func!(
        CopyTexImage2D,
        (),
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint
    );
    func!(
        CopyTexSubImage1D,
        (),
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei
    );
    func!(
        CopyTexSubImage2D,
        (),
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei
    );
    func!(
        CopyTexSubImage3D,
        (),
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei
    );
    func!(CreateProgram, GLuint,);
    func!(CreateShader, GLuint, type_: GLenum);
    func!(CullFace, (), mode: GLenum);
    func!(DeleteBuffers, (), n: GLsizei, buffers: *const GLuint);
    func!(
        DeleteFramebuffers,
        (),
        n: GLsizei,
        framebuffers: *const GLuint
    );
    func!(DeleteProgram, (), program: GLuint);
    func!(DeleteQueries, (), n: GLsizei, ids: *const GLuint);
    func!(
        DeleteRenderbuffers,
        (),
        n: GLsizei,
        renderbuffers: *const GLuint
    );
    func!(DeleteSamplers, (), count: GLsizei, samplers: *const GLuint);
    func!(DeleteShader, (), shader: GLuint);
    func!(DeleteSync, (), sync: GLsync);
    func!(DeleteTextures, (), n: GLsizei, textures: *const GLuint);
    func!(DeleteVertexArrays, (), n: GLsizei, arrays: *const GLuint);
    func!(DepthFunc, (), func: GLenum);
    func!(DepthMask, (), flag: GLboolean);
    func!(DepthRange, (), n: GLdouble, f: GLdouble);
    func!(DetachShader, (), program: GLuint, shader: GLuint);
    func!(Disable, (), cap: GLenum);
    func!(DisableVertexAttribArray, (), index: GLuint);
    func!(Disablei, (), target: GLenum, index: GLuint);
    func!(DrawArrays, (), mode: GLenum, first: GLint, count: GLsizei);
    func!(
        DrawArraysInstanced,
        (),
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instancecount: GLsizei
    );
    func!(DrawBuffer, (), buf: GLenum);
    func!(DrawBuffers, (), n: GLsizei, bufs: *const GLenum);
    func!(
        DrawElements,
        (),
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void
    );
    func!(
        DrawElementsBaseVertex,
        (),
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
        basevertex: GLint
    );
    func!(
        DrawElementsInstanced,
        (),
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
        instancecount: GLsizei
    );
    func!(
        DrawElementsInstancedBaseVertex,
        (),
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
        instancecount: GLsizei,
        basevertex: GLint
    );
    func!(
        DrawRangeElements,
        (),
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void
    );
    func!(
        DrawRangeElementsBaseVertex,
        (),
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
        basevertex: GLint
    );
    func!(Enable, (), cap: GLenum);
    func!(EnableVertexAttribArray, (), index: GLuint);
    func!(Enablei, (), target: GLenum, index: GLuint);
    func!(EndConditionalRender, (),);
    func!(EndQuery, (), target: GLenum);
    func!(EndTransformFeedback, (),);
    func!(FenceSync, GLsync, condition: GLenum, flags: GLbitfield);
    func!(Finish, (),);
    func!(Flush, (),);
    func!(
        FlushMappedBufferRange,
        (),
        target: GLenum,
        offset: GLintptr,
        length: GLsizeiptr
    );
    func!(
        FramebufferRenderbuffer,
        (),
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint
    );
    func!(
        FramebufferTexture,
        (),
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint
    );
    func!(
        FramebufferTexture1D,
        (),
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint
    );
    func!(
        FramebufferTexture2D,
        (),
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint
    );
    func!(
        FramebufferTexture3D,
        (),
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
        zoffset: GLint
    );
    func!(
        FramebufferTextureLayer,
        (),
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
        layer: GLint
    );
    func!(FrontFace, (), mode: GLenum);
    func!(GenBuffers, (), n: GLsizei, buffers: *mut GLuint);
    func!(GenFramebuffers, (), n: GLsizei, framebuffers: *mut GLuint);
    func!(GenQueries, (), n: GLsizei, ids: *mut GLuint);
    func!(GenRenderbuffers, (), n: GLsizei, renderbuffers: *mut GLuint);
    func!(GenSamplers, (), count: GLsizei, samplers: *mut GLuint);
    func!(GenTextures, (), n: GLsizei, textures: *mut GLuint);
    func!(GenVertexArrays, (), n: GLsizei, arrays: *mut GLuint);
    func!(GenerateMipmap, (), target: GLenum);
    func!(
        GetActiveAttrib,
        (),
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        type_: *mut GLenum,
        name: *mut GLchar
    );
    func!(
        GetActiveUniform,
        (),
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        type_: *mut GLenum,
        name: *mut GLchar
    );
    func!(
        GetActiveUniformBlockName,
        (),
        program: GLuint,
        uniformBlockIndex: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        uniformBlockName: *mut GLchar
    );
    func!(
        GetActiveUniformBlockiv,
        (),
        program: GLuint,
        uniformBlockIndex: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetActiveUniformName,
        (),
        program: GLuint,
        uniformIndex: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        uniformName: *mut GLchar
    );
    func!(
        GetActiveUniformsiv,
        (),
        program: GLuint,
        uniformCount: GLsizei,
        uniformIndices: *const GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetAttachedShaders,
        (),
        program: GLuint,
        maxCount: GLsizei,
        count: *mut GLsizei,
        shaders: *mut GLuint
    );
    func!(
        GetAttribLocation,
        GLint,
        program: GLuint,
        name: *const GLchar
    );
    func!(
        GetBooleani_v,
        (),
        target: GLenum,
        index: GLuint,
        data: *mut GLboolean
    );
    func!(GetBooleanv, (), pname: GLenum, data: *mut GLboolean);
    func!(
        GetBufferParameteri64v,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut GLint64
    );
    func!(
        GetBufferParameteriv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetBufferPointerv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut *mut c_void
    );
    func!(
        GetBufferSubData,
        (),
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *mut c_void
    );
    func!(
        GetCompressedTexImage,
        (),
        target: GLenum,
        level: GLint,
        img: *mut c_void
    );
    func!(GetDoublev, (), pname: GLenum, data: *mut GLdouble);
    func!(GetError, GLenum,);
    func!(GetFloatv, (), pname: GLenum, data: *mut GLfloat);
    func!(
        GetFragDataIndex,
        GLint,
        program: GLuint,
        name: *const GLchar
    );
    func!(
        GetFragDataLocation,
        GLint,
        program: GLuint,
        name: *const GLchar
    );
    func!(
        GetFramebufferAttachmentParameteriv,
        (),
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetInteger64i_v,
        (),
        target: GLenum,
        index: GLuint,
        data: *mut GLint64
    );
    func!(GetInteger64v, (), pname: GLenum, data: *mut GLint64);
    func!(
        GetIntegeri_v,
        (),
        target: GLenum,
        index: GLuint,
        data: *mut GLint
    );
    func!(GetIntegerv, (), pname: GLenum, data: *mut GLint);
    func!(
        GetMultisamplefv,
        (),
        pname: GLenum,
        index: GLuint,
        val: *mut GLfloat
    );
    func!(
        GetProgramInfoLog,
        (),
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar
    );
    func!(
        GetProgramiv,
        (),
        program: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetQueryObjecti64v,
        (),
        id: GLuint,
        pname: GLenum,
        params: *mut GLint64
    );
    func!(
        GetQueryObjectiv,
        (),
        id: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetQueryObjectui64v,
        (),
        id: GLuint,
        pname: GLenum,
        params: *mut GLuint64
    );
    func!(
        GetQueryObjectuiv,
        (),
        id: GLuint,
        pname: GLenum,
        params: *mut GLuint
    );
    func!(
        GetQueryiv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetRenderbufferParameteriv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetSamplerParameterIiv,
        (),
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetSamplerParameterIuiv,
        (),
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLuint
    );
    func!(
        GetSamplerParameterfv,
        (),
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLfloat
    );
    func!(
        GetSamplerParameteriv,
        (),
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetShaderInfoLog,
        (),
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar
    );
    func!(
        GetShaderSource,
        (),
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        source: *mut GLchar
    );
    func!(
        GetShaderiv,
        (),
        shader: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(GetString, *const GLubyte, name: GLenum);
    func!(GetStringi, *const GLubyte, name: GLenum, index: GLuint);
    func!(
        GetSynciv,
        (),
        sync: GLsync,
        pname: GLenum,
        count: GLsizei,
        length: *mut GLsizei,
        values: *mut GLint
    );
    func!(
        GetTexImage,
        (),
        target: GLenum,
        level: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *mut c_void
    );
    func!(
        GetTexLevelParameterfv,
        (),
        target: GLenum,
        level: GLint,
        pname: GLenum,
        params: *mut GLfloat
    );
    func!(
        GetTexLevelParameteriv,
        (),
        target: GLenum,
        level: GLint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetTexParameterIiv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetTexParameterIuiv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut GLuint
    );
    func!(
        GetTexParameterfv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut GLfloat
    );
    func!(
        GetTexParameteriv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetTransformFeedbackVarying,
        (),
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLsizei,
        type_: *mut GLenum,
        name: *mut GLchar
    );
    func!(
        GetUniformBlockIndex,
        GLuint,
        program: GLuint,
        uniformBlockName: *const GLchar
    );
    func!(
        GetUniformIndices,
        (),
        program: GLuint,
        uniformCount: GLsizei,
        uniformNames: *const *const GLchar,
        uniformIndices: *mut GLuint
    );
    func!(
        GetUniformLocation,
        GLint,
        program: GLuint,
        name: *const GLchar
    );
    func!(
        GetUniformfv,
        (),
        program: GLuint,
        location: GLint,
        params: *mut GLfloat
    );
    func!(
        GetUniformiv,
        (),
        program: GLuint,
        location: GLint,
        params: *mut GLint
    );
    func!(
        GetUniformuiv,
        (),
        program: GLuint,
        location: GLint,
        params: *mut GLuint
    );
    func!(
        GetVertexAttribIiv,
        (),
        index: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetVertexAttribIuiv,
        (),
        index: GLuint,
        pname: GLenum,
        params: *mut GLuint
    );
    func!(
        GetVertexAttribPointerv,
        (),
        index: GLuint,
        pname: GLenum,
        pointer: *mut *mut c_void
    );
    func!(
        GetVertexAttribdv,
        (),
        index: GLuint,
        pname: GLenum,
        params: *mut GLdouble
    );
    func!(
        GetVertexAttribfv,
        (),
        index: GLuint,
        pname: GLenum,
        params: *mut GLfloat
    );
    func!(
        GetVertexAttribiv,
        (),
        index: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(Hint, (), target: GLenum, mode: GLenum);
    func!(IsBuffer, GLboolean, buffer: GLuint);
    func!(IsEnabled, GLboolean, cap: GLenum);
    func!(IsEnabledi, GLboolean, target: GLenum, index: GLuint);
    func!(IsFramebuffer, GLboolean, framebuffer: GLuint);
    func!(IsProgram, GLboolean, program: GLuint);
    func!(IsQuery, GLboolean, id: GLuint);
    func!(IsRenderbuffer, GLboolean, renderbuffer: GLuint);
    func!(IsSampler, GLboolean, sampler: GLuint);
    func!(IsShader, GLboolean, shader: GLuint);
    func!(IsSync, GLboolean, sync: GLsync);
    func!(IsTexture, GLboolean, texture: GLuint);
    func!(IsVertexArray, GLboolean, array: GLuint);
    func!(LineWidth, (), width: GLfloat);
    func!(LinkProgram, (), program: GLuint);
    func!(LogicOp, (), opcode: GLenum);
    func!(MapBuffer, *mut c_void, target: GLenum, access: GLenum);
    func!(
        MapBufferRange,
        *mut c_void,
        target: GLenum,
        offset: GLintptr,
        length: GLsizeiptr,
        access: GLbitfield
    );
    func!(
        MultiDrawArrays,
        (),
        mode: GLenum,
        first: *const GLint,
        count: *const GLsizei,
        drawcount: GLsizei
    );
    func!(
        MultiDrawElements,
        (),
        mode: GLenum,
        count: *const GLsizei,
        type_: GLenum,
        indices: *const *const c_void,
        drawcount: GLsizei
    );
    func!(
        MultiDrawElementsBaseVertex,
        (),
        mode: GLenum,
        count: *const GLsizei,
        type_: GLenum,
        indices: *const *const c_void,
        drawcount: GLsizei,
        basevertex: *const GLint
    );
    func!(PixelStoref, (), pname: GLenum, param: GLfloat);
    func!(PixelStorei, (), pname: GLenum, param: GLint);
    func!(PointParameterf, (), pname: GLenum, param: GLfloat);
    func!(PointParameterfv, (), pname: GLenum, params: *const GLfloat);
    func!(PointParameteri, (), pname: GLenum, param: GLint);
    func!(PointParameteriv, (), pname: GLenum, params: *const GLint);
    func!(PointSize, (), size: GLfloat);
    func!(PolygonMode, (), face: GLenum, mode: GLenum);
    func!(PolygonOffset, (), factor: GLfloat, units: GLfloat);
    func!(PrimitiveRestartIndex, (), index: GLuint);
    func!(ProvokingVertex, (), mode: GLenum);
    func!(QueryCounter, (), id: GLuint, target: GLenum);
    func!(ReadBuffer, (), src: GLenum);
    func!(
        ReadPixels,
        (),
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *mut c_void
    );
    func!(
        RenderbufferStorage,
        (),
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei
    );
    func!(
        RenderbufferStorageMultisample,
        (),
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei
    );
    func!(SampleCoverage, (), value: GLfloat, invert: GLboolean);
    func!(SampleMaski, (), maskNumber: GLuint, mask: GLbitfield);
    func!(
        SamplerParameterIiv,
        (),
        sampler: GLuint,
        pname: GLenum,
        param: *const GLint
    );
    func!(
        SamplerParameterIuiv,
        (),
        sampler: GLuint,
        pname: GLenum,
        param: *const GLuint
    );
    func!(
        SamplerParameterf,
        (),
        sampler: GLuint,
        pname: GLenum,
        param: GLfloat
    );
    func!(
        SamplerParameterfv,
        (),
        sampler: GLuint,
        pname: GLenum,
        param: *const GLfloat
    );
    func!(
        SamplerParameteri,
        (),
        sampler: GLuint,
        pname: GLenum,
        param: GLint
    );
    func!(
        SamplerParameteriv,
        (),
        sampler: GLuint,
        pname: GLenum,
        param: *const GLint
    );
    func!(
        Scissor,
        (),
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei
    );
    func!(
        ShaderSource,
        (),
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint
    );
    func!(StencilFunc, (), func: GLenum, ref_: GLint, mask: GLuint);
    func!(
        StencilFuncSeparate,
        (),
        face: GLenum,
        func: GLenum,
        ref_: GLint,
        mask: GLuint
    );
    func!(StencilMask, (), mask: GLuint);
    func!(StencilMaskSeparate, (), face: GLenum, mask: GLuint);
    func!(StencilOp, (), fail: GLenum, zfail: GLenum, zpass: GLenum);
    func!(
        StencilOpSeparate,
        (),
        face: GLenum,
        sfail: GLenum,
        dpfail: GLenum,
        dppass: GLenum
    );
    func!(
        TexBuffer,
        (),
        target: GLenum,
        internalformat: GLenum,
        buffer: GLuint
    );
    func!(
        TexImage1D,
        (),
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void
    );
    func!(
        TexImage2D,
        (),
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void
    );
    func!(
        TexImage2DMultisample,
        (),
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        fixedsamplelocations: GLboolean
    );
    func!(
        TexImage3D,
        (),
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void
    );
    func!(
        TexImage3DMultisample,
        (),
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        fixedsamplelocations: GLboolean
    );
    func!(
        TexParameterIiv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *const GLint
    );
    func!(
        TexParameterIuiv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *const GLuint
    );
    func!(
        TexParameterf,
        (),
        target: GLenum,
        pname: GLenum,
        param: GLfloat
    );
    func!(
        TexParameterfv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *const GLfloat
    );
    func!(
        TexParameteri,
        (),
        target: GLenum,
        pname: GLenum,
        param: GLint
    );
    func!(
        TexParameteriv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *const GLint
    );
    func!(
        TexSubImage1D,
        (),
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void
    );
    func!(
        TexSubImage2D,
        (),
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void
    );
    func!(
        TexSubImage3D,
        (),
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void
    );
    func!(
        TransformFeedbackVaryings,
        (),
        program: GLuint,
        count: GLsizei,
        varyings: *const *const GLchar,
        bufferMode: GLenum
    );
    func!(Uniform1f, (), location: GLint, v0: GLfloat);
    func!(
        Uniform1fv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLfloat
    );
    func!(Uniform1i, (), location: GLint, v0: GLint);
    func!(
        Uniform1iv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLint
    );
    func!(Uniform1ui, (), location: GLint, v0: GLuint);
    func!(
        Uniform1uiv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLuint
    );
    func!(Uniform2f, (), location: GLint, v0: GLfloat, v1: GLfloat);
    func!(
        Uniform2fv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLfloat
    );
    func!(Uniform2i, (), location: GLint, v0: GLint, v1: GLint);
    func!(
        Uniform2iv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLint
    );
    func!(Uniform2ui, (), location: GLint, v0: GLuint, v1: GLuint);
    func!(
        Uniform2uiv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLuint
    );
    func!(
        Uniform3f,
        (),
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat
    );
    func!(
        Uniform3fv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLfloat
    );
    func!(
        Uniform3i,
        (),
        location: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint
    );
    func!(
        Uniform3iv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLint
    );
    func!(
        Uniform3ui,
        (),
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint
    );
    func!(
        Uniform3uiv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLuint
    );
    func!(
        Uniform4f,
        (),
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
        v3: GLfloat
    );
    func!(
        Uniform4fv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLfloat
    );
    func!(
        Uniform4i,
        (),
        location: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint,
        v3: GLint
    );
    func!(
        Uniform4iv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLint
    );
    func!(
        Uniform4ui,
        (),
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint
    );
    func!(
        Uniform4uiv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLuint
    );
    func!(
        UniformBlockBinding,
        (),
        program: GLuint,
        uniformBlockIndex: GLuint,
        uniformBlockBinding: GLuint
    );
    func!(
        UniformMatrix2fv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        UniformMatrix2x3fv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        UniformMatrix2x4fv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        UniformMatrix3fv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        UniformMatrix3x2fv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        UniformMatrix3x4fv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        UniformMatrix4fv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        UniformMatrix4x2fv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        UniformMatrix4x3fv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(UnmapBuffer, GLboolean, target: GLenum);
    func!(UseProgram, (), program: GLuint);
    func!(ValidateProgram, (), program: GLuint);
    func!(VertexAttrib1d, (), index: GLuint, x: GLdouble);
    func!(VertexAttrib1dv, (), index: GLuint, v: *const GLdouble);
    func!(VertexAttrib1f, (), index: GLuint, x: GLfloat);
    func!(VertexAttrib1fv, (), index: GLuint, v: *const GLfloat);
    func!(VertexAttrib1s, (), index: GLuint, x: GLshort);
    func!(VertexAttrib1sv, (), index: GLuint, v: *const GLshort);
    func!(VertexAttrib2d, (), index: GLuint, x: GLdouble, y: GLdouble);
    func!(VertexAttrib2dv, (), index: GLuint, v: *const GLdouble);
    func!(VertexAttrib2f, (), index: GLuint, x: GLfloat, y: GLfloat);
    func!(VertexAttrib2fv, (), index: GLuint, v: *const GLfloat);
    func!(VertexAttrib2s, (), index: GLuint, x: GLshort, y: GLshort);
    func!(VertexAttrib2sv, (), index: GLuint, v: *const GLshort);
    func!(
        VertexAttrib3d,
        (),
        index: GLuint,
        x: GLdouble,
        y: GLdouble,
        z: GLdouble
    );
    func!(VertexAttrib3dv, (), index: GLuint, v: *const GLdouble);
    func!(
        VertexAttrib3f,
        (),
        index: GLuint,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat
    );
    func!(VertexAttrib3fv, (), index: GLuint, v: *const GLfloat);
    func!(
        VertexAttrib3s,
        (),
        index: GLuint,
        x: GLshort,
        y: GLshort,
        z: GLshort
    );
    func!(VertexAttrib3sv, (), index: GLuint, v: *const GLshort);
    func!(VertexAttrib4Nbv, (), index: GLuint, v: *const GLbyte);
    func!(VertexAttrib4Niv, (), index: GLuint, v: *const GLint);
    func!(VertexAttrib4Nsv, (), index: GLuint, v: *const GLshort);
    func!(
        VertexAttrib4Nub,
        (),
        index: GLuint,
        x: GLubyte,
        y: GLubyte,
        z: GLubyte,
        w: GLubyte
    );
    func!(VertexAttrib4Nubv, (), index: GLuint, v: *const GLubyte);
    func!(VertexAttrib4Nuiv, (), index: GLuint, v: *const GLuint);
    func!(VertexAttrib4Nusv, (), index: GLuint, v: *const GLushort);
    func!(VertexAttrib4bv, (), index: GLuint, v: *const GLbyte);
    func!(
        VertexAttrib4d,
        (),
        index: GLuint,
        x: GLdouble,
        y: GLdouble,
        z: GLdouble,
        w: GLdouble
    );
    func!(VertexAttrib4dv, (), index: GLuint, v: *const GLdouble);
    func!(
        VertexAttrib4f,
        (),
        index: GLuint,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
        w: GLfloat
    );
    func!(VertexAttrib4fv, (), index: GLuint, v: *const GLfloat);
    func!(VertexAttrib4iv, (), index: GLuint, v: *const GLint);
    func!(
        VertexAttrib4s,
        (),
        index: GLuint,
        x: GLshort,
        y: GLshort,
        z: GLshort,
        w: GLshort
    );
    func!(VertexAttrib4sv, (), index: GLuint, v: *const GLshort);
    func!(VertexAttrib4ubv, (), index: GLuint, v: *const GLubyte);
    func!(VertexAttrib4uiv, (), index: GLuint, v: *const GLuint);
    func!(VertexAttrib4usv, (), index: GLuint, v: *const GLushort);
    func!(VertexAttribDivisor, (), index: GLuint, divisor: GLuint);
    func!(VertexAttribI1i, (), index: GLuint, x: GLint);
    func!(VertexAttribI1iv, (), index: GLuint, v: *const GLint);
    func!(VertexAttribI1ui, (), index: GLuint, x: GLuint);
    func!(VertexAttribI1uiv, (), index: GLuint, v: *const GLuint);
    func!(VertexAttribI2i, (), index: GLuint, x: GLint, y: GLint);
    func!(VertexAttribI2iv, (), index: GLuint, v: *const GLint);
    func!(VertexAttribI2ui, (), index: GLuint, x: GLuint, y: GLuint);
    func!(VertexAttribI2uiv, (), index: GLuint, v: *const GLuint);
    func!(
        VertexAttribI3i,
        (),
        index: GLuint,
        x: GLint,
        y: GLint,
        z: GLint
    );
    func!(VertexAttribI3iv, (), index: GLuint, v: *const GLint);
    func!(
        VertexAttribI3ui,
        (),
        index: GLuint,
        x: GLuint,
        y: GLuint,
        z: GLuint
    );
    func!(VertexAttribI3uiv, (), index: GLuint, v: *const GLuint);
    func!(VertexAttribI4bv, (), index: GLuint, v: *const GLbyte);
    func!(
        VertexAttribI4i,
        (),
        index: GLuint,
        x: GLint,
        y: GLint,
        z: GLint,
        w: GLint
    );
    func!(VertexAttribI4iv, (), index: GLuint, v: *const GLint);
    func!(VertexAttribI4sv, (), index: GLuint, v: *const GLshort);
    func!(VertexAttribI4ubv, (), index: GLuint, v: *const GLubyte);
    func!(
        VertexAttribI4ui,
        (),
        index: GLuint,
        x: GLuint,
        y: GLuint,
        z: GLuint,
        w: GLuint
    );
    func!(VertexAttribI4uiv, (), index: GLuint, v: *const GLuint);
    func!(VertexAttribI4usv, (), index: GLuint, v: *const GLushort);
    func!(
        VertexAttribIPointer,
        (),
        index: GLuint,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        pointer: *const c_void
    );
    func!(
        VertexAttribP1ui,
        (),
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: GLuint
    );
    func!(
        VertexAttribP1uiv,
        (),
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: *const GLuint
    );
    func!(
        VertexAttribP2ui,
        (),
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: GLuint
    );
    func!(
        VertexAttribP2uiv,
        (),
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: *const GLuint
    );
    func!(
        VertexAttribP3ui,
        (),
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: GLuint
    );
    func!(
        VertexAttribP3uiv,
        (),
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: *const GLuint
    );
    func!(
        VertexAttribP4ui,
        (),
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: GLuint
    );
    func!(
        VertexAttribP4uiv,
        (),
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: *const GLuint
    );
    func!(
        VertexAttribPointer,
        (),
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void
    );
    func!(
        Viewport,
        (),
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei
    );
    func!(
        WaitSync,
        (),
        sync: GLsync,
        flags: GLbitfield,
        timeout: GLuint64
    );
}

mod storage {
    #![allow(non_snake_case, non_upper_case_globals)]

    use super::FnPtr;
    use std::os::raw::*;

    macro_rules! store {
        ($name:ident) => {
            pub(super) static mut $name: FnPtr = FnPtr {
                ptr: FnPtr::not_initialized as *const c_void,
                is_loaded: false,
            };
        };
    }

    store!(ActiveTexture);
    store!(AttachShader);
    store!(BeginConditionalRender);
    store!(BeginQuery);
    store!(BeginTransformFeedback);
    store!(BindAttribLocation);
    store!(BindBuffer);
    store!(BindBufferBase);
    store!(BindBufferRange);
    store!(BindFragDataLocation);
    store!(BindFragDataLocationIndexed);
    store!(BindFramebuffer);
    store!(BindRenderbuffer);
    store!(BindSampler);
    store!(BindTexture);
    store!(BindVertexArray);
    store!(BlendColor);
    store!(BlendEquation);
    store!(BlendEquationSeparate);
    store!(BlendFunc);
    store!(BlendFuncSeparate);
    store!(BlitFramebuffer);
    store!(BufferData);
    store!(BufferSubData);
    store!(CheckFramebufferStatus);
    store!(ClampColor);
    store!(Clear);
    store!(ClearBufferfi);
    store!(ClearBufferfv);
    store!(ClearBufferiv);
    store!(ClearBufferuiv);
    store!(ClearColor);
    store!(ClearDepth);
    store!(ClearStencil);
    store!(ClientWaitSync);
    store!(ColorMask);
    store!(ColorMaski);
    store!(CompileShader);
    store!(CompressedTexImage1D);
    store!(CompressedTexImage2D);
    store!(CompressedTexImage3D);
    store!(CompressedTexSubImage1D);
    store!(CompressedTexSubImage2D);
    store!(CompressedTexSubImage3D);
    store!(CopyBufferSubData);
    store!(CopyTexImage1D);
    store!(CopyTexImage2D);
    store!(CopyTexSubImage1D);
    store!(CopyTexSubImage2D);
    store!(CopyTexSubImage3D);
    store!(CreateProgram);
    store!(CreateShader);
    store!(CullFace);
    store!(DeleteBuffers);
    store!(DeleteFramebuffers);
    store!(DeleteProgram);
    store!(DeleteQueries);
    store!(DeleteRenderbuffers);
    store!(DeleteSamplers);
    store!(DeleteShader);
    store!(DeleteSync);
    store!(DeleteTextures);
    store!(DeleteVertexArrays);
    store!(DepthFunc);
    store!(DepthMask);
    store!(DepthRange);
    store!(DetachShader);
    store!(Disable);
    store!(DisableVertexAttribArray);
    store!(Disablei);
    store!(DrawArrays);
    store!(DrawArraysInstanced);
    store!(DrawBuffer);
    store!(DrawBuffers);
    store!(DrawElements);
    store!(DrawElementsBaseVertex);
    store!(DrawElementsInstanced);
    store!(DrawElementsInstancedBaseVertex);
    store!(DrawRangeElements);
    store!(DrawRangeElementsBaseVertex);
    store!(Enable);
    store!(EnableVertexAttribArray);
    store!(Enablei);
    store!(EndConditionalRender);
    store!(EndQuery);
    store!(EndTransformFeedback);
    store!(FenceSync);
    store!(Finish);
    store!(Flush);
    store!(FlushMappedBufferRange);
    store!(FramebufferRenderbuffer);
    store!(FramebufferTexture);
    store!(FramebufferTexture1D);
    store!(FramebufferTexture2D);
    store!(FramebufferTexture3D);
    store!(FramebufferTextureLayer);
    store!(FrontFace);
    store!(GenBuffers);
    store!(GenFramebuffers);
    store!(GenQueries);
    store!(GenRenderbuffers);
    store!(GenSamplers);
    store!(GenTextures);
    store!(GenVertexArrays);
    store!(GenerateMipmap);
    store!(GetActiveAttrib);
    store!(GetActiveUniform);
    store!(GetActiveUniformBlockName);
    store!(GetActiveUniformBlockiv);
    store!(GetActiveUniformName);
    store!(GetActiveUniformsiv);
    store!(GetAttachedShaders);
    store!(GetAttribLocation);
    store!(GetBooleani_v);
    store!(GetBooleanv);
    store!(GetBufferParameteri64v);
    store!(GetBufferParameteriv);
    store!(GetBufferPointerv);
    store!(GetBufferSubData);
    store!(GetCompressedTexImage);
    store!(GetDoublev);
    store!(GetError);
    store!(GetFloatv);
    store!(GetFragDataIndex);
    store!(GetFragDataLocation);
    store!(GetFramebufferAttachmentParameteriv);
    store!(GetInteger64i_v);
    store!(GetInteger64v);
    store!(GetIntegeri_v);
    store!(GetIntegerv);
    store!(GetMultisamplefv);
    store!(GetProgramInfoLog);
    store!(GetProgramiv);
    store!(GetQueryObjecti64v);
    store!(GetQueryObjectiv);
    store!(GetQueryObjectui64v);
    store!(GetQueryObjectuiv);
    store!(GetQueryiv);
    store!(GetRenderbufferParameteriv);
    store!(GetSamplerParameterIiv);
    store!(GetSamplerParameterIuiv);
    store!(GetSamplerParameterfv);
    store!(GetSamplerParameteriv);
    store!(GetShaderInfoLog);
    store!(GetShaderSource);
    store!(GetShaderiv);
    store!(GetString);
    store!(GetStringi);
    store!(GetSynciv);
    store!(GetTexImage);
    store!(GetTexLevelParameterfv);
    store!(GetTexLevelParameteriv);
    store!(GetTexParameterIiv);
    store!(GetTexParameterIuiv);
    store!(GetTexParameterfv);
    store!(GetTexParameteriv);
    store!(GetTransformFeedbackVarying);
    store!(GetUniformBlockIndex);
    store!(GetUniformIndices);
    store!(GetUniformLocation);
    store!(GetUniformfv);
    store!(GetUniformiv);
    store!(GetUniformuiv);
    store!(GetVertexAttribIiv);
    store!(GetVertexAttribIuiv);
    store!(GetVertexAttribPointerv);
    store!(GetVertexAttribdv);
    store!(GetVertexAttribfv);
    store!(GetVertexAttribiv);
    store!(Hint);
    store!(IsBuffer);
    store!(IsEnabled);
    store!(IsEnabledi);
    store!(IsFramebuffer);
    store!(IsProgram);
    store!(IsQuery);
    store!(IsRenderbuffer);
    store!(IsSampler);
    store!(IsShader);
    store!(IsSync);
    store!(IsTexture);
    store!(IsVertexArray);
    store!(LineWidth);
    store!(LinkProgram);
    store!(LogicOp);
    store!(MapBuffer);
    store!(MapBufferRange);
    store!(MultiDrawArrays);
    store!(MultiDrawElements);
    store!(MultiDrawElementsBaseVertex);
    store!(PixelStoref);
    store!(PixelStorei);
    store!(PointParameterf);
    store!(PointParameterfv);
    store!(PointParameteri);
    store!(PointParameteriv);
    store!(PointSize);
    store!(PolygonMode);
    store!(PolygonOffset);
    store!(PrimitiveRestartIndex);
    store!(ProvokingVertex);
    store!(QueryCounter);
    store!(ReadBuffer);
    store!(ReadPixels);
    store!(RenderbufferStorage);
    store!(RenderbufferStorageMultisample);
    store!(SampleCoverage);
    store!(SampleMaski);
    store!(SamplerParameterIiv);
    store!(SamplerParameterIuiv);
    store!(SamplerParameterf);
    store!(SamplerParameterfv);
    store!(SamplerParameteri);
    store!(SamplerParameteriv);
    store!(Scissor);
    store!(ShaderSource);
    store!(StencilFunc);
    store!(StencilFuncSeparate);
    store!(StencilMask);
    store!(StencilMaskSeparate);
    store!(StencilOp);
    store!(StencilOpSeparate);
    store!(TexBuffer);
    store!(TexImage1D);
    store!(TexImage2D);
    store!(TexImage2DMultisample);
    store!(TexImage3D);
    store!(TexImage3DMultisample);
    store!(TexParameterIiv);
    store!(TexParameterIuiv);
    store!(TexParameterf);
    store!(TexParameterfv);
    store!(TexParameteri);
    store!(TexParameteriv);
    store!(TexSubImage1D);
    store!(TexSubImage2D);
    store!(TexSubImage3D);
    store!(TransformFeedbackVaryings);
    store!(Uniform1f);
    store!(Uniform1fv);
    store!(Uniform1i);
    store!(Uniform1iv);
    store!(Uniform1ui);
    store!(Uniform1uiv);
    store!(Uniform2f);
    store!(Uniform2fv);
    store!(Uniform2i);
    store!(Uniform2iv);
    store!(Uniform2ui);
    store!(Uniform2uiv);
    store!(Uniform3f);
    store!(Uniform3fv);
    store!(Uniform3i);
    store!(Uniform3iv);
    store!(Uniform3ui);
    store!(Uniform3uiv);
    store!(Uniform4f);
    store!(Uniform4fv);
    store!(Uniform4i);
    store!(Uniform4iv);
    store!(Uniform4ui);
    store!(Uniform4uiv);
    store!(UniformBlockBinding);
    store!(UniformMatrix2fv);
    store!(UniformMatrix2x3fv);
    store!(UniformMatrix2x4fv);
    store!(UniformMatrix3fv);
    store!(UniformMatrix3x2fv);
    store!(UniformMatrix3x4fv);
    store!(UniformMatrix4fv);
    store!(UniformMatrix4x2fv);
    store!(UniformMatrix4x3fv);
    store!(UnmapBuffer);
    store!(UseProgram);
    store!(ValidateProgram);
    store!(VertexAttrib1d);
    store!(VertexAttrib1dv);
    store!(VertexAttrib1f);
    store!(VertexAttrib1fv);
    store!(VertexAttrib1s);
    store!(VertexAttrib1sv);
    store!(VertexAttrib2d);
    store!(VertexAttrib2dv);
    store!(VertexAttrib2f);
    store!(VertexAttrib2fv);
    store!(VertexAttrib2s);
    store!(VertexAttrib2sv);
    store!(VertexAttrib3d);
    store!(VertexAttrib3dv);
    store!(VertexAttrib3f);
    store!(VertexAttrib3fv);
    store!(VertexAttrib3s);
    store!(VertexAttrib3sv);
    store!(VertexAttrib4Nbv);
    store!(VertexAttrib4Niv);
    store!(VertexAttrib4Nsv);
    store!(VertexAttrib4Nub);
    store!(VertexAttrib4Nubv);
    store!(VertexAttrib4Nuiv);
    store!(VertexAttrib4Nusv);
    store!(VertexAttrib4bv);
    store!(VertexAttrib4d);
    store!(VertexAttrib4dv);
    store!(VertexAttrib4f);
    store!(VertexAttrib4fv);
    store!(VertexAttrib4iv);
    store!(VertexAttrib4s);
    store!(VertexAttrib4sv);
    store!(VertexAttrib4ubv);
    store!(VertexAttrib4uiv);
    store!(VertexAttrib4usv);
    store!(VertexAttribDivisor);
    store!(VertexAttribI1i);
    store!(VertexAttribI1iv);
    store!(VertexAttribI1ui);
    store!(VertexAttribI1uiv);
    store!(VertexAttribI2i);
    store!(VertexAttribI2iv);
    store!(VertexAttribI2ui);
    store!(VertexAttribI2uiv);
    store!(VertexAttribI3i);
    store!(VertexAttribI3iv);
    store!(VertexAttribI3ui);
    store!(VertexAttribI3uiv);
    store!(VertexAttribI4bv);
    store!(VertexAttribI4i);
    store!(VertexAttribI4iv);
    store!(VertexAttribI4sv);
    store!(VertexAttribI4ubv);
    store!(VertexAttribI4ui);
    store!(VertexAttribI4uiv);
    store!(VertexAttribI4usv);
    store!(VertexAttribIPointer);
    store!(VertexAttribP1ui);
    store!(VertexAttribP1uiv);
    store!(VertexAttribP2ui);
    store!(VertexAttribP2uiv);
    store!(VertexAttribP3ui);
    store!(VertexAttribP3uiv);
    store!(VertexAttribP4ui);
    store!(VertexAttribP4uiv);
    store!(VertexAttribPointer);
    store!(Viewport);
    store!(WaitSync);
}

pub fn load<F>(mut loadfn: F)
where
    F: FnMut(&'static str) -> *const c_void,
{
    unsafe {
        storage::ActiveTexture.set_ptr(loadfn("glActiveTexture"));
        storage::AttachShader.set_ptr(loadfn("glAttachShader"));
        storage::BeginConditionalRender.set_ptr(loadfn("glBeginConditionalRender"));
        storage::BeginQuery.set_ptr(loadfn("glBeginQuery"));
        storage::BeginTransformFeedback.set_ptr(loadfn("glBeginTransformFeedback"));
        storage::BindAttribLocation.set_ptr(loadfn("glBindAttribLocation"));
        storage::BindBuffer.set_ptr(loadfn("glBindBuffer"));
        storage::BindBufferBase.set_ptr(loadfn("glBindBufferBase"));
        storage::BindBufferRange.set_ptr(loadfn("glBindBufferRange"));
        storage::BindFragDataLocation.set_ptr(loadfn("glBindFragDataLocation"));
        storage::BindFragDataLocationIndexed.set_ptr(loadfn("glBindFragDataLocationIndexed"));
        storage::BindFramebuffer.set_ptr(loadfn("glBindFramebuffer"));
        storage::BindRenderbuffer.set_ptr(loadfn("glBindRenderbuffer"));
        storage::BindSampler.set_ptr(loadfn("glBindSampler"));
        storage::BindTexture.set_ptr(loadfn("glBindTexture"));
        storage::BindVertexArray.set_ptr(loadfn("glBindVertexArray"));
        storage::BlendColor.set_ptr(loadfn("glBlendColor"));
        storage::BlendEquation.set_ptr(loadfn("glBlendEquation"));
        storage::BlendEquationSeparate.set_ptr(loadfn("glBlendEquationSeparate"));
        storage::BlendFunc.set_ptr(loadfn("glBlendFunc"));
        storage::BlendFuncSeparate.set_ptr(loadfn("glBlendFuncSeparate"));
        storage::BlitFramebuffer.set_ptr(loadfn("glBlitFramebuffer"));
        storage::BufferData.set_ptr(loadfn("glBufferData"));
        storage::BufferSubData.set_ptr(loadfn("glBufferSubData"));
        storage::CheckFramebufferStatus.set_ptr(loadfn("glCheckFramebufferStatus"));
        storage::ClampColor.set_ptr(loadfn("glClampColor"));
        storage::Clear.set_ptr(loadfn("glClear"));
        storage::ClearBufferfi.set_ptr(loadfn("glClearBufferfi"));
        storage::ClearBufferfv.set_ptr(loadfn("glClearBufferfv"));
        storage::ClearBufferiv.set_ptr(loadfn("glClearBufferiv"));
        storage::ClearBufferuiv.set_ptr(loadfn("glClearBufferuiv"));
        storage::ClearColor.set_ptr(loadfn("glClearColor"));
        storage::ClearDepth.set_ptr(loadfn("glClearDepth"));
        storage::ClearStencil.set_ptr(loadfn("glClearStencil"));
        storage::ClientWaitSync.set_ptr(loadfn("glClientWaitSync"));
        storage::ColorMask.set_ptr(loadfn("glColorMask"));
        storage::ColorMaski.set_ptr(loadfn("glColorMaski"));
        storage::CompileShader.set_ptr(loadfn("glCompileShader"));
        storage::CompressedTexImage1D.set_ptr(loadfn("glCompressedTexImage1D"));
        storage::CompressedTexImage2D.set_ptr(loadfn("glCompressedTexImage2D"));
        storage::CompressedTexImage3D.set_ptr(loadfn("glCompressedTexImage3D"));
        storage::CompressedTexSubImage1D.set_ptr(loadfn("glCompressedTexSubImage1D"));
        storage::CompressedTexSubImage2D.set_ptr(loadfn("glCompressedTexSubImage2D"));
        storage::CompressedTexSubImage3D.set_ptr(loadfn("glCompressedTexSubImage3D"));
        storage::CopyBufferSubData.set_ptr(loadfn("glCopyBufferSubData"));
        storage::CopyTexImage1D.set_ptr(loadfn("glCopyTexImage1D"));
        storage::CopyTexImage2D.set_ptr(loadfn("glCopyTexImage2D"));
        storage::CopyTexSubImage1D.set_ptr(loadfn("glCopyTexSubImage1D"));
        storage::CopyTexSubImage2D.set_ptr(loadfn("glCopyTexSubImage2D"));
        storage::CopyTexSubImage3D.set_ptr(loadfn("glCopyTexSubImage3D"));
        storage::CreateProgram.set_ptr(loadfn("glCreateProgram"));
        storage::CreateShader.set_ptr(loadfn("glCreateShader"));
        storage::CullFace.set_ptr(loadfn("glCullFace"));
        storage::DeleteBuffers.set_ptr(loadfn("glDeleteBuffers"));
        storage::DeleteFramebuffers.set_ptr(loadfn("glDeleteFramebuffers"));
        storage::DeleteProgram.set_ptr(loadfn("glDeleteProgram"));
        storage::DeleteQueries.set_ptr(loadfn("glDeleteQueries"));
        storage::DeleteRenderbuffers.set_ptr(loadfn("glDeleteRenderbuffers"));
        storage::DeleteSamplers.set_ptr(loadfn("glDeleteSamplers"));
        storage::DeleteShader.set_ptr(loadfn("glDeleteShader"));
        storage::DeleteSync.set_ptr(loadfn("glDeleteSync"));
        storage::DeleteTextures.set_ptr(loadfn("glDeleteTextures"));
        storage::DeleteVertexArrays.set_ptr(loadfn("glDeleteVertexArrays"));
        storage::DepthFunc.set_ptr(loadfn("glDepthFunc"));
        storage::DepthMask.set_ptr(loadfn("glDepthMask"));
        storage::DepthRange.set_ptr(loadfn("glDepthRange"));
        storage::DetachShader.set_ptr(loadfn("glDetachShader"));
        storage::Disable.set_ptr(loadfn("glDisable"));
        storage::DisableVertexAttribArray.set_ptr(loadfn("glDisableVertexAttribArray"));
        storage::Disablei.set_ptr(loadfn("glDisablei"));
        storage::DrawArrays.set_ptr(loadfn("glDrawArrays"));
        storage::DrawArraysInstanced.set_ptr(loadfn("glDrawArraysInstanced"));
        storage::DrawBuffer.set_ptr(loadfn("glDrawBuffer"));
        storage::DrawBuffers.set_ptr(loadfn("glDrawBuffers"));
        storage::DrawElements.set_ptr(loadfn("glDrawElements"));
        storage::DrawElementsBaseVertex.set_ptr(loadfn("glDrawElementsBaseVertex"));
        storage::DrawElementsInstanced.set_ptr(loadfn("glDrawElementsInstanced"));
        storage::DrawElementsInstancedBaseVertex
            .set_ptr(loadfn("glDrawElementsInstancedBaseVertex"));
        storage::DrawRangeElements.set_ptr(loadfn("glDrawRangeElements"));
        storage::DrawRangeElementsBaseVertex.set_ptr(loadfn("glDrawRangeElementsBaseVertex"));
        storage::Enable.set_ptr(loadfn("glEnable"));
        storage::EnableVertexAttribArray.set_ptr(loadfn("glEnableVertexAttribArray"));
        storage::Enablei.set_ptr(loadfn("glEnablei"));
        storage::EndConditionalRender.set_ptr(loadfn("glEndConditionalRender"));
        storage::EndQuery.set_ptr(loadfn("glEndQuery"));
        storage::EndTransformFeedback.set_ptr(loadfn("glEndTransformFeedback"));
        storage::FenceSync.set_ptr(loadfn("glFenceSync"));
        storage::Finish.set_ptr(loadfn("glFinish"));
        storage::Flush.set_ptr(loadfn("glFlush"));
        storage::FlushMappedBufferRange.set_ptr(loadfn("glFlushMappedBufferRange"));
        storage::FramebufferRenderbuffer.set_ptr(loadfn("glFramebufferRenderbuffer"));
        storage::FramebufferTexture.set_ptr(loadfn("glFramebufferTexture"));
        storage::FramebufferTexture1D.set_ptr(loadfn("glFramebufferTexture1D"));
        storage::FramebufferTexture2D.set_ptr(loadfn("glFramebufferTexture2D"));
        storage::FramebufferTexture3D.set_ptr(loadfn("glFramebufferTexture3D"));
        storage::FramebufferTextureLayer.set_ptr(loadfn("glFramebufferTextureLayer"));
        storage::FrontFace.set_ptr(loadfn("glFrontFace"));
        storage::GenBuffers.set_ptr(loadfn("glGenBuffers"));
        storage::GenFramebuffers.set_ptr(loadfn("glGenFramebuffers"));
        storage::GenQueries.set_ptr(loadfn("glGenQueries"));
        storage::GenRenderbuffers.set_ptr(loadfn("glGenRenderbuffers"));
        storage::GenSamplers.set_ptr(loadfn("glGenSamplers"));
        storage::GenTextures.set_ptr(loadfn("glGenTextures"));
        storage::GenVertexArrays.set_ptr(loadfn("glGenVertexArrays"));
        storage::GenerateMipmap.set_ptr(loadfn("glGenerateMipmap"));
        storage::GetActiveAttrib.set_ptr(loadfn("glGetActiveAttrib"));
        storage::GetActiveUniform.set_ptr(loadfn("glGetActiveUniform"));
        storage::GetActiveUniformBlockName.set_ptr(loadfn("glGetActiveUniformBlockName"));
        storage::GetActiveUniformBlockiv.set_ptr(loadfn("glGetActiveUniformBlockiv"));
        storage::GetActiveUniformName.set_ptr(loadfn("glGetActiveUniformName"));
        storage::GetActiveUniformsiv.set_ptr(loadfn("glGetActiveUniformsiv"));
        storage::GetAttachedShaders.set_ptr(loadfn("glGetAttachedShaders"));
        storage::GetAttribLocation.set_ptr(loadfn("glGetAttribLocation"));
        storage::GetBooleani_v.set_ptr(loadfn("glGetBooleani_v"));
        storage::GetBooleanv.set_ptr(loadfn("glGetBooleanv"));
        storage::GetBufferParameteri64v.set_ptr(loadfn("glGetBufferParameteri64v"));
        storage::GetBufferParameteriv.set_ptr(loadfn("glGetBufferParameteriv"));
        storage::GetBufferPointerv.set_ptr(loadfn("glGetBufferPointerv"));
        storage::GetBufferSubData.set_ptr(loadfn("glGetBufferSubData"));
        storage::GetCompressedTexImage.set_ptr(loadfn("glGetCompressedTexImage"));
        storage::GetDoublev.set_ptr(loadfn("glGetDoublev"));
        storage::GetError.set_ptr(loadfn("glGetError"));
        storage::GetFloatv.set_ptr(loadfn("glGetFloatv"));
        storage::GetFragDataIndex.set_ptr(loadfn("glGetFragDataIndex"));
        storage::GetFragDataLocation.set_ptr(loadfn("glGetFragDataLocation"));
        storage::GetFramebufferAttachmentParameteriv
            .set_ptr(loadfn("glGetFramebufferAttachmentParameteriv"));
        storage::GetInteger64i_v.set_ptr(loadfn("glGetInteger64i_v"));
        storage::GetInteger64v.set_ptr(loadfn("glGetInteger64v"));
        storage::GetIntegeri_v.set_ptr(loadfn("glGetIntegeri_v"));
        storage::GetIntegerv.set_ptr(loadfn("glGetIntegerv"));
        storage::GetMultisamplefv.set_ptr(loadfn("glGetMultisamplefv"));
        storage::GetProgramInfoLog.set_ptr(loadfn("glGetProgramInfoLog"));
        storage::GetProgramiv.set_ptr(loadfn("glGetProgramiv"));
        storage::GetQueryObjecti64v.set_ptr(loadfn("glGetQueryObjecti64v"));
        storage::GetQueryObjectiv.set_ptr(loadfn("glGetQueryObjectiv"));
        storage::GetQueryObjectui64v.set_ptr(loadfn("glGetQueryObjectui64v"));
        storage::GetQueryObjectuiv.set_ptr(loadfn("glGetQueryObjectuiv"));
        storage::GetQueryiv.set_ptr(loadfn("glGetQueryiv"));
        storage::GetRenderbufferParameteriv.set_ptr(loadfn("glGetRenderbufferParameteriv"));
        storage::GetSamplerParameterIiv.set_ptr(loadfn("glGetSamplerParameterIiv"));
        storage::GetSamplerParameterIuiv.set_ptr(loadfn("glGetSamplerParameterIuiv"));
        storage::GetSamplerParameterfv.set_ptr(loadfn("glGetSamplerParameterfv"));
        storage::GetSamplerParameteriv.set_ptr(loadfn("glGetSamplerParameteriv"));
        storage::GetShaderInfoLog.set_ptr(loadfn("glGetShaderInfoLog"));
        storage::GetShaderSource.set_ptr(loadfn("glGetShaderSource"));
        storage::GetShaderiv.set_ptr(loadfn("glGetShaderiv"));
        storage::GetString.set_ptr(loadfn("glGetString"));
        storage::GetStringi.set_ptr(loadfn("glGetStringi"));
        storage::GetSynciv.set_ptr(loadfn("glGetSynciv"));
        storage::GetTexImage.set_ptr(loadfn("glGetTexImage"));
        storage::GetTexLevelParameterfv.set_ptr(loadfn("glGetTexLevelParameterfv"));
        storage::GetTexLevelParameteriv.set_ptr(loadfn("glGetTexLevelParameteriv"));
        storage::GetTexParameterIiv.set_ptr(loadfn("glGetTexParameterIiv"));
        storage::GetTexParameterIuiv.set_ptr(loadfn("glGetTexParameterIuiv"));
        storage::GetTexParameterfv.set_ptr(loadfn("glGetTexParameterfv"));
        storage::GetTexParameteriv.set_ptr(loadfn("glGetTexParameteriv"));
        storage::GetTransformFeedbackVarying.set_ptr(loadfn("glGetTransformFeedbackVarying"));
        storage::GetUniformBlockIndex.set_ptr(loadfn("glGetUniformBlockIndex"));
        storage::GetUniformIndices.set_ptr(loadfn("glGetUniformIndices"));
        storage::GetUniformLocation.set_ptr(loadfn("glGetUniformLocation"));
        storage::GetUniformfv.set_ptr(loadfn("glGetUniformfv"));
        storage::GetUniformiv.set_ptr(loadfn("glGetUniformiv"));
        storage::GetUniformuiv.set_ptr(loadfn("glGetUniformuiv"));
        storage::GetVertexAttribIiv.set_ptr(loadfn("glGetVertexAttribIiv"));
        storage::GetVertexAttribIuiv.set_ptr(loadfn("glGetVertexAttribIuiv"));
        storage::GetVertexAttribPointerv.set_ptr(loadfn("glGetVertexAttribPointerv"));
        storage::GetVertexAttribdv.set_ptr(loadfn("glGetVertexAttribdv"));
        storage::GetVertexAttribfv.set_ptr(loadfn("glGetVertexAttribfv"));
        storage::GetVertexAttribiv.set_ptr(loadfn("glGetVertexAttribiv"));
        storage::Hint.set_ptr(loadfn("glHint"));
        storage::IsBuffer.set_ptr(loadfn("glIsBuffer"));
        storage::IsEnabled.set_ptr(loadfn("glIsEnabled"));
        storage::IsEnabledi.set_ptr(loadfn("glIsEnabledi"));
        storage::IsFramebuffer.set_ptr(loadfn("glIsFramebuffer"));
        storage::IsProgram.set_ptr(loadfn("glIsProgram"));
        storage::IsQuery.set_ptr(loadfn("glIsQuery"));
        storage::IsRenderbuffer.set_ptr(loadfn("glIsRenderbuffer"));
        storage::IsSampler.set_ptr(loadfn("glIsSampler"));
        storage::IsShader.set_ptr(loadfn("glIsShader"));
        storage::IsSync.set_ptr(loadfn("glIsSync"));
        storage::IsTexture.set_ptr(loadfn("glIsTexture"));
        storage::IsVertexArray.set_ptr(loadfn("glIsVertexArray"));
        storage::LineWidth.set_ptr(loadfn("glLineWidth"));
        storage::LinkProgram.set_ptr(loadfn("glLinkProgram"));
        storage::LogicOp.set_ptr(loadfn("glLogicOp"));
        storage::MapBuffer.set_ptr(loadfn("glMapBuffer"));
        storage::MapBufferRange.set_ptr(loadfn("glMapBufferRange"));
        storage::MultiDrawArrays.set_ptr(loadfn("glMultiDrawArrays"));
        storage::MultiDrawElements.set_ptr(loadfn("glMultiDrawElements"));
        storage::MultiDrawElementsBaseVertex.set_ptr(loadfn("glMultiDrawElementsBaseVertex"));
        storage::PixelStoref.set_ptr(loadfn("glPixelStoref"));
        storage::PixelStorei.set_ptr(loadfn("glPixelStorei"));
        storage::PointParameterf.set_ptr(loadfn("glPointParameterf"));
        storage::PointParameterfv.set_ptr(loadfn("glPointParameterfv"));
        storage::PointParameteri.set_ptr(loadfn("glPointParameteri"));
        storage::PointParameteriv.set_ptr(loadfn("glPointParameteriv"));
        storage::PointSize.set_ptr(loadfn("glPointSize"));
        storage::PolygonMode.set_ptr(loadfn("glPolygonMode"));
        storage::PolygonOffset.set_ptr(loadfn("glPolygonOffset"));
        storage::PrimitiveRestartIndex.set_ptr(loadfn("glPrimitiveRestartIndex"));
        storage::ProvokingVertex.set_ptr(loadfn("glProvokingVertex"));
        storage::QueryCounter.set_ptr(loadfn("glQueryCounter"));
        storage::ReadBuffer.set_ptr(loadfn("glReadBuffer"));
        storage::ReadPixels.set_ptr(loadfn("glReadPixels"));
        storage::RenderbufferStorage.set_ptr(loadfn("glRenderbufferStorage"));
        storage::RenderbufferStorageMultisample.set_ptr(loadfn("glRenderbufferStorageMultisample"));
        storage::SampleCoverage.set_ptr(loadfn("glSampleCoverage"));
        storage::SampleMaski.set_ptr(loadfn("glSampleMaski"));
        storage::SamplerParameterIiv.set_ptr(loadfn("glSamplerParameterIiv"));
        storage::SamplerParameterIuiv.set_ptr(loadfn("glSamplerParameterIuiv"));
        storage::SamplerParameterf.set_ptr(loadfn("glSamplerParameterf"));
        storage::SamplerParameterfv.set_ptr(loadfn("glSamplerParameterfv"));
        storage::SamplerParameteri.set_ptr(loadfn("glSamplerParameteri"));
        storage::SamplerParameteriv.set_ptr(loadfn("glSamplerParameteriv"));
        storage::Scissor.set_ptr(loadfn("glScissor"));
        storage::ShaderSource.set_ptr(loadfn("glShaderSource"));
        storage::StencilFunc.set_ptr(loadfn("glStencilFunc"));
        storage::StencilFuncSeparate.set_ptr(loadfn("glStencilFuncSeparate"));
        storage::StencilMask.set_ptr(loadfn("glStencilMask"));
        storage::StencilMaskSeparate.set_ptr(loadfn("glStencilMaskSeparate"));
        storage::StencilOp.set_ptr(loadfn("glStencilOp"));
        storage::StencilOpSeparate.set_ptr(loadfn("glStencilOpSeparate"));
        storage::TexBuffer.set_ptr(loadfn("glTexBuffer"));
        storage::TexImage1D.set_ptr(loadfn("glTexImage1D"));
        storage::TexImage2D.set_ptr(loadfn("glTexImage2D"));
        storage::TexImage2DMultisample.set_ptr(loadfn("glTexImage2DMultisample"));
        storage::TexImage3D.set_ptr(loadfn("glTexImage3D"));
        storage::TexImage3DMultisample.set_ptr(loadfn("glTexImage3DMultisample"));
        storage::TexParameterIiv.set_ptr(loadfn("glTexParameterIiv"));
        storage::TexParameterIuiv.set_ptr(loadfn("glTexParameterIuiv"));
        storage::TexParameterf.set_ptr(loadfn("glTexParameterf"));
        storage::TexParameterfv.set_ptr(loadfn("glTexParameterfv"));
        storage::TexParameteri.set_ptr(loadfn("glTexParameteri"));
        storage::TexParameteriv.set_ptr(loadfn("glTexParameteriv"));
        storage::TexSubImage1D.set_ptr(loadfn("glTexSubImage1D"));
        storage::TexSubImage2D.set_ptr(loadfn("glTexSubImage2D"));
        storage::TexSubImage3D.set_ptr(loadfn("glTexSubImage3D"));
        storage::TransformFeedbackVaryings.set_ptr(loadfn("glTransformFeedbackVaryings"));
        storage::Uniform1f.set_ptr(loadfn("glUniform1f"));
        storage::Uniform1fv.set_ptr(loadfn("glUniform1fv"));
        storage::Uniform1i.set_ptr(loadfn("glUniform1i"));
        storage::Uniform1iv.set_ptr(loadfn("glUniform1iv"));
        storage::Uniform1ui.set_ptr(loadfn("glUniform1ui"));
        storage::Uniform1uiv.set_ptr(loadfn("glUniform1uiv"));
        storage::Uniform2f.set_ptr(loadfn("glUniform2f"));
        storage::Uniform2fv.set_ptr(loadfn("glUniform2fv"));
        storage::Uniform2i.set_ptr(loadfn("glUniform2i"));
        storage::Uniform2iv.set_ptr(loadfn("glUniform2iv"));
        storage::Uniform2ui.set_ptr(loadfn("glUniform2ui"));
        storage::Uniform2uiv.set_ptr(loadfn("glUniform2uiv"));
        storage::Uniform3f.set_ptr(loadfn("glUniform3f"));
        storage::Uniform3fv.set_ptr(loadfn("glUniform3fv"));
        storage::Uniform3i.set_ptr(loadfn("glUniform3i"));
        storage::Uniform3iv.set_ptr(loadfn("glUniform3iv"));
        storage::Uniform3ui.set_ptr(loadfn("glUniform3ui"));
        storage::Uniform3uiv.set_ptr(loadfn("glUniform3uiv"));
        storage::Uniform4f.set_ptr(loadfn("glUniform4f"));
        storage::Uniform4fv.set_ptr(loadfn("glUniform4fv"));
        storage::Uniform4i.set_ptr(loadfn("glUniform4i"));
        storage::Uniform4iv.set_ptr(loadfn("glUniform4iv"));
        storage::Uniform4ui.set_ptr(loadfn("glUniform4ui"));
        storage::Uniform4uiv.set_ptr(loadfn("glUniform4uiv"));
        storage::UniformBlockBinding.set_ptr(loadfn("glUniformBlockBinding"));
        storage::UniformMatrix2fv.set_ptr(loadfn("glUniformMatrix2fv"));
        storage::UniformMatrix2x3fv.set_ptr(loadfn("glUniformMatrix2x3fv"));
        storage::UniformMatrix2x4fv.set_ptr(loadfn("glUniformMatrix2x4fv"));
        storage::UniformMatrix3fv.set_ptr(loadfn("glUniformMatrix3fv"));
        storage::UniformMatrix3x2fv.set_ptr(loadfn("glUniformMatrix3x2fv"));
        storage::UniformMatrix3x4fv.set_ptr(loadfn("glUniformMatrix3x4fv"));
        storage::UniformMatrix4fv.set_ptr(loadfn("glUniformMatrix4fv"));
        storage::UniformMatrix4x2fv.set_ptr(loadfn("glUniformMatrix4x2fv"));
        storage::UniformMatrix4x3fv.set_ptr(loadfn("glUniformMatrix4x3fv"));
        storage::UnmapBuffer.set_ptr(loadfn("glUnmapBuffer"));
        storage::UseProgram.set_ptr(loadfn("glUseProgram"));
        storage::ValidateProgram.set_ptr(loadfn("glValidateProgram"));
        storage::VertexAttrib1d.set_ptr(loadfn("glVertexAttrib1d"));
        storage::VertexAttrib1dv.set_ptr(loadfn("glVertexAttrib1dv"));
        storage::VertexAttrib1f.set_ptr(loadfn("glVertexAttrib1f"));
        storage::VertexAttrib1fv.set_ptr(loadfn("glVertexAttrib1fv"));
        storage::VertexAttrib1s.set_ptr(loadfn("glVertexAttrib1s"));
        storage::VertexAttrib1sv.set_ptr(loadfn("glVertexAttrib1sv"));
        storage::VertexAttrib2d.set_ptr(loadfn("glVertexAttrib2d"));
        storage::VertexAttrib2dv.set_ptr(loadfn("glVertexAttrib2dv"));
        storage::VertexAttrib2f.set_ptr(loadfn("glVertexAttrib2f"));
        storage::VertexAttrib2fv.set_ptr(loadfn("glVertexAttrib2fv"));
        storage::VertexAttrib2s.set_ptr(loadfn("glVertexAttrib2s"));
        storage::VertexAttrib2sv.set_ptr(loadfn("glVertexAttrib2sv"));
        storage::VertexAttrib3d.set_ptr(loadfn("glVertexAttrib3d"));
        storage::VertexAttrib3dv.set_ptr(loadfn("glVertexAttrib3dv"));
        storage::VertexAttrib3f.set_ptr(loadfn("glVertexAttrib3f"));
        storage::VertexAttrib3fv.set_ptr(loadfn("glVertexAttrib3fv"));
        storage::VertexAttrib3s.set_ptr(loadfn("glVertexAttrib3s"));
        storage::VertexAttrib3sv.set_ptr(loadfn("glVertexAttrib3sv"));
        storage::VertexAttrib4Nbv.set_ptr(loadfn("glVertexAttrib4Nbv"));
        storage::VertexAttrib4Niv.set_ptr(loadfn("glVertexAttrib4Niv"));
        storage::VertexAttrib4Nsv.set_ptr(loadfn("glVertexAttrib4Nsv"));
        storage::VertexAttrib4Nub.set_ptr(loadfn("glVertexAttrib4Nub"));
        storage::VertexAttrib4Nubv.set_ptr(loadfn("glVertexAttrib4Nubv"));
        storage::VertexAttrib4Nuiv.set_ptr(loadfn("glVertexAttrib4Nuiv"));
        storage::VertexAttrib4Nusv.set_ptr(loadfn("glVertexAttrib4Nusv"));
        storage::VertexAttrib4bv.set_ptr(loadfn("glVertexAttrib4bv"));
        storage::VertexAttrib4d.set_ptr(loadfn("glVertexAttrib4d"));
        storage::VertexAttrib4dv.set_ptr(loadfn("glVertexAttrib4dv"));
        storage::VertexAttrib4f.set_ptr(loadfn("glVertexAttrib4f"));
        storage::VertexAttrib4fv.set_ptr(loadfn("glVertexAttrib4fv"));
        storage::VertexAttrib4iv.set_ptr(loadfn("glVertexAttrib4iv"));
        storage::VertexAttrib4s.set_ptr(loadfn("glVertexAttrib4s"));
        storage::VertexAttrib4sv.set_ptr(loadfn("glVertexAttrib4sv"));
        storage::VertexAttrib4ubv.set_ptr(loadfn("glVertexAttrib4ubv"));
        storage::VertexAttrib4uiv.set_ptr(loadfn("glVertexAttrib4uiv"));
        storage::VertexAttrib4usv.set_ptr(loadfn("glVertexAttrib4usv"));
        storage::VertexAttribDivisor.set_ptr(loadfn("glVertexAttribDivisor"));
        storage::VertexAttribI1i.set_ptr(loadfn("glVertexAttribI1i"));
        storage::VertexAttribI1iv.set_ptr(loadfn("glVertexAttribI1iv"));
        storage::VertexAttribI1ui.set_ptr(loadfn("glVertexAttribI1ui"));
        storage::VertexAttribI1uiv.set_ptr(loadfn("glVertexAttribI1uiv"));
        storage::VertexAttribI2i.set_ptr(loadfn("glVertexAttribI2i"));
        storage::VertexAttribI2iv.set_ptr(loadfn("glVertexAttribI2iv"));
        storage::VertexAttribI2ui.set_ptr(loadfn("glVertexAttribI2ui"));
        storage::VertexAttribI2uiv.set_ptr(loadfn("glVertexAttribI2uiv"));
        storage::VertexAttribI3i.set_ptr(loadfn("glVertexAttribI3i"));
        storage::VertexAttribI3iv.set_ptr(loadfn("glVertexAttribI3iv"));
        storage::VertexAttribI3ui.set_ptr(loadfn("glVertexAttribI3ui"));
        storage::VertexAttribI3uiv.set_ptr(loadfn("glVertexAttribI3uiv"));
        storage::VertexAttribI4bv.set_ptr(loadfn("glVertexAttribI4bv"));
        storage::VertexAttribI4i.set_ptr(loadfn("glVertexAttribI4i"));
        storage::VertexAttribI4iv.set_ptr(loadfn("glVertexAttribI4iv"));
        storage::VertexAttribI4sv.set_ptr(loadfn("glVertexAttribI4sv"));
        storage::VertexAttribI4ubv.set_ptr(loadfn("glVertexAttribI4ubv"));
        storage::VertexAttribI4ui.set_ptr(loadfn("glVertexAttribI4ui"));
        storage::VertexAttribI4uiv.set_ptr(loadfn("glVertexAttribI4uiv"));
        storage::VertexAttribI4usv.set_ptr(loadfn("glVertexAttribI4usv"));
        storage::VertexAttribIPointer.set_ptr(loadfn("glVertexAttribIPointer"));
        storage::VertexAttribP1ui.set_ptr(loadfn("glVertexAttribP1ui"));
        storage::VertexAttribP1uiv.set_ptr(loadfn("glVertexAttribP1uiv"));
        storage::VertexAttribP2ui.set_ptr(loadfn("glVertexAttribP2ui"));
        storage::VertexAttribP2uiv.set_ptr(loadfn("glVertexAttribP2uiv"));
        storage::VertexAttribP3ui.set_ptr(loadfn("glVertexAttribP3ui"));
        storage::VertexAttribP3uiv.set_ptr(loadfn("glVertexAttribP3uiv"));
        storage::VertexAttribP4ui.set_ptr(loadfn("glVertexAttribP4ui"));
        storage::VertexAttribP4uiv.set_ptr(loadfn("glVertexAttribP4uiv"));
        storage::VertexAttribPointer.set_ptr(loadfn("glVertexAttribPointer"));
        storage::Viewport.set_ptr(loadfn("glViewport"));
        storage::WaitSync.set_ptr(loadfn("glWaitSync"));
    }
}
