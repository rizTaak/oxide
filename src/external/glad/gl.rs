pub use self::types::*;
pub use self::enumerations::*;
pub use self::functions::*;

use std::os::raw::c_void;


#[derive(Copy, Clone)]
struct FnPtr {
    ptr: *const c_void,
    is_loaded: bool
}

#[allow(dead_code)]
impl FnPtr {
    fn new(ptr: *const c_void) -> FnPtr {
        if !ptr.is_null() {
            FnPtr { ptr, is_loaded: true }
        } else {
            FnPtr { ptr: FnPtr::not_initialized as *const c_void, is_loaded: false }
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
    fn not_initialized() -> ! { panic!("gl: function not initialized") }
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


pub type GLDEBUGPROC = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut raw::c_void,
);
pub type GLDEBUGPROCARB = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut raw::c_void,
);
pub type GLDEBUGPROCKHR = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut GLvoid,
);
pub type GLDEBUGPROCAMD = extern "system" fn (
    id: GLuint,
    category: GLenum,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut GLvoid,
);
pub type GLVULKANPROCNV = extern "system" fn ();
}

pub mod enumerations {
    #![allow(dead_code, non_upper_case_globals, unused_imports)]

    use std::os::raw::*;
    use super::types::*;

    pub const ACTIVE_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92D9;
    pub const ACTIVE_ATTRIBUTES: c_uint = 0x8B89;
    pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: c_uint = 0x8B8A;
    pub const ACTIVE_PROGRAM: c_uint = 0x8259;
    pub const ACTIVE_RESOURCES: c_uint = 0x92F5;
    pub const ACTIVE_SUBROUTINES: c_uint = 0x8DE5;
    pub const ACTIVE_SUBROUTINE_MAX_LENGTH: c_uint = 0x8E48;
    pub const ACTIVE_SUBROUTINE_UNIFORMS: c_uint = 0x8DE6;
    pub const ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: c_uint = 0x8E47;
    pub const ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: c_uint = 0x8E49;
    pub const ACTIVE_TEXTURE: c_uint = 0x84E0;
    pub const ACTIVE_TEXTURE_ARB: c_uint = 0x84E0;
    pub const ACTIVE_UNIFORMS: c_uint = 0x8B86;
    pub const ACTIVE_UNIFORM_BLOCKS: c_uint = 0x8A36;
    pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: c_uint = 0x8A35;
    pub const ACTIVE_UNIFORM_MAX_LENGTH: c_uint = 0x8B87;
    pub const ACTIVE_VARIABLES: c_uint = 0x9305;
    pub const ACTIVE_VARYINGS_NV: c_uint = 0x8C81;
    pub const ACTIVE_VARYING_MAX_LENGTH_NV: c_uint = 0x8C82;
    pub const ALIASED_LINE_WIDTH_RANGE: c_uint = 0x846E;
    pub const ALL_BARRIER_BITS: c_uint = 0xFFFFFFFF;
    pub const ALL_BARRIER_BITS_EXT: c_uint = 0xFFFFFFFF;
    pub const ALL_SHADER_BITS: c_uint = 0xFFFFFFFF;
    pub const ALPHA: c_uint = 0x1906;
    pub const ALPHA16F_EXT: c_uint = 0x881C;
    pub const ALPHA16I_EXT: c_uint = 0x8D8A;
    pub const ALPHA16UI_EXT: c_uint = 0x8D78;
    pub const ALPHA32F_EXT: c_uint = 0x8816;
    pub const ALPHA32I_EXT: c_uint = 0x8D84;
    pub const ALPHA32UI_EXT: c_uint = 0x8D72;
    pub const ALPHA8I_EXT: c_uint = 0x8D90;
    pub const ALPHA8UI_EXT: c_uint = 0x8D7E;
    pub const ALPHA8_EXT: c_uint = 0x803C;
    pub const ALPHA_INTEGER_EXT: c_uint = 0x8D97;
    pub const ALREADY_SIGNALED: c_uint = 0x911A;
    pub const ALWAYS: c_uint = 0x0207;
    pub const AND: c_uint = 0x1501;
    pub const AND_INVERTED: c_uint = 0x1504;
    pub const AND_REVERSE: c_uint = 0x1502;
    pub const ANY_SAMPLES_PASSED: c_uint = 0x8C2F;
    pub const ANY_SAMPLES_PASSED_CONSERVATIVE: c_uint = 0x8D6A;
    pub const ARRAY_BUFFER: c_uint = 0x8892;
    pub const ARRAY_BUFFER_ARB: c_uint = 0x8892;
    pub const ARRAY_BUFFER_BINDING: c_uint = 0x8894;
    pub const ARRAY_BUFFER_BINDING_ARB: c_uint = 0x8894;
    pub const ARRAY_SIZE: c_uint = 0x92FB;
    pub const ARRAY_STRIDE: c_uint = 0x92FE;
    pub const ATOMIC_COUNTER_BARRIER_BIT: c_uint = 0x00001000;
    pub const ATOMIC_COUNTER_BARRIER_BIT_EXT: c_uint = 0x00001000;
    pub const ATOMIC_COUNTER_BUFFER: c_uint = 0x92C0;
    pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: c_uint = 0x92C5;
    pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: c_uint = 0x92C6;
    pub const ATOMIC_COUNTER_BUFFER_BINDING: c_uint = 0x92C1;
    pub const ATOMIC_COUNTER_BUFFER_DATA_SIZE: c_uint = 0x92C4;
    pub const ATOMIC_COUNTER_BUFFER_INDEX: c_uint = 0x9301;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: c_uint = 0x90ED;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: c_uint = 0x92CB;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: c_uint = 0x92CA;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: c_uint = 0x92C8;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: c_uint = 0x92C9;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: c_uint = 0x92C7;
    pub const ATOMIC_COUNTER_BUFFER_SIZE: c_uint = 0x92C3;
    pub const ATOMIC_COUNTER_BUFFER_START: c_uint = 0x92C2;
    pub const ATTACHED_SHADERS: c_uint = 0x8B85;
    pub const ATTRIB_ARRAY_POINTER_NV: c_uint = 0x8645;
    pub const ATTRIB_ARRAY_SIZE_NV: c_uint = 0x8623;
    pub const ATTRIB_ARRAY_STRIDE_NV: c_uint = 0x8624;
    pub const ATTRIB_ARRAY_TYPE_NV: c_uint = 0x8625;
    pub const AUTO_GENERATE_MIPMAP: c_uint = 0x8295;
    pub const BACK: c_uint = 0x0405;
    pub const BACK_LEFT: c_uint = 0x0402;
    pub const BACK_PRIMARY_COLOR_NV: c_uint = 0x8C77;
    pub const BACK_RIGHT: c_uint = 0x0403;
    pub const BACK_SECONDARY_COLOR_NV: c_uint = 0x8C78;
    pub const BGR: c_uint = 0x80E0;
    pub const BGRA: c_uint = 0x80E1;
    pub const BGRA8_EXT: c_uint = 0x93A1;
    pub const BGRA_INTEGER: c_uint = 0x8D9B;
    pub const BGRA_INTEGER_EXT: c_uint = 0x8D9B;
    pub const BGR_INTEGER: c_uint = 0x8D9A;
    pub const BGR_INTEGER_EXT: c_uint = 0x8D9A;
    pub const BLEND: c_uint = 0x0BE2;
    pub const BLEND_COLOR: c_uint = 0x8005;
    pub const BLEND_COLOR_EXT: c_uint = 0x8005;
    pub const BLEND_DST: c_uint = 0x0BE0;
    pub const BLEND_DST_ALPHA: c_uint = 0x80CA;
    pub const BLEND_DST_ALPHA_EXT: c_uint = 0x80CA;
    pub const BLEND_DST_RGB: c_uint = 0x80C8;
    pub const BLEND_DST_RGB_EXT: c_uint = 0x80C8;
    pub const BLEND_EQUATION: c_uint = 0x8009;
    pub const BLEND_EQUATION_ALPHA: c_uint = 0x883D;
    pub const BLEND_EQUATION_ALPHA_EXT: c_uint = 0x883D;
    pub const BLEND_EQUATION_EXT: c_uint = 0x8009;
    pub const BLEND_EQUATION_RGB: c_uint = 0x8009;
    pub const BLEND_EQUATION_RGB_EXT: c_uint = 0x8009;
    pub const BLEND_SRC: c_uint = 0x0BE1;
    pub const BLEND_SRC_ALPHA: c_uint = 0x80CB;
    pub const BLEND_SRC_ALPHA_EXT: c_uint = 0x80CB;
    pub const BLEND_SRC_RGB: c_uint = 0x80C9;
    pub const BLEND_SRC_RGB_EXT: c_uint = 0x80C9;
    pub const BLOCK_INDEX: c_uint = 0x92FD;
    pub const BLUE: c_uint = 0x1905;
    pub const BLUE_INTEGER: c_uint = 0x8D96;
    pub const BLUE_INTEGER_EXT: c_uint = 0x8D96;
    pub const BOOL: c_uint = 0x8B56;
    pub const BOOL_ARB: c_uint = 0x8B56;
    pub const BOOL_VEC2: c_uint = 0x8B57;
    pub const BOOL_VEC2_ARB: c_uint = 0x8B57;
    pub const BOOL_VEC3: c_uint = 0x8B58;
    pub const BOOL_VEC3_ARB: c_uint = 0x8B58;
    pub const BOOL_VEC4: c_uint = 0x8B59;
    pub const BOOL_VEC4_ARB: c_uint = 0x8B59;
    pub const BUFFER: c_uint = 0x82E0;
    pub const BUFFER_ACCESS: c_uint = 0x88BB;
    pub const BUFFER_ACCESS_ARB: c_uint = 0x88BB;
    pub const BUFFER_ACCESS_FLAGS: c_uint = 0x911F;
    pub const BUFFER_BINDING: c_uint = 0x9302;
    pub const BUFFER_DATA_SIZE: c_uint = 0x9303;
    pub const BUFFER_FLUSHING_UNMAP_APPLE: c_uint = 0x8A13;
    pub const BUFFER_IMMUTABLE_STORAGE: c_uint = 0x821F;
    pub const BUFFER_MAPPED: c_uint = 0x88BC;
    pub const BUFFER_MAPPED_ARB: c_uint = 0x88BC;
    pub const BUFFER_MAP_LENGTH: c_uint = 0x9120;
    pub const BUFFER_MAP_OFFSET: c_uint = 0x9121;
    pub const BUFFER_MAP_POINTER: c_uint = 0x88BD;
    pub const BUFFER_MAP_POINTER_ARB: c_uint = 0x88BD;
    pub const BUFFER_SERIALIZED_MODIFY_APPLE: c_uint = 0x8A12;
    pub const BUFFER_SIZE: c_uint = 0x8764;
    pub const BUFFER_SIZE_ARB: c_uint = 0x8764;
    pub const BUFFER_STORAGE_FLAGS: c_uint = 0x8220;
    pub const BUFFER_UPDATE_BARRIER_BIT: c_uint = 0x00000200;
    pub const BUFFER_UPDATE_BARRIER_BIT_EXT: c_uint = 0x00000200;
    pub const BUFFER_USAGE: c_uint = 0x8765;
    pub const BUFFER_USAGE_ARB: c_uint = 0x8765;
    pub const BUFFER_VARIABLE: c_uint = 0x92E5;
    pub const BYTE: c_uint = 0x1400;
    pub const CAVEAT_SUPPORT: c_uint = 0x82B8;
    pub const CCW: c_uint = 0x0901;
    pub const CLAMP_FRAGMENT_COLOR_ARB: c_uint = 0x891B;
    pub const CLAMP_READ_COLOR: c_uint = 0x891C;
    pub const CLAMP_READ_COLOR_ARB: c_uint = 0x891C;
    pub const CLAMP_TO_BORDER: c_uint = 0x812D;
    pub const CLAMP_TO_EDGE: c_uint = 0x812F;
    pub const CLAMP_VERTEX_COLOR_ARB: c_uint = 0x891A;
    pub const CLEAR: c_uint = 0x1500;
    pub const CLEAR_BUFFER: c_uint = 0x82B4;
    pub const CLEAR_TEXTURE: c_uint = 0x9365;
    pub const CLIENT_ACTIVE_TEXTURE_ARB: c_uint = 0x84E1;
    pub const CLIENT_MAPPED_BUFFER_BARRIER_BIT: c_uint = 0x00004000;
    pub const CLIENT_STORAGE_BIT: c_uint = 0x0200;
    pub const CLIPPING_INPUT_PRIMITIVES: c_uint = 0x82F6;
    pub const CLIPPING_OUTPUT_PRIMITIVES: c_uint = 0x82F7;
    pub const CLIP_DEPTH_MODE: c_uint = 0x935D;
    pub const CLIP_DISTANCE0: c_uint = 0x3000;
    pub const CLIP_DISTANCE1: c_uint = 0x3001;
    pub const CLIP_DISTANCE2: c_uint = 0x3002;
    pub const CLIP_DISTANCE3: c_uint = 0x3003;
    pub const CLIP_DISTANCE4: c_uint = 0x3004;
    pub const CLIP_DISTANCE5: c_uint = 0x3005;
    pub const CLIP_DISTANCE6: c_uint = 0x3006;
    pub const CLIP_DISTANCE7: c_uint = 0x3007;
    pub const CLIP_DISTANCE_NV: c_uint = 0x8C7A;
    pub const CLIP_ORIGIN: c_uint = 0x935C;
    pub const COLOR: c_uint = 0x1800;
    pub const COLOR_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x8898;
    pub const COLOR_ARRAY_COUNT_EXT: c_uint = 0x8084;
    pub const COLOR_ARRAY_EXT: c_uint = 0x8076;
    pub const COLOR_ARRAY_POINTER_EXT: c_uint = 0x8090;
    pub const COLOR_ARRAY_SIZE_EXT: c_uint = 0x8081;
    pub const COLOR_ARRAY_STRIDE_EXT: c_uint = 0x8083;
    pub const COLOR_ARRAY_TYPE_EXT: c_uint = 0x8082;
    pub const COLOR_ATTACHMENT0: c_uint = 0x8CE0;
    pub const COLOR_ATTACHMENT0_EXT: c_uint = 0x8CE0;
    pub const COLOR_ATTACHMENT1: c_uint = 0x8CE1;
    pub const COLOR_ATTACHMENT10: c_uint = 0x8CEA;
    pub const COLOR_ATTACHMENT10_EXT: c_uint = 0x8CEA;
    pub const COLOR_ATTACHMENT11: c_uint = 0x8CEB;
    pub const COLOR_ATTACHMENT11_EXT: c_uint = 0x8CEB;
    pub const COLOR_ATTACHMENT12: c_uint = 0x8CEC;
    pub const COLOR_ATTACHMENT12_EXT: c_uint = 0x8CEC;
    pub const COLOR_ATTACHMENT13: c_uint = 0x8CED;
    pub const COLOR_ATTACHMENT13_EXT: c_uint = 0x8CED;
    pub const COLOR_ATTACHMENT14: c_uint = 0x8CEE;
    pub const COLOR_ATTACHMENT14_EXT: c_uint = 0x8CEE;
    pub const COLOR_ATTACHMENT15: c_uint = 0x8CEF;
    pub const COLOR_ATTACHMENT15_EXT: c_uint = 0x8CEF;
    pub const COLOR_ATTACHMENT16: c_uint = 0x8CF0;
    pub const COLOR_ATTACHMENT17: c_uint = 0x8CF1;
    pub const COLOR_ATTACHMENT18: c_uint = 0x8CF2;
    pub const COLOR_ATTACHMENT19: c_uint = 0x8CF3;
    pub const COLOR_ATTACHMENT1_EXT: c_uint = 0x8CE1;
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
    pub const COLOR_ATTACHMENT2_EXT: c_uint = 0x8CE2;
    pub const COLOR_ATTACHMENT3: c_uint = 0x8CE3;
    pub const COLOR_ATTACHMENT30: c_uint = 0x8CFE;
    pub const COLOR_ATTACHMENT31: c_uint = 0x8CFF;
    pub const COLOR_ATTACHMENT3_EXT: c_uint = 0x8CE3;
    pub const COLOR_ATTACHMENT4: c_uint = 0x8CE4;
    pub const COLOR_ATTACHMENT4_EXT: c_uint = 0x8CE4;
    pub const COLOR_ATTACHMENT5: c_uint = 0x8CE5;
    pub const COLOR_ATTACHMENT5_EXT: c_uint = 0x8CE5;
    pub const COLOR_ATTACHMENT6: c_uint = 0x8CE6;
    pub const COLOR_ATTACHMENT6_EXT: c_uint = 0x8CE6;
    pub const COLOR_ATTACHMENT7: c_uint = 0x8CE7;
    pub const COLOR_ATTACHMENT7_EXT: c_uint = 0x8CE7;
    pub const COLOR_ATTACHMENT8: c_uint = 0x8CE8;
    pub const COLOR_ATTACHMENT8_EXT: c_uint = 0x8CE8;
    pub const COLOR_ATTACHMENT9: c_uint = 0x8CE9;
    pub const COLOR_ATTACHMENT9_EXT: c_uint = 0x8CE9;
    pub const COLOR_BUFFER_BIT: c_uint = 0x00004000;
    pub const COLOR_CLEAR_VALUE: c_uint = 0x0C22;
    pub const COLOR_COMPONENTS: c_uint = 0x8283;
    pub const COLOR_ENCODING: c_uint = 0x8296;
    pub const COLOR_LOGIC_OP: c_uint = 0x0BF2;
    pub const COLOR_RENDERABLE: c_uint = 0x8286;
    pub const COLOR_SUM_ARB: c_uint = 0x8458;
    pub const COLOR_WRITEMASK: c_uint = 0x0C23;
    pub const COMMAND_BARRIER_BIT: c_uint = 0x00000040;
    pub const COMMAND_BARRIER_BIT_EXT: c_uint = 0x00000040;
    pub const COMPARE_REF_DEPTH_TO_TEXTURE_EXT: c_uint = 0x884E;
    pub const COMPARE_REF_TO_TEXTURE: c_uint = 0x884E;
    pub const COMPATIBLE_SUBROUTINES: c_uint = 0x8E4B;
    pub const COMPILE_STATUS: c_uint = 0x8B81;
    pub const COMPRESSED_ALPHA_ARB: c_uint = 0x84E9;
    pub const COMPRESSED_INTENSITY_ARB: c_uint = 0x84EC;
    pub const COMPRESSED_LUMINANCE_ALPHA_ARB: c_uint = 0x84EB;
    pub const COMPRESSED_LUMINANCE_ARB: c_uint = 0x84EA;
    pub const COMPRESSED_R11_EAC: c_uint = 0x9270;
    pub const COMPRESSED_RED: c_uint = 0x8225;
    pub const COMPRESSED_RED_RGTC1: c_uint = 0x8DBB;
    pub const COMPRESSED_RG: c_uint = 0x8226;
    pub const COMPRESSED_RG11_EAC: c_uint = 0x9272;
    pub const COMPRESSED_RGB: c_uint = 0x84ED;
    pub const COMPRESSED_RGB8_ETC2: c_uint = 0x9274;
    pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: c_uint = 0x9276;
    pub const COMPRESSED_RGBA: c_uint = 0x84EE;
    pub const COMPRESSED_RGBA8_ETC2_EAC: c_uint = 0x9278;
    pub const COMPRESSED_RGBA_ARB: c_uint = 0x84EE;
    pub const COMPRESSED_RGBA_BPTC_UNORM: c_uint = 0x8E8C;
    pub const COMPRESSED_RGB_ARB: c_uint = 0x84ED;
    pub const COMPRESSED_RGB_BPTC_SIGNED_FLOAT: c_uint = 0x8E8E;
    pub const COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: c_uint = 0x8E8F;
    pub const COMPRESSED_RG_RGTC2: c_uint = 0x8DBD;
    pub const COMPRESSED_SIGNED_R11_EAC: c_uint = 0x9271;
    pub const COMPRESSED_SIGNED_RED_RGTC1: c_uint = 0x8DBC;
    pub const COMPRESSED_SIGNED_RG11_EAC: c_uint = 0x9273;
    pub const COMPRESSED_SIGNED_RG_RGTC2: c_uint = 0x8DBE;
    pub const COMPRESSED_SRGB: c_uint = 0x8C48;
    pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: c_uint = 0x9279;
    pub const COMPRESSED_SRGB8_ETC2: c_uint = 0x9275;
    pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: c_uint = 0x9277;
    pub const COMPRESSED_SRGB_ALPHA: c_uint = 0x8C49;
    pub const COMPRESSED_SRGB_ALPHA_BPTC_UNORM: c_uint = 0x8E8D;
    pub const COMPRESSED_TEXTURE_FORMATS: c_uint = 0x86A3;
    pub const COMPRESSED_TEXTURE_FORMATS_ARB: c_uint = 0x86A3;
    pub const COMPUTE_SHADER: c_uint = 0x91B9;
    pub const COMPUTE_SHADER_BIT: c_uint = 0x00000020;
    pub const COMPUTE_SHADER_INVOCATIONS: c_uint = 0x82F5;
    pub const COMPUTE_SUBROUTINE: c_uint = 0x92ED;
    pub const COMPUTE_SUBROUTINE_UNIFORM: c_uint = 0x92F3;
    pub const COMPUTE_TEXTURE: c_uint = 0x82A0;
    pub const COMPUTE_WORK_GROUP_SIZE: c_uint = 0x8267;
    pub const CONDITION_SATISFIED: c_uint = 0x911C;
    pub const CONSTANT_ALPHA: c_uint = 0x8003;
    pub const CONSTANT_ALPHA_EXT: c_uint = 0x8003;
    pub const CONSTANT_COLOR: c_uint = 0x8001;
    pub const CONSTANT_COLOR_EXT: c_uint = 0x8001;
    pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: c_uint = 0x00000002;
    pub const CONTEXT_CORE_PROFILE_BIT: c_uint = 0x00000001;
    pub const CONTEXT_FLAGS: c_uint = 0x821E;
    pub const CONTEXT_FLAG_DEBUG_BIT: c_uint = 0x00000002;
    pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: c_uint = 0x00000001;
    pub const CONTEXT_FLAG_NO_ERROR_BIT: c_uint = 0x00000008;
    pub const CONTEXT_FLAG_ROBUST_ACCESS_BIT: c_uint = 0x00000004;
    pub const CONTEXT_FLAG_ROBUST_ACCESS_BIT_ARB: c_uint = 0x00000004;
    pub const CONTEXT_LOST: c_uint = 0x0507;
    pub const CONTEXT_PROFILE_MASK: c_uint = 0x9126;
    pub const CONTEXT_RELEASE_BEHAVIOR: c_uint = 0x82FB;
    pub const CONTEXT_RELEASE_BEHAVIOR_FLUSH: c_uint = 0x82FC;
    pub const CONTEXT_ROBUST_ACCESS: c_uint = 0x90F3;
    pub const COORD_REPLACE_NV: c_uint = 0x8862;
    pub const COPY: c_uint = 0x1503;
    pub const COPY_INVERTED: c_uint = 0x150C;
    pub const COPY_READ_BUFFER: c_uint = 0x8F36;
    pub const COPY_READ_BUFFER_BINDING: c_uint = 0x8F36;
    pub const COPY_WRITE_BUFFER: c_uint = 0x8F37;
    pub const COPY_WRITE_BUFFER_BINDING: c_uint = 0x8F37;
    pub const CULL_FACE: c_uint = 0x0B44;
    pub const CULL_FACE_MODE: c_uint = 0x0B45;
    pub const CURRENT_ATTRIB_NV: c_uint = 0x8626;
    pub const CURRENT_MATRIX_ARB: c_uint = 0x8641;
    pub const CURRENT_MATRIX_NV: c_uint = 0x8641;
    pub const CURRENT_MATRIX_STACK_DEPTH_ARB: c_uint = 0x8640;
    pub const CURRENT_MATRIX_STACK_DEPTH_NV: c_uint = 0x8640;
    pub const CURRENT_PROGRAM: c_uint = 0x8B8D;
    pub const CURRENT_QUERY: c_uint = 0x8865;
    pub const CURRENT_QUERY_ARB: c_uint = 0x8865;
    pub const CURRENT_VERTEX_ATTRIB: c_uint = 0x8626;
    pub const CURRENT_VERTEX_ATTRIB_ARB: c_uint = 0x8626;
    pub const CW: c_uint = 0x0900;
    pub const DEBUG_CALLBACK_FUNCTION: c_uint = 0x8244;
    pub const DEBUG_CALLBACK_FUNCTION_ARB: c_uint = 0x8244;
    pub const DEBUG_CALLBACK_USER_PARAM: c_uint = 0x8245;
    pub const DEBUG_CALLBACK_USER_PARAM_ARB: c_uint = 0x8245;
    pub const DEBUG_GROUP_STACK_DEPTH: c_uint = 0x826D;
    pub const DEBUG_LOGGED_MESSAGES: c_uint = 0x9145;
    pub const DEBUG_LOGGED_MESSAGES_ARB: c_uint = 0x9145;
    pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: c_uint = 0x8243;
    pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_ARB: c_uint = 0x8243;
    pub const DEBUG_OUTPUT: c_uint = 0x92E0;
    pub const DEBUG_OUTPUT_SYNCHRONOUS: c_uint = 0x8242;
    pub const DEBUG_OUTPUT_SYNCHRONOUS_ARB: c_uint = 0x8242;
    pub const DEBUG_SEVERITY_HIGH: c_uint = 0x9146;
    pub const DEBUG_SEVERITY_HIGH_ARB: c_uint = 0x9146;
    pub const DEBUG_SEVERITY_LOW: c_uint = 0x9148;
    pub const DEBUG_SEVERITY_LOW_ARB: c_uint = 0x9148;
    pub const DEBUG_SEVERITY_MEDIUM: c_uint = 0x9147;
    pub const DEBUG_SEVERITY_MEDIUM_ARB: c_uint = 0x9147;
    pub const DEBUG_SEVERITY_NOTIFICATION: c_uint = 0x826B;
    pub const DEBUG_SOURCE_API: c_uint = 0x8246;
    pub const DEBUG_SOURCE_API_ARB: c_uint = 0x8246;
    pub const DEBUG_SOURCE_APPLICATION: c_uint = 0x824A;
    pub const DEBUG_SOURCE_APPLICATION_ARB: c_uint = 0x824A;
    pub const DEBUG_SOURCE_OTHER: c_uint = 0x824B;
    pub const DEBUG_SOURCE_OTHER_ARB: c_uint = 0x824B;
    pub const DEBUG_SOURCE_SHADER_COMPILER: c_uint = 0x8248;
    pub const DEBUG_SOURCE_SHADER_COMPILER_ARB: c_uint = 0x8248;
    pub const DEBUG_SOURCE_THIRD_PARTY: c_uint = 0x8249;
    pub const DEBUG_SOURCE_THIRD_PARTY_ARB: c_uint = 0x8249;
    pub const DEBUG_SOURCE_WINDOW_SYSTEM: c_uint = 0x8247;
    pub const DEBUG_SOURCE_WINDOW_SYSTEM_ARB: c_uint = 0x8247;
    pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR: c_uint = 0x824D;
    pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR_ARB: c_uint = 0x824D;
    pub const DEBUG_TYPE_ERROR: c_uint = 0x824C;
    pub const DEBUG_TYPE_ERROR_ARB: c_uint = 0x824C;
    pub const DEBUG_TYPE_MARKER: c_uint = 0x8268;
    pub const DEBUG_TYPE_OTHER: c_uint = 0x8251;
    pub const DEBUG_TYPE_OTHER_ARB: c_uint = 0x8251;
    pub const DEBUG_TYPE_PERFORMANCE: c_uint = 0x8250;
    pub const DEBUG_TYPE_PERFORMANCE_ARB: c_uint = 0x8250;
    pub const DEBUG_TYPE_POP_GROUP: c_uint = 0x826A;
    pub const DEBUG_TYPE_PORTABILITY: c_uint = 0x824F;
    pub const DEBUG_TYPE_PORTABILITY_ARB: c_uint = 0x824F;
    pub const DEBUG_TYPE_PUSH_GROUP: c_uint = 0x8269;
    pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR: c_uint = 0x824E;
    pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR_ARB: c_uint = 0x824E;
    pub const DECR: c_uint = 0x1E03;
    pub const DECR_WRAP: c_uint = 0x8508;
    pub const DELETE_STATUS: c_uint = 0x8B80;
    pub const DEPTH: c_uint = 0x1801;
    pub const DEPTH24_STENCIL8: c_uint = 0x88F0;
    pub const DEPTH32F_STENCIL8: c_uint = 0x8CAD;
    pub const DEPTH_ATTACHMENT: c_uint = 0x8D00;
    pub const DEPTH_ATTACHMENT_EXT: c_uint = 0x8D00;
    pub const DEPTH_BUFFER_BIT: c_uint = 0x00000100;
    pub const DEPTH_CLAMP: c_uint = 0x864F;
    pub const DEPTH_CLEAR_VALUE: c_uint = 0x0B73;
    pub const DEPTH_COMPONENT: c_uint = 0x1902;
    pub const DEPTH_COMPONENT16: c_uint = 0x81A5;
    pub const DEPTH_COMPONENT24: c_uint = 0x81A6;
    pub const DEPTH_COMPONENT32: c_uint = 0x81A7;
    pub const DEPTH_COMPONENT32F: c_uint = 0x8CAC;
    pub const DEPTH_COMPONENTS: c_uint = 0x8284;
    pub const DEPTH_FUNC: c_uint = 0x0B74;
    pub const DEPTH_RANGE: c_uint = 0x0B70;
    pub const DEPTH_RENDERABLE: c_uint = 0x8287;
    pub const DEPTH_STENCIL: c_uint = 0x84F9;
    pub const DEPTH_STENCIL_ATTACHMENT: c_uint = 0x821A;
    pub const DEPTH_STENCIL_TEXTURE_MODE: c_uint = 0x90EA;
    pub const DEPTH_TEST: c_uint = 0x0B71;
    pub const DEPTH_WRITEMASK: c_uint = 0x0B72;
    pub const DISPATCH_INDIRECT_BUFFER: c_uint = 0x90EE;
    pub const DISPATCH_INDIRECT_BUFFER_BINDING: c_uint = 0x90EF;
    pub const DISTANCE_ATTENUATION_EXT: c_uint = 0x8129;
    pub const DISTANCE_ATTENUATION_SGIS: c_uint = 0x8129;
    pub const DITHER: c_uint = 0x0BD0;
    pub const DONT_CARE: c_uint = 0x1100;
    pub const DOUBLE: c_uint = 0x140A;
    pub const DOUBLEBUFFER: c_uint = 0x0C32;
    pub const DOUBLE_MAT2: c_uint = 0x8F46;
    pub const DOUBLE_MAT2_EXT: c_uint = 0x8F46;
    pub const DOUBLE_MAT2x3: c_uint = 0x8F49;
    pub const DOUBLE_MAT2x3_EXT: c_uint = 0x8F49;
    pub const DOUBLE_MAT2x4: c_uint = 0x8F4A;
    pub const DOUBLE_MAT2x4_EXT: c_uint = 0x8F4A;
    pub const DOUBLE_MAT3: c_uint = 0x8F47;
    pub const DOUBLE_MAT3_EXT: c_uint = 0x8F47;
    pub const DOUBLE_MAT3x2: c_uint = 0x8F4B;
    pub const DOUBLE_MAT3x2_EXT: c_uint = 0x8F4B;
    pub const DOUBLE_MAT3x4: c_uint = 0x8F4C;
    pub const DOUBLE_MAT3x4_EXT: c_uint = 0x8F4C;
    pub const DOUBLE_MAT4: c_uint = 0x8F48;
    pub const DOUBLE_MAT4_EXT: c_uint = 0x8F48;
    pub const DOUBLE_MAT4x2: c_uint = 0x8F4D;
    pub const DOUBLE_MAT4x2_EXT: c_uint = 0x8F4D;
    pub const DOUBLE_MAT4x3: c_uint = 0x8F4E;
    pub const DOUBLE_MAT4x3_EXT: c_uint = 0x8F4E;
    pub const DOUBLE_VEC2: c_uint = 0x8FFC;
    pub const DOUBLE_VEC2_EXT: c_uint = 0x8FFC;
    pub const DOUBLE_VEC3: c_uint = 0x8FFD;
    pub const DOUBLE_VEC3_EXT: c_uint = 0x8FFD;
    pub const DOUBLE_VEC4: c_uint = 0x8FFE;
    pub const DOUBLE_VEC4_EXT: c_uint = 0x8FFE;
    pub const DRAW_BUFFER: c_uint = 0x0C01;
    pub const DRAW_BUFFER0: c_uint = 0x8825;
    pub const DRAW_BUFFER0_ARB: c_uint = 0x8825;
    pub const DRAW_BUFFER0_ATI: c_uint = 0x8825;
    pub const DRAW_BUFFER1: c_uint = 0x8826;
    pub const DRAW_BUFFER10: c_uint = 0x882F;
    pub const DRAW_BUFFER10_ARB: c_uint = 0x882F;
    pub const DRAW_BUFFER10_ATI: c_uint = 0x882F;
    pub const DRAW_BUFFER11: c_uint = 0x8830;
    pub const DRAW_BUFFER11_ARB: c_uint = 0x8830;
    pub const DRAW_BUFFER11_ATI: c_uint = 0x8830;
    pub const DRAW_BUFFER12: c_uint = 0x8831;
    pub const DRAW_BUFFER12_ARB: c_uint = 0x8831;
    pub const DRAW_BUFFER12_ATI: c_uint = 0x8831;
    pub const DRAW_BUFFER13: c_uint = 0x8832;
    pub const DRAW_BUFFER13_ARB: c_uint = 0x8832;
    pub const DRAW_BUFFER13_ATI: c_uint = 0x8832;
    pub const DRAW_BUFFER14: c_uint = 0x8833;
    pub const DRAW_BUFFER14_ARB: c_uint = 0x8833;
    pub const DRAW_BUFFER14_ATI: c_uint = 0x8833;
    pub const DRAW_BUFFER15: c_uint = 0x8834;
    pub const DRAW_BUFFER15_ARB: c_uint = 0x8834;
    pub const DRAW_BUFFER15_ATI: c_uint = 0x8834;
    pub const DRAW_BUFFER1_ARB: c_uint = 0x8826;
    pub const DRAW_BUFFER1_ATI: c_uint = 0x8826;
    pub const DRAW_BUFFER2: c_uint = 0x8827;
    pub const DRAW_BUFFER2_ARB: c_uint = 0x8827;
    pub const DRAW_BUFFER2_ATI: c_uint = 0x8827;
    pub const DRAW_BUFFER3: c_uint = 0x8828;
    pub const DRAW_BUFFER3_ARB: c_uint = 0x8828;
    pub const DRAW_BUFFER3_ATI: c_uint = 0x8828;
    pub const DRAW_BUFFER4: c_uint = 0x8829;
    pub const DRAW_BUFFER4_ARB: c_uint = 0x8829;
    pub const DRAW_BUFFER4_ATI: c_uint = 0x8829;
    pub const DRAW_BUFFER5: c_uint = 0x882A;
    pub const DRAW_BUFFER5_ARB: c_uint = 0x882A;
    pub const DRAW_BUFFER5_ATI: c_uint = 0x882A;
    pub const DRAW_BUFFER6: c_uint = 0x882B;
    pub const DRAW_BUFFER6_ARB: c_uint = 0x882B;
    pub const DRAW_BUFFER6_ATI: c_uint = 0x882B;
    pub const DRAW_BUFFER7: c_uint = 0x882C;
    pub const DRAW_BUFFER7_ARB: c_uint = 0x882C;
    pub const DRAW_BUFFER7_ATI: c_uint = 0x882C;
    pub const DRAW_BUFFER8: c_uint = 0x882D;
    pub const DRAW_BUFFER8_ARB: c_uint = 0x882D;
    pub const DRAW_BUFFER8_ATI: c_uint = 0x882D;
    pub const DRAW_BUFFER9: c_uint = 0x882E;
    pub const DRAW_BUFFER9_ARB: c_uint = 0x882E;
    pub const DRAW_BUFFER9_ATI: c_uint = 0x882E;
    pub const DRAW_FRAMEBUFFER: c_uint = 0x8CA9;
    pub const DRAW_FRAMEBUFFER_BINDING: c_uint = 0x8CA6;
    pub const DRAW_FRAMEBUFFER_BINDING_EXT: c_uint = 0x8CA6;
    pub const DRAW_FRAMEBUFFER_EXT: c_uint = 0x8CA9;
    pub const DRAW_INDIRECT_BUFFER: c_uint = 0x8F3F;
    pub const DRAW_INDIRECT_BUFFER_BINDING: c_uint = 0x8F43;
    pub const DST_ALPHA: c_uint = 0x0304;
    pub const DST_COLOR: c_uint = 0x0306;
    pub const DYNAMIC_COPY: c_uint = 0x88EA;
    pub const DYNAMIC_COPY_ARB: c_uint = 0x88EA;
    pub const DYNAMIC_DRAW: c_uint = 0x88E8;
    pub const DYNAMIC_DRAW_ARB: c_uint = 0x88E8;
    pub const DYNAMIC_READ: c_uint = 0x88E9;
    pub const DYNAMIC_READ_ARB: c_uint = 0x88E9;
    pub const DYNAMIC_STORAGE_BIT: c_uint = 0x0100;
    pub const EDGE_FLAG_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x889B;
    pub const EDGE_FLAG_ARRAY_COUNT_EXT: c_uint = 0x808D;
    pub const EDGE_FLAG_ARRAY_EXT: c_uint = 0x8079;
    pub const EDGE_FLAG_ARRAY_POINTER_EXT: c_uint = 0x8093;
    pub const EDGE_FLAG_ARRAY_STRIDE_EXT: c_uint = 0x808C;
    pub const ELEMENT_ARRAY_BARRIER_BIT: c_uint = 0x00000002;
    pub const ELEMENT_ARRAY_BARRIER_BIT_EXT: c_uint = 0x00000002;
    pub const ELEMENT_ARRAY_BUFFER: c_uint = 0x8893;
    pub const ELEMENT_ARRAY_BUFFER_ARB: c_uint = 0x8893;
    pub const ELEMENT_ARRAY_BUFFER_BINDING: c_uint = 0x8895;
    pub const ELEMENT_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x8895;
    pub const EQUAL: c_uint = 0x0202;
    pub const EQUIV: c_uint = 0x1509;
    pub const EXTENSIONS: c_uint = 0x1F03;
    pub const FALSE: c_uchar = 0;
    pub const FASTEST: c_uint = 0x1101;
    pub const FILL: c_uint = 0x1B02;
    pub const FILTER: c_uint = 0x829A;
    pub const FIRST_VERTEX_CONVENTION: c_uint = 0x8E4D;
    pub const FIRST_VERTEX_CONVENTION_EXT: c_uint = 0x8E4D;
    pub const FIXED: c_uint = 0x140C;
    pub const FIXED_ONLY: c_uint = 0x891D;
    pub const FIXED_ONLY_ARB: c_uint = 0x891D;
    pub const FLOAT: c_uint = 0x1406;
    pub const FLOAT_32_UNSIGNED_INT_24_8_REV: c_uint = 0x8DAD;
    pub const FLOAT_MAT2: c_uint = 0x8B5A;
    pub const FLOAT_MAT2_ARB: c_uint = 0x8B5A;
    pub const FLOAT_MAT2x3: c_uint = 0x8B65;
    pub const FLOAT_MAT2x4: c_uint = 0x8B66;
    pub const FLOAT_MAT3: c_uint = 0x8B5B;
    pub const FLOAT_MAT3_ARB: c_uint = 0x8B5B;
    pub const FLOAT_MAT3x2: c_uint = 0x8B67;
    pub const FLOAT_MAT3x4: c_uint = 0x8B68;
    pub const FLOAT_MAT4: c_uint = 0x8B5C;
    pub const FLOAT_MAT4_ARB: c_uint = 0x8B5C;
    pub const FLOAT_MAT4x2: c_uint = 0x8B69;
    pub const FLOAT_MAT4x3: c_uint = 0x8B6A;
    pub const FLOAT_VEC2: c_uint = 0x8B50;
    pub const FLOAT_VEC2_ARB: c_uint = 0x8B50;
    pub const FLOAT_VEC3: c_uint = 0x8B51;
    pub const FLOAT_VEC3_ARB: c_uint = 0x8B51;
    pub const FLOAT_VEC4: c_uint = 0x8B52;
    pub const FLOAT_VEC4_ARB: c_uint = 0x8B52;
    pub const FOG_COORDINATE_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x889D;
    pub const FRACTIONAL_EVEN: c_uint = 0x8E7C;
    pub const FRACTIONAL_ODD: c_uint = 0x8E7B;
    pub const FRAGMENT_INTERPOLATION_OFFSET_BITS: c_uint = 0x8E5D;
    pub const FRAGMENT_SHADER: c_uint = 0x8B30;
    pub const FRAGMENT_SHADER_BIT: c_uint = 0x00000002;
    pub const FRAGMENT_SHADER_DERIVATIVE_HINT: c_uint = 0x8B8B;
    pub const FRAGMENT_SHADER_INVOCATIONS: c_uint = 0x82F4;
    pub const FRAGMENT_SUBROUTINE: c_uint = 0x92EC;
    pub const FRAGMENT_SUBROUTINE_UNIFORM: c_uint = 0x92F2;
    pub const FRAGMENT_TEXTURE: c_uint = 0x829F;
    pub const FRAMEBUFFER: c_uint = 0x8D40;
    pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: c_uint = 0x8215;
    pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: c_uint = 0x8214;
    pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: c_uint = 0x8210;
    pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: c_uint = 0x8211;
    pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: c_uint = 0x8216;
    pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: c_uint = 0x8213;
    pub const FRAMEBUFFER_ATTACHMENT_LAYERED: c_uint = 0x8DA7;
    pub const FRAMEBUFFER_ATTACHMENT_LAYERED_ARB: c_uint = 0x8DA7;
    pub const FRAMEBUFFER_ATTACHMENT_LAYERED_EXT: c_uint = 0x8DA7;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: c_uint = 0x8CD1;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME_EXT: c_uint = 0x8CD1;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: c_uint = 0x8CD0;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE_EXT: c_uint = 0x8CD0;
    pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: c_uint = 0x8212;
    pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: c_uint = 0x8217;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_3D_ZOFFSET_EXT: c_uint = 0x8CD4;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: c_uint = 0x8CD3;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE_EXT: c_uint = 0x8CD3;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: c_uint = 0x8CD4;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER_EXT: c_uint = 0x8CD4;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: c_uint = 0x8CD2;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL_EXT: c_uint = 0x8CD2;
    pub const FRAMEBUFFER_BARRIER_BIT: c_uint = 0x00000400;
    pub const FRAMEBUFFER_BARRIER_BIT_EXT: c_uint = 0x00000400;
    pub const FRAMEBUFFER_BINDING: c_uint = 0x8CA6;
    pub const FRAMEBUFFER_BINDING_EXT: c_uint = 0x8CA6;
    pub const FRAMEBUFFER_BLEND: c_uint = 0x828B;
    pub const FRAMEBUFFER_COMPLETE: c_uint = 0x8CD5;
    pub const FRAMEBUFFER_COMPLETE_EXT: c_uint = 0x8CD5;
    pub const FRAMEBUFFER_DEFAULT: c_uint = 0x8218;
    pub const FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: c_uint = 0x9314;
    pub const FRAMEBUFFER_DEFAULT_HEIGHT: c_uint = 0x9311;
    pub const FRAMEBUFFER_DEFAULT_LAYERS: c_uint = 0x9312;
    pub const FRAMEBUFFER_DEFAULT_SAMPLES: c_uint = 0x9313;
    pub const FRAMEBUFFER_DEFAULT_WIDTH: c_uint = 0x9310;
    pub const FRAMEBUFFER_EXT: c_uint = 0x8D40;
    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: c_uint = 0x8CD6;
    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT_EXT: c_uint = 0x8CD6;
    pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS_EXT: c_uint = 0x8CD9;
    pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: c_uint = 0x8CDB;
    pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER_EXT: c_uint = 0x8CDB;
    pub const FRAMEBUFFER_INCOMPLETE_FORMATS_EXT: c_uint = 0x8CDA;
    pub const FRAMEBUFFER_INCOMPLETE_LAYER_COUNT_ARB: c_uint = 0x8DA9;
    pub const FRAMEBUFFER_INCOMPLETE_LAYER_COUNT_EXT: c_uint = 0x8DA9;
    pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: c_uint = 0x8DA8;
    pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS_ARB: c_uint = 0x8DA8;
    pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS_EXT: c_uint = 0x8DA8;
    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: c_uint = 0x8CD7;
    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT_EXT: c_uint = 0x8CD7;
    pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: c_uint = 0x8D56;
    pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_EXT: c_uint = 0x8D56;
    pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: c_uint = 0x8CDC;
    pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER_EXT: c_uint = 0x8CDC;
    pub const FRAMEBUFFER_RENDERABLE: c_uint = 0x8289;
    pub const FRAMEBUFFER_RENDERABLE_LAYERED: c_uint = 0x828A;
    pub const FRAMEBUFFER_SRGB: c_uint = 0x8DB9;
    pub const FRAMEBUFFER_UNDEFINED: c_uint = 0x8219;
    pub const FRAMEBUFFER_UNSUPPORTED: c_uint = 0x8CDD;
    pub const FRAMEBUFFER_UNSUPPORTED_EXT: c_uint = 0x8CDD;
    pub const FRONT: c_uint = 0x0404;
    pub const FRONT_AND_BACK: c_uint = 0x0408;
    pub const FRONT_FACE: c_uint = 0x0B46;
    pub const FRONT_LEFT: c_uint = 0x0400;
    pub const FRONT_RIGHT: c_uint = 0x0401;
    pub const FULL_SUPPORT: c_uint = 0x82B7;
    pub const FUNC_ADD: c_uint = 0x8006;
    pub const FUNC_ADD_EXT: c_uint = 0x8006;
    pub const FUNC_REVERSE_SUBTRACT: c_uint = 0x800B;
    pub const FUNC_SUBTRACT: c_uint = 0x800A;
    pub const GENERIC_ATTRIB_NV: c_uint = 0x8C7D;
    pub const GEOMETRY_INPUT_TYPE: c_uint = 0x8917;
    pub const GEOMETRY_INPUT_TYPE_ARB: c_uint = 0x8DDB;
    pub const GEOMETRY_INPUT_TYPE_EXT: c_uint = 0x8DDB;
    pub const GEOMETRY_OUTPUT_TYPE: c_uint = 0x8918;
    pub const GEOMETRY_OUTPUT_TYPE_ARB: c_uint = 0x8DDC;
    pub const GEOMETRY_OUTPUT_TYPE_EXT: c_uint = 0x8DDC;
    pub const GEOMETRY_PROGRAM_NV: c_uint = 0x8C26;
    pub const GEOMETRY_SHADER: c_uint = 0x8DD9;
    pub const GEOMETRY_SHADER_ARB: c_uint = 0x8DD9;
    pub const GEOMETRY_SHADER_BIT: c_uint = 0x00000004;
    pub const GEOMETRY_SHADER_EXT: c_uint = 0x8DD9;
    pub const GEOMETRY_SHADER_INVOCATIONS: c_uint = 0x887F;
    pub const GEOMETRY_SHADER_PRIMITIVES_EMITTED: c_uint = 0x82F3;
    pub const GEOMETRY_SUBROUTINE: c_uint = 0x92EB;
    pub const GEOMETRY_SUBROUTINE_UNIFORM: c_uint = 0x92F1;
    pub const GEOMETRY_TEXTURE: c_uint = 0x829E;
    pub const GEOMETRY_VERTICES_OUT: c_uint = 0x8916;
    pub const GEOMETRY_VERTICES_OUT_ARB: c_uint = 0x8DDA;
    pub const GEOMETRY_VERTICES_OUT_EXT: c_uint = 0x8DDA;
    pub const GEQUAL: c_uint = 0x0206;
    pub const GET_TEXTURE_IMAGE_FORMAT: c_uint = 0x8291;
    pub const GET_TEXTURE_IMAGE_TYPE: c_uint = 0x8292;
    pub const GREATER: c_uint = 0x0204;
    pub const GREEN: c_uint = 0x1904;
    pub const GREEN_INTEGER: c_uint = 0x8D95;
    pub const GREEN_INTEGER_EXT: c_uint = 0x8D95;
    pub const GUILTY_CONTEXT_RESET: c_uint = 0x8253;
    pub const GUILTY_CONTEXT_RESET_ARB: c_uint = 0x8253;
    pub const HALF_FLOAT: c_uint = 0x140B;
    pub const HIGH_FLOAT: c_uint = 0x8DF2;
    pub const HIGH_INT: c_uint = 0x8DF5;
    pub const IDENTITY_NV: c_uint = 0x862A;
    pub const IMAGE_1D: c_uint = 0x904C;
    pub const IMAGE_1D_ARRAY: c_uint = 0x9052;
    pub const IMAGE_1D_ARRAY_EXT: c_uint = 0x9052;
    pub const IMAGE_1D_EXT: c_uint = 0x904C;
    pub const IMAGE_2D: c_uint = 0x904D;
    pub const IMAGE_2D_ARRAY: c_uint = 0x9053;
    pub const IMAGE_2D_ARRAY_EXT: c_uint = 0x9053;
    pub const IMAGE_2D_EXT: c_uint = 0x904D;
    pub const IMAGE_2D_MULTISAMPLE: c_uint = 0x9055;
    pub const IMAGE_2D_MULTISAMPLE_ARRAY: c_uint = 0x9056;
    pub const IMAGE_2D_MULTISAMPLE_ARRAY_EXT: c_uint = 0x9056;
    pub const IMAGE_2D_MULTISAMPLE_EXT: c_uint = 0x9055;
    pub const IMAGE_2D_RECT: c_uint = 0x904F;
    pub const IMAGE_2D_RECT_EXT: c_uint = 0x904F;
    pub const IMAGE_3D: c_uint = 0x904E;
    pub const IMAGE_3D_EXT: c_uint = 0x904E;
    pub const IMAGE_BINDING_ACCESS: c_uint = 0x8F3E;
    pub const IMAGE_BINDING_ACCESS_EXT: c_uint = 0x8F3E;
    pub const IMAGE_BINDING_FORMAT: c_uint = 0x906E;
    pub const IMAGE_BINDING_FORMAT_EXT: c_uint = 0x906E;
    pub const IMAGE_BINDING_LAYER: c_uint = 0x8F3D;
    pub const IMAGE_BINDING_LAYERED: c_uint = 0x8F3C;
    pub const IMAGE_BINDING_LAYERED_EXT: c_uint = 0x8F3C;
    pub const IMAGE_BINDING_LAYER_EXT: c_uint = 0x8F3D;
    pub const IMAGE_BINDING_LEVEL: c_uint = 0x8F3B;
    pub const IMAGE_BINDING_LEVEL_EXT: c_uint = 0x8F3B;
    pub const IMAGE_BINDING_NAME: c_uint = 0x8F3A;
    pub const IMAGE_BINDING_NAME_EXT: c_uint = 0x8F3A;
    pub const IMAGE_BUFFER: c_uint = 0x9051;
    pub const IMAGE_BUFFER_EXT: c_uint = 0x9051;
    pub const IMAGE_CLASS_10_10_10_2: c_uint = 0x82C3;
    pub const IMAGE_CLASS_11_11_10: c_uint = 0x82C2;
    pub const IMAGE_CLASS_1_X_16: c_uint = 0x82BE;
    pub const IMAGE_CLASS_1_X_32: c_uint = 0x82BB;
    pub const IMAGE_CLASS_1_X_8: c_uint = 0x82C1;
    pub const IMAGE_CLASS_2_X_16: c_uint = 0x82BD;
    pub const IMAGE_CLASS_2_X_32: c_uint = 0x82BA;
    pub const IMAGE_CLASS_2_X_8: c_uint = 0x82C0;
    pub const IMAGE_CLASS_4_X_16: c_uint = 0x82BC;
    pub const IMAGE_CLASS_4_X_32: c_uint = 0x82B9;
    pub const IMAGE_CLASS_4_X_8: c_uint = 0x82BF;
    pub const IMAGE_COMPATIBILITY_CLASS: c_uint = 0x82A8;
    pub const IMAGE_CUBE: c_uint = 0x9050;
    pub const IMAGE_CUBE_EXT: c_uint = 0x9050;
    pub const IMAGE_CUBE_MAP_ARRAY: c_uint = 0x9054;
    pub const IMAGE_CUBE_MAP_ARRAY_EXT: c_uint = 0x9054;
    pub const IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: c_uint = 0x90C9;
    pub const IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: c_uint = 0x90C8;
    pub const IMAGE_FORMAT_COMPATIBILITY_TYPE: c_uint = 0x90C7;
    pub const IMAGE_PIXEL_FORMAT: c_uint = 0x82A9;
    pub const IMAGE_PIXEL_TYPE: c_uint = 0x82AA;
    pub const IMAGE_TEXEL_SIZE: c_uint = 0x82A7;
    pub const IMPLEMENTATION_COLOR_READ_FORMAT: c_uint = 0x8B9B;
    pub const IMPLEMENTATION_COLOR_READ_TYPE: c_uint = 0x8B9A;
    pub const INCR: c_uint = 0x1E02;
    pub const INCR_WRAP: c_uint = 0x8507;
    pub const INDEX_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x8899;
    pub const INDEX_ARRAY_COUNT_EXT: c_uint = 0x8087;
    pub const INDEX_ARRAY_EXT: c_uint = 0x8077;
    pub const INDEX_ARRAY_POINTER_EXT: c_uint = 0x8091;
    pub const INDEX_ARRAY_STRIDE_EXT: c_uint = 0x8086;
    pub const INDEX_ARRAY_TYPE_EXT: c_uint = 0x8085;
    pub const INFO_LOG_LENGTH: c_uint = 0x8B84;
    pub const INNOCENT_CONTEXT_RESET: c_uint = 0x8254;
    pub const INNOCENT_CONTEXT_RESET_ARB: c_uint = 0x8254;
    pub const INT: c_uint = 0x1404;
    pub const INTENSITY16I_EXT: c_uint = 0x8D8B;
    pub const INTENSITY16UI_EXT: c_uint = 0x8D79;
    pub const INTENSITY32I_EXT: c_uint = 0x8D85;
    pub const INTENSITY32UI_EXT: c_uint = 0x8D73;
    pub const INTENSITY8I_EXT: c_uint = 0x8D91;
    pub const INTENSITY8UI_EXT: c_uint = 0x8D7F;
    pub const INTERLEAVED_ATTRIBS: c_uint = 0x8C8C;
    pub const INTERLEAVED_ATTRIBS_EXT: c_uint = 0x8C8C;
    pub const INTERLEAVED_ATTRIBS_NV: c_uint = 0x8C8C;
    pub const INTERNALFORMAT_ALPHA_SIZE: c_uint = 0x8274;
    pub const INTERNALFORMAT_ALPHA_TYPE: c_uint = 0x827B;
    pub const INTERNALFORMAT_BLUE_SIZE: c_uint = 0x8273;
    pub const INTERNALFORMAT_BLUE_TYPE: c_uint = 0x827A;
    pub const INTERNALFORMAT_DEPTH_SIZE: c_uint = 0x8275;
    pub const INTERNALFORMAT_DEPTH_TYPE: c_uint = 0x827C;
    pub const INTERNALFORMAT_GREEN_SIZE: c_uint = 0x8272;
    pub const INTERNALFORMAT_GREEN_TYPE: c_uint = 0x8279;
    pub const INTERNALFORMAT_PREFERRED: c_uint = 0x8270;
    pub const INTERNALFORMAT_RED_SIZE: c_uint = 0x8271;
    pub const INTERNALFORMAT_RED_TYPE: c_uint = 0x8278;
    pub const INTERNALFORMAT_SHARED_SIZE: c_uint = 0x8277;
    pub const INTERNALFORMAT_STENCIL_SIZE: c_uint = 0x8276;
    pub const INTERNALFORMAT_STENCIL_TYPE: c_uint = 0x827D;
    pub const INTERNALFORMAT_SUPPORTED: c_uint = 0x826F;
    pub const INT_2_10_10_10_REV: c_uint = 0x8D9F;
    pub const INT_IMAGE_1D: c_uint = 0x9057;
    pub const INT_IMAGE_1D_ARRAY: c_uint = 0x905D;
    pub const INT_IMAGE_1D_ARRAY_EXT: c_uint = 0x905D;
    pub const INT_IMAGE_1D_EXT: c_uint = 0x9057;
    pub const INT_IMAGE_2D: c_uint = 0x9058;
    pub const INT_IMAGE_2D_ARRAY: c_uint = 0x905E;
    pub const INT_IMAGE_2D_ARRAY_EXT: c_uint = 0x905E;
    pub const INT_IMAGE_2D_EXT: c_uint = 0x9058;
    pub const INT_IMAGE_2D_MULTISAMPLE: c_uint = 0x9060;
    pub const INT_IMAGE_2D_MULTISAMPLE_ARRAY: c_uint = 0x9061;
    pub const INT_IMAGE_2D_MULTISAMPLE_ARRAY_EXT: c_uint = 0x9061;
    pub const INT_IMAGE_2D_MULTISAMPLE_EXT: c_uint = 0x9060;
    pub const INT_IMAGE_2D_RECT: c_uint = 0x905A;
    pub const INT_IMAGE_2D_RECT_EXT: c_uint = 0x905A;
    pub const INT_IMAGE_3D: c_uint = 0x9059;
    pub const INT_IMAGE_3D_EXT: c_uint = 0x9059;
    pub const INT_IMAGE_BUFFER: c_uint = 0x905C;
    pub const INT_IMAGE_BUFFER_EXT: c_uint = 0x905C;
    pub const INT_IMAGE_CUBE: c_uint = 0x905B;
    pub const INT_IMAGE_CUBE_EXT: c_uint = 0x905B;
    pub const INT_IMAGE_CUBE_MAP_ARRAY: c_uint = 0x905F;
    pub const INT_IMAGE_CUBE_MAP_ARRAY_EXT: c_uint = 0x905F;
    pub const INT_SAMPLER_1D: c_uint = 0x8DC9;
    pub const INT_SAMPLER_1D_ARRAY: c_uint = 0x8DCE;
    pub const INT_SAMPLER_1D_ARRAY_EXT: c_uint = 0x8DCE;
    pub const INT_SAMPLER_1D_EXT: c_uint = 0x8DC9;
    pub const INT_SAMPLER_2D: c_uint = 0x8DCA;
    pub const INT_SAMPLER_2D_ARRAY: c_uint = 0x8DCF;
    pub const INT_SAMPLER_2D_ARRAY_EXT: c_uint = 0x8DCF;
    pub const INT_SAMPLER_2D_EXT: c_uint = 0x8DCA;
    pub const INT_SAMPLER_2D_MULTISAMPLE: c_uint = 0x9109;
    pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: c_uint = 0x910C;
    pub const INT_SAMPLER_2D_RECT: c_uint = 0x8DCD;
    pub const INT_SAMPLER_2D_RECT_EXT: c_uint = 0x8DCD;
    pub const INT_SAMPLER_3D: c_uint = 0x8DCB;
    pub const INT_SAMPLER_3D_EXT: c_uint = 0x8DCB;
    pub const INT_SAMPLER_BUFFER: c_uint = 0x8DD0;
    pub const INT_SAMPLER_BUFFER_EXT: c_uint = 0x8DD0;
    pub const INT_SAMPLER_CUBE: c_uint = 0x8DCC;
    pub const INT_SAMPLER_CUBE_EXT: c_uint = 0x8DCC;
    pub const INT_SAMPLER_CUBE_MAP_ARRAY: c_uint = 0x900E;
    pub const INT_SAMPLER_RENDERBUFFER_NV: c_uint = 0x8E57;
    pub const INT_VEC2: c_uint = 0x8B53;
    pub const INT_VEC2_ARB: c_uint = 0x8B53;
    pub const INT_VEC3: c_uint = 0x8B54;
    pub const INT_VEC3_ARB: c_uint = 0x8B54;
    pub const INT_VEC4: c_uint = 0x8B55;
    pub const INT_VEC4_ARB: c_uint = 0x8B55;
    pub const INVALID_ENUM: c_uint = 0x0500;
    pub const INVALID_FRAMEBUFFER_OPERATION: c_uint = 0x0506;
    pub const INVALID_FRAMEBUFFER_OPERATION_EXT: c_uint = 0x0506;
    pub const INVALID_INDEX: c_uint = 0xFFFFFFFF;
    pub const INVALID_OPERATION: c_uint = 0x0502;
    pub const INVALID_VALUE: c_uint = 0x0501;
    pub const INVERSE_NV: c_uint = 0x862B;
    pub const INVERSE_TRANSPOSE_NV: c_uint = 0x862D;
    pub const INVERT: c_uint = 0x150A;
    pub const ISOLINES: c_uint = 0x8E7A;
    pub const IS_PER_PATCH: c_uint = 0x92E7;
    pub const IS_ROW_MAJOR: c_uint = 0x9300;
    pub const KEEP: c_uint = 0x1E00;
    pub const LAST_VERTEX_CONVENTION: c_uint = 0x8E4E;
    pub const LAST_VERTEX_CONVENTION_EXT: c_uint = 0x8E4E;
    pub const LAYER_NV: c_uint = 0x8DAA;
    pub const LAYER_PROVOKING_VERTEX: c_uint = 0x825E;
    pub const LEFT: c_uint = 0x0406;
    pub const LEQUAL: c_uint = 0x0203;
    pub const LESS: c_uint = 0x0201;
    pub const LINE: c_uint = 0x1B01;
    pub const LINEAR: c_uint = 0x2601;
    pub const LINEAR_MIPMAP_LINEAR: c_uint = 0x2703;
    pub const LINEAR_MIPMAP_NEAREST: c_uint = 0x2701;
    pub const LINES: c_uint = 0x0001;
    pub const LINES_ADJACENCY: c_uint = 0x000A;
    pub const LINES_ADJACENCY_ARB: c_uint = 0x000A;
    pub const LINES_ADJACENCY_EXT: c_uint = 0x000A;
    pub const LINE_LOOP: c_uint = 0x0002;
    pub const LINE_SMOOTH: c_uint = 0x0B20;
    pub const LINE_SMOOTH_HINT: c_uint = 0x0C52;
    pub const LINE_STRIP: c_uint = 0x0003;
    pub const LINE_STRIP_ADJACENCY: c_uint = 0x000B;
    pub const LINE_STRIP_ADJACENCY_ARB: c_uint = 0x000B;
    pub const LINE_STRIP_ADJACENCY_EXT: c_uint = 0x000B;
    pub const LINE_WIDTH: c_uint = 0x0B21;
    pub const LINE_WIDTH_GRANULARITY: c_uint = 0x0B23;
    pub const LINE_WIDTH_RANGE: c_uint = 0x0B22;
    pub const LINK_STATUS: c_uint = 0x8B82;
    pub const LOCATION: c_uint = 0x930E;
    pub const LOCATION_COMPONENT: c_uint = 0x934A;
    pub const LOCATION_INDEX: c_uint = 0x930F;
    pub const LOGIC_OP_MODE: c_uint = 0x0BF0;
    pub const LOSE_CONTEXT_ON_RESET: c_uint = 0x8252;
    pub const LOSE_CONTEXT_ON_RESET_ARB: c_uint = 0x8252;
    pub const LOWER_LEFT: c_uint = 0x8CA1;
    pub const LOW_FLOAT: c_uint = 0x8DF0;
    pub const LOW_INT: c_uint = 0x8DF3;
    pub const LUMINANCE16F_EXT: c_uint = 0x881E;
    pub const LUMINANCE16I_EXT: c_uint = 0x8D8C;
    pub const LUMINANCE16UI_EXT: c_uint = 0x8D7A;
    pub const LUMINANCE32F_EXT: c_uint = 0x8818;
    pub const LUMINANCE32I_EXT: c_uint = 0x8D86;
    pub const LUMINANCE32UI_EXT: c_uint = 0x8D74;
    pub const LUMINANCE8I_EXT: c_uint = 0x8D92;
    pub const LUMINANCE8UI_EXT: c_uint = 0x8D80;
    pub const LUMINANCE8_ALPHA8_EXT: c_uint = 0x8045;
    pub const LUMINANCE8_EXT: c_uint = 0x8040;
    pub const LUMINANCE_ALPHA16F_EXT: c_uint = 0x881F;
    pub const LUMINANCE_ALPHA16I_EXT: c_uint = 0x8D8D;
    pub const LUMINANCE_ALPHA16UI_EXT: c_uint = 0x8D7B;
    pub const LUMINANCE_ALPHA32F_EXT: c_uint = 0x8819;
    pub const LUMINANCE_ALPHA32I_EXT: c_uint = 0x8D87;
    pub const LUMINANCE_ALPHA32UI_EXT: c_uint = 0x8D75;
    pub const LUMINANCE_ALPHA8I_EXT: c_uint = 0x8D93;
    pub const LUMINANCE_ALPHA8UI_EXT: c_uint = 0x8D81;
    pub const LUMINANCE_ALPHA_INTEGER_EXT: c_uint = 0x8D9D;
    pub const LUMINANCE_INTEGER_EXT: c_uint = 0x8D9C;
    pub const MAJOR_VERSION: c_uint = 0x821B;
    pub const MANUAL_GENERATE_MIPMAP: c_uint = 0x8294;
    pub const MAP1_VERTEX_ATTRIB0_4_NV: c_uint = 0x8660;
    pub const MAP1_VERTEX_ATTRIB10_4_NV: c_uint = 0x866A;
    pub const MAP1_VERTEX_ATTRIB11_4_NV: c_uint = 0x866B;
    pub const MAP1_VERTEX_ATTRIB12_4_NV: c_uint = 0x866C;
    pub const MAP1_VERTEX_ATTRIB13_4_NV: c_uint = 0x866D;
    pub const MAP1_VERTEX_ATTRIB14_4_NV: c_uint = 0x866E;
    pub const MAP1_VERTEX_ATTRIB15_4_NV: c_uint = 0x866F;
    pub const MAP1_VERTEX_ATTRIB1_4_NV: c_uint = 0x8661;
    pub const MAP1_VERTEX_ATTRIB2_4_NV: c_uint = 0x8662;
    pub const MAP1_VERTEX_ATTRIB3_4_NV: c_uint = 0x8663;
    pub const MAP1_VERTEX_ATTRIB4_4_NV: c_uint = 0x8664;
    pub const MAP1_VERTEX_ATTRIB5_4_NV: c_uint = 0x8665;
    pub const MAP1_VERTEX_ATTRIB6_4_NV: c_uint = 0x8666;
    pub const MAP1_VERTEX_ATTRIB7_4_NV: c_uint = 0x8667;
    pub const MAP1_VERTEX_ATTRIB8_4_NV: c_uint = 0x8668;
    pub const MAP1_VERTEX_ATTRIB9_4_NV: c_uint = 0x8669;
    pub const MAP2_VERTEX_ATTRIB0_4_NV: c_uint = 0x8670;
    pub const MAP2_VERTEX_ATTRIB10_4_NV: c_uint = 0x867A;
    pub const MAP2_VERTEX_ATTRIB11_4_NV: c_uint = 0x867B;
    pub const MAP2_VERTEX_ATTRIB12_4_NV: c_uint = 0x867C;
    pub const MAP2_VERTEX_ATTRIB13_4_NV: c_uint = 0x867D;
    pub const MAP2_VERTEX_ATTRIB14_4_NV: c_uint = 0x867E;
    pub const MAP2_VERTEX_ATTRIB15_4_NV: c_uint = 0x867F;
    pub const MAP2_VERTEX_ATTRIB1_4_NV: c_uint = 0x8671;
    pub const MAP2_VERTEX_ATTRIB2_4_NV: c_uint = 0x8672;
    pub const MAP2_VERTEX_ATTRIB3_4_NV: c_uint = 0x8673;
    pub const MAP2_VERTEX_ATTRIB4_4_NV: c_uint = 0x8674;
    pub const MAP2_VERTEX_ATTRIB5_4_NV: c_uint = 0x8675;
    pub const MAP2_VERTEX_ATTRIB6_4_NV: c_uint = 0x8676;
    pub const MAP2_VERTEX_ATTRIB7_4_NV: c_uint = 0x8677;
    pub const MAP2_VERTEX_ATTRIB8_4_NV: c_uint = 0x8678;
    pub const MAP2_VERTEX_ATTRIB9_4_NV: c_uint = 0x8679;
    pub const MAP_COHERENT_BIT: c_uint = 0x0080;
    pub const MAP_FLUSH_EXPLICIT_BIT: c_uint = 0x0010;
    pub const MAP_INVALIDATE_BUFFER_BIT: c_uint = 0x0008;
    pub const MAP_INVALIDATE_RANGE_BIT: c_uint = 0x0004;
    pub const MAP_PERSISTENT_BIT: c_uint = 0x0040;
    pub const MAP_READ_BIT: c_uint = 0x0001;
    pub const MAP_UNSYNCHRONIZED_BIT: c_uint = 0x0020;
    pub const MAP_WRITE_BIT: c_uint = 0x0002;
    pub const MATRIX0_ARB: c_uint = 0x88C0;
    pub const MATRIX0_NV: c_uint = 0x8630;
    pub const MATRIX10_ARB: c_uint = 0x88CA;
    pub const MATRIX11_ARB: c_uint = 0x88CB;
    pub const MATRIX12_ARB: c_uint = 0x88CC;
    pub const MATRIX13_ARB: c_uint = 0x88CD;
    pub const MATRIX14_ARB: c_uint = 0x88CE;
    pub const MATRIX15_ARB: c_uint = 0x88CF;
    pub const MATRIX16_ARB: c_uint = 0x88D0;
    pub const MATRIX17_ARB: c_uint = 0x88D1;
    pub const MATRIX18_ARB: c_uint = 0x88D2;
    pub const MATRIX19_ARB: c_uint = 0x88D3;
    pub const MATRIX1_ARB: c_uint = 0x88C1;
    pub const MATRIX1_NV: c_uint = 0x8631;
    pub const MATRIX20_ARB: c_uint = 0x88D4;
    pub const MATRIX21_ARB: c_uint = 0x88D5;
    pub const MATRIX22_ARB: c_uint = 0x88D6;
    pub const MATRIX23_ARB: c_uint = 0x88D7;
    pub const MATRIX24_ARB: c_uint = 0x88D8;
    pub const MATRIX25_ARB: c_uint = 0x88D9;
    pub const MATRIX26_ARB: c_uint = 0x88DA;
    pub const MATRIX27_ARB: c_uint = 0x88DB;
    pub const MATRIX28_ARB: c_uint = 0x88DC;
    pub const MATRIX29_ARB: c_uint = 0x88DD;
    pub const MATRIX2_ARB: c_uint = 0x88C2;
    pub const MATRIX2_NV: c_uint = 0x8632;
    pub const MATRIX30_ARB: c_uint = 0x88DE;
    pub const MATRIX31_ARB: c_uint = 0x88DF;
    pub const MATRIX3_ARB: c_uint = 0x88C3;
    pub const MATRIX3_NV: c_uint = 0x8633;
    pub const MATRIX4_ARB: c_uint = 0x88C4;
    pub const MATRIX4_NV: c_uint = 0x8634;
    pub const MATRIX5_ARB: c_uint = 0x88C5;
    pub const MATRIX5_NV: c_uint = 0x8635;
    pub const MATRIX6_ARB: c_uint = 0x88C6;
    pub const MATRIX6_NV: c_uint = 0x8636;
    pub const MATRIX7_ARB: c_uint = 0x88C7;
    pub const MATRIX7_NV: c_uint = 0x8637;
    pub const MATRIX8_ARB: c_uint = 0x88C8;
    pub const MATRIX9_ARB: c_uint = 0x88C9;
    pub const MATRIX_STRIDE: c_uint = 0x92FF;
    pub const MAX: c_uint = 0x8008;
    pub const MAX_3D_TEXTURE_SIZE: c_uint = 0x8073;
    pub const MAX_3D_TEXTURE_SIZE_EXT: c_uint = 0x8073;
    pub const MAX_ARRAY_TEXTURE_LAYERS: c_uint = 0x88FF;
    pub const MAX_ARRAY_TEXTURE_LAYERS_EXT: c_uint = 0x88FF;
    pub const MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: c_uint = 0x92DC;
    pub const MAX_ATOMIC_COUNTER_BUFFER_SIZE: c_uint = 0x92D8;
    pub const MAX_CLIP_DISTANCES: c_uint = 0x0D32;
    pub const MAX_COLOR_ATTACHMENTS: c_uint = 0x8CDF;
    pub const MAX_COLOR_ATTACHMENTS_EXT: c_uint = 0x8CDF;
    pub const MAX_COLOR_TEXTURE_SAMPLES: c_uint = 0x910E;
    pub const MAX_COMBINED_ATOMIC_COUNTERS: c_uint = 0x92D7;
    pub const MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92D1;
    pub const MAX_COMBINED_CLIP_AND_CULL_DISTANCES: c_uint = 0x82FA;
    pub const MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: c_uint = 0x8266;
    pub const MAX_COMBINED_DIMENSIONS: c_uint = 0x8282;
    pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: c_uint = 0x8A33;
    pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: c_uint = 0x8A32;
    pub const MAX_COMBINED_IMAGE_UNIFORMS: c_uint = 0x90CF;
    pub const MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: c_uint = 0x8F39;
    pub const MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS_EXT: c_uint = 0x8F39;
    pub const MAX_COMBINED_SHADER_OUTPUT_RESOURCES: c_uint = 0x8F39;
    pub const MAX_COMBINED_SHADER_STORAGE_BLOCKS: c_uint = 0x90DC;
    pub const MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: c_uint = 0x8E1E;
    pub const MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: c_uint = 0x8E1F;
    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: c_uint = 0x8B4D;
    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS_ARB: c_uint = 0x8B4D;
    pub const MAX_COMBINED_UNIFORM_BLOCKS: c_uint = 0x8A2E;
    pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: c_uint = 0x8A31;
    pub const MAX_COMPUTE_ATOMIC_COUNTERS: c_uint = 0x8265;
    pub const MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: c_uint = 0x8264;
    pub const MAX_COMPUTE_IMAGE_UNIFORMS: c_uint = 0x91BD;
    pub const MAX_COMPUTE_SHADER_STORAGE_BLOCKS: c_uint = 0x90DB;
    pub const MAX_COMPUTE_SHARED_MEMORY_SIZE: c_uint = 0x8262;
    pub const MAX_COMPUTE_TEXTURE_IMAGE_UNITS: c_uint = 0x91BC;
    pub const MAX_COMPUTE_UNIFORM_BLOCKS: c_uint = 0x91BB;
    pub const MAX_COMPUTE_UNIFORM_COMPONENTS: c_uint = 0x8263;
    pub const MAX_COMPUTE_WORK_GROUP_COUNT: c_uint = 0x91BE;
    pub const MAX_COMPUTE_WORK_GROUP_INVOCATIONS: c_uint = 0x90EB;
    pub const MAX_COMPUTE_WORK_GROUP_SIZE: c_uint = 0x91BF;
    pub const MAX_CUBE_MAP_TEXTURE_SIZE: c_uint = 0x851C;
    pub const MAX_CULL_DISTANCES: c_uint = 0x82F9;
    pub const MAX_DEBUG_GROUP_STACK_DEPTH: c_uint = 0x826C;
    pub const MAX_DEBUG_LOGGED_MESSAGES: c_uint = 0x9144;
    pub const MAX_DEBUG_LOGGED_MESSAGES_ARB: c_uint = 0x9144;
    pub const MAX_DEBUG_MESSAGE_LENGTH: c_uint = 0x9143;
    pub const MAX_DEBUG_MESSAGE_LENGTH_ARB: c_uint = 0x9143;
    pub const MAX_DEPTH: c_uint = 0x8280;
    pub const MAX_DEPTH_TEXTURE_SAMPLES: c_uint = 0x910F;
    pub const MAX_DRAW_BUFFERS: c_uint = 0x8824;
    pub const MAX_DRAW_BUFFERS_ARB: c_uint = 0x8824;
    pub const MAX_DRAW_BUFFERS_ATI: c_uint = 0x8824;
    pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: c_uint = 0x88FC;
    pub const MAX_ELEMENTS_INDICES: c_uint = 0x80E9;
    pub const MAX_ELEMENTS_INDICES_EXT: c_uint = 0x80E9;
    pub const MAX_ELEMENTS_VERTICES: c_uint = 0x80E8;
    pub const MAX_ELEMENTS_VERTICES_EXT: c_uint = 0x80E8;
    pub const MAX_ELEMENT_INDEX: c_uint = 0x8D6B;
    pub const MAX_EXT: c_uint = 0x8008;
    pub const MAX_FRAGMENT_ATOMIC_COUNTERS: c_uint = 0x92D6;
    pub const MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92D0;
    pub const MAX_FRAGMENT_IMAGE_UNIFORMS: c_uint = 0x90CE;
    pub const MAX_FRAGMENT_INPUT_COMPONENTS: c_uint = 0x9125;
    pub const MAX_FRAGMENT_INTERPOLATION_OFFSET: c_uint = 0x8E5C;
    pub const MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: c_uint = 0x90DA;
    pub const MAX_FRAGMENT_UNIFORM_BLOCKS: c_uint = 0x8A2D;
    pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: c_uint = 0x8B49;
    pub const MAX_FRAGMENT_UNIFORM_VECTORS: c_uint = 0x8DFD;
    pub const MAX_FRAMEBUFFER_HEIGHT: c_uint = 0x9316;
    pub const MAX_FRAMEBUFFER_LAYERS: c_uint = 0x9317;
    pub const MAX_FRAMEBUFFER_SAMPLES: c_uint = 0x9318;
    pub const MAX_FRAMEBUFFER_WIDTH: c_uint = 0x9315;
    pub const MAX_GEOMETRY_ATOMIC_COUNTERS: c_uint = 0x92D5;
    pub const MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92CF;
    pub const MAX_GEOMETRY_IMAGE_UNIFORMS: c_uint = 0x90CD;
    pub const MAX_GEOMETRY_INPUT_COMPONENTS: c_uint = 0x9123;
    pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: c_uint = 0x9124;
    pub const MAX_GEOMETRY_OUTPUT_VERTICES: c_uint = 0x8DE0;
    pub const MAX_GEOMETRY_OUTPUT_VERTICES_ARB: c_uint = 0x8DE0;
    pub const MAX_GEOMETRY_OUTPUT_VERTICES_EXT: c_uint = 0x8DE0;
    pub const MAX_GEOMETRY_SHADER_INVOCATIONS: c_uint = 0x8E5A;
    pub const MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: c_uint = 0x90D7;
    pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: c_uint = 0x8C29;
    pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS_ARB: c_uint = 0x8C29;
    pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS_EXT: c_uint = 0x8C29;
    pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: c_uint = 0x8DE1;
    pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS_ARB: c_uint = 0x8DE1;
    pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS_EXT: c_uint = 0x8DE1;
    pub const MAX_GEOMETRY_UNIFORM_BLOCKS: c_uint = 0x8A2C;
    pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: c_uint = 0x8DDF;
    pub const MAX_GEOMETRY_UNIFORM_COMPONENTS_ARB: c_uint = 0x8DDF;
    pub const MAX_GEOMETRY_UNIFORM_COMPONENTS_EXT: c_uint = 0x8DDF;
    pub const MAX_GEOMETRY_VARYING_COMPONENTS_ARB: c_uint = 0x8DDD;
    pub const MAX_GEOMETRY_VARYING_COMPONENTS_EXT: c_uint = 0x8DDD;
    pub const MAX_HEIGHT: c_uint = 0x827F;
    pub const MAX_IMAGE_SAMPLES: c_uint = 0x906D;
    pub const MAX_IMAGE_SAMPLES_EXT: c_uint = 0x906D;
    pub const MAX_IMAGE_UNITS: c_uint = 0x8F38;
    pub const MAX_IMAGE_UNITS_EXT: c_uint = 0x8F38;
    pub const MAX_INTEGER_SAMPLES: c_uint = 0x9110;
    pub const MAX_LABEL_LENGTH: c_uint = 0x82E8;
    pub const MAX_LAYERS: c_uint = 0x8281;
    pub const MAX_NAME_LENGTH: c_uint = 0x92F6;
    pub const MAX_NUM_ACTIVE_VARIABLES: c_uint = 0x92F7;
    pub const MAX_NUM_COMPATIBLE_SUBROUTINES: c_uint = 0x92F8;
    pub const MAX_PATCH_VERTICES: c_uint = 0x8E7D;
    pub const MAX_PROGRAM_ADDRESS_REGISTERS_ARB: c_uint = 0x88B1;
    pub const MAX_PROGRAM_ATTRIBS_ARB: c_uint = 0x88AD;
    pub const MAX_PROGRAM_ENV_PARAMETERS_ARB: c_uint = 0x88B5;
    pub const MAX_PROGRAM_INSTRUCTIONS_ARB: c_uint = 0x88A1;
    pub const MAX_PROGRAM_LOCAL_PARAMETERS_ARB: c_uint = 0x88B4;
    pub const MAX_PROGRAM_MATRICES_ARB: c_uint = 0x862F;
    pub const MAX_PROGRAM_MATRIX_STACK_DEPTH_ARB: c_uint = 0x862E;
    pub const MAX_PROGRAM_NATIVE_ADDRESS_REGISTERS_ARB: c_uint = 0x88B3;
    pub const MAX_PROGRAM_NATIVE_ATTRIBS_ARB: c_uint = 0x88AF;
    pub const MAX_PROGRAM_NATIVE_INSTRUCTIONS_ARB: c_uint = 0x88A3;
    pub const MAX_PROGRAM_NATIVE_PARAMETERS_ARB: c_uint = 0x88AB;
    pub const MAX_PROGRAM_NATIVE_TEMPORARIES_ARB: c_uint = 0x88A7;
    pub const MAX_PROGRAM_OUTPUT_VERTICES_NV: c_uint = 0x8C27;
    pub const MAX_PROGRAM_PARAMETERS_ARB: c_uint = 0x88A9;
    pub const MAX_PROGRAM_TEMPORARIES_ARB: c_uint = 0x88A5;
    pub const MAX_PROGRAM_TEXEL_OFFSET: c_uint = 0x8905;
    pub const MAX_PROGRAM_TEXEL_OFFSET_EXT: c_uint = 0x8905;
    pub const MAX_PROGRAM_TEXTURE_GATHER_OFFSET: c_uint = 0x8E5F;
    pub const MAX_PROGRAM_TOTAL_OUTPUT_COMPONENTS_NV: c_uint = 0x8C28;
    pub const MAX_RECTANGLE_TEXTURE_SIZE: c_uint = 0x84F8;
    pub const MAX_RENDERBUFFER_SIZE: c_uint = 0x84E8;
    pub const MAX_RENDERBUFFER_SIZE_EXT: c_uint = 0x84E8;
    pub const MAX_SAMPLES: c_uint = 0x8D57;
    pub const MAX_SAMPLES_EXT: c_uint = 0x8D57;
    pub const MAX_SAMPLE_MASK_WORDS: c_uint = 0x8E59;
    pub const MAX_SAMPLE_MASK_WORDS_NV: c_uint = 0x8E59;
    pub const MAX_SERVER_WAIT_TIMEOUT: c_uint = 0x9111;
    pub const MAX_SHADER_STORAGE_BLOCK_SIZE: c_uint = 0x90DE;
    pub const MAX_SHADER_STORAGE_BUFFER_BINDINGS: c_uint = 0x90DD;
    pub const MAX_SUBROUTINES: c_uint = 0x8DE7;
    pub const MAX_SUBROUTINE_UNIFORM_LOCATIONS: c_uint = 0x8DE8;
    pub const MAX_TESS_CONTROL_ATOMIC_COUNTERS: c_uint = 0x92D3;
    pub const MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92CD;
    pub const MAX_TESS_CONTROL_IMAGE_UNIFORMS: c_uint = 0x90CB;
    pub const MAX_TESS_CONTROL_INPUT_COMPONENTS: c_uint = 0x886C;
    pub const MAX_TESS_CONTROL_OUTPUT_COMPONENTS: c_uint = 0x8E83;
    pub const MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: c_uint = 0x90D8;
    pub const MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: c_uint = 0x8E81;
    pub const MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: c_uint = 0x8E85;
    pub const MAX_TESS_CONTROL_UNIFORM_BLOCKS: c_uint = 0x8E89;
    pub const MAX_TESS_CONTROL_UNIFORM_COMPONENTS: c_uint = 0x8E7F;
    pub const MAX_TESS_EVALUATION_ATOMIC_COUNTERS: c_uint = 0x92D4;
    pub const MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92CE;
    pub const MAX_TESS_EVALUATION_IMAGE_UNIFORMS: c_uint = 0x90CC;
    pub const MAX_TESS_EVALUATION_INPUT_COMPONENTS: c_uint = 0x886D;
    pub const MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: c_uint = 0x8E86;
    pub const MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: c_uint = 0x90D9;
    pub const MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: c_uint = 0x8E82;
    pub const MAX_TESS_EVALUATION_UNIFORM_BLOCKS: c_uint = 0x8E8A;
    pub const MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: c_uint = 0x8E80;
    pub const MAX_TESS_GEN_LEVEL: c_uint = 0x8E7E;
    pub const MAX_TESS_PATCH_COMPONENTS: c_uint = 0x8E84;
    pub const MAX_TEXTURE_BUFFER_SIZE: c_uint = 0x8C2B;
    pub const MAX_TEXTURE_BUFFER_SIZE_ARB: c_uint = 0x8C2B;
    pub const MAX_TEXTURE_BUFFER_SIZE_EXT: c_uint = 0x8C2B;
    pub const MAX_TEXTURE_COORDS_ARB: c_uint = 0x8871;
    pub const MAX_TEXTURE_IMAGE_UNITS: c_uint = 0x8872;
    pub const MAX_TEXTURE_IMAGE_UNITS_ARB: c_uint = 0x8872;
    pub const MAX_TEXTURE_LOD_BIAS: c_uint = 0x84FD;
    pub const MAX_TEXTURE_MAX_ANISOTROPY: c_uint = 0x84FF;
    pub const MAX_TEXTURE_SIZE: c_uint = 0x0D33;
    pub const MAX_TEXTURE_UNITS_ARB: c_uint = 0x84E2;
    pub const MAX_TRACK_MATRICES_NV: c_uint = 0x862F;
    pub const MAX_TRACK_MATRIX_STACK_DEPTH_NV: c_uint = 0x862E;
    pub const MAX_TRANSFORM_FEEDBACK_BUFFERS: c_uint = 0x8E70;
    pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: c_uint = 0x8C8A;
    pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS_EXT: c_uint = 0x8C8A;
    pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS_NV: c_uint = 0x8C8A;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: c_uint = 0x8C8B;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS_EXT: c_uint = 0x8C8B;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS_NV: c_uint = 0x8C8B;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: c_uint = 0x8C80;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS_EXT: c_uint = 0x8C80;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS_NV: c_uint = 0x8C80;
    pub const MAX_UNIFORM_BLOCK_SIZE: c_uint = 0x8A30;
    pub const MAX_UNIFORM_BUFFER_BINDINGS: c_uint = 0x8A2F;
    pub const MAX_UNIFORM_LOCATIONS: c_uint = 0x826E;
    pub const MAX_VARYING_COMPONENTS: c_uint = 0x8B4B;
    pub const MAX_VARYING_COMPONENTS_EXT: c_uint = 0x8B4B;
    pub const MAX_VARYING_FLOATS: c_uint = 0x8B4B;
    pub const MAX_VARYING_FLOATS_ARB: c_uint = 0x8B4B;
    pub const MAX_VARYING_VECTORS: c_uint = 0x8DFC;
    pub const MAX_VERTEX_ATOMIC_COUNTERS: c_uint = 0x92D2;
    pub const MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92CC;
    pub const MAX_VERTEX_ATTRIBS: c_uint = 0x8869;
    pub const MAX_VERTEX_ATTRIBS_ARB: c_uint = 0x8869;
    pub const MAX_VERTEX_ATTRIB_BINDINGS: c_uint = 0x82DA;
    pub const MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: c_uint = 0x82D9;
    pub const MAX_VERTEX_ATTRIB_STRIDE: c_uint = 0x82E5;
    pub const MAX_VERTEX_IMAGE_UNIFORMS: c_uint = 0x90CA;
    pub const MAX_VERTEX_OUTPUT_COMPONENTS: c_uint = 0x9122;
    pub const MAX_VERTEX_SHADER_STORAGE_BLOCKS: c_uint = 0x90D6;
    pub const MAX_VERTEX_STREAMS: c_uint = 0x8E71;
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: c_uint = 0x8B4C;
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS_ARB: c_uint = 0x8B4C;
    pub const MAX_VERTEX_UNIFORM_BLOCKS: c_uint = 0x8A2B;
    pub const MAX_VERTEX_UNIFORM_COMPONENTS: c_uint = 0x8B4A;
    pub const MAX_VERTEX_UNIFORM_COMPONENTS_ARB: c_uint = 0x8B4A;
    pub const MAX_VERTEX_UNIFORM_VECTORS: c_uint = 0x8DFB;
    pub const MAX_VERTEX_VARYING_COMPONENTS_ARB: c_uint = 0x8DDE;
    pub const MAX_VERTEX_VARYING_COMPONENTS_EXT: c_uint = 0x8DDE;
    pub const MAX_VIEWPORTS: c_uint = 0x825B;
    pub const MAX_VIEWPORT_DIMS: c_uint = 0x0D3A;
    pub const MAX_WIDTH: c_uint = 0x827E;
    pub const MEDIUM_FLOAT: c_uint = 0x8DF1;
    pub const MEDIUM_INT: c_uint = 0x8DF4;
    pub const MIN: c_uint = 0x8007;
    pub const MINOR_VERSION: c_uint = 0x821C;
    pub const MIN_EXT: c_uint = 0x8007;
    pub const MIN_FRAGMENT_INTERPOLATION_OFFSET: c_uint = 0x8E5B;
    pub const MIN_MAP_BUFFER_ALIGNMENT: c_uint = 0x90BC;
    pub const MIN_PROGRAM_TEXEL_OFFSET: c_uint = 0x8904;
    pub const MIN_PROGRAM_TEXEL_OFFSET_EXT: c_uint = 0x8904;
    pub const MIN_PROGRAM_TEXTURE_GATHER_OFFSET: c_uint = 0x8E5E;
    pub const MIN_SAMPLE_SHADING_VALUE: c_uint = 0x8C37;
    pub const MIN_SAMPLE_SHADING_VALUE_ARB: c_uint = 0x8C37;
    pub const MIPMAP: c_uint = 0x8293;
    pub const MIRRORED_REPEAT: c_uint = 0x8370;
    pub const MIRROR_CLAMP_TO_EDGE: c_uint = 0x8743;
    pub const MODELVIEW_PROJECTION_NV: c_uint = 0x8629;
    pub const MULTISAMPLE: c_uint = 0x809D;
    pub const MULTISAMPLE_ARB: c_uint = 0x809D;
    pub const MULTISAMPLE_BIT_ARB: c_uint = 0x20000000;
    pub const NAME_LENGTH: c_uint = 0x92F9;
    pub const NAND: c_uint = 0x150E;
    pub const NEAREST: c_uint = 0x2600;
    pub const NEAREST_MIPMAP_LINEAR: c_uint = 0x2702;
    pub const NEAREST_MIPMAP_NEAREST: c_uint = 0x2700;
    pub const NEGATIVE_ONE_TO_ONE: c_uint = 0x935E;
    pub const NEVER: c_uint = 0x0200;
    pub const NEXT_BUFFER_NV: c_int = -2;
    pub const NICEST: c_uint = 0x1102;
    pub const NONE: c_uint = 0;
    pub const NOOP: c_uint = 0x1505;
    pub const NOR: c_uint = 0x1508;
    pub const NORMAL_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x8897;
    pub const NORMAL_ARRAY_COUNT_EXT: c_uint = 0x8080;
    pub const NORMAL_ARRAY_EXT: c_uint = 0x8075;
    pub const NORMAL_ARRAY_POINTER_EXT: c_uint = 0x808F;
    pub const NORMAL_ARRAY_STRIDE_EXT: c_uint = 0x807F;
    pub const NORMAL_ARRAY_TYPE_EXT: c_uint = 0x807E;
    pub const NOTEQUAL: c_uint = 0x0205;
    pub const NO_ERROR: c_uint = 0;
    pub const NO_RESET_NOTIFICATION: c_uint = 0x8261;
    pub const NO_RESET_NOTIFICATION_ARB: c_uint = 0x8261;
    pub const NUM_ACTIVE_VARIABLES: c_uint = 0x9304;
    pub const NUM_COMPATIBLE_SUBROUTINES: c_uint = 0x8E4A;
    pub const NUM_COMPRESSED_TEXTURE_FORMATS: c_uint = 0x86A2;
    pub const NUM_COMPRESSED_TEXTURE_FORMATS_ARB: c_uint = 0x86A2;
    pub const NUM_EXTENSIONS: c_uint = 0x821D;
    pub const NUM_PROGRAM_BINARY_FORMATS: c_uint = 0x87FE;
    pub const NUM_SAMPLE_COUNTS: c_uint = 0x9380;
    pub const NUM_SHADER_BINARY_FORMATS: c_uint = 0x8DF9;
    pub const NUM_SHADING_LANGUAGE_VERSIONS: c_uint = 0x82E9;
    pub const NUM_SPIR_V_EXTENSIONS: c_uint = 0x9554;
    pub const OBJECT_ACTIVE_ATTRIBUTES_ARB: c_uint = 0x8B89;
    pub const OBJECT_ACTIVE_ATTRIBUTE_MAX_LENGTH_ARB: c_uint = 0x8B8A;
    pub const OBJECT_ACTIVE_UNIFORMS_ARB: c_uint = 0x8B86;
    pub const OBJECT_ACTIVE_UNIFORM_MAX_LENGTH_ARB: c_uint = 0x8B87;
    pub const OBJECT_ATTACHED_OBJECTS_ARB: c_uint = 0x8B85;
    pub const OBJECT_COMPILE_STATUS_ARB: c_uint = 0x8B81;
    pub const OBJECT_DELETE_STATUS_ARB: c_uint = 0x8B80;
    pub const OBJECT_INFO_LOG_LENGTH_ARB: c_uint = 0x8B84;
    pub const OBJECT_LINK_STATUS_ARB: c_uint = 0x8B82;
    pub const OBJECT_SHADER_SOURCE_LENGTH_ARB: c_uint = 0x8B88;
    pub const OBJECT_SUBTYPE_ARB: c_uint = 0x8B4F;
    pub const OBJECT_TYPE: c_uint = 0x9112;
    pub const OBJECT_TYPE_ARB: c_uint = 0x8B4E;
    pub const OBJECT_VALIDATE_STATUS_ARB: c_uint = 0x8B83;
    pub const OFFSET: c_uint = 0x92FC;
    pub const ONE: c_uint = 1;
    pub const ONE_MINUS_CONSTANT_ALPHA: c_uint = 0x8004;
    pub const ONE_MINUS_CONSTANT_ALPHA_EXT: c_uint = 0x8004;
    pub const ONE_MINUS_CONSTANT_COLOR: c_uint = 0x8002;
    pub const ONE_MINUS_CONSTANT_COLOR_EXT: c_uint = 0x8002;
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
    pub const PACK_COMPRESSED_BLOCK_DEPTH: c_uint = 0x912D;
    pub const PACK_COMPRESSED_BLOCK_HEIGHT: c_uint = 0x912C;
    pub const PACK_COMPRESSED_BLOCK_SIZE: c_uint = 0x912E;
    pub const PACK_COMPRESSED_BLOCK_WIDTH: c_uint = 0x912B;
    pub const PACK_IMAGE_HEIGHT: c_uint = 0x806C;
    pub const PACK_IMAGE_HEIGHT_EXT: c_uint = 0x806C;
    pub const PACK_LSB_FIRST: c_uint = 0x0D01;
    pub const PACK_ROW_LENGTH: c_uint = 0x0D02;
    pub const PACK_SKIP_IMAGES: c_uint = 0x806B;
    pub const PACK_SKIP_IMAGES_EXT: c_uint = 0x806B;
    pub const PACK_SKIP_PIXELS: c_uint = 0x0D04;
    pub const PACK_SKIP_ROWS: c_uint = 0x0D03;
    pub const PACK_SWAP_BYTES: c_uint = 0x0D00;
    pub const PARAMETER_BUFFER: c_uint = 0x80EE;
    pub const PARAMETER_BUFFER_ARB: c_uint = 0x80EE;
    pub const PARAMETER_BUFFER_BINDING: c_uint = 0x80EF;
    pub const PARAMETER_BUFFER_BINDING_ARB: c_uint = 0x80EF;
    pub const PATCHES: c_uint = 0x000E;
    pub const PATCH_DEFAULT_INNER_LEVEL: c_uint = 0x8E73;
    pub const PATCH_DEFAULT_OUTER_LEVEL: c_uint = 0x8E74;
    pub const PATCH_VERTICES: c_uint = 0x8E72;
    pub const PIXEL_BUFFER_BARRIER_BIT: c_uint = 0x00000080;
    pub const PIXEL_BUFFER_BARRIER_BIT_EXT: c_uint = 0x00000080;
    pub const PIXEL_PACK_BUFFER: c_uint = 0x88EB;
    pub const PIXEL_PACK_BUFFER_BINDING: c_uint = 0x88ED;
    pub const PIXEL_UNPACK_BUFFER: c_uint = 0x88EC;
    pub const PIXEL_UNPACK_BUFFER_BINDING: c_uint = 0x88EF;
    pub const POINT: c_uint = 0x1B00;
    pub const POINTS: c_uint = 0x0000;
    pub const POINT_DISTANCE_ATTENUATION_ARB: c_uint = 0x8129;
    pub const POINT_FADE_THRESHOLD_SIZE: c_uint = 0x8128;
    pub const POINT_FADE_THRESHOLD_SIZE_ARB: c_uint = 0x8128;
    pub const POINT_FADE_THRESHOLD_SIZE_EXT: c_uint = 0x8128;
    pub const POINT_FADE_THRESHOLD_SIZE_SGIS: c_uint = 0x8128;
    pub const POINT_SIZE: c_uint = 0x0B11;
    pub const POINT_SIZE_GRANULARITY: c_uint = 0x0B13;
    pub const POINT_SIZE_MAX_ARB: c_uint = 0x8127;
    pub const POINT_SIZE_MAX_EXT: c_uint = 0x8127;
    pub const POINT_SIZE_MAX_SGIS: c_uint = 0x8127;
    pub const POINT_SIZE_MIN_ARB: c_uint = 0x8126;
    pub const POINT_SIZE_MIN_EXT: c_uint = 0x8126;
    pub const POINT_SIZE_MIN_SGIS: c_uint = 0x8126;
    pub const POINT_SIZE_RANGE: c_uint = 0x0B12;
    pub const POINT_SPRITE_COORD_ORIGIN: c_uint = 0x8CA0;
    pub const POINT_SPRITE_NV: c_uint = 0x8861;
    pub const POINT_SPRITE_R_MODE_NV: c_uint = 0x8863;
    pub const POLYGON_MODE: c_uint = 0x0B40;
    pub const POLYGON_OFFSET_CLAMP: c_uint = 0x8E1B;
    pub const POLYGON_OFFSET_CLAMP_EXT: c_uint = 0x8E1B;
    pub const POLYGON_OFFSET_FACTOR: c_uint = 0x8038;
    pub const POLYGON_OFFSET_FILL: c_uint = 0x8037;
    pub const POLYGON_OFFSET_LINE: c_uint = 0x2A02;
    pub const POLYGON_OFFSET_POINT: c_uint = 0x2A01;
    pub const POLYGON_OFFSET_UNITS: c_uint = 0x2A00;
    pub const POLYGON_SMOOTH: c_uint = 0x0B41;
    pub const POLYGON_SMOOTH_HINT: c_uint = 0x0C53;
    pub const PRIMITIVES_GENERATED: c_uint = 0x8C87;
    pub const PRIMITIVES_GENERATED_EXT: c_uint = 0x8C87;
    pub const PRIMITIVES_GENERATED_NV: c_uint = 0x8C87;
    pub const PRIMITIVES_SUBMITTED: c_uint = 0x82EF;
    pub const PRIMITIVE_ID_NV: c_uint = 0x8C7C;
    pub const PRIMITIVE_RESTART: c_uint = 0x8F9D;
    pub const PRIMITIVE_RESTART_FIXED_INDEX: c_uint = 0x8D69;
    pub const PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: c_uint = 0x8221;
    pub const PRIMITIVE_RESTART_INDEX: c_uint = 0x8F9E;
    pub const PROGRAM: c_uint = 0x82E2;
    pub const PROGRAM_ADDRESS_REGISTERS_ARB: c_uint = 0x88B0;
    pub const PROGRAM_ATTRIBS_ARB: c_uint = 0x88AC;
    pub const PROGRAM_BINARY_FORMATS: c_uint = 0x87FF;
    pub const PROGRAM_BINARY_LENGTH: c_uint = 0x8741;
    pub const PROGRAM_BINARY_RETRIEVABLE_HINT: c_uint = 0x8257;
    pub const PROGRAM_BINDING_ARB: c_uint = 0x8677;
    pub const PROGRAM_ERROR_POSITION_ARB: c_uint = 0x864B;
    pub const PROGRAM_ERROR_POSITION_NV: c_uint = 0x864B;
    pub const PROGRAM_ERROR_STRING_ARB: c_uint = 0x8874;
    pub const PROGRAM_FORMAT_ARB: c_uint = 0x8876;
    pub const PROGRAM_FORMAT_ASCII_ARB: c_uint = 0x8875;
    pub const PROGRAM_INPUT: c_uint = 0x92E3;
    pub const PROGRAM_INSTRUCTIONS_ARB: c_uint = 0x88A0;
    pub const PROGRAM_LENGTH_ARB: c_uint = 0x8627;
    pub const PROGRAM_LENGTH_NV: c_uint = 0x8627;
    pub const PROGRAM_MATRIX_EXT: c_uint = 0x8E2D;
    pub const PROGRAM_MATRIX_STACK_DEPTH_EXT: c_uint = 0x8E2F;
    pub const PROGRAM_NATIVE_ADDRESS_REGISTERS_ARB: c_uint = 0x88B2;
    pub const PROGRAM_NATIVE_ATTRIBS_ARB: c_uint = 0x88AE;
    pub const PROGRAM_NATIVE_INSTRUCTIONS_ARB: c_uint = 0x88A2;
    pub const PROGRAM_NATIVE_PARAMETERS_ARB: c_uint = 0x88AA;
    pub const PROGRAM_NATIVE_TEMPORARIES_ARB: c_uint = 0x88A6;
    pub const PROGRAM_OBJECT_ARB: c_uint = 0x8B40;
    pub const PROGRAM_OUTPUT: c_uint = 0x92E4;
    pub const PROGRAM_PARAMETERS_ARB: c_uint = 0x88A8;
    pub const PROGRAM_PARAMETER_NV: c_uint = 0x8644;
    pub const PROGRAM_PIPELINE: c_uint = 0x82E4;
    pub const PROGRAM_PIPELINE_BINDING: c_uint = 0x825A;
    pub const PROGRAM_POINT_SIZE: c_uint = 0x8642;
    pub const PROGRAM_POINT_SIZE_ARB: c_uint = 0x8642;
    pub const PROGRAM_POINT_SIZE_EXT: c_uint = 0x8642;
    pub const PROGRAM_RESIDENT_NV: c_uint = 0x8647;
    pub const PROGRAM_SEPARABLE: c_uint = 0x8258;
    pub const PROGRAM_STRING_ARB: c_uint = 0x8628;
    pub const PROGRAM_STRING_NV: c_uint = 0x8628;
    pub const PROGRAM_TARGET_NV: c_uint = 0x8646;
    pub const PROGRAM_TEMPORARIES_ARB: c_uint = 0x88A4;
    pub const PROGRAM_UNDER_NATIVE_LIMITS_ARB: c_uint = 0x88B6;
    pub const PROVOKING_VERTEX: c_uint = 0x8E4F;
    pub const PROVOKING_VERTEX_EXT: c_uint = 0x8E4F;
    pub const PROXY_TEXTURE_1D: c_uint = 0x8063;
    pub const PROXY_TEXTURE_1D_ARRAY: c_uint = 0x8C19;
    pub const PROXY_TEXTURE_1D_ARRAY_EXT: c_uint = 0x8C19;
    pub const PROXY_TEXTURE_2D: c_uint = 0x8064;
    pub const PROXY_TEXTURE_2D_ARRAY: c_uint = 0x8C1B;
    pub const PROXY_TEXTURE_2D_ARRAY_EXT: c_uint = 0x8C1B;
    pub const PROXY_TEXTURE_2D_MULTISAMPLE: c_uint = 0x9101;
    pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: c_uint = 0x9103;
    pub const PROXY_TEXTURE_3D: c_uint = 0x8070;
    pub const PROXY_TEXTURE_3D_EXT: c_uint = 0x8070;
    pub const PROXY_TEXTURE_CUBE_MAP: c_uint = 0x851B;
    pub const PROXY_TEXTURE_CUBE_MAP_ARRAY: c_uint = 0x900B;
    pub const PROXY_TEXTURE_RECTANGLE: c_uint = 0x84F7;
    pub const QUADS: c_uint = 0x0007;
    pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: c_uint = 0x8E4C;
    pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION_EXT: c_uint = 0x8E4C;
    pub const QUERY: c_uint = 0x82E3;
    pub const QUERY_BUFFER: c_uint = 0x9192;
    pub const QUERY_BUFFER_BARRIER_BIT: c_uint = 0x00008000;
    pub const QUERY_BUFFER_BINDING: c_uint = 0x9193;
    pub const QUERY_BY_REGION_NO_WAIT: c_uint = 0x8E16;
    pub const QUERY_BY_REGION_NO_WAIT_INVERTED: c_uint = 0x8E1A;
    pub const QUERY_BY_REGION_NO_WAIT_NV: c_uint = 0x8E16;
    pub const QUERY_BY_REGION_WAIT: c_uint = 0x8E15;
    pub const QUERY_BY_REGION_WAIT_INVERTED: c_uint = 0x8E19;
    pub const QUERY_BY_REGION_WAIT_NV: c_uint = 0x8E15;
    pub const QUERY_COUNTER_BITS: c_uint = 0x8864;
    pub const QUERY_COUNTER_BITS_ARB: c_uint = 0x8864;
    pub const QUERY_NO_WAIT: c_uint = 0x8E14;
    pub const QUERY_NO_WAIT_INVERTED: c_uint = 0x8E18;
    pub const QUERY_NO_WAIT_NV: c_uint = 0x8E14;
    pub const QUERY_RESULT: c_uint = 0x8866;
    pub const QUERY_RESULT_ARB: c_uint = 0x8866;
    pub const QUERY_RESULT_AVAILABLE: c_uint = 0x8867;
    pub const QUERY_RESULT_AVAILABLE_ARB: c_uint = 0x8867;
    pub const QUERY_RESULT_NO_WAIT: c_uint = 0x9194;
    pub const QUERY_TARGET: c_uint = 0x82EA;
    pub const QUERY_WAIT: c_uint = 0x8E13;
    pub const QUERY_WAIT_INVERTED: c_uint = 0x8E17;
    pub const QUERY_WAIT_NV: c_uint = 0x8E13;
    pub const R11F_G11F_B10F: c_uint = 0x8C3A;
    pub const R16: c_uint = 0x822A;
    pub const R16F: c_uint = 0x822D;
    pub const R16F_EXT: c_uint = 0x822D;
    pub const R16I: c_uint = 0x8233;
    pub const R16UI: c_uint = 0x8234;
    pub const R16_SNORM: c_uint = 0x8F98;
    pub const R32F: c_uint = 0x822E;
    pub const R32F_EXT: c_uint = 0x822E;
    pub const R32I: c_uint = 0x8235;
    pub const R32UI: c_uint = 0x8236;
    pub const R3_G3_B2: c_uint = 0x2A10;
    pub const R8: c_uint = 0x8229;
    pub const R8I: c_uint = 0x8231;
    pub const R8UI: c_uint = 0x8232;
    pub const R8_EXT: c_uint = 0x8229;
    pub const R8_SNORM: c_uint = 0x8F94;
    pub const RASTERIZER_DISCARD: c_uint = 0x8C89;
    pub const RASTERIZER_DISCARD_EXT: c_uint = 0x8C89;
    pub const RASTERIZER_DISCARD_NV: c_uint = 0x8C89;
    pub const READ_BUFFER: c_uint = 0x0C02;
    pub const READ_FRAMEBUFFER: c_uint = 0x8CA8;
    pub const READ_FRAMEBUFFER_BINDING: c_uint = 0x8CAA;
    pub const READ_FRAMEBUFFER_BINDING_EXT: c_uint = 0x8CAA;
    pub const READ_FRAMEBUFFER_EXT: c_uint = 0x8CA8;
    pub const READ_ONLY: c_uint = 0x88B8;
    pub const READ_ONLY_ARB: c_uint = 0x88B8;
    pub const READ_PIXELS: c_uint = 0x828C;
    pub const READ_PIXELS_FORMAT: c_uint = 0x828D;
    pub const READ_PIXELS_TYPE: c_uint = 0x828E;
    pub const READ_WRITE: c_uint = 0x88BA;
    pub const READ_WRITE_ARB: c_uint = 0x88BA;
    pub const RED: c_uint = 0x1903;
    pub const RED_INTEGER: c_uint = 0x8D94;
    pub const RED_INTEGER_EXT: c_uint = 0x8D94;
    pub const REFERENCED_BY_COMPUTE_SHADER: c_uint = 0x930B;
    pub const REFERENCED_BY_FRAGMENT_SHADER: c_uint = 0x930A;
    pub const REFERENCED_BY_GEOMETRY_SHADER: c_uint = 0x9309;
    pub const REFERENCED_BY_TESS_CONTROL_SHADER: c_uint = 0x9307;
    pub const REFERENCED_BY_TESS_EVALUATION_SHADER: c_uint = 0x9308;
    pub const REFERENCED_BY_VERTEX_SHADER: c_uint = 0x9306;
    pub const RENDERBUFFER: c_uint = 0x8D41;
    pub const RENDERBUFFER_ALPHA_SIZE: c_uint = 0x8D53;
    pub const RENDERBUFFER_ALPHA_SIZE_EXT: c_uint = 0x8D53;
    pub const RENDERBUFFER_BINDING: c_uint = 0x8CA7;
    pub const RENDERBUFFER_BINDING_EXT: c_uint = 0x8CA7;
    pub const RENDERBUFFER_BLUE_SIZE: c_uint = 0x8D52;
    pub const RENDERBUFFER_BLUE_SIZE_EXT: c_uint = 0x8D52;
    pub const RENDERBUFFER_DEPTH_SIZE: c_uint = 0x8D54;
    pub const RENDERBUFFER_DEPTH_SIZE_EXT: c_uint = 0x8D54;
    pub const RENDERBUFFER_EXT: c_uint = 0x8D41;
    pub const RENDERBUFFER_GREEN_SIZE: c_uint = 0x8D51;
    pub const RENDERBUFFER_GREEN_SIZE_EXT: c_uint = 0x8D51;
    pub const RENDERBUFFER_HEIGHT: c_uint = 0x8D43;
    pub const RENDERBUFFER_HEIGHT_EXT: c_uint = 0x8D43;
    pub const RENDERBUFFER_INTERNAL_FORMAT: c_uint = 0x8D44;
    pub const RENDERBUFFER_INTERNAL_FORMAT_EXT: c_uint = 0x8D44;
    pub const RENDERBUFFER_RED_SIZE: c_uint = 0x8D50;
    pub const RENDERBUFFER_RED_SIZE_EXT: c_uint = 0x8D50;
    pub const RENDERBUFFER_SAMPLES: c_uint = 0x8CAB;
    pub const RENDERBUFFER_SAMPLES_EXT: c_uint = 0x8CAB;
    pub const RENDERBUFFER_STENCIL_SIZE: c_uint = 0x8D55;
    pub const RENDERBUFFER_STENCIL_SIZE_EXT: c_uint = 0x8D55;
    pub const RENDERBUFFER_WIDTH: c_uint = 0x8D42;
    pub const RENDERBUFFER_WIDTH_EXT: c_uint = 0x8D42;
    pub const RENDERER: c_uint = 0x1F01;
    pub const REPEAT: c_uint = 0x2901;
    pub const REPLACE: c_uint = 0x1E01;
    pub const RESET_NOTIFICATION_STRATEGY: c_uint = 0x8256;
    pub const RESET_NOTIFICATION_STRATEGY_ARB: c_uint = 0x8256;
    pub const RG: c_uint = 0x8227;
    pub const RG16: c_uint = 0x822C;
    pub const RG16F: c_uint = 0x822F;
    pub const RG16F_EXT: c_uint = 0x822F;
    pub const RG16I: c_uint = 0x8239;
    pub const RG16UI: c_uint = 0x823A;
    pub const RG16_SNORM: c_uint = 0x8F99;
    pub const RG32F: c_uint = 0x8230;
    pub const RG32F_EXT: c_uint = 0x8230;
    pub const RG32I: c_uint = 0x823B;
    pub const RG32UI: c_uint = 0x823C;
    pub const RG8: c_uint = 0x822B;
    pub const RG8I: c_uint = 0x8237;
    pub const RG8UI: c_uint = 0x8238;
    pub const RG8_EXT: c_uint = 0x822B;
    pub const RG8_SNORM: c_uint = 0x8F95;
    pub const RGB: c_uint = 0x1907;
    pub const RGB10: c_uint = 0x8052;
    pub const RGB10_A2: c_uint = 0x8059;
    pub const RGB10_A2UI: c_uint = 0x906F;
    pub const RGB10_A2_EXT: c_uint = 0x8059;
    pub const RGB10_EXT: c_uint = 0x8052;
    pub const RGB12: c_uint = 0x8053;
    pub const RGB16: c_uint = 0x8054;
    pub const RGB16F: c_uint = 0x881B;
    pub const RGB16F_EXT: c_uint = 0x881B;
    pub const RGB16I: c_uint = 0x8D89;
    pub const RGB16I_EXT: c_uint = 0x8D89;
    pub const RGB16UI: c_uint = 0x8D77;
    pub const RGB16UI_EXT: c_uint = 0x8D77;
    pub const RGB16_SNORM: c_uint = 0x8F9A;
    pub const RGB32F: c_uint = 0x8815;
    pub const RGB32F_EXT: c_uint = 0x8815;
    pub const RGB32I: c_uint = 0x8D83;
    pub const RGB32I_EXT: c_uint = 0x8D83;
    pub const RGB32UI: c_uint = 0x8D71;
    pub const RGB32UI_EXT: c_uint = 0x8D71;
    pub const RGB4: c_uint = 0x804F;
    pub const RGB5: c_uint = 0x8050;
    pub const RGB565: c_uint = 0x8D62;
    pub const RGB5_A1: c_uint = 0x8057;
    pub const RGB8: c_uint = 0x8051;
    pub const RGB8I: c_uint = 0x8D8F;
    pub const RGB8I_EXT: c_uint = 0x8D8F;
    pub const RGB8UI: c_uint = 0x8D7D;
    pub const RGB8UI_EXT: c_uint = 0x8D7D;
    pub const RGB8_SNORM: c_uint = 0x8F96;
    pub const RGB9_E5: c_uint = 0x8C3D;
    pub const RGBA: c_uint = 0x1908;
    pub const RGBA12: c_uint = 0x805A;
    pub const RGBA16: c_uint = 0x805B;
    pub const RGBA16F: c_uint = 0x881A;
    pub const RGBA16F_EXT: c_uint = 0x881A;
    pub const RGBA16I: c_uint = 0x8D88;
    pub const RGBA16I_EXT: c_uint = 0x8D88;
    pub const RGBA16UI: c_uint = 0x8D76;
    pub const RGBA16UI_EXT: c_uint = 0x8D76;
    pub const RGBA16_SNORM: c_uint = 0x8F9B;
    pub const RGBA2: c_uint = 0x8055;
    pub const RGBA32F: c_uint = 0x8814;
    pub const RGBA32F_EXT: c_uint = 0x8814;
    pub const RGBA32I: c_uint = 0x8D82;
    pub const RGBA32I_EXT: c_uint = 0x8D82;
    pub const RGBA32UI: c_uint = 0x8D70;
    pub const RGBA32UI_EXT: c_uint = 0x8D70;
    pub const RGBA4: c_uint = 0x8056;
    pub const RGBA8: c_uint = 0x8058;
    pub const RGBA8I: c_uint = 0x8D8E;
    pub const RGBA8I_EXT: c_uint = 0x8D8E;
    pub const RGBA8UI: c_uint = 0x8D7C;
    pub const RGBA8UI_EXT: c_uint = 0x8D7C;
    pub const RGBA8_SNORM: c_uint = 0x8F97;
    pub const RGBA_FLOAT_MODE_ARB: c_uint = 0x8820;
    pub const RGBA_INTEGER: c_uint = 0x8D99;
    pub const RGBA_INTEGER_EXT: c_uint = 0x8D99;
    pub const RGBA_INTEGER_MODE_EXT: c_uint = 0x8D9E;
    pub const RGB_INTEGER: c_uint = 0x8D98;
    pub const RGB_INTEGER_EXT: c_uint = 0x8D98;
    pub const RG_INTEGER: c_uint = 0x8228;
    pub const RIGHT: c_uint = 0x0407;
    pub const SAMPLER: c_uint = 0x82E6;
    pub const SAMPLER_1D: c_uint = 0x8B5D;
    pub const SAMPLER_1D_ARB: c_uint = 0x8B5D;
    pub const SAMPLER_1D_ARRAY: c_uint = 0x8DC0;
    pub const SAMPLER_1D_ARRAY_EXT: c_uint = 0x8DC0;
    pub const SAMPLER_1D_ARRAY_SHADOW: c_uint = 0x8DC3;
    pub const SAMPLER_1D_ARRAY_SHADOW_EXT: c_uint = 0x8DC3;
    pub const SAMPLER_1D_SHADOW: c_uint = 0x8B61;
    pub const SAMPLER_1D_SHADOW_ARB: c_uint = 0x8B61;
    pub const SAMPLER_2D: c_uint = 0x8B5E;
    pub const SAMPLER_2D_ARB: c_uint = 0x8B5E;
    pub const SAMPLER_2D_ARRAY: c_uint = 0x8DC1;
    pub const SAMPLER_2D_ARRAY_EXT: c_uint = 0x8DC1;
    pub const SAMPLER_2D_ARRAY_SHADOW: c_uint = 0x8DC4;
    pub const SAMPLER_2D_ARRAY_SHADOW_EXT: c_uint = 0x8DC4;
    pub const SAMPLER_2D_MULTISAMPLE: c_uint = 0x9108;
    pub const SAMPLER_2D_MULTISAMPLE_ARRAY: c_uint = 0x910B;
    pub const SAMPLER_2D_RECT: c_uint = 0x8B63;
    pub const SAMPLER_2D_RECT_ARB: c_uint = 0x8B63;
    pub const SAMPLER_2D_RECT_SHADOW: c_uint = 0x8B64;
    pub const SAMPLER_2D_RECT_SHADOW_ARB: c_uint = 0x8B64;
    pub const SAMPLER_2D_SHADOW: c_uint = 0x8B62;
    pub const SAMPLER_2D_SHADOW_ARB: c_uint = 0x8B62;
    pub const SAMPLER_3D: c_uint = 0x8B5F;
    pub const SAMPLER_3D_ARB: c_uint = 0x8B5F;
    pub const SAMPLER_BINDING: c_uint = 0x8919;
    pub const SAMPLER_BUFFER: c_uint = 0x8DC2;
    pub const SAMPLER_BUFFER_EXT: c_uint = 0x8DC2;
    pub const SAMPLER_CUBE: c_uint = 0x8B60;
    pub const SAMPLER_CUBE_ARB: c_uint = 0x8B60;
    pub const SAMPLER_CUBE_MAP_ARRAY: c_uint = 0x900C;
    pub const SAMPLER_CUBE_MAP_ARRAY_SHADOW: c_uint = 0x900D;
    pub const SAMPLER_CUBE_SHADOW: c_uint = 0x8DC5;
    pub const SAMPLER_CUBE_SHADOW_EXT: c_uint = 0x8DC5;
    pub const SAMPLER_RENDERBUFFER_NV: c_uint = 0x8E56;
    pub const SAMPLES: c_uint = 0x80A9;
    pub const SAMPLES_ARB: c_uint = 0x80A9;
    pub const SAMPLES_PASSED: c_uint = 0x8914;
    pub const SAMPLES_PASSED_ARB: c_uint = 0x8914;
    pub const SAMPLE_ALPHA_TO_COVERAGE: c_uint = 0x809E;
    pub const SAMPLE_ALPHA_TO_COVERAGE_ARB: c_uint = 0x809E;
    pub const SAMPLE_ALPHA_TO_ONE: c_uint = 0x809F;
    pub const SAMPLE_ALPHA_TO_ONE_ARB: c_uint = 0x809F;
    pub const SAMPLE_BUFFERS: c_uint = 0x80A8;
    pub const SAMPLE_BUFFERS_ARB: c_uint = 0x80A8;
    pub const SAMPLE_COVERAGE: c_uint = 0x80A0;
    pub const SAMPLE_COVERAGE_ARB: c_uint = 0x80A0;
    pub const SAMPLE_COVERAGE_INVERT: c_uint = 0x80AB;
    pub const SAMPLE_COVERAGE_INVERT_ARB: c_uint = 0x80AB;
    pub const SAMPLE_COVERAGE_VALUE: c_uint = 0x80AA;
    pub const SAMPLE_COVERAGE_VALUE_ARB: c_uint = 0x80AA;
    pub const SAMPLE_MASK: c_uint = 0x8E51;
    pub const SAMPLE_MASK_NV: c_uint = 0x8E51;
    pub const SAMPLE_MASK_VALUE: c_uint = 0x8E52;
    pub const SAMPLE_MASK_VALUE_NV: c_uint = 0x8E52;
    pub const SAMPLE_POSITION: c_uint = 0x8E50;
    pub const SAMPLE_POSITION_NV: c_uint = 0x8E50;
    pub const SAMPLE_SHADING: c_uint = 0x8C36;
    pub const SAMPLE_SHADING_ARB: c_uint = 0x8C36;
    pub const SCISSOR_BOX: c_uint = 0x0C10;
    pub const SCISSOR_TEST: c_uint = 0x0C11;
    pub const SECONDARY_COLOR_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x889C;
    pub const SEPARATE_ATTRIBS: c_uint = 0x8C8D;
    pub const SEPARATE_ATTRIBS_EXT: c_uint = 0x8C8D;
    pub const SEPARATE_ATTRIBS_NV: c_uint = 0x8C8D;
    pub const SET: c_uint = 0x150F;
    pub const SHADER: c_uint = 0x82E1;
    pub const SHADER_BINARY_FORMATS: c_uint = 0x8DF8;
    pub const SHADER_BINARY_FORMAT_SPIR_V: c_uint = 0x9551;
    pub const SHADER_BINARY_FORMAT_SPIR_V_ARB: c_uint = 0x9551;
    pub const SHADER_COMPILER: c_uint = 0x8DFA;
    pub const SHADER_IMAGE_ACCESS_BARRIER_BIT: c_uint = 0x00000020;
    pub const SHADER_IMAGE_ACCESS_BARRIER_BIT_EXT: c_uint = 0x00000020;
    pub const SHADER_IMAGE_ATOMIC: c_uint = 0x82A6;
    pub const SHADER_IMAGE_LOAD: c_uint = 0x82A4;
    pub const SHADER_IMAGE_STORE: c_uint = 0x82A5;
    pub const SHADER_OBJECT_ARB: c_uint = 0x8B48;
    pub const SHADER_SOURCE_LENGTH: c_uint = 0x8B88;
    pub const SHADER_STORAGE_BARRIER_BIT: c_uint = 0x00002000;
    pub const SHADER_STORAGE_BLOCK: c_uint = 0x92E6;
    pub const SHADER_STORAGE_BUFFER: c_uint = 0x90D2;
    pub const SHADER_STORAGE_BUFFER_BINDING: c_uint = 0x90D3;
    pub const SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: c_uint = 0x90DF;
    pub const SHADER_STORAGE_BUFFER_SIZE: c_uint = 0x90D5;
    pub const SHADER_STORAGE_BUFFER_START: c_uint = 0x90D4;
    pub const SHADER_TYPE: c_uint = 0x8B4F;
    pub const SHADING_LANGUAGE_VERSION: c_uint = 0x8B8C;
    pub const SHORT: c_uint = 0x1402;
    pub const SIGNALED: c_uint = 0x9119;
    pub const SIGNED_NORMALIZED: c_uint = 0x8F9C;
    pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: c_uint = 0x82AC;
    pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: c_uint = 0x82AE;
    pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: c_uint = 0x82AD;
    pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: c_uint = 0x82AF;
    pub const SKIP_COMPONENTS1_NV: c_int = -6;
    pub const SKIP_COMPONENTS2_NV: c_int = -5;
    pub const SKIP_COMPONENTS3_NV: c_int = -4;
    pub const SKIP_COMPONENTS4_NV: c_int = -3;
    pub const SMOOTH_LINE_WIDTH_GRANULARITY: c_uint = 0x0B23;
    pub const SMOOTH_LINE_WIDTH_RANGE: c_uint = 0x0B22;
    pub const SMOOTH_POINT_SIZE_GRANULARITY: c_uint = 0x0B13;
    pub const SMOOTH_POINT_SIZE_RANGE: c_uint = 0x0B12;
    pub const SOURCE1_ALPHA: c_uint = 0x8589;
    pub const SPIR_V_BINARY: c_uint = 0x9552;
    pub const SPIR_V_BINARY_ARB: c_uint = 0x9552;
    pub const SPIR_V_EXTENSIONS: c_uint = 0x9553;
    pub const SRC1_ALPHA: c_uint = 0x8589;
    pub const SRC1_COLOR: c_uint = 0x88F9;
    pub const SRC_ALPHA: c_uint = 0x0302;
    pub const SRC_ALPHA_SATURATE: c_uint = 0x0308;
    pub const SRC_COLOR: c_uint = 0x0300;
    pub const SRGB: c_uint = 0x8C40;
    pub const SRGB8: c_uint = 0x8C41;
    pub const SRGB8_ALPHA8: c_uint = 0x8C43;
    pub const SRGB_ALPHA: c_uint = 0x8C42;
    pub const SRGB_DECODE_ARB: c_uint = 0x8299;
    pub const SRGB_READ: c_uint = 0x8297;
    pub const SRGB_WRITE: c_uint = 0x8298;
    pub const STACK_OVERFLOW: c_uint = 0x0503;
    pub const STACK_UNDERFLOW: c_uint = 0x0504;
    pub const STATIC_COPY: c_uint = 0x88E6;
    pub const STATIC_COPY_ARB: c_uint = 0x88E6;
    pub const STATIC_DRAW: c_uint = 0x88E4;
    pub const STATIC_DRAW_ARB: c_uint = 0x88E4;
    pub const STATIC_READ: c_uint = 0x88E5;
    pub const STATIC_READ_ARB: c_uint = 0x88E5;
    pub const STENCIL: c_uint = 0x1802;
    pub const STENCIL_ATTACHMENT: c_uint = 0x8D20;
    pub const STENCIL_ATTACHMENT_EXT: c_uint = 0x8D20;
    pub const STENCIL_BACK_FAIL: c_uint = 0x8801;
    pub const STENCIL_BACK_FAIL_ATI: c_uint = 0x8801;
    pub const STENCIL_BACK_FUNC: c_uint = 0x8800;
    pub const STENCIL_BACK_FUNC_ATI: c_uint = 0x8800;
    pub const STENCIL_BACK_PASS_DEPTH_FAIL: c_uint = 0x8802;
    pub const STENCIL_BACK_PASS_DEPTH_FAIL_ATI: c_uint = 0x8802;
    pub const STENCIL_BACK_PASS_DEPTH_PASS: c_uint = 0x8803;
    pub const STENCIL_BACK_PASS_DEPTH_PASS_ATI: c_uint = 0x8803;
    pub const STENCIL_BACK_REF: c_uint = 0x8CA3;
    pub const STENCIL_BACK_VALUE_MASK: c_uint = 0x8CA4;
    pub const STENCIL_BACK_WRITEMASK: c_uint = 0x8CA5;
    pub const STENCIL_BUFFER_BIT: c_uint = 0x00000400;
    pub const STENCIL_CLEAR_VALUE: c_uint = 0x0B91;
    pub const STENCIL_COMPONENTS: c_uint = 0x8285;
    pub const STENCIL_FAIL: c_uint = 0x0B94;
    pub const STENCIL_FUNC: c_uint = 0x0B92;
    pub const STENCIL_INDEX: c_uint = 0x1901;
    pub const STENCIL_INDEX1: c_uint = 0x8D46;
    pub const STENCIL_INDEX16: c_uint = 0x8D49;
    pub const STENCIL_INDEX16_EXT: c_uint = 0x8D49;
    pub const STENCIL_INDEX1_EXT: c_uint = 0x8D46;
    pub const STENCIL_INDEX4: c_uint = 0x8D47;
    pub const STENCIL_INDEX4_EXT: c_uint = 0x8D47;
    pub const STENCIL_INDEX8: c_uint = 0x8D48;
    pub const STENCIL_INDEX8_EXT: c_uint = 0x8D48;
    pub const STENCIL_PASS_DEPTH_FAIL: c_uint = 0x0B95;
    pub const STENCIL_PASS_DEPTH_PASS: c_uint = 0x0B96;
    pub const STENCIL_REF: c_uint = 0x0B97;
    pub const STENCIL_RENDERABLE: c_uint = 0x8288;
    pub const STENCIL_TEST: c_uint = 0x0B90;
    pub const STENCIL_VALUE_MASK: c_uint = 0x0B93;
    pub const STENCIL_WRITEMASK: c_uint = 0x0B98;
    pub const STEREO: c_uint = 0x0C33;
    pub const STREAM_COPY: c_uint = 0x88E2;
    pub const STREAM_COPY_ARB: c_uint = 0x88E2;
    pub const STREAM_DRAW: c_uint = 0x88E0;
    pub const STREAM_DRAW_ARB: c_uint = 0x88E0;
    pub const STREAM_READ: c_uint = 0x88E1;
    pub const STREAM_READ_ARB: c_uint = 0x88E1;
    pub const SUBPIXEL_BITS: c_uint = 0x0D50;
    pub const SYNC_CONDITION: c_uint = 0x9113;
    pub const SYNC_FENCE: c_uint = 0x9116;
    pub const SYNC_FLAGS: c_uint = 0x9115;
    pub const SYNC_FLUSH_COMMANDS_BIT: c_uint = 0x00000001;
    pub const SYNC_GPU_COMMANDS_COMPLETE: c_uint = 0x9117;
    pub const SYNC_STATUS: c_uint = 0x9114;
    pub const TESS_CONTROL_OUTPUT_VERTICES: c_uint = 0x8E75;
    pub const TESS_CONTROL_SHADER: c_uint = 0x8E88;
    pub const TESS_CONTROL_SHADER_BIT: c_uint = 0x00000008;
    pub const TESS_CONTROL_SHADER_PATCHES: c_uint = 0x82F1;
    pub const TESS_CONTROL_SUBROUTINE: c_uint = 0x92E9;
    pub const TESS_CONTROL_SUBROUTINE_UNIFORM: c_uint = 0x92EF;
    pub const TESS_CONTROL_TEXTURE: c_uint = 0x829C;
    pub const TESS_EVALUATION_SHADER: c_uint = 0x8E87;
    pub const TESS_EVALUATION_SHADER_BIT: c_uint = 0x00000010;
    pub const TESS_EVALUATION_SHADER_INVOCATIONS: c_uint = 0x82F2;
    pub const TESS_EVALUATION_SUBROUTINE: c_uint = 0x92EA;
    pub const TESS_EVALUATION_SUBROUTINE_UNIFORM: c_uint = 0x92F0;
    pub const TESS_EVALUATION_TEXTURE: c_uint = 0x829D;
    pub const TESS_GEN_MODE: c_uint = 0x8E76;
    pub const TESS_GEN_POINT_MODE: c_uint = 0x8E79;
    pub const TESS_GEN_SPACING: c_uint = 0x8E77;
    pub const TESS_GEN_VERTEX_ORDER: c_uint = 0x8E78;
    pub const TEXTURE: c_uint = 0x1702;
    pub const TEXTURE0: c_uint = 0x84C0;
    pub const TEXTURE0_ARB: c_uint = 0x84C0;
    pub const TEXTURE1: c_uint = 0x84C1;
    pub const TEXTURE10: c_uint = 0x84CA;
    pub const TEXTURE10_ARB: c_uint = 0x84CA;
    pub const TEXTURE11: c_uint = 0x84CB;
    pub const TEXTURE11_ARB: c_uint = 0x84CB;
    pub const TEXTURE12: c_uint = 0x84CC;
    pub const TEXTURE12_ARB: c_uint = 0x84CC;
    pub const TEXTURE13: c_uint = 0x84CD;
    pub const TEXTURE13_ARB: c_uint = 0x84CD;
    pub const TEXTURE14: c_uint = 0x84CE;
    pub const TEXTURE14_ARB: c_uint = 0x84CE;
    pub const TEXTURE15: c_uint = 0x84CF;
    pub const TEXTURE15_ARB: c_uint = 0x84CF;
    pub const TEXTURE16: c_uint = 0x84D0;
    pub const TEXTURE16_ARB: c_uint = 0x84D0;
    pub const TEXTURE17: c_uint = 0x84D1;
    pub const TEXTURE17_ARB: c_uint = 0x84D1;
    pub const TEXTURE18: c_uint = 0x84D2;
    pub const TEXTURE18_ARB: c_uint = 0x84D2;
    pub const TEXTURE19: c_uint = 0x84D3;
    pub const TEXTURE19_ARB: c_uint = 0x84D3;
    pub const TEXTURE1_ARB: c_uint = 0x84C1;
    pub const TEXTURE2: c_uint = 0x84C2;
    pub const TEXTURE20: c_uint = 0x84D4;
    pub const TEXTURE20_ARB: c_uint = 0x84D4;
    pub const TEXTURE21: c_uint = 0x84D5;
    pub const TEXTURE21_ARB: c_uint = 0x84D5;
    pub const TEXTURE22: c_uint = 0x84D6;
    pub const TEXTURE22_ARB: c_uint = 0x84D6;
    pub const TEXTURE23: c_uint = 0x84D7;
    pub const TEXTURE23_ARB: c_uint = 0x84D7;
    pub const TEXTURE24: c_uint = 0x84D8;
    pub const TEXTURE24_ARB: c_uint = 0x84D8;
    pub const TEXTURE25: c_uint = 0x84D9;
    pub const TEXTURE25_ARB: c_uint = 0x84D9;
    pub const TEXTURE26: c_uint = 0x84DA;
    pub const TEXTURE26_ARB: c_uint = 0x84DA;
    pub const TEXTURE27: c_uint = 0x84DB;
    pub const TEXTURE27_ARB: c_uint = 0x84DB;
    pub const TEXTURE28: c_uint = 0x84DC;
    pub const TEXTURE28_ARB: c_uint = 0x84DC;
    pub const TEXTURE29: c_uint = 0x84DD;
    pub const TEXTURE29_ARB: c_uint = 0x84DD;
    pub const TEXTURE2_ARB: c_uint = 0x84C2;
    pub const TEXTURE3: c_uint = 0x84C3;
    pub const TEXTURE30: c_uint = 0x84DE;
    pub const TEXTURE30_ARB: c_uint = 0x84DE;
    pub const TEXTURE31: c_uint = 0x84DF;
    pub const TEXTURE31_ARB: c_uint = 0x84DF;
    pub const TEXTURE3_ARB: c_uint = 0x84C3;
    pub const TEXTURE4: c_uint = 0x84C4;
    pub const TEXTURE4_ARB: c_uint = 0x84C4;
    pub const TEXTURE5: c_uint = 0x84C5;
    pub const TEXTURE5_ARB: c_uint = 0x84C5;
    pub const TEXTURE6: c_uint = 0x84C6;
    pub const TEXTURE6_ARB: c_uint = 0x84C6;
    pub const TEXTURE7: c_uint = 0x84C7;
    pub const TEXTURE7_ARB: c_uint = 0x84C7;
    pub const TEXTURE8: c_uint = 0x84C8;
    pub const TEXTURE8_ARB: c_uint = 0x84C8;
    pub const TEXTURE9: c_uint = 0x84C9;
    pub const TEXTURE9_ARB: c_uint = 0x84C9;
    pub const TEXTURE_1D: c_uint = 0x0DE0;
    pub const TEXTURE_1D_ARRAY: c_uint = 0x8C18;
    pub const TEXTURE_1D_ARRAY_EXT: c_uint = 0x8C18;
    pub const TEXTURE_1D_BINDING_EXT: c_uint = 0x8068;
    pub const TEXTURE_2D: c_uint = 0x0DE1;
    pub const TEXTURE_2D_ARRAY: c_uint = 0x8C1A;
    pub const TEXTURE_2D_ARRAY_EXT: c_uint = 0x8C1A;
    pub const TEXTURE_2D_BINDING_EXT: c_uint = 0x8069;
    pub const TEXTURE_2D_MULTISAMPLE: c_uint = 0x9100;
    pub const TEXTURE_2D_MULTISAMPLE_ARRAY: c_uint = 0x9102;
    pub const TEXTURE_3D: c_uint = 0x806F;
    pub const TEXTURE_3D_BINDING_EXT: c_uint = 0x806A;
    pub const TEXTURE_3D_EXT: c_uint = 0x806F;
    pub const TEXTURE_ALPHA_SIZE: c_uint = 0x805F;
    pub const TEXTURE_ALPHA_TYPE: c_uint = 0x8C13;
    pub const TEXTURE_BASE_LEVEL: c_uint = 0x813C;
    pub const TEXTURE_BINDING_1D: c_uint = 0x8068;
    pub const TEXTURE_BINDING_1D_ARRAY: c_uint = 0x8C1C;
    pub const TEXTURE_BINDING_1D_ARRAY_EXT: c_uint = 0x8C1C;
    pub const TEXTURE_BINDING_2D: c_uint = 0x8069;
    pub const TEXTURE_BINDING_2D_ARRAY: c_uint = 0x8C1D;
    pub const TEXTURE_BINDING_2D_ARRAY_EXT: c_uint = 0x8C1D;
    pub const TEXTURE_BINDING_2D_MULTISAMPLE: c_uint = 0x9104;
    pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: c_uint = 0x9105;
    pub const TEXTURE_BINDING_3D: c_uint = 0x806A;
    pub const TEXTURE_BINDING_BUFFER: c_uint = 0x8C2C;
    pub const TEXTURE_BINDING_BUFFER_ARB: c_uint = 0x8C2C;
    pub const TEXTURE_BINDING_BUFFER_EXT: c_uint = 0x8C2C;
    pub const TEXTURE_BINDING_CUBE_MAP: c_uint = 0x8514;
    pub const TEXTURE_BINDING_CUBE_MAP_ARRAY: c_uint = 0x900A;
    pub const TEXTURE_BINDING_RECTANGLE: c_uint = 0x84F6;
    pub const TEXTURE_BINDING_RENDERBUFFER_NV: c_uint = 0x8E53;
    pub const TEXTURE_BLUE_SIZE: c_uint = 0x805E;
    pub const TEXTURE_BLUE_TYPE: c_uint = 0x8C12;
    pub const TEXTURE_BORDER_COLOR: c_uint = 0x1004;
    pub const TEXTURE_BUFFER: c_uint = 0x8C2A;
    pub const TEXTURE_BUFFER_ARB: c_uint = 0x8C2A;
    pub const TEXTURE_BUFFER_BINDING: c_uint = 0x8C2A;
    pub const TEXTURE_BUFFER_DATA_STORE_BINDING: c_uint = 0x8C2D;
    pub const TEXTURE_BUFFER_DATA_STORE_BINDING_ARB: c_uint = 0x8C2D;
    pub const TEXTURE_BUFFER_DATA_STORE_BINDING_EXT: c_uint = 0x8C2D;
    pub const TEXTURE_BUFFER_EXT: c_uint = 0x8C2A;
    pub const TEXTURE_BUFFER_FORMAT_ARB: c_uint = 0x8C2E;
    pub const TEXTURE_BUFFER_FORMAT_EXT: c_uint = 0x8C2E;
    pub const TEXTURE_BUFFER_OFFSET: c_uint = 0x919D;
    pub const TEXTURE_BUFFER_OFFSET_ALIGNMENT: c_uint = 0x919F;
    pub const TEXTURE_BUFFER_SIZE: c_uint = 0x919E;
    pub const TEXTURE_COMPARE_FUNC: c_uint = 0x884D;
    pub const TEXTURE_COMPARE_MODE: c_uint = 0x884C;
    pub const TEXTURE_COMPRESSED: c_uint = 0x86A1;
    pub const TEXTURE_COMPRESSED_ARB: c_uint = 0x86A1;
    pub const TEXTURE_COMPRESSED_BLOCK_HEIGHT: c_uint = 0x82B2;
    pub const TEXTURE_COMPRESSED_BLOCK_SIZE: c_uint = 0x82B3;
    pub const TEXTURE_COMPRESSED_BLOCK_WIDTH: c_uint = 0x82B1;
    pub const TEXTURE_COMPRESSED_IMAGE_SIZE: c_uint = 0x86A0;
    pub const TEXTURE_COMPRESSED_IMAGE_SIZE_ARB: c_uint = 0x86A0;
    pub const TEXTURE_COMPRESSION_HINT: c_uint = 0x84EF;
    pub const TEXTURE_COMPRESSION_HINT_ARB: c_uint = 0x84EF;
    pub const TEXTURE_COORD_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x889A;
    pub const TEXTURE_COORD_ARRAY_COUNT_EXT: c_uint = 0x808B;
    pub const TEXTURE_COORD_ARRAY_EXT: c_uint = 0x8078;
    pub const TEXTURE_COORD_ARRAY_POINTER_EXT: c_uint = 0x8092;
    pub const TEXTURE_COORD_ARRAY_SIZE_EXT: c_uint = 0x8088;
    pub const TEXTURE_COORD_ARRAY_STRIDE_EXT: c_uint = 0x808A;
    pub const TEXTURE_COORD_ARRAY_TYPE_EXT: c_uint = 0x8089;
    pub const TEXTURE_COORD_NV: c_uint = 0x8C79;
    pub const TEXTURE_CUBE_MAP: c_uint = 0x8513;
    pub const TEXTURE_CUBE_MAP_ARRAY: c_uint = 0x9009;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X: c_uint = 0x8516;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: c_uint = 0x8518;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: c_uint = 0x851A;
    pub const TEXTURE_CUBE_MAP_POSITIVE_X: c_uint = 0x8515;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y: c_uint = 0x8517;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z: c_uint = 0x8519;
    pub const TEXTURE_CUBE_MAP_SEAMLESS: c_uint = 0x884F;
    pub const TEXTURE_DEPTH: c_uint = 0x8071;
    pub const TEXTURE_DEPTH_EXT: c_uint = 0x8071;
    pub const TEXTURE_DEPTH_SIZE: c_uint = 0x884A;
    pub const TEXTURE_DEPTH_TYPE: c_uint = 0x8C16;
    pub const TEXTURE_FETCH_BARRIER_BIT: c_uint = 0x00000008;
    pub const TEXTURE_FETCH_BARRIER_BIT_EXT: c_uint = 0x00000008;
    pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: c_uint = 0x9107;
    pub const TEXTURE_GATHER: c_uint = 0x82A2;
    pub const TEXTURE_GATHER_SHADOW: c_uint = 0x82A3;
    pub const TEXTURE_GREEN_SIZE: c_uint = 0x805D;
    pub const TEXTURE_GREEN_TYPE: c_uint = 0x8C11;
    pub const TEXTURE_HEIGHT: c_uint = 0x1001;
    pub const TEXTURE_IMAGE_FORMAT: c_uint = 0x828F;
    pub const TEXTURE_IMAGE_TYPE: c_uint = 0x8290;
    pub const TEXTURE_IMMUTABLE_FORMAT: c_uint = 0x912F;
    pub const TEXTURE_IMMUTABLE_FORMAT_EXT: c_uint = 0x912F;
    pub const TEXTURE_IMMUTABLE_LEVELS: c_uint = 0x82DF;
    pub const TEXTURE_INTERNAL_FORMAT: c_uint = 0x1003;
    pub const TEXTURE_LOD_BIAS: c_uint = 0x8501;
    pub const TEXTURE_MAG_FILTER: c_uint = 0x2800;
    pub const TEXTURE_MAX_ANISOTROPY: c_uint = 0x84FE;
    pub const TEXTURE_MAX_LEVEL: c_uint = 0x813D;
    pub const TEXTURE_MAX_LOD: c_uint = 0x813B;
    pub const TEXTURE_MIN_FILTER: c_uint = 0x2801;
    pub const TEXTURE_MIN_LOD: c_uint = 0x813A;
    pub const TEXTURE_PRIORITY_EXT: c_uint = 0x8066;
    pub const TEXTURE_RECTANGLE: c_uint = 0x84F5;
    pub const TEXTURE_RED_SIZE: c_uint = 0x805C;
    pub const TEXTURE_RED_TYPE: c_uint = 0x8C10;
    pub const TEXTURE_RENDERBUFFER_DATA_STORE_BINDING_NV: c_uint = 0x8E54;
    pub const TEXTURE_RENDERBUFFER_NV: c_uint = 0x8E55;
    pub const TEXTURE_RESIDENT_EXT: c_uint = 0x8067;
    pub const TEXTURE_SAMPLES: c_uint = 0x9106;
    pub const TEXTURE_SHADOW: c_uint = 0x82A1;
    pub const TEXTURE_SHARED_SIZE: c_uint = 0x8C3F;
    pub const TEXTURE_STENCIL_SIZE: c_uint = 0x88F1;
    pub const TEXTURE_SWIZZLE_A: c_uint = 0x8E45;
    pub const TEXTURE_SWIZZLE_B: c_uint = 0x8E44;
    pub const TEXTURE_SWIZZLE_G: c_uint = 0x8E43;
    pub const TEXTURE_SWIZZLE_R: c_uint = 0x8E42;
    pub const TEXTURE_SWIZZLE_RGBA: c_uint = 0x8E46;
    pub const TEXTURE_TARGET: c_uint = 0x1006;
    pub const TEXTURE_UPDATE_BARRIER_BIT: c_uint = 0x00000100;
    pub const TEXTURE_UPDATE_BARRIER_BIT_EXT: c_uint = 0x00000100;
    pub const TEXTURE_VIEW: c_uint = 0x82B5;
    pub const TEXTURE_VIEW_MIN_LAYER: c_uint = 0x82DD;
    pub const TEXTURE_VIEW_MIN_LEVEL: c_uint = 0x82DB;
    pub const TEXTURE_VIEW_NUM_LAYERS: c_uint = 0x82DE;
    pub const TEXTURE_VIEW_NUM_LEVELS: c_uint = 0x82DC;
    pub const TEXTURE_WIDTH: c_uint = 0x1000;
    pub const TEXTURE_WRAP_R: c_uint = 0x8072;
    pub const TEXTURE_WRAP_R_EXT: c_uint = 0x8072;
    pub const TEXTURE_WRAP_S: c_uint = 0x2802;
    pub const TEXTURE_WRAP_T: c_uint = 0x2803;
    pub const TIMEOUT_EXPIRED: c_uint = 0x911B;
    pub const TIMEOUT_IGNORED: u64 = 0xFFFFFFFFFFFFFFFF;
    pub const TIMESTAMP: c_uint = 0x8E28;
    pub const TIME_ELAPSED: c_uint = 0x88BF;
    pub const TIME_ELAPSED_EXT: c_uint = 0x88BF;
    pub const TOP_LEVEL_ARRAY_SIZE: c_uint = 0x930C;
    pub const TOP_LEVEL_ARRAY_STRIDE: c_uint = 0x930D;
    pub const TRACK_MATRIX_NV: c_uint = 0x8648;
    pub const TRACK_MATRIX_TRANSFORM_NV: c_uint = 0x8649;
    pub const TRANSFORM_FEEDBACK: c_uint = 0x8E22;
    pub const TRANSFORM_FEEDBACK_ACTIVE: c_uint = 0x8E24;
    pub const TRANSFORM_FEEDBACK_ATTRIBS_NV: c_uint = 0x8C7E;
    pub const TRANSFORM_FEEDBACK_BARRIER_BIT: c_uint = 0x00000800;
    pub const TRANSFORM_FEEDBACK_BARRIER_BIT_EXT: c_uint = 0x00000800;
    pub const TRANSFORM_FEEDBACK_BINDING: c_uint = 0x8E25;
    pub const TRANSFORM_FEEDBACK_BINDING_NV: c_uint = 0x8E25;
    pub const TRANSFORM_FEEDBACK_BUFFER: c_uint = 0x8C8E;
    pub const TRANSFORM_FEEDBACK_BUFFER_ACTIVE: c_uint = 0x8E24;
    pub const TRANSFORM_FEEDBACK_BUFFER_ACTIVE_NV: c_uint = 0x8E24;
    pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: c_uint = 0x8C8F;
    pub const TRANSFORM_FEEDBACK_BUFFER_BINDING_EXT: c_uint = 0x8C8F;
    pub const TRANSFORM_FEEDBACK_BUFFER_BINDING_NV: c_uint = 0x8C8F;
    pub const TRANSFORM_FEEDBACK_BUFFER_EXT: c_uint = 0x8C8E;
    pub const TRANSFORM_FEEDBACK_BUFFER_INDEX: c_uint = 0x934B;
    pub const TRANSFORM_FEEDBACK_BUFFER_MODE: c_uint = 0x8C7F;
    pub const TRANSFORM_FEEDBACK_BUFFER_MODE_EXT: c_uint = 0x8C7F;
    pub const TRANSFORM_FEEDBACK_BUFFER_MODE_NV: c_uint = 0x8C7F;
    pub const TRANSFORM_FEEDBACK_BUFFER_NV: c_uint = 0x8C8E;
    pub const TRANSFORM_FEEDBACK_BUFFER_PAUSED: c_uint = 0x8E23;
    pub const TRANSFORM_FEEDBACK_BUFFER_PAUSED_NV: c_uint = 0x8E23;
    pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: c_uint = 0x8C85;
    pub const TRANSFORM_FEEDBACK_BUFFER_SIZE_EXT: c_uint = 0x8C85;
    pub const TRANSFORM_FEEDBACK_BUFFER_SIZE_NV: c_uint = 0x8C85;
    pub const TRANSFORM_FEEDBACK_BUFFER_START: c_uint = 0x8C84;
    pub const TRANSFORM_FEEDBACK_BUFFER_START_EXT: c_uint = 0x8C84;
    pub const TRANSFORM_FEEDBACK_BUFFER_START_NV: c_uint = 0x8C84;
    pub const TRANSFORM_FEEDBACK_BUFFER_STRIDE: c_uint = 0x934C;
    pub const TRANSFORM_FEEDBACK_NV: c_uint = 0x8E22;
    pub const TRANSFORM_FEEDBACK_OVERFLOW: c_uint = 0x82EC;
    pub const TRANSFORM_FEEDBACK_PAUSED: c_uint = 0x8E23;
    pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: c_uint = 0x8C88;
    pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN_EXT: c_uint = 0x8C88;
    pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN_NV: c_uint = 0x8C88;
    pub const TRANSFORM_FEEDBACK_RECORD_NV: c_uint = 0x8C86;
    pub const TRANSFORM_FEEDBACK_STREAM_OVERFLOW: c_uint = 0x82ED;
    pub const TRANSFORM_FEEDBACK_VARYING: c_uint = 0x92F4;
    pub const TRANSFORM_FEEDBACK_VARYINGS: c_uint = 0x8C83;
    pub const TRANSFORM_FEEDBACK_VARYINGS_EXT: c_uint = 0x8C83;
    pub const TRANSFORM_FEEDBACK_VARYINGS_NV: c_uint = 0x8C83;
    pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: c_uint = 0x8C76;
    pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH_EXT: c_uint = 0x8C76;
    pub const TRANSPOSE_CURRENT_MATRIX_ARB: c_uint = 0x88B7;
    pub const TRANSPOSE_NV: c_uint = 0x862C;
    pub const TRANSPOSE_PROGRAM_MATRIX_EXT: c_uint = 0x8E2E;
    pub const TRIANGLES: c_uint = 0x0004;
    pub const TRIANGLES_ADJACENCY: c_uint = 0x000C;
    pub const TRIANGLES_ADJACENCY_ARB: c_uint = 0x000C;
    pub const TRIANGLES_ADJACENCY_EXT: c_uint = 0x000C;
    pub const TRIANGLE_FAN: c_uint = 0x0006;
    pub const TRIANGLE_STRIP: c_uint = 0x0005;
    pub const TRIANGLE_STRIP_ADJACENCY: c_uint = 0x000D;
    pub const TRIANGLE_STRIP_ADJACENCY_ARB: c_uint = 0x000D;
    pub const TRIANGLE_STRIP_ADJACENCY_EXT: c_uint = 0x000D;
    pub const TRUE: c_uchar = 1;
    pub const TYPE: c_uint = 0x92FA;
    pub const UNDEFINED_VERTEX: c_uint = 0x8260;
    pub const UNIFORM: c_uint = 0x92E1;
    pub const UNIFORM_ARRAY_STRIDE: c_uint = 0x8A3C;
    pub const UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: c_uint = 0x92DA;
    pub const UNIFORM_BARRIER_BIT: c_uint = 0x00000004;
    pub const UNIFORM_BARRIER_BIT_EXT: c_uint = 0x00000004;
    pub const UNIFORM_BLOCK: c_uint = 0x92E2;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: c_uint = 0x8A42;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: c_uint = 0x8A43;
    pub const UNIFORM_BLOCK_BINDING: c_uint = 0x8A3F;
    pub const UNIFORM_BLOCK_DATA_SIZE: c_uint = 0x8A40;
    pub const UNIFORM_BLOCK_INDEX: c_uint = 0x8A3A;
    pub const UNIFORM_BLOCK_NAME_LENGTH: c_uint = 0x8A41;
    pub const UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: c_uint = 0x90EC;
    pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: c_uint = 0x8A46;
    pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: c_uint = 0x8A45;
    pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: c_uint = 0x84F0;
    pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: c_uint = 0x84F1;
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
    pub const UNKNOWN_CONTEXT_RESET: c_uint = 0x8255;
    pub const UNKNOWN_CONTEXT_RESET_ARB: c_uint = 0x8255;
    pub const UNPACK_ALIGNMENT: c_uint = 0x0CF5;
    pub const UNPACK_COMPRESSED_BLOCK_DEPTH: c_uint = 0x9129;
    pub const UNPACK_COMPRESSED_BLOCK_HEIGHT: c_uint = 0x9128;
    pub const UNPACK_COMPRESSED_BLOCK_SIZE: c_uint = 0x912A;
    pub const UNPACK_COMPRESSED_BLOCK_WIDTH: c_uint = 0x9127;
    pub const UNPACK_IMAGE_HEIGHT: c_uint = 0x806E;
    pub const UNPACK_IMAGE_HEIGHT_EXT: c_uint = 0x806E;
    pub const UNPACK_LSB_FIRST: c_uint = 0x0CF1;
    pub const UNPACK_ROW_LENGTH: c_uint = 0x0CF2;
    pub const UNPACK_SKIP_IMAGES: c_uint = 0x806D;
    pub const UNPACK_SKIP_IMAGES_EXT: c_uint = 0x806D;
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
    pub const UNSIGNED_INT_ATOMIC_COUNTER: c_uint = 0x92DB;
    pub const UNSIGNED_INT_IMAGE_1D: c_uint = 0x9062;
    pub const UNSIGNED_INT_IMAGE_1D_ARRAY: c_uint = 0x9068;
    pub const UNSIGNED_INT_IMAGE_1D_ARRAY_EXT: c_uint = 0x9068;
    pub const UNSIGNED_INT_IMAGE_1D_EXT: c_uint = 0x9062;
    pub const UNSIGNED_INT_IMAGE_2D: c_uint = 0x9063;
    pub const UNSIGNED_INT_IMAGE_2D_ARRAY: c_uint = 0x9069;
    pub const UNSIGNED_INT_IMAGE_2D_ARRAY_EXT: c_uint = 0x9069;
    pub const UNSIGNED_INT_IMAGE_2D_EXT: c_uint = 0x9063;
    pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: c_uint = 0x906B;
    pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: c_uint = 0x906C;
    pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY_EXT: c_uint = 0x906C;
    pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_EXT: c_uint = 0x906B;
    pub const UNSIGNED_INT_IMAGE_2D_RECT: c_uint = 0x9065;
    pub const UNSIGNED_INT_IMAGE_2D_RECT_EXT: c_uint = 0x9065;
    pub const UNSIGNED_INT_IMAGE_3D: c_uint = 0x9064;
    pub const UNSIGNED_INT_IMAGE_3D_EXT: c_uint = 0x9064;
    pub const UNSIGNED_INT_IMAGE_BUFFER: c_uint = 0x9067;
    pub const UNSIGNED_INT_IMAGE_BUFFER_EXT: c_uint = 0x9067;
    pub const UNSIGNED_INT_IMAGE_CUBE: c_uint = 0x9066;
    pub const UNSIGNED_INT_IMAGE_CUBE_EXT: c_uint = 0x9066;
    pub const UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: c_uint = 0x906A;
    pub const UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY_EXT: c_uint = 0x906A;
    pub const UNSIGNED_INT_SAMPLER_1D: c_uint = 0x8DD1;
    pub const UNSIGNED_INT_SAMPLER_1D_ARRAY: c_uint = 0x8DD6;
    pub const UNSIGNED_INT_SAMPLER_1D_ARRAY_EXT: c_uint = 0x8DD6;
    pub const UNSIGNED_INT_SAMPLER_1D_EXT: c_uint = 0x8DD1;
    pub const UNSIGNED_INT_SAMPLER_2D: c_uint = 0x8DD2;
    pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: c_uint = 0x8DD7;
    pub const UNSIGNED_INT_SAMPLER_2D_ARRAY_EXT: c_uint = 0x8DD7;
    pub const UNSIGNED_INT_SAMPLER_2D_EXT: c_uint = 0x8DD2;
    pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: c_uint = 0x910A;
    pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: c_uint = 0x910D;
    pub const UNSIGNED_INT_SAMPLER_2D_RECT: c_uint = 0x8DD5;
    pub const UNSIGNED_INT_SAMPLER_2D_RECT_EXT: c_uint = 0x8DD5;
    pub const UNSIGNED_INT_SAMPLER_3D: c_uint = 0x8DD3;
    pub const UNSIGNED_INT_SAMPLER_3D_EXT: c_uint = 0x8DD3;
    pub const UNSIGNED_INT_SAMPLER_BUFFER: c_uint = 0x8DD8;
    pub const UNSIGNED_INT_SAMPLER_BUFFER_EXT: c_uint = 0x8DD8;
    pub const UNSIGNED_INT_SAMPLER_CUBE: c_uint = 0x8DD4;
    pub const UNSIGNED_INT_SAMPLER_CUBE_EXT: c_uint = 0x8DD4;
    pub const UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: c_uint = 0x900F;
    pub const UNSIGNED_INT_SAMPLER_RENDERBUFFER_NV: c_uint = 0x8E58;
    pub const UNSIGNED_INT_VEC2: c_uint = 0x8DC6;
    pub const UNSIGNED_INT_VEC2_EXT: c_uint = 0x8DC6;
    pub const UNSIGNED_INT_VEC3: c_uint = 0x8DC7;
    pub const UNSIGNED_INT_VEC3_EXT: c_uint = 0x8DC7;
    pub const UNSIGNED_INT_VEC4: c_uint = 0x8DC8;
    pub const UNSIGNED_INT_VEC4_EXT: c_uint = 0x8DC8;
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
    pub const VERTEX_ARRAY: c_uint = 0x8074;
    pub const VERTEX_ARRAY_BINDING: c_uint = 0x85B5;
    pub const VERTEX_ARRAY_BINDING_APPLE: c_uint = 0x85B5;
    pub const VERTEX_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x8896;
    pub const VERTEX_ARRAY_COUNT_EXT: c_uint = 0x807D;
    pub const VERTEX_ARRAY_EXT: c_uint = 0x8074;
    pub const VERTEX_ARRAY_POINTER_EXT: c_uint = 0x808E;
    pub const VERTEX_ARRAY_SIZE_EXT: c_uint = 0x807A;
    pub const VERTEX_ARRAY_STRIDE_EXT: c_uint = 0x807C;
    pub const VERTEX_ARRAY_TYPE_EXT: c_uint = 0x807B;
    pub const VERTEX_ATTRIB_ARRAY0_NV: c_uint = 0x8650;
    pub const VERTEX_ATTRIB_ARRAY10_NV: c_uint = 0x865A;
    pub const VERTEX_ATTRIB_ARRAY11_NV: c_uint = 0x865B;
    pub const VERTEX_ATTRIB_ARRAY12_NV: c_uint = 0x865C;
    pub const VERTEX_ATTRIB_ARRAY13_NV: c_uint = 0x865D;
    pub const VERTEX_ATTRIB_ARRAY14_NV: c_uint = 0x865E;
    pub const VERTEX_ATTRIB_ARRAY15_NV: c_uint = 0x865F;
    pub const VERTEX_ATTRIB_ARRAY1_NV: c_uint = 0x8651;
    pub const VERTEX_ATTRIB_ARRAY2_NV: c_uint = 0x8652;
    pub const VERTEX_ATTRIB_ARRAY3_NV: c_uint = 0x8653;
    pub const VERTEX_ATTRIB_ARRAY4_NV: c_uint = 0x8654;
    pub const VERTEX_ATTRIB_ARRAY5_NV: c_uint = 0x8655;
    pub const VERTEX_ATTRIB_ARRAY6_NV: c_uint = 0x8656;
    pub const VERTEX_ATTRIB_ARRAY7_NV: c_uint = 0x8657;
    pub const VERTEX_ATTRIB_ARRAY8_NV: c_uint = 0x8658;
    pub const VERTEX_ATTRIB_ARRAY9_NV: c_uint = 0x8659;
    pub const VERTEX_ATTRIB_ARRAY_BARRIER_BIT: c_uint = 0x00000001;
    pub const VERTEX_ATTRIB_ARRAY_BARRIER_BIT_EXT: c_uint = 0x00000001;
    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: c_uint = 0x889F;
    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x889F;
    pub const VERTEX_ATTRIB_ARRAY_DIVISOR: c_uint = 0x88FE;
    pub const VERTEX_ATTRIB_ARRAY_DIVISOR_ARB: c_uint = 0x88FE;
    pub const VERTEX_ATTRIB_ARRAY_ENABLED: c_uint = 0x8622;
    pub const VERTEX_ATTRIB_ARRAY_ENABLED_ARB: c_uint = 0x8622;
    pub const VERTEX_ATTRIB_ARRAY_INTEGER: c_uint = 0x88FD;
    pub const VERTEX_ATTRIB_ARRAY_INTEGER_EXT: c_uint = 0x88FD;
    pub const VERTEX_ATTRIB_ARRAY_INTEGER_NV: c_uint = 0x88FD;
    pub const VERTEX_ATTRIB_ARRAY_LONG: c_uint = 0x874E;
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: c_uint = 0x886A;
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED_ARB: c_uint = 0x886A;
    pub const VERTEX_ATTRIB_ARRAY_POINTER: c_uint = 0x8645;
    pub const VERTEX_ATTRIB_ARRAY_POINTER_ARB: c_uint = 0x8645;
    pub const VERTEX_ATTRIB_ARRAY_SIZE: c_uint = 0x8623;
    pub const VERTEX_ATTRIB_ARRAY_SIZE_ARB: c_uint = 0x8623;
    pub const VERTEX_ATTRIB_ARRAY_STRIDE: c_uint = 0x8624;
    pub const VERTEX_ATTRIB_ARRAY_STRIDE_ARB: c_uint = 0x8624;
    pub const VERTEX_ATTRIB_ARRAY_TYPE: c_uint = 0x8625;
    pub const VERTEX_ATTRIB_ARRAY_TYPE_ARB: c_uint = 0x8625;
    pub const VERTEX_ATTRIB_BINDING: c_uint = 0x82D4;
    pub const VERTEX_ATTRIB_RELATIVE_OFFSET: c_uint = 0x82D5;
    pub const VERTEX_BINDING_BUFFER: c_uint = 0x8F4F;
    pub const VERTEX_BINDING_DIVISOR: c_uint = 0x82D6;
    pub const VERTEX_BINDING_OFFSET: c_uint = 0x82D7;
    pub const VERTEX_BINDING_STRIDE: c_uint = 0x82D8;
    pub const VERTEX_ID_NV: c_uint = 0x8C7B;
    pub const VERTEX_PROGRAM_ARB: c_uint = 0x8620;
    pub const VERTEX_PROGRAM_BINDING_NV: c_uint = 0x864A;
    pub const VERTEX_PROGRAM_NV: c_uint = 0x8620;
    pub const VERTEX_PROGRAM_POINT_SIZE: c_uint = 0x8642;
    pub const VERTEX_PROGRAM_POINT_SIZE_ARB: c_uint = 0x8642;
    pub const VERTEX_PROGRAM_POINT_SIZE_NV: c_uint = 0x8642;
    pub const VERTEX_PROGRAM_TWO_SIDE_ARB: c_uint = 0x8643;
    pub const VERTEX_PROGRAM_TWO_SIDE_NV: c_uint = 0x8643;
    pub const VERTEX_SHADER: c_uint = 0x8B31;
    pub const VERTEX_SHADER_ARB: c_uint = 0x8B31;
    pub const VERTEX_SHADER_BIT: c_uint = 0x00000001;
    pub const VERTEX_SHADER_INVOCATIONS: c_uint = 0x82F0;
    pub const VERTEX_STATE_PROGRAM_NV: c_uint = 0x8621;
    pub const VERTEX_SUBROUTINE: c_uint = 0x92E8;
    pub const VERTEX_SUBROUTINE_UNIFORM: c_uint = 0x92EE;
    pub const VERTEX_TEXTURE: c_uint = 0x829B;
    pub const VERTICES_SUBMITTED: c_uint = 0x82EE;
    pub const VIEWPORT: c_uint = 0x0BA2;
    pub const VIEWPORT_BOUNDS_RANGE: c_uint = 0x825D;
    pub const VIEWPORT_INDEX_PROVOKING_VERTEX: c_uint = 0x825F;
    pub const VIEWPORT_SUBPIXEL_BITS: c_uint = 0x825C;
    pub const VIEW_CLASS_128_BITS: c_uint = 0x82C4;
    pub const VIEW_CLASS_16_BITS: c_uint = 0x82CA;
    pub const VIEW_CLASS_24_BITS: c_uint = 0x82C9;
    pub const VIEW_CLASS_32_BITS: c_uint = 0x82C8;
    pub const VIEW_CLASS_48_BITS: c_uint = 0x82C7;
    pub const VIEW_CLASS_64_BITS: c_uint = 0x82C6;
    pub const VIEW_CLASS_8_BITS: c_uint = 0x82CB;
    pub const VIEW_CLASS_96_BITS: c_uint = 0x82C5;
    pub const VIEW_CLASS_ASTC_10x10_RGBA: c_uint = 0x9393;
    pub const VIEW_CLASS_ASTC_10x5_RGBA: c_uint = 0x9390;
    pub const VIEW_CLASS_ASTC_10x6_RGBA: c_uint = 0x9391;
    pub const VIEW_CLASS_ASTC_10x8_RGBA: c_uint = 0x9392;
    pub const VIEW_CLASS_ASTC_12x10_RGBA: c_uint = 0x9394;
    pub const VIEW_CLASS_ASTC_12x12_RGBA: c_uint = 0x9395;
    pub const VIEW_CLASS_ASTC_4x4_RGBA: c_uint = 0x9388;
    pub const VIEW_CLASS_ASTC_5x4_RGBA: c_uint = 0x9389;
    pub const VIEW_CLASS_ASTC_5x5_RGBA: c_uint = 0x938A;
    pub const VIEW_CLASS_ASTC_6x5_RGBA: c_uint = 0x938B;
    pub const VIEW_CLASS_ASTC_6x6_RGBA: c_uint = 0x938C;
    pub const VIEW_CLASS_ASTC_8x5_RGBA: c_uint = 0x938D;
    pub const VIEW_CLASS_ASTC_8x6_RGBA: c_uint = 0x938E;
    pub const VIEW_CLASS_ASTC_8x8_RGBA: c_uint = 0x938F;
    pub const VIEW_CLASS_BPTC_FLOAT: c_uint = 0x82D3;
    pub const VIEW_CLASS_BPTC_UNORM: c_uint = 0x82D2;
    pub const VIEW_CLASS_EAC_R11: c_uint = 0x9383;
    pub const VIEW_CLASS_EAC_RG11: c_uint = 0x9384;
    pub const VIEW_CLASS_ETC2_EAC_RGBA: c_uint = 0x9387;
    pub const VIEW_CLASS_ETC2_RGB: c_uint = 0x9385;
    pub const VIEW_CLASS_ETC2_RGBA: c_uint = 0x9386;
    pub const VIEW_CLASS_RGTC1_RED: c_uint = 0x82D0;
    pub const VIEW_CLASS_RGTC2_RG: c_uint = 0x82D1;
    pub const VIEW_CLASS_S3TC_DXT1_RGB: c_uint = 0x82CC;
    pub const VIEW_CLASS_S3TC_DXT1_RGBA: c_uint = 0x82CD;
    pub const VIEW_CLASS_S3TC_DXT3_RGBA: c_uint = 0x82CE;
    pub const VIEW_CLASS_S3TC_DXT5_RGBA: c_uint = 0x82CF;
    pub const VIEW_COMPATIBILITY_CLASS: c_uint = 0x82B6;
    pub const WAIT_FAILED: c_uint = 0x911D;
    pub const WEIGHT_ARRAY_BUFFER_BINDING_ARB: c_uint = 0x889E;
    pub const WRITE_ONLY: c_uint = 0x88B9;
    pub const WRITE_ONLY_ARB: c_uint = 0x88B9;
    pub const XOR: c_uint = 0x1506;
    pub const ZERO: c_uint = 0;
    pub const ZERO_TO_ONE: c_uint = 0x935F;
}

pub mod functions {
    #![allow(non_snake_case, unused_variables, dead_code, unused_imports)]

    use std::mem::transmute;
    use std::os::raw::*;
    use super::*;
    use super::types::*;

    macro_rules! func {
        ($fun:ident, $ret:ty, $($name:ident: $typ:ty),*) => {
            #[inline] pub unsafe fn $fun(&self, $($name: $typ),*) -> $ret {
                transmute::<_, extern "system" fn($($typ),*) -> $ret>(self.$fun.ptr)($($name),*)
            }
        }
    }

    pub struct Gl {
         pub(super) ActiveShaderProgram: FnPtr,
         pub(super) ActiveTexture: FnPtr,
         pub(super) ActiveTextureARB: FnPtr,
         pub(super) ActiveVaryingNV: FnPtr,
         pub(super) AreProgramsResidentNV: FnPtr,
         pub(super) AreTexturesResidentEXT: FnPtr,
         pub(super) ArrayElementEXT: FnPtr,
         pub(super) AttachObjectARB: FnPtr,
         pub(super) AttachShader: FnPtr,
         pub(super) BeginConditionalRender: FnPtr,
         pub(super) BeginConditionalRenderNV: FnPtr,
         pub(super) BeginConditionalRenderNVX: FnPtr,
         pub(super) BeginQuery: FnPtr,
         pub(super) BeginQueryARB: FnPtr,
         pub(super) BeginQueryIndexed: FnPtr,
         pub(super) BeginTransformFeedback: FnPtr,
         pub(super) BeginTransformFeedbackEXT: FnPtr,
         pub(super) BeginTransformFeedbackNV: FnPtr,
         pub(super) BindAttribLocation: FnPtr,
         pub(super) BindAttribLocationARB: FnPtr,
         pub(super) BindBuffer: FnPtr,
         pub(super) BindBufferARB: FnPtr,
         pub(super) BindBufferBase: FnPtr,
         pub(super) BindBufferBaseEXT: FnPtr,
         pub(super) BindBufferBaseNV: FnPtr,
         pub(super) BindBufferOffsetEXT: FnPtr,
         pub(super) BindBufferOffsetNV: FnPtr,
         pub(super) BindBufferRange: FnPtr,
         pub(super) BindBufferRangeEXT: FnPtr,
         pub(super) BindBufferRangeNV: FnPtr,
         pub(super) BindBuffersBase: FnPtr,
         pub(super) BindBuffersRange: FnPtr,
         pub(super) BindFragDataLocation: FnPtr,
         pub(super) BindFragDataLocationEXT: FnPtr,
         pub(super) BindFragDataLocationIndexed: FnPtr,
         pub(super) BindFramebuffer: FnPtr,
         pub(super) BindFramebufferEXT: FnPtr,
         pub(super) BindImageTexture: FnPtr,
         pub(super) BindImageTextureEXT: FnPtr,
         pub(super) BindImageTextures: FnPtr,
         pub(super) BindMultiTextureEXT: FnPtr,
         pub(super) BindProgramARB: FnPtr,
         pub(super) BindProgramNV: FnPtr,
         pub(super) BindProgramPipeline: FnPtr,
         pub(super) BindRenderbuffer: FnPtr,
         pub(super) BindRenderbufferEXT: FnPtr,
         pub(super) BindSampler: FnPtr,
         pub(super) BindSamplers: FnPtr,
         pub(super) BindTexture: FnPtr,
         pub(super) BindTextureEXT: FnPtr,
         pub(super) BindTextureUnit: FnPtr,
         pub(super) BindTextures: FnPtr,
         pub(super) BindTransformFeedback: FnPtr,
         pub(super) BindTransformFeedbackNV: FnPtr,
         pub(super) BindVertexArray: FnPtr,
         pub(super) BindVertexArrayAPPLE: FnPtr,
         pub(super) BindVertexBuffer: FnPtr,
         pub(super) BindVertexBuffers: FnPtr,
         pub(super) BlendColor: FnPtr,
         pub(super) BlendColorEXT: FnPtr,
         pub(super) BlendEquation: FnPtr,
         pub(super) BlendEquationEXT: FnPtr,
         pub(super) BlendEquationIndexedAMD: FnPtr,
         pub(super) BlendEquationSeparate: FnPtr,
         pub(super) BlendEquationSeparateEXT: FnPtr,
         pub(super) BlendEquationSeparateIndexedAMD: FnPtr,
         pub(super) BlendEquationSeparatei: FnPtr,
         pub(super) BlendEquationSeparateiARB: FnPtr,
         pub(super) BlendEquationi: FnPtr,
         pub(super) BlendEquationiARB: FnPtr,
         pub(super) BlendFunc: FnPtr,
         pub(super) BlendFuncIndexedAMD: FnPtr,
         pub(super) BlendFuncSeparate: FnPtr,
         pub(super) BlendFuncSeparateEXT: FnPtr,
         pub(super) BlendFuncSeparateINGR: FnPtr,
         pub(super) BlendFuncSeparateIndexedAMD: FnPtr,
         pub(super) BlendFuncSeparatei: FnPtr,
         pub(super) BlendFuncSeparateiARB: FnPtr,
         pub(super) BlendFunci: FnPtr,
         pub(super) BlendFunciARB: FnPtr,
         pub(super) BlitFramebuffer: FnPtr,
         pub(super) BlitFramebufferEXT: FnPtr,
         pub(super) BlitNamedFramebuffer: FnPtr,
         pub(super) BufferData: FnPtr,
         pub(super) BufferDataARB: FnPtr,
         pub(super) BufferParameteriAPPLE: FnPtr,
         pub(super) BufferStorage: FnPtr,
         pub(super) BufferSubData: FnPtr,
         pub(super) BufferSubDataARB: FnPtr,
         pub(super) CheckFramebufferStatus: FnPtr,
         pub(super) CheckFramebufferStatusEXT: FnPtr,
         pub(super) CheckNamedFramebufferStatus: FnPtr,
         pub(super) CheckNamedFramebufferStatusEXT: FnPtr,
         pub(super) ClampColor: FnPtr,
         pub(super) ClampColorARB: FnPtr,
         pub(super) Clear: FnPtr,
         pub(super) ClearBufferData: FnPtr,
         pub(super) ClearBufferSubData: FnPtr,
         pub(super) ClearBufferfi: FnPtr,
         pub(super) ClearBufferfv: FnPtr,
         pub(super) ClearBufferiv: FnPtr,
         pub(super) ClearBufferuiv: FnPtr,
         pub(super) ClearColor: FnPtr,
         pub(super) ClearColorIiEXT: FnPtr,
         pub(super) ClearColorIuiEXT: FnPtr,
         pub(super) ClearDepth: FnPtr,
         pub(super) ClearDepthf: FnPtr,
         pub(super) ClearDepthfOES: FnPtr,
         pub(super) ClearNamedBufferData: FnPtr,
         pub(super) ClearNamedBufferDataEXT: FnPtr,
         pub(super) ClearNamedBufferSubData: FnPtr,
         pub(super) ClearNamedBufferSubDataEXT: FnPtr,
         pub(super) ClearNamedFramebufferfi: FnPtr,
         pub(super) ClearNamedFramebufferfv: FnPtr,
         pub(super) ClearNamedFramebufferiv: FnPtr,
         pub(super) ClearNamedFramebufferuiv: FnPtr,
         pub(super) ClearStencil: FnPtr,
         pub(super) ClearTexImage: FnPtr,
         pub(super) ClearTexSubImage: FnPtr,
         pub(super) ClientActiveTextureARB: FnPtr,
         pub(super) ClientAttribDefaultEXT: FnPtr,
         pub(super) ClientWaitSync: FnPtr,
         pub(super) ClipControl: FnPtr,
         pub(super) ClipPlanefOES: FnPtr,
         pub(super) ColorMask: FnPtr,
         pub(super) ColorMaskIndexedEXT: FnPtr,
         pub(super) ColorMaski: FnPtr,
         pub(super) ColorPointerEXT: FnPtr,
         pub(super) CompileShader: FnPtr,
         pub(super) CompileShaderARB: FnPtr,
         pub(super) CompressedMultiTexImage1DEXT: FnPtr,
         pub(super) CompressedMultiTexImage2DEXT: FnPtr,
         pub(super) CompressedMultiTexImage3DEXT: FnPtr,
         pub(super) CompressedMultiTexSubImage1DEXT: FnPtr,
         pub(super) CompressedMultiTexSubImage2DEXT: FnPtr,
         pub(super) CompressedMultiTexSubImage3DEXT: FnPtr,
         pub(super) CompressedTexImage1D: FnPtr,
         pub(super) CompressedTexImage1DARB: FnPtr,
         pub(super) CompressedTexImage2D: FnPtr,
         pub(super) CompressedTexImage2DARB: FnPtr,
         pub(super) CompressedTexImage3D: FnPtr,
         pub(super) CompressedTexImage3DARB: FnPtr,
         pub(super) CompressedTexSubImage1D: FnPtr,
         pub(super) CompressedTexSubImage1DARB: FnPtr,
         pub(super) CompressedTexSubImage2D: FnPtr,
         pub(super) CompressedTexSubImage2DARB: FnPtr,
         pub(super) CompressedTexSubImage3D: FnPtr,
         pub(super) CompressedTexSubImage3DARB: FnPtr,
         pub(super) CompressedTextureImage1DEXT: FnPtr,
         pub(super) CompressedTextureImage2DEXT: FnPtr,
         pub(super) CompressedTextureImage3DEXT: FnPtr,
         pub(super) CompressedTextureSubImage1D: FnPtr,
         pub(super) CompressedTextureSubImage1DEXT: FnPtr,
         pub(super) CompressedTextureSubImage2D: FnPtr,
         pub(super) CompressedTextureSubImage2DEXT: FnPtr,
         pub(super) CompressedTextureSubImage3D: FnPtr,
         pub(super) CompressedTextureSubImage3DEXT: FnPtr,
         pub(super) CopyBufferSubData: FnPtr,
         pub(super) CopyImageSubData: FnPtr,
         pub(super) CopyMultiTexImage1DEXT: FnPtr,
         pub(super) CopyMultiTexImage2DEXT: FnPtr,
         pub(super) CopyMultiTexSubImage1DEXT: FnPtr,
         pub(super) CopyMultiTexSubImage2DEXT: FnPtr,
         pub(super) CopyMultiTexSubImage3DEXT: FnPtr,
         pub(super) CopyNamedBufferSubData: FnPtr,
         pub(super) CopyTexImage1D: FnPtr,
         pub(super) CopyTexImage1DEXT: FnPtr,
         pub(super) CopyTexImage2D: FnPtr,
         pub(super) CopyTexImage2DEXT: FnPtr,
         pub(super) CopyTexSubImage1D: FnPtr,
         pub(super) CopyTexSubImage1DEXT: FnPtr,
         pub(super) CopyTexSubImage2D: FnPtr,
         pub(super) CopyTexSubImage2DEXT: FnPtr,
         pub(super) CopyTexSubImage3D: FnPtr,
         pub(super) CopyTexSubImage3DEXT: FnPtr,
         pub(super) CopyTextureImage1DEXT: FnPtr,
         pub(super) CopyTextureImage2DEXT: FnPtr,
         pub(super) CopyTextureSubImage1D: FnPtr,
         pub(super) CopyTextureSubImage1DEXT: FnPtr,
         pub(super) CopyTextureSubImage2D: FnPtr,
         pub(super) CopyTextureSubImage2DEXT: FnPtr,
         pub(super) CopyTextureSubImage3D: FnPtr,
         pub(super) CopyTextureSubImage3DEXT: FnPtr,
         pub(super) CreateBuffers: FnPtr,
         pub(super) CreateFramebuffers: FnPtr,
         pub(super) CreateProgram: FnPtr,
         pub(super) CreateProgramObjectARB: FnPtr,
         pub(super) CreateProgramPipelines: FnPtr,
         pub(super) CreateQueries: FnPtr,
         pub(super) CreateRenderbuffers: FnPtr,
         pub(super) CreateSamplers: FnPtr,
         pub(super) CreateShader: FnPtr,
         pub(super) CreateShaderObjectARB: FnPtr,
         pub(super) CreateShaderProgramv: FnPtr,
         pub(super) CreateTextures: FnPtr,
         pub(super) CreateTransformFeedbacks: FnPtr,
         pub(super) CreateVertexArrays: FnPtr,
         pub(super) CullFace: FnPtr,
         pub(super) DebugMessageCallback: FnPtr,
         pub(super) DebugMessageCallbackARB: FnPtr,
         pub(super) DebugMessageControl: FnPtr,
         pub(super) DebugMessageControlARB: FnPtr,
         pub(super) DebugMessageInsert: FnPtr,
         pub(super) DebugMessageInsertARB: FnPtr,
         pub(super) DeleteBuffers: FnPtr,
         pub(super) DeleteBuffersARB: FnPtr,
         pub(super) DeleteFramebuffers: FnPtr,
         pub(super) DeleteFramebuffersEXT: FnPtr,
         pub(super) DeleteObjectARB: FnPtr,
         pub(super) DeleteProgram: FnPtr,
         pub(super) DeleteProgramPipelines: FnPtr,
         pub(super) DeleteProgramsARB: FnPtr,
         pub(super) DeleteProgramsNV: FnPtr,
         pub(super) DeleteQueries: FnPtr,
         pub(super) DeleteQueriesARB: FnPtr,
         pub(super) DeleteRenderbuffers: FnPtr,
         pub(super) DeleteRenderbuffersEXT: FnPtr,
         pub(super) DeleteSamplers: FnPtr,
         pub(super) DeleteShader: FnPtr,
         pub(super) DeleteSync: FnPtr,
         pub(super) DeleteTextures: FnPtr,
         pub(super) DeleteTexturesEXT: FnPtr,
         pub(super) DeleteTransformFeedbacks: FnPtr,
         pub(super) DeleteTransformFeedbacksNV: FnPtr,
         pub(super) DeleteVertexArrays: FnPtr,
         pub(super) DeleteVertexArraysAPPLE: FnPtr,
         pub(super) DepthFunc: FnPtr,
         pub(super) DepthMask: FnPtr,
         pub(super) DepthRange: FnPtr,
         pub(super) DepthRangeArraydvNV: FnPtr,
         pub(super) DepthRangeArrayv: FnPtr,
         pub(super) DepthRangeIndexed: FnPtr,
         pub(super) DepthRangeIndexeddNV: FnPtr,
         pub(super) DepthRangef: FnPtr,
         pub(super) DepthRangefOES: FnPtr,
         pub(super) DetachObjectARB: FnPtr,
         pub(super) DetachShader: FnPtr,
         pub(super) Disable: FnPtr,
         pub(super) DisableClientStateIndexedEXT: FnPtr,
         pub(super) DisableClientStateiEXT: FnPtr,
         pub(super) DisableIndexedEXT: FnPtr,
         pub(super) DisableVertexArrayAttrib: FnPtr,
         pub(super) DisableVertexArrayAttribEXT: FnPtr,
         pub(super) DisableVertexArrayEXT: FnPtr,
         pub(super) DisableVertexAttribArray: FnPtr,
         pub(super) DisableVertexAttribArrayARB: FnPtr,
         pub(super) Disablei: FnPtr,
         pub(super) DispatchCompute: FnPtr,
         pub(super) DispatchComputeIndirect: FnPtr,
         pub(super) DrawArrays: FnPtr,
         pub(super) DrawArraysEXT: FnPtr,
         pub(super) DrawArraysIndirect: FnPtr,
         pub(super) DrawArraysInstanced: FnPtr,
         pub(super) DrawArraysInstancedARB: FnPtr,
         pub(super) DrawArraysInstancedBaseInstance: FnPtr,
         pub(super) DrawArraysInstancedEXT: FnPtr,
         pub(super) DrawBuffer: FnPtr,
         pub(super) DrawBuffers: FnPtr,
         pub(super) DrawBuffersARB: FnPtr,
         pub(super) DrawBuffersATI: FnPtr,
         pub(super) DrawElements: FnPtr,
         pub(super) DrawElementsBaseVertex: FnPtr,
         pub(super) DrawElementsIndirect: FnPtr,
         pub(super) DrawElementsInstanced: FnPtr,
         pub(super) DrawElementsInstancedARB: FnPtr,
         pub(super) DrawElementsInstancedBaseInstance: FnPtr,
         pub(super) DrawElementsInstancedBaseVertex: FnPtr,
         pub(super) DrawElementsInstancedBaseVertexBaseInstance: FnPtr,
         pub(super) DrawElementsInstancedEXT: FnPtr,
         pub(super) DrawRangeElements: FnPtr,
         pub(super) DrawRangeElementsBaseVertex: FnPtr,
         pub(super) DrawRangeElementsEXT: FnPtr,
         pub(super) DrawTransformFeedback: FnPtr,
         pub(super) DrawTransformFeedbackInstanced: FnPtr,
         pub(super) DrawTransformFeedbackNV: FnPtr,
         pub(super) DrawTransformFeedbackStream: FnPtr,
         pub(super) DrawTransformFeedbackStreamInstanced: FnPtr,
         pub(super) EdgeFlagPointerEXT: FnPtr,
         pub(super) Enable: FnPtr,
         pub(super) EnableClientStateIndexedEXT: FnPtr,
         pub(super) EnableClientStateiEXT: FnPtr,
         pub(super) EnableIndexedEXT: FnPtr,
         pub(super) EnableVertexArrayAttrib: FnPtr,
         pub(super) EnableVertexArrayAttribEXT: FnPtr,
         pub(super) EnableVertexArrayEXT: FnPtr,
         pub(super) EnableVertexAttribArray: FnPtr,
         pub(super) EnableVertexAttribArrayARB: FnPtr,
         pub(super) Enablei: FnPtr,
         pub(super) EndConditionalRender: FnPtr,
         pub(super) EndConditionalRenderNV: FnPtr,
         pub(super) EndConditionalRenderNVX: FnPtr,
         pub(super) EndQuery: FnPtr,
         pub(super) EndQueryARB: FnPtr,
         pub(super) EndQueryIndexed: FnPtr,
         pub(super) EndTransformFeedback: FnPtr,
         pub(super) EndTransformFeedbackEXT: FnPtr,
         pub(super) EndTransformFeedbackNV: FnPtr,
         pub(super) ExecuteProgramNV: FnPtr,
         pub(super) FenceSync: FnPtr,
         pub(super) Finish: FnPtr,
         pub(super) Flush: FnPtr,
         pub(super) FlushMappedBufferRange: FnPtr,
         pub(super) FlushMappedBufferRangeAPPLE: FnPtr,
         pub(super) FlushMappedNamedBufferRange: FnPtr,
         pub(super) FlushMappedNamedBufferRangeEXT: FnPtr,
         pub(super) FramebufferDrawBufferEXT: FnPtr,
         pub(super) FramebufferDrawBuffersEXT: FnPtr,
         pub(super) FramebufferParameteri: FnPtr,
         pub(super) FramebufferReadBufferEXT: FnPtr,
         pub(super) FramebufferRenderbuffer: FnPtr,
         pub(super) FramebufferRenderbufferEXT: FnPtr,
         pub(super) FramebufferTexture: FnPtr,
         pub(super) FramebufferTexture1D: FnPtr,
         pub(super) FramebufferTexture1DEXT: FnPtr,
         pub(super) FramebufferTexture2D: FnPtr,
         pub(super) FramebufferTexture2DEXT: FnPtr,
         pub(super) FramebufferTexture3D: FnPtr,
         pub(super) FramebufferTexture3DEXT: FnPtr,
         pub(super) FramebufferTextureARB: FnPtr,
         pub(super) FramebufferTextureEXT: FnPtr,
         pub(super) FramebufferTextureFaceARB: FnPtr,
         pub(super) FramebufferTextureFaceEXT: FnPtr,
         pub(super) FramebufferTextureLayer: FnPtr,
         pub(super) FramebufferTextureLayerARB: FnPtr,
         pub(super) FramebufferTextureLayerEXT: FnPtr,
         pub(super) FrontFace: FnPtr,
         pub(super) FrustumfOES: FnPtr,
         pub(super) GenBuffers: FnPtr,
         pub(super) GenBuffersARB: FnPtr,
         pub(super) GenFramebuffers: FnPtr,
         pub(super) GenFramebuffersEXT: FnPtr,
         pub(super) GenProgramPipelines: FnPtr,
         pub(super) GenProgramsARB: FnPtr,
         pub(super) GenProgramsNV: FnPtr,
         pub(super) GenQueries: FnPtr,
         pub(super) GenQueriesARB: FnPtr,
         pub(super) GenRenderbuffers: FnPtr,
         pub(super) GenRenderbuffersEXT: FnPtr,
         pub(super) GenSamplers: FnPtr,
         pub(super) GenTextures: FnPtr,
         pub(super) GenTexturesEXT: FnPtr,
         pub(super) GenTransformFeedbacks: FnPtr,
         pub(super) GenTransformFeedbacksNV: FnPtr,
         pub(super) GenVertexArrays: FnPtr,
         pub(super) GenVertexArraysAPPLE: FnPtr,
         pub(super) GenerateMipmap: FnPtr,
         pub(super) GenerateMipmapEXT: FnPtr,
         pub(super) GenerateMultiTexMipmapEXT: FnPtr,
         pub(super) GenerateTextureMipmap: FnPtr,
         pub(super) GenerateTextureMipmapEXT: FnPtr,
         pub(super) GetActiveAtomicCounterBufferiv: FnPtr,
         pub(super) GetActiveAttrib: FnPtr,
         pub(super) GetActiveAttribARB: FnPtr,
         pub(super) GetActiveSubroutineName: FnPtr,
         pub(super) GetActiveSubroutineUniformName: FnPtr,
         pub(super) GetActiveSubroutineUniformiv: FnPtr,
         pub(super) GetActiveUniform: FnPtr,
         pub(super) GetActiveUniformARB: FnPtr,
         pub(super) GetActiveUniformBlockName: FnPtr,
         pub(super) GetActiveUniformBlockiv: FnPtr,
         pub(super) GetActiveUniformName: FnPtr,
         pub(super) GetActiveUniformsiv: FnPtr,
         pub(super) GetActiveVaryingNV: FnPtr,
         pub(super) GetAttachedObjectsARB: FnPtr,
         pub(super) GetAttachedShaders: FnPtr,
         pub(super) GetAttribLocation: FnPtr,
         pub(super) GetAttribLocationARB: FnPtr,
         pub(super) GetBooleanIndexedvEXT: FnPtr,
         pub(super) GetBooleani_v: FnPtr,
         pub(super) GetBooleanv: FnPtr,
         pub(super) GetBufferParameteri64v: FnPtr,
         pub(super) GetBufferParameteriv: FnPtr,
         pub(super) GetBufferParameterivARB: FnPtr,
         pub(super) GetBufferPointerv: FnPtr,
         pub(super) GetBufferPointervARB: FnPtr,
         pub(super) GetBufferSubData: FnPtr,
         pub(super) GetBufferSubDataARB: FnPtr,
         pub(super) GetClipPlanefOES: FnPtr,
         pub(super) GetCompressedMultiTexImageEXT: FnPtr,
         pub(super) GetCompressedTexImage: FnPtr,
         pub(super) GetCompressedTexImageARB: FnPtr,
         pub(super) GetCompressedTextureImage: FnPtr,
         pub(super) GetCompressedTextureImageEXT: FnPtr,
         pub(super) GetCompressedTextureSubImage: FnPtr,
         pub(super) GetDebugMessageLog: FnPtr,
         pub(super) GetDebugMessageLogARB: FnPtr,
         pub(super) GetDoubleIndexedvEXT: FnPtr,
         pub(super) GetDoublei_v: FnPtr,
         pub(super) GetDoublei_vEXT: FnPtr,
         pub(super) GetDoublev: FnPtr,
         pub(super) GetError: FnPtr,
         pub(super) GetFloatIndexedvEXT: FnPtr,
         pub(super) GetFloati_v: FnPtr,
         pub(super) GetFloati_vEXT: FnPtr,
         pub(super) GetFloatv: FnPtr,
         pub(super) GetFragDataIndex: FnPtr,
         pub(super) GetFragDataLocation: FnPtr,
         pub(super) GetFragDataLocationEXT: FnPtr,
         pub(super) GetFramebufferAttachmentParameteriv: FnPtr,
         pub(super) GetFramebufferAttachmentParameterivEXT: FnPtr,
         pub(super) GetFramebufferParameteriv: FnPtr,
         pub(super) GetFramebufferParameterivEXT: FnPtr,
         pub(super) GetGraphicsResetStatus: FnPtr,
         pub(super) GetGraphicsResetStatusARB: FnPtr,
         pub(super) GetHandleARB: FnPtr,
         pub(super) GetInfoLogARB: FnPtr,
         pub(super) GetInteger64i_v: FnPtr,
         pub(super) GetInteger64v: FnPtr,
         pub(super) GetIntegerIndexedvEXT: FnPtr,
         pub(super) GetIntegeri_v: FnPtr,
         pub(super) GetIntegerv: FnPtr,
         pub(super) GetInternalformati64v: FnPtr,
         pub(super) GetInternalformativ: FnPtr,
         pub(super) GetMultiTexEnvfvEXT: FnPtr,
         pub(super) GetMultiTexEnvivEXT: FnPtr,
         pub(super) GetMultiTexGendvEXT: FnPtr,
         pub(super) GetMultiTexGenfvEXT: FnPtr,
         pub(super) GetMultiTexGenivEXT: FnPtr,
         pub(super) GetMultiTexImageEXT: FnPtr,
         pub(super) GetMultiTexLevelParameterfvEXT: FnPtr,
         pub(super) GetMultiTexLevelParameterivEXT: FnPtr,
         pub(super) GetMultiTexParameterIivEXT: FnPtr,
         pub(super) GetMultiTexParameterIuivEXT: FnPtr,
         pub(super) GetMultiTexParameterfvEXT: FnPtr,
         pub(super) GetMultiTexParameterivEXT: FnPtr,
         pub(super) GetMultisamplefv: FnPtr,
         pub(super) GetMultisamplefvNV: FnPtr,
         pub(super) GetNamedBufferParameteri64v: FnPtr,
         pub(super) GetNamedBufferParameteriv: FnPtr,
         pub(super) GetNamedBufferParameterivEXT: FnPtr,
         pub(super) GetNamedBufferPointerv: FnPtr,
         pub(super) GetNamedBufferPointervEXT: FnPtr,
         pub(super) GetNamedBufferSubData: FnPtr,
         pub(super) GetNamedBufferSubDataEXT: FnPtr,
         pub(super) GetNamedFramebufferAttachmentParameteriv: FnPtr,
         pub(super) GetNamedFramebufferAttachmentParameterivEXT: FnPtr,
         pub(super) GetNamedFramebufferParameteriv: FnPtr,
         pub(super) GetNamedFramebufferParameterivEXT: FnPtr,
         pub(super) GetNamedProgramLocalParameterIivEXT: FnPtr,
         pub(super) GetNamedProgramLocalParameterIuivEXT: FnPtr,
         pub(super) GetNamedProgramLocalParameterdvEXT: FnPtr,
         pub(super) GetNamedProgramLocalParameterfvEXT: FnPtr,
         pub(super) GetNamedProgramStringEXT: FnPtr,
         pub(super) GetNamedProgramivEXT: FnPtr,
         pub(super) GetNamedRenderbufferParameteriv: FnPtr,
         pub(super) GetNamedRenderbufferParameterivEXT: FnPtr,
         pub(super) GetObjectLabel: FnPtr,
         pub(super) GetObjectParameterfvARB: FnPtr,
         pub(super) GetObjectParameterivARB: FnPtr,
         pub(super) GetObjectPtrLabel: FnPtr,
         pub(super) GetPointerIndexedvEXT: FnPtr,
         pub(super) GetPointeri_vEXT: FnPtr,
         pub(super) GetPointerv: FnPtr,
         pub(super) GetPointervEXT: FnPtr,
         pub(super) GetProgramBinary: FnPtr,
         pub(super) GetProgramEnvParameterdvARB: FnPtr,
         pub(super) GetProgramEnvParameterfvARB: FnPtr,
         pub(super) GetProgramInfoLog: FnPtr,
         pub(super) GetProgramInterfaceiv: FnPtr,
         pub(super) GetProgramLocalParameterdvARB: FnPtr,
         pub(super) GetProgramLocalParameterfvARB: FnPtr,
         pub(super) GetProgramParameterdvNV: FnPtr,
         pub(super) GetProgramParameterfvNV: FnPtr,
         pub(super) GetProgramPipelineInfoLog: FnPtr,
         pub(super) GetProgramPipelineiv: FnPtr,
         pub(super) GetProgramResourceIndex: FnPtr,
         pub(super) GetProgramResourceLocation: FnPtr,
         pub(super) GetProgramResourceLocationIndex: FnPtr,
         pub(super) GetProgramResourceName: FnPtr,
         pub(super) GetProgramResourceiv: FnPtr,
         pub(super) GetProgramStageiv: FnPtr,
         pub(super) GetProgramStringARB: FnPtr,
         pub(super) GetProgramStringNV: FnPtr,
         pub(super) GetProgramiv: FnPtr,
         pub(super) GetProgramivARB: FnPtr,
         pub(super) GetProgramivNV: FnPtr,
         pub(super) GetQueryBufferObjecti64v: FnPtr,
         pub(super) GetQueryBufferObjectiv: FnPtr,
         pub(super) GetQueryBufferObjectui64v: FnPtr,
         pub(super) GetQueryBufferObjectuiv: FnPtr,
         pub(super) GetQueryIndexediv: FnPtr,
         pub(super) GetQueryObjecti64v: FnPtr,
         pub(super) GetQueryObjecti64vEXT: FnPtr,
         pub(super) GetQueryObjectiv: FnPtr,
         pub(super) GetQueryObjectivARB: FnPtr,
         pub(super) GetQueryObjectui64v: FnPtr,
         pub(super) GetQueryObjectui64vEXT: FnPtr,
         pub(super) GetQueryObjectuiv: FnPtr,
         pub(super) GetQueryObjectuivARB: FnPtr,
         pub(super) GetQueryiv: FnPtr,
         pub(super) GetQueryivARB: FnPtr,
         pub(super) GetRenderbufferParameteriv: FnPtr,
         pub(super) GetRenderbufferParameterivEXT: FnPtr,
         pub(super) GetSamplerParameterIiv: FnPtr,
         pub(super) GetSamplerParameterIuiv: FnPtr,
         pub(super) GetSamplerParameterfv: FnPtr,
         pub(super) GetSamplerParameteriv: FnPtr,
         pub(super) GetShaderInfoLog: FnPtr,
         pub(super) GetShaderPrecisionFormat: FnPtr,
         pub(super) GetShaderSource: FnPtr,
         pub(super) GetShaderSourceARB: FnPtr,
         pub(super) GetShaderiv: FnPtr,
         pub(super) GetString: FnPtr,
         pub(super) GetStringi: FnPtr,
         pub(super) GetSubroutineIndex: FnPtr,
         pub(super) GetSubroutineUniformLocation: FnPtr,
         pub(super) GetSynciv: FnPtr,
         pub(super) GetTexImage: FnPtr,
         pub(super) GetTexLevelParameterfv: FnPtr,
         pub(super) GetTexLevelParameteriv: FnPtr,
         pub(super) GetTexParameterIiv: FnPtr,
         pub(super) GetTexParameterIivEXT: FnPtr,
         pub(super) GetTexParameterIuiv: FnPtr,
         pub(super) GetTexParameterIuivEXT: FnPtr,
         pub(super) GetTexParameterfv: FnPtr,
         pub(super) GetTexParameteriv: FnPtr,
         pub(super) GetTextureImage: FnPtr,
         pub(super) GetTextureImageEXT: FnPtr,
         pub(super) GetTextureLevelParameterfv: FnPtr,
         pub(super) GetTextureLevelParameterfvEXT: FnPtr,
         pub(super) GetTextureLevelParameteriv: FnPtr,
         pub(super) GetTextureLevelParameterivEXT: FnPtr,
         pub(super) GetTextureParameterIiv: FnPtr,
         pub(super) GetTextureParameterIivEXT: FnPtr,
         pub(super) GetTextureParameterIuiv: FnPtr,
         pub(super) GetTextureParameterIuivEXT: FnPtr,
         pub(super) GetTextureParameterfv: FnPtr,
         pub(super) GetTextureParameterfvEXT: FnPtr,
         pub(super) GetTextureParameteriv: FnPtr,
         pub(super) GetTextureParameterivEXT: FnPtr,
         pub(super) GetTextureSubImage: FnPtr,
         pub(super) GetTrackMatrixivNV: FnPtr,
         pub(super) GetTransformFeedbackVarying: FnPtr,
         pub(super) GetTransformFeedbackVaryingEXT: FnPtr,
         pub(super) GetTransformFeedbackVaryingNV: FnPtr,
         pub(super) GetTransformFeedbacki64_v: FnPtr,
         pub(super) GetTransformFeedbacki_v: FnPtr,
         pub(super) GetTransformFeedbackiv: FnPtr,
         pub(super) GetUniformBlockIndex: FnPtr,
         pub(super) GetUniformIndices: FnPtr,
         pub(super) GetUniformLocation: FnPtr,
         pub(super) GetUniformLocationARB: FnPtr,
         pub(super) GetUniformSubroutineuiv: FnPtr,
         pub(super) GetUniformdv: FnPtr,
         pub(super) GetUniformfv: FnPtr,
         pub(super) GetUniformfvARB: FnPtr,
         pub(super) GetUniformiv: FnPtr,
         pub(super) GetUniformivARB: FnPtr,
         pub(super) GetUniformuiv: FnPtr,
         pub(super) GetUniformuivEXT: FnPtr,
         pub(super) GetVaryingLocationNV: FnPtr,
         pub(super) GetVertexArrayIndexed64iv: FnPtr,
         pub(super) GetVertexArrayIndexediv: FnPtr,
         pub(super) GetVertexArrayIntegeri_vEXT: FnPtr,
         pub(super) GetVertexArrayIntegervEXT: FnPtr,
         pub(super) GetVertexArrayPointeri_vEXT: FnPtr,
         pub(super) GetVertexArrayPointervEXT: FnPtr,
         pub(super) GetVertexArrayiv: FnPtr,
         pub(super) GetVertexAttribIiv: FnPtr,
         pub(super) GetVertexAttribIivEXT: FnPtr,
         pub(super) GetVertexAttribIuiv: FnPtr,
         pub(super) GetVertexAttribIuivEXT: FnPtr,
         pub(super) GetVertexAttribLdv: FnPtr,
         pub(super) GetVertexAttribLdvEXT: FnPtr,
         pub(super) GetVertexAttribPointerv: FnPtr,
         pub(super) GetVertexAttribPointervARB: FnPtr,
         pub(super) GetVertexAttribPointervNV: FnPtr,
         pub(super) GetVertexAttribdv: FnPtr,
         pub(super) GetVertexAttribdvARB: FnPtr,
         pub(super) GetVertexAttribdvNV: FnPtr,
         pub(super) GetVertexAttribfv: FnPtr,
         pub(super) GetVertexAttribfvARB: FnPtr,
         pub(super) GetVertexAttribfvNV: FnPtr,
         pub(super) GetVertexAttribiv: FnPtr,
         pub(super) GetVertexAttribivARB: FnPtr,
         pub(super) GetVertexAttribivNV: FnPtr,
         pub(super) GetnCompressedTexImage: FnPtr,
         pub(super) GetnCompressedTexImageARB: FnPtr,
         pub(super) GetnTexImage: FnPtr,
         pub(super) GetnTexImageARB: FnPtr,
         pub(super) GetnUniformdv: FnPtr,
         pub(super) GetnUniformdvARB: FnPtr,
         pub(super) GetnUniformfv: FnPtr,
         pub(super) GetnUniformfvARB: FnPtr,
         pub(super) GetnUniformiv: FnPtr,
         pub(super) GetnUniformivARB: FnPtr,
         pub(super) GetnUniformuiv: FnPtr,
         pub(super) GetnUniformuivARB: FnPtr,
         pub(super) Hint: FnPtr,
         pub(super) IndexPointerEXT: FnPtr,
         pub(super) InvalidateBufferData: FnPtr,
         pub(super) InvalidateBufferSubData: FnPtr,
         pub(super) InvalidateFramebuffer: FnPtr,
         pub(super) InvalidateNamedFramebufferData: FnPtr,
         pub(super) InvalidateNamedFramebufferSubData: FnPtr,
         pub(super) InvalidateSubFramebuffer: FnPtr,
         pub(super) InvalidateTexImage: FnPtr,
         pub(super) InvalidateTexSubImage: FnPtr,
         pub(super) IsBuffer: FnPtr,
         pub(super) IsBufferARB: FnPtr,
         pub(super) IsEnabled: FnPtr,
         pub(super) IsEnabledIndexedEXT: FnPtr,
         pub(super) IsEnabledi: FnPtr,
         pub(super) IsFramebuffer: FnPtr,
         pub(super) IsFramebufferEXT: FnPtr,
         pub(super) IsProgram: FnPtr,
         pub(super) IsProgramARB: FnPtr,
         pub(super) IsProgramNV: FnPtr,
         pub(super) IsProgramPipeline: FnPtr,
         pub(super) IsQuery: FnPtr,
         pub(super) IsQueryARB: FnPtr,
         pub(super) IsRenderbuffer: FnPtr,
         pub(super) IsRenderbufferEXT: FnPtr,
         pub(super) IsSampler: FnPtr,
         pub(super) IsShader: FnPtr,
         pub(super) IsSync: FnPtr,
         pub(super) IsTexture: FnPtr,
         pub(super) IsTextureEXT: FnPtr,
         pub(super) IsTransformFeedback: FnPtr,
         pub(super) IsTransformFeedbackNV: FnPtr,
         pub(super) IsVertexArray: FnPtr,
         pub(super) IsVertexArrayAPPLE: FnPtr,
         pub(super) LineWidth: FnPtr,
         pub(super) LinkProgram: FnPtr,
         pub(super) LinkProgramARB: FnPtr,
         pub(super) LoadProgramNV: FnPtr,
         pub(super) LogicOp: FnPtr,
         pub(super) MapBuffer: FnPtr,
         pub(super) MapBufferARB: FnPtr,
         pub(super) MapBufferRange: FnPtr,
         pub(super) MapNamedBuffer: FnPtr,
         pub(super) MapNamedBufferEXT: FnPtr,
         pub(super) MapNamedBufferRange: FnPtr,
         pub(super) MapNamedBufferRangeEXT: FnPtr,
         pub(super) MatrixFrustumEXT: FnPtr,
         pub(super) MatrixLoadIdentityEXT: FnPtr,
         pub(super) MatrixLoadTransposedEXT: FnPtr,
         pub(super) MatrixLoadTransposefEXT: FnPtr,
         pub(super) MatrixLoaddEXT: FnPtr,
         pub(super) MatrixLoadfEXT: FnPtr,
         pub(super) MatrixMultTransposedEXT: FnPtr,
         pub(super) MatrixMultTransposefEXT: FnPtr,
         pub(super) MatrixMultdEXT: FnPtr,
         pub(super) MatrixMultfEXT: FnPtr,
         pub(super) MatrixOrthoEXT: FnPtr,
         pub(super) MatrixPopEXT: FnPtr,
         pub(super) MatrixPushEXT: FnPtr,
         pub(super) MatrixRotatedEXT: FnPtr,
         pub(super) MatrixRotatefEXT: FnPtr,
         pub(super) MatrixScaledEXT: FnPtr,
         pub(super) MatrixScalefEXT: FnPtr,
         pub(super) MatrixTranslatedEXT: FnPtr,
         pub(super) MatrixTranslatefEXT: FnPtr,
         pub(super) MemoryBarrier: FnPtr,
         pub(super) MemoryBarrierByRegion: FnPtr,
         pub(super) MemoryBarrierEXT: FnPtr,
         pub(super) MinSampleShading: FnPtr,
         pub(super) MinSampleShadingARB: FnPtr,
         pub(super) MultiDrawArrays: FnPtr,
         pub(super) MultiDrawArraysEXT: FnPtr,
         pub(super) MultiDrawArraysIndirect: FnPtr,
         pub(super) MultiDrawArraysIndirectAMD: FnPtr,
         pub(super) MultiDrawArraysIndirectCount: FnPtr,
         pub(super) MultiDrawArraysIndirectCountARB: FnPtr,
         pub(super) MultiDrawElements: FnPtr,
         pub(super) MultiDrawElementsBaseVertex: FnPtr,
         pub(super) MultiDrawElementsEXT: FnPtr,
         pub(super) MultiDrawElementsIndirect: FnPtr,
         pub(super) MultiDrawElementsIndirectAMD: FnPtr,
         pub(super) MultiDrawElementsIndirectCount: FnPtr,
         pub(super) MultiDrawElementsIndirectCountARB: FnPtr,
         pub(super) MultiTexBufferEXT: FnPtr,
         pub(super) MultiTexCoord1dARB: FnPtr,
         pub(super) MultiTexCoord1dvARB: FnPtr,
         pub(super) MultiTexCoord1fARB: FnPtr,
         pub(super) MultiTexCoord1fvARB: FnPtr,
         pub(super) MultiTexCoord1iARB: FnPtr,
         pub(super) MultiTexCoord1ivARB: FnPtr,
         pub(super) MultiTexCoord1sARB: FnPtr,
         pub(super) MultiTexCoord1svARB: FnPtr,
         pub(super) MultiTexCoord2dARB: FnPtr,
         pub(super) MultiTexCoord2dvARB: FnPtr,
         pub(super) MultiTexCoord2fARB: FnPtr,
         pub(super) MultiTexCoord2fvARB: FnPtr,
         pub(super) MultiTexCoord2iARB: FnPtr,
         pub(super) MultiTexCoord2ivARB: FnPtr,
         pub(super) MultiTexCoord2sARB: FnPtr,
         pub(super) MultiTexCoord2svARB: FnPtr,
         pub(super) MultiTexCoord3dARB: FnPtr,
         pub(super) MultiTexCoord3dvARB: FnPtr,
         pub(super) MultiTexCoord3fARB: FnPtr,
         pub(super) MultiTexCoord3fvARB: FnPtr,
         pub(super) MultiTexCoord3iARB: FnPtr,
         pub(super) MultiTexCoord3ivARB: FnPtr,
         pub(super) MultiTexCoord3sARB: FnPtr,
         pub(super) MultiTexCoord3svARB: FnPtr,
         pub(super) MultiTexCoord4dARB: FnPtr,
         pub(super) MultiTexCoord4dvARB: FnPtr,
         pub(super) MultiTexCoord4fARB: FnPtr,
         pub(super) MultiTexCoord4fvARB: FnPtr,
         pub(super) MultiTexCoord4iARB: FnPtr,
         pub(super) MultiTexCoord4ivARB: FnPtr,
         pub(super) MultiTexCoord4sARB: FnPtr,
         pub(super) MultiTexCoord4svARB: FnPtr,
         pub(super) MultiTexCoordPointerEXT: FnPtr,
         pub(super) MultiTexEnvfEXT: FnPtr,
         pub(super) MultiTexEnvfvEXT: FnPtr,
         pub(super) MultiTexEnviEXT: FnPtr,
         pub(super) MultiTexEnvivEXT: FnPtr,
         pub(super) MultiTexGendEXT: FnPtr,
         pub(super) MultiTexGendvEXT: FnPtr,
         pub(super) MultiTexGenfEXT: FnPtr,
         pub(super) MultiTexGenfvEXT: FnPtr,
         pub(super) MultiTexGeniEXT: FnPtr,
         pub(super) MultiTexGenivEXT: FnPtr,
         pub(super) MultiTexImage1DEXT: FnPtr,
         pub(super) MultiTexImage2DEXT: FnPtr,
         pub(super) MultiTexImage3DEXT: FnPtr,
         pub(super) MultiTexParameterIivEXT: FnPtr,
         pub(super) MultiTexParameterIuivEXT: FnPtr,
         pub(super) MultiTexParameterfEXT: FnPtr,
         pub(super) MultiTexParameterfvEXT: FnPtr,
         pub(super) MultiTexParameteriEXT: FnPtr,
         pub(super) MultiTexParameterivEXT: FnPtr,
         pub(super) MultiTexRenderbufferEXT: FnPtr,
         pub(super) MultiTexSubImage1DEXT: FnPtr,
         pub(super) MultiTexSubImage2DEXT: FnPtr,
         pub(super) MultiTexSubImage3DEXT: FnPtr,
         pub(super) NamedBufferData: FnPtr,
         pub(super) NamedBufferDataEXT: FnPtr,
         pub(super) NamedBufferStorage: FnPtr,
         pub(super) NamedBufferStorageEXT: FnPtr,
         pub(super) NamedBufferSubData: FnPtr,
         pub(super) NamedBufferSubDataEXT: FnPtr,
         pub(super) NamedCopyBufferSubDataEXT: FnPtr,
         pub(super) NamedFramebufferDrawBuffer: FnPtr,
         pub(super) NamedFramebufferDrawBuffers: FnPtr,
         pub(super) NamedFramebufferParameteri: FnPtr,
         pub(super) NamedFramebufferParameteriEXT: FnPtr,
         pub(super) NamedFramebufferReadBuffer: FnPtr,
         pub(super) NamedFramebufferRenderbuffer: FnPtr,
         pub(super) NamedFramebufferRenderbufferEXT: FnPtr,
         pub(super) NamedFramebufferTexture: FnPtr,
         pub(super) NamedFramebufferTexture1DEXT: FnPtr,
         pub(super) NamedFramebufferTexture2DEXT: FnPtr,
         pub(super) NamedFramebufferTexture3DEXT: FnPtr,
         pub(super) NamedFramebufferTextureEXT: FnPtr,
         pub(super) NamedFramebufferTextureFaceEXT: FnPtr,
         pub(super) NamedFramebufferTextureLayer: FnPtr,
         pub(super) NamedFramebufferTextureLayerEXT: FnPtr,
         pub(super) NamedProgramLocalParameter4dEXT: FnPtr,
         pub(super) NamedProgramLocalParameter4dvEXT: FnPtr,
         pub(super) NamedProgramLocalParameter4fEXT: FnPtr,
         pub(super) NamedProgramLocalParameter4fvEXT: FnPtr,
         pub(super) NamedProgramLocalParameterI4iEXT: FnPtr,
         pub(super) NamedProgramLocalParameterI4ivEXT: FnPtr,
         pub(super) NamedProgramLocalParameterI4uiEXT: FnPtr,
         pub(super) NamedProgramLocalParameterI4uivEXT: FnPtr,
         pub(super) NamedProgramLocalParameters4fvEXT: FnPtr,
         pub(super) NamedProgramLocalParametersI4ivEXT: FnPtr,
         pub(super) NamedProgramLocalParametersI4uivEXT: FnPtr,
         pub(super) NamedProgramStringEXT: FnPtr,
         pub(super) NamedRenderbufferStorage: FnPtr,
         pub(super) NamedRenderbufferStorageEXT: FnPtr,
         pub(super) NamedRenderbufferStorageMultisample: FnPtr,
         pub(super) NamedRenderbufferStorageMultisampleCoverageEXT: FnPtr,
         pub(super) NamedRenderbufferStorageMultisampleEXT: FnPtr,
         pub(super) NormalPointerEXT: FnPtr,
         pub(super) ObjectLabel: FnPtr,
         pub(super) ObjectPtrLabel: FnPtr,
         pub(super) OrthofOES: FnPtr,
         pub(super) PatchParameterfv: FnPtr,
         pub(super) PatchParameteri: FnPtr,
         pub(super) PauseTransformFeedback: FnPtr,
         pub(super) PauseTransformFeedbackNV: FnPtr,
         pub(super) PixelStoref: FnPtr,
         pub(super) PixelStorei: FnPtr,
         pub(super) PointParameterf: FnPtr,
         pub(super) PointParameterfARB: FnPtr,
         pub(super) PointParameterfEXT: FnPtr,
         pub(super) PointParameterfSGIS: FnPtr,
         pub(super) PointParameterfv: FnPtr,
         pub(super) PointParameterfvARB: FnPtr,
         pub(super) PointParameterfvEXT: FnPtr,
         pub(super) PointParameterfvSGIS: FnPtr,
         pub(super) PointParameteri: FnPtr,
         pub(super) PointParameteriNV: FnPtr,
         pub(super) PointParameteriv: FnPtr,
         pub(super) PointParameterivNV: FnPtr,
         pub(super) PointSize: FnPtr,
         pub(super) PolygonMode: FnPtr,
         pub(super) PolygonOffset: FnPtr,
         pub(super) PolygonOffsetClamp: FnPtr,
         pub(super) PolygonOffsetClampEXT: FnPtr,
         pub(super) PopDebugGroup: FnPtr,
         pub(super) PrimitiveRestartIndex: FnPtr,
         pub(super) PrioritizeTexturesEXT: FnPtr,
         pub(super) ProgramBinary: FnPtr,
         pub(super) ProgramEnvParameter4dARB: FnPtr,
         pub(super) ProgramEnvParameter4dvARB: FnPtr,
         pub(super) ProgramEnvParameter4fARB: FnPtr,
         pub(super) ProgramEnvParameter4fvARB: FnPtr,
         pub(super) ProgramLocalParameter4dARB: FnPtr,
         pub(super) ProgramLocalParameter4dvARB: FnPtr,
         pub(super) ProgramLocalParameter4fARB: FnPtr,
         pub(super) ProgramLocalParameter4fvARB: FnPtr,
         pub(super) ProgramParameter4dNV: FnPtr,
         pub(super) ProgramParameter4dvNV: FnPtr,
         pub(super) ProgramParameter4fNV: FnPtr,
         pub(super) ProgramParameter4fvNV: FnPtr,
         pub(super) ProgramParameteri: FnPtr,
         pub(super) ProgramParameteriARB: FnPtr,
         pub(super) ProgramParameteriEXT: FnPtr,
         pub(super) ProgramParameters4dvNV: FnPtr,
         pub(super) ProgramParameters4fvNV: FnPtr,
         pub(super) ProgramStringARB: FnPtr,
         pub(super) ProgramUniform1d: FnPtr,
         pub(super) ProgramUniform1dEXT: FnPtr,
         pub(super) ProgramUniform1dv: FnPtr,
         pub(super) ProgramUniform1dvEXT: FnPtr,
         pub(super) ProgramUniform1f: FnPtr,
         pub(super) ProgramUniform1fEXT: FnPtr,
         pub(super) ProgramUniform1fv: FnPtr,
         pub(super) ProgramUniform1fvEXT: FnPtr,
         pub(super) ProgramUniform1i: FnPtr,
         pub(super) ProgramUniform1iEXT: FnPtr,
         pub(super) ProgramUniform1iv: FnPtr,
         pub(super) ProgramUniform1ivEXT: FnPtr,
         pub(super) ProgramUniform1ui: FnPtr,
         pub(super) ProgramUniform1uiEXT: FnPtr,
         pub(super) ProgramUniform1uiv: FnPtr,
         pub(super) ProgramUniform1uivEXT: FnPtr,
         pub(super) ProgramUniform2d: FnPtr,
         pub(super) ProgramUniform2dEXT: FnPtr,
         pub(super) ProgramUniform2dv: FnPtr,
         pub(super) ProgramUniform2dvEXT: FnPtr,
         pub(super) ProgramUniform2f: FnPtr,
         pub(super) ProgramUniform2fEXT: FnPtr,
         pub(super) ProgramUniform2fv: FnPtr,
         pub(super) ProgramUniform2fvEXT: FnPtr,
         pub(super) ProgramUniform2i: FnPtr,
         pub(super) ProgramUniform2iEXT: FnPtr,
         pub(super) ProgramUniform2iv: FnPtr,
         pub(super) ProgramUniform2ivEXT: FnPtr,
         pub(super) ProgramUniform2ui: FnPtr,
         pub(super) ProgramUniform2uiEXT: FnPtr,
         pub(super) ProgramUniform2uiv: FnPtr,
         pub(super) ProgramUniform2uivEXT: FnPtr,
         pub(super) ProgramUniform3d: FnPtr,
         pub(super) ProgramUniform3dEXT: FnPtr,
         pub(super) ProgramUniform3dv: FnPtr,
         pub(super) ProgramUniform3dvEXT: FnPtr,
         pub(super) ProgramUniform3f: FnPtr,
         pub(super) ProgramUniform3fEXT: FnPtr,
         pub(super) ProgramUniform3fv: FnPtr,
         pub(super) ProgramUniform3fvEXT: FnPtr,
         pub(super) ProgramUniform3i: FnPtr,
         pub(super) ProgramUniform3iEXT: FnPtr,
         pub(super) ProgramUniform3iv: FnPtr,
         pub(super) ProgramUniform3ivEXT: FnPtr,
         pub(super) ProgramUniform3ui: FnPtr,
         pub(super) ProgramUniform3uiEXT: FnPtr,
         pub(super) ProgramUniform3uiv: FnPtr,
         pub(super) ProgramUniform3uivEXT: FnPtr,
         pub(super) ProgramUniform4d: FnPtr,
         pub(super) ProgramUniform4dEXT: FnPtr,
         pub(super) ProgramUniform4dv: FnPtr,
         pub(super) ProgramUniform4dvEXT: FnPtr,
         pub(super) ProgramUniform4f: FnPtr,
         pub(super) ProgramUniform4fEXT: FnPtr,
         pub(super) ProgramUniform4fv: FnPtr,
         pub(super) ProgramUniform4fvEXT: FnPtr,
         pub(super) ProgramUniform4i: FnPtr,
         pub(super) ProgramUniform4iEXT: FnPtr,
         pub(super) ProgramUniform4iv: FnPtr,
         pub(super) ProgramUniform4ivEXT: FnPtr,
         pub(super) ProgramUniform4ui: FnPtr,
         pub(super) ProgramUniform4uiEXT: FnPtr,
         pub(super) ProgramUniform4uiv: FnPtr,
         pub(super) ProgramUniform4uivEXT: FnPtr,
         pub(super) ProgramUniformMatrix2dv: FnPtr,
         pub(super) ProgramUniformMatrix2dvEXT: FnPtr,
         pub(super) ProgramUniformMatrix2fv: FnPtr,
         pub(super) ProgramUniformMatrix2fvEXT: FnPtr,
         pub(super) ProgramUniformMatrix2x3dv: FnPtr,
         pub(super) ProgramUniformMatrix2x3dvEXT: FnPtr,
         pub(super) ProgramUniformMatrix2x3fv: FnPtr,
         pub(super) ProgramUniformMatrix2x3fvEXT: FnPtr,
         pub(super) ProgramUniformMatrix2x4dv: FnPtr,
         pub(super) ProgramUniformMatrix2x4dvEXT: FnPtr,
         pub(super) ProgramUniformMatrix2x4fv: FnPtr,
         pub(super) ProgramUniformMatrix2x4fvEXT: FnPtr,
         pub(super) ProgramUniformMatrix3dv: FnPtr,
         pub(super) ProgramUniformMatrix3dvEXT: FnPtr,
         pub(super) ProgramUniformMatrix3fv: FnPtr,
         pub(super) ProgramUniformMatrix3fvEXT: FnPtr,
         pub(super) ProgramUniformMatrix3x2dv: FnPtr,
         pub(super) ProgramUniformMatrix3x2dvEXT: FnPtr,
         pub(super) ProgramUniformMatrix3x2fv: FnPtr,
         pub(super) ProgramUniformMatrix3x2fvEXT: FnPtr,
         pub(super) ProgramUniformMatrix3x4dv: FnPtr,
         pub(super) ProgramUniformMatrix3x4dvEXT: FnPtr,
         pub(super) ProgramUniformMatrix3x4fv: FnPtr,
         pub(super) ProgramUniformMatrix3x4fvEXT: FnPtr,
         pub(super) ProgramUniformMatrix4dv: FnPtr,
         pub(super) ProgramUniformMatrix4dvEXT: FnPtr,
         pub(super) ProgramUniformMatrix4fv: FnPtr,
         pub(super) ProgramUniformMatrix4fvEXT: FnPtr,
         pub(super) ProgramUniformMatrix4x2dv: FnPtr,
         pub(super) ProgramUniformMatrix4x2dvEXT: FnPtr,
         pub(super) ProgramUniformMatrix4x2fv: FnPtr,
         pub(super) ProgramUniformMatrix4x2fvEXT: FnPtr,
         pub(super) ProgramUniformMatrix4x3dv: FnPtr,
         pub(super) ProgramUniformMatrix4x3dvEXT: FnPtr,
         pub(super) ProgramUniformMatrix4x3fv: FnPtr,
         pub(super) ProgramUniformMatrix4x3fvEXT: FnPtr,
         pub(super) ProgramVertexLimitNV: FnPtr,
         pub(super) ProvokingVertex: FnPtr,
         pub(super) ProvokingVertexEXT: FnPtr,
         pub(super) PushClientAttribDefaultEXT: FnPtr,
         pub(super) PushDebugGroup: FnPtr,
         pub(super) QueryCounter: FnPtr,
         pub(super) ReadBuffer: FnPtr,
         pub(super) ReadPixels: FnPtr,
         pub(super) ReadnPixels: FnPtr,
         pub(super) ReadnPixelsARB: FnPtr,
         pub(super) ReleaseShaderCompiler: FnPtr,
         pub(super) RenderbufferStorage: FnPtr,
         pub(super) RenderbufferStorageEXT: FnPtr,
         pub(super) RenderbufferStorageMultisample: FnPtr,
         pub(super) RenderbufferStorageMultisampleEXT: FnPtr,
         pub(super) RequestResidentProgramsNV: FnPtr,
         pub(super) ResumeTransformFeedback: FnPtr,
         pub(super) ResumeTransformFeedbackNV: FnPtr,
         pub(super) SampleCoverage: FnPtr,
         pub(super) SampleCoverageARB: FnPtr,
         pub(super) SampleMaskIndexedNV: FnPtr,
         pub(super) SampleMaski: FnPtr,
         pub(super) SamplerParameterIiv: FnPtr,
         pub(super) SamplerParameterIuiv: FnPtr,
         pub(super) SamplerParameterf: FnPtr,
         pub(super) SamplerParameterfv: FnPtr,
         pub(super) SamplerParameteri: FnPtr,
         pub(super) SamplerParameteriv: FnPtr,
         pub(super) Scissor: FnPtr,
         pub(super) ScissorArrayv: FnPtr,
         pub(super) ScissorIndexed: FnPtr,
         pub(super) ScissorIndexedv: FnPtr,
         pub(super) ShaderBinary: FnPtr,
         pub(super) ShaderSource: FnPtr,
         pub(super) ShaderSourceARB: FnPtr,
         pub(super) ShaderStorageBlockBinding: FnPtr,
         pub(super) SpecializeShader: FnPtr,
         pub(super) SpecializeShaderARB: FnPtr,
         pub(super) StencilFunc: FnPtr,
         pub(super) StencilFuncSeparate: FnPtr,
         pub(super) StencilFuncSeparateATI: FnPtr,
         pub(super) StencilMask: FnPtr,
         pub(super) StencilMaskSeparate: FnPtr,
         pub(super) StencilOp: FnPtr,
         pub(super) StencilOpSeparate: FnPtr,
         pub(super) StencilOpSeparateATI: FnPtr,
         pub(super) TexBuffer: FnPtr,
         pub(super) TexBufferARB: FnPtr,
         pub(super) TexBufferEXT: FnPtr,
         pub(super) TexBufferRange: FnPtr,
         pub(super) TexCoordPointerEXT: FnPtr,
         pub(super) TexImage1D: FnPtr,
         pub(super) TexImage2D: FnPtr,
         pub(super) TexImage2DMultisample: FnPtr,
         pub(super) TexImage3D: FnPtr,
         pub(super) TexImage3DEXT: FnPtr,
         pub(super) TexImage3DMultisample: FnPtr,
         pub(super) TexParameterIiv: FnPtr,
         pub(super) TexParameterIivEXT: FnPtr,
         pub(super) TexParameterIuiv: FnPtr,
         pub(super) TexParameterIuivEXT: FnPtr,
         pub(super) TexParameterf: FnPtr,
         pub(super) TexParameterfv: FnPtr,
         pub(super) TexParameteri: FnPtr,
         pub(super) TexParameteriv: FnPtr,
         pub(super) TexRenderbufferNV: FnPtr,
         pub(super) TexStorage1D: FnPtr,
         pub(super) TexStorage1DEXT: FnPtr,
         pub(super) TexStorage2D: FnPtr,
         pub(super) TexStorage2DEXT: FnPtr,
         pub(super) TexStorage2DMultisample: FnPtr,
         pub(super) TexStorage3D: FnPtr,
         pub(super) TexStorage3DEXT: FnPtr,
         pub(super) TexStorage3DMultisample: FnPtr,
         pub(super) TexSubImage1D: FnPtr,
         pub(super) TexSubImage1DEXT: FnPtr,
         pub(super) TexSubImage2D: FnPtr,
         pub(super) TexSubImage2DEXT: FnPtr,
         pub(super) TexSubImage3D: FnPtr,
         pub(super) TexSubImage3DEXT: FnPtr,
         pub(super) TextureBarrier: FnPtr,
         pub(super) TextureBuffer: FnPtr,
         pub(super) TextureBufferEXT: FnPtr,
         pub(super) TextureBufferRange: FnPtr,
         pub(super) TextureBufferRangeEXT: FnPtr,
         pub(super) TextureImage1DEXT: FnPtr,
         pub(super) TextureImage2DEXT: FnPtr,
         pub(super) TextureImage3DEXT: FnPtr,
         pub(super) TexturePageCommitmentEXT: FnPtr,
         pub(super) TextureParameterIiv: FnPtr,
         pub(super) TextureParameterIivEXT: FnPtr,
         pub(super) TextureParameterIuiv: FnPtr,
         pub(super) TextureParameterIuivEXT: FnPtr,
         pub(super) TextureParameterf: FnPtr,
         pub(super) TextureParameterfEXT: FnPtr,
         pub(super) TextureParameterfv: FnPtr,
         pub(super) TextureParameterfvEXT: FnPtr,
         pub(super) TextureParameteri: FnPtr,
         pub(super) TextureParameteriEXT: FnPtr,
         pub(super) TextureParameteriv: FnPtr,
         pub(super) TextureParameterivEXT: FnPtr,
         pub(super) TextureRenderbufferEXT: FnPtr,
         pub(super) TextureStorage1D: FnPtr,
         pub(super) TextureStorage1DEXT: FnPtr,
         pub(super) TextureStorage2D: FnPtr,
         pub(super) TextureStorage2DEXT: FnPtr,
         pub(super) TextureStorage2DMultisample: FnPtr,
         pub(super) TextureStorage2DMultisampleEXT: FnPtr,
         pub(super) TextureStorage3D: FnPtr,
         pub(super) TextureStorage3DEXT: FnPtr,
         pub(super) TextureStorage3DMultisample: FnPtr,
         pub(super) TextureStorage3DMultisampleEXT: FnPtr,
         pub(super) TextureSubImage1D: FnPtr,
         pub(super) TextureSubImage1DEXT: FnPtr,
         pub(super) TextureSubImage2D: FnPtr,
         pub(super) TextureSubImage2DEXT: FnPtr,
         pub(super) TextureSubImage3D: FnPtr,
         pub(super) TextureSubImage3DEXT: FnPtr,
         pub(super) TextureView: FnPtr,
         pub(super) TrackMatrixNV: FnPtr,
         pub(super) TransformFeedbackAttribsNV: FnPtr,
         pub(super) TransformFeedbackBufferBase: FnPtr,
         pub(super) TransformFeedbackBufferRange: FnPtr,
         pub(super) TransformFeedbackStreamAttribsNV: FnPtr,
         pub(super) TransformFeedbackVaryings: FnPtr,
         pub(super) TransformFeedbackVaryingsEXT: FnPtr,
         pub(super) TransformFeedbackVaryingsNV: FnPtr,
         pub(super) Uniform1d: FnPtr,
         pub(super) Uniform1dv: FnPtr,
         pub(super) Uniform1f: FnPtr,
         pub(super) Uniform1fARB: FnPtr,
         pub(super) Uniform1fv: FnPtr,
         pub(super) Uniform1fvARB: FnPtr,
         pub(super) Uniform1i: FnPtr,
         pub(super) Uniform1iARB: FnPtr,
         pub(super) Uniform1iv: FnPtr,
         pub(super) Uniform1ivARB: FnPtr,
         pub(super) Uniform1ui: FnPtr,
         pub(super) Uniform1uiEXT: FnPtr,
         pub(super) Uniform1uiv: FnPtr,
         pub(super) Uniform1uivEXT: FnPtr,
         pub(super) Uniform2d: FnPtr,
         pub(super) Uniform2dv: FnPtr,
         pub(super) Uniform2f: FnPtr,
         pub(super) Uniform2fARB: FnPtr,
         pub(super) Uniform2fv: FnPtr,
         pub(super) Uniform2fvARB: FnPtr,
         pub(super) Uniform2i: FnPtr,
         pub(super) Uniform2iARB: FnPtr,
         pub(super) Uniform2iv: FnPtr,
         pub(super) Uniform2ivARB: FnPtr,
         pub(super) Uniform2ui: FnPtr,
         pub(super) Uniform2uiEXT: FnPtr,
         pub(super) Uniform2uiv: FnPtr,
         pub(super) Uniform2uivEXT: FnPtr,
         pub(super) Uniform3d: FnPtr,
         pub(super) Uniform3dv: FnPtr,
         pub(super) Uniform3f: FnPtr,
         pub(super) Uniform3fARB: FnPtr,
         pub(super) Uniform3fv: FnPtr,
         pub(super) Uniform3fvARB: FnPtr,
         pub(super) Uniform3i: FnPtr,
         pub(super) Uniform3iARB: FnPtr,
         pub(super) Uniform3iv: FnPtr,
         pub(super) Uniform3ivARB: FnPtr,
         pub(super) Uniform3ui: FnPtr,
         pub(super) Uniform3uiEXT: FnPtr,
         pub(super) Uniform3uiv: FnPtr,
         pub(super) Uniform3uivEXT: FnPtr,
         pub(super) Uniform4d: FnPtr,
         pub(super) Uniform4dv: FnPtr,
         pub(super) Uniform4f: FnPtr,
         pub(super) Uniform4fARB: FnPtr,
         pub(super) Uniform4fv: FnPtr,
         pub(super) Uniform4fvARB: FnPtr,
         pub(super) Uniform4i: FnPtr,
         pub(super) Uniform4iARB: FnPtr,
         pub(super) Uniform4iv: FnPtr,
         pub(super) Uniform4ivARB: FnPtr,
         pub(super) Uniform4ui: FnPtr,
         pub(super) Uniform4uiEXT: FnPtr,
         pub(super) Uniform4uiv: FnPtr,
         pub(super) Uniform4uivEXT: FnPtr,
         pub(super) UniformBlockBinding: FnPtr,
         pub(super) UniformMatrix2dv: FnPtr,
         pub(super) UniformMatrix2fv: FnPtr,
         pub(super) UniformMatrix2fvARB: FnPtr,
         pub(super) UniformMatrix2x3dv: FnPtr,
         pub(super) UniformMatrix2x3fv: FnPtr,
         pub(super) UniformMatrix2x4dv: FnPtr,
         pub(super) UniformMatrix2x4fv: FnPtr,
         pub(super) UniformMatrix3dv: FnPtr,
         pub(super) UniformMatrix3fv: FnPtr,
         pub(super) UniformMatrix3fvARB: FnPtr,
         pub(super) UniformMatrix3x2dv: FnPtr,
         pub(super) UniformMatrix3x2fv: FnPtr,
         pub(super) UniformMatrix3x4dv: FnPtr,
         pub(super) UniformMatrix3x4fv: FnPtr,
         pub(super) UniformMatrix4dv: FnPtr,
         pub(super) UniformMatrix4fv: FnPtr,
         pub(super) UniformMatrix4fvARB: FnPtr,
         pub(super) UniformMatrix4x2dv: FnPtr,
         pub(super) UniformMatrix4x2fv: FnPtr,
         pub(super) UniformMatrix4x3dv: FnPtr,
         pub(super) UniformMatrix4x3fv: FnPtr,
         pub(super) UniformSubroutinesuiv: FnPtr,
         pub(super) UnmapBuffer: FnPtr,
         pub(super) UnmapBufferARB: FnPtr,
         pub(super) UnmapNamedBuffer: FnPtr,
         pub(super) UnmapNamedBufferEXT: FnPtr,
         pub(super) UseProgram: FnPtr,
         pub(super) UseProgramObjectARB: FnPtr,
         pub(super) UseProgramStages: FnPtr,
         pub(super) ValidateProgram: FnPtr,
         pub(super) ValidateProgramARB: FnPtr,
         pub(super) ValidateProgramPipeline: FnPtr,
         pub(super) VertexArrayAttribBinding: FnPtr,
         pub(super) VertexArrayAttribFormat: FnPtr,
         pub(super) VertexArrayAttribIFormat: FnPtr,
         pub(super) VertexArrayAttribLFormat: FnPtr,
         pub(super) VertexArrayBindVertexBufferEXT: FnPtr,
         pub(super) VertexArrayBindingDivisor: FnPtr,
         pub(super) VertexArrayColorOffsetEXT: FnPtr,
         pub(super) VertexArrayEdgeFlagOffsetEXT: FnPtr,
         pub(super) VertexArrayElementBuffer: FnPtr,
         pub(super) VertexArrayFogCoordOffsetEXT: FnPtr,
         pub(super) VertexArrayIndexOffsetEXT: FnPtr,
         pub(super) VertexArrayMultiTexCoordOffsetEXT: FnPtr,
         pub(super) VertexArrayNormalOffsetEXT: FnPtr,
         pub(super) VertexArraySecondaryColorOffsetEXT: FnPtr,
         pub(super) VertexArrayTexCoordOffsetEXT: FnPtr,
         pub(super) VertexArrayVertexAttribBindingEXT: FnPtr,
         pub(super) VertexArrayVertexAttribDivisorEXT: FnPtr,
         pub(super) VertexArrayVertexAttribFormatEXT: FnPtr,
         pub(super) VertexArrayVertexAttribIFormatEXT: FnPtr,
         pub(super) VertexArrayVertexAttribIOffsetEXT: FnPtr,
         pub(super) VertexArrayVertexAttribLFormatEXT: FnPtr,
         pub(super) VertexArrayVertexAttribLOffsetEXT: FnPtr,
         pub(super) VertexArrayVertexAttribOffsetEXT: FnPtr,
         pub(super) VertexArrayVertexBindingDivisorEXT: FnPtr,
         pub(super) VertexArrayVertexBuffer: FnPtr,
         pub(super) VertexArrayVertexBuffers: FnPtr,
         pub(super) VertexArrayVertexOffsetEXT: FnPtr,
         pub(super) VertexAttrib1d: FnPtr,
         pub(super) VertexAttrib1dARB: FnPtr,
         pub(super) VertexAttrib1dNV: FnPtr,
         pub(super) VertexAttrib1dv: FnPtr,
         pub(super) VertexAttrib1dvARB: FnPtr,
         pub(super) VertexAttrib1dvNV: FnPtr,
         pub(super) VertexAttrib1f: FnPtr,
         pub(super) VertexAttrib1fARB: FnPtr,
         pub(super) VertexAttrib1fNV: FnPtr,
         pub(super) VertexAttrib1fv: FnPtr,
         pub(super) VertexAttrib1fvARB: FnPtr,
         pub(super) VertexAttrib1fvNV: FnPtr,
         pub(super) VertexAttrib1s: FnPtr,
         pub(super) VertexAttrib1sARB: FnPtr,
         pub(super) VertexAttrib1sNV: FnPtr,
         pub(super) VertexAttrib1sv: FnPtr,
         pub(super) VertexAttrib1svARB: FnPtr,
         pub(super) VertexAttrib1svNV: FnPtr,
         pub(super) VertexAttrib2d: FnPtr,
         pub(super) VertexAttrib2dARB: FnPtr,
         pub(super) VertexAttrib2dNV: FnPtr,
         pub(super) VertexAttrib2dv: FnPtr,
         pub(super) VertexAttrib2dvARB: FnPtr,
         pub(super) VertexAttrib2dvNV: FnPtr,
         pub(super) VertexAttrib2f: FnPtr,
         pub(super) VertexAttrib2fARB: FnPtr,
         pub(super) VertexAttrib2fNV: FnPtr,
         pub(super) VertexAttrib2fv: FnPtr,
         pub(super) VertexAttrib2fvARB: FnPtr,
         pub(super) VertexAttrib2fvNV: FnPtr,
         pub(super) VertexAttrib2s: FnPtr,
         pub(super) VertexAttrib2sARB: FnPtr,
         pub(super) VertexAttrib2sNV: FnPtr,
         pub(super) VertexAttrib2sv: FnPtr,
         pub(super) VertexAttrib2svARB: FnPtr,
         pub(super) VertexAttrib2svNV: FnPtr,
         pub(super) VertexAttrib3d: FnPtr,
         pub(super) VertexAttrib3dARB: FnPtr,
         pub(super) VertexAttrib3dNV: FnPtr,
         pub(super) VertexAttrib3dv: FnPtr,
         pub(super) VertexAttrib3dvARB: FnPtr,
         pub(super) VertexAttrib3dvNV: FnPtr,
         pub(super) VertexAttrib3f: FnPtr,
         pub(super) VertexAttrib3fARB: FnPtr,
         pub(super) VertexAttrib3fNV: FnPtr,
         pub(super) VertexAttrib3fv: FnPtr,
         pub(super) VertexAttrib3fvARB: FnPtr,
         pub(super) VertexAttrib3fvNV: FnPtr,
         pub(super) VertexAttrib3s: FnPtr,
         pub(super) VertexAttrib3sARB: FnPtr,
         pub(super) VertexAttrib3sNV: FnPtr,
         pub(super) VertexAttrib3sv: FnPtr,
         pub(super) VertexAttrib3svARB: FnPtr,
         pub(super) VertexAttrib3svNV: FnPtr,
         pub(super) VertexAttrib4Nbv: FnPtr,
         pub(super) VertexAttrib4NbvARB: FnPtr,
         pub(super) VertexAttrib4Niv: FnPtr,
         pub(super) VertexAttrib4NivARB: FnPtr,
         pub(super) VertexAttrib4Nsv: FnPtr,
         pub(super) VertexAttrib4NsvARB: FnPtr,
         pub(super) VertexAttrib4Nub: FnPtr,
         pub(super) VertexAttrib4NubARB: FnPtr,
         pub(super) VertexAttrib4Nubv: FnPtr,
         pub(super) VertexAttrib4NubvARB: FnPtr,
         pub(super) VertexAttrib4Nuiv: FnPtr,
         pub(super) VertexAttrib4NuivARB: FnPtr,
         pub(super) VertexAttrib4Nusv: FnPtr,
         pub(super) VertexAttrib4NusvARB: FnPtr,
         pub(super) VertexAttrib4bv: FnPtr,
         pub(super) VertexAttrib4bvARB: FnPtr,
         pub(super) VertexAttrib4d: FnPtr,
         pub(super) VertexAttrib4dARB: FnPtr,
         pub(super) VertexAttrib4dNV: FnPtr,
         pub(super) VertexAttrib4dv: FnPtr,
         pub(super) VertexAttrib4dvARB: FnPtr,
         pub(super) VertexAttrib4dvNV: FnPtr,
         pub(super) VertexAttrib4f: FnPtr,
         pub(super) VertexAttrib4fARB: FnPtr,
         pub(super) VertexAttrib4fNV: FnPtr,
         pub(super) VertexAttrib4fv: FnPtr,
         pub(super) VertexAttrib4fvARB: FnPtr,
         pub(super) VertexAttrib4fvNV: FnPtr,
         pub(super) VertexAttrib4iv: FnPtr,
         pub(super) VertexAttrib4ivARB: FnPtr,
         pub(super) VertexAttrib4s: FnPtr,
         pub(super) VertexAttrib4sARB: FnPtr,
         pub(super) VertexAttrib4sNV: FnPtr,
         pub(super) VertexAttrib4sv: FnPtr,
         pub(super) VertexAttrib4svARB: FnPtr,
         pub(super) VertexAttrib4svNV: FnPtr,
         pub(super) VertexAttrib4ubNV: FnPtr,
         pub(super) VertexAttrib4ubv: FnPtr,
         pub(super) VertexAttrib4ubvARB: FnPtr,
         pub(super) VertexAttrib4ubvNV: FnPtr,
         pub(super) VertexAttrib4uiv: FnPtr,
         pub(super) VertexAttrib4uivARB: FnPtr,
         pub(super) VertexAttrib4usv: FnPtr,
         pub(super) VertexAttrib4usvARB: FnPtr,
         pub(super) VertexAttribBinding: FnPtr,
         pub(super) VertexAttribDivisor: FnPtr,
         pub(super) VertexAttribDivisorARB: FnPtr,
         pub(super) VertexAttribFormat: FnPtr,
         pub(super) VertexAttribI1i: FnPtr,
         pub(super) VertexAttribI1iEXT: FnPtr,
         pub(super) VertexAttribI1iv: FnPtr,
         pub(super) VertexAttribI1ivEXT: FnPtr,
         pub(super) VertexAttribI1ui: FnPtr,
         pub(super) VertexAttribI1uiEXT: FnPtr,
         pub(super) VertexAttribI1uiv: FnPtr,
         pub(super) VertexAttribI1uivEXT: FnPtr,
         pub(super) VertexAttribI2i: FnPtr,
         pub(super) VertexAttribI2iEXT: FnPtr,
         pub(super) VertexAttribI2iv: FnPtr,
         pub(super) VertexAttribI2ivEXT: FnPtr,
         pub(super) VertexAttribI2ui: FnPtr,
         pub(super) VertexAttribI2uiEXT: FnPtr,
         pub(super) VertexAttribI2uiv: FnPtr,
         pub(super) VertexAttribI2uivEXT: FnPtr,
         pub(super) VertexAttribI3i: FnPtr,
         pub(super) VertexAttribI3iEXT: FnPtr,
         pub(super) VertexAttribI3iv: FnPtr,
         pub(super) VertexAttribI3ivEXT: FnPtr,
         pub(super) VertexAttribI3ui: FnPtr,
         pub(super) VertexAttribI3uiEXT: FnPtr,
         pub(super) VertexAttribI3uiv: FnPtr,
         pub(super) VertexAttribI3uivEXT: FnPtr,
         pub(super) VertexAttribI4bv: FnPtr,
         pub(super) VertexAttribI4bvEXT: FnPtr,
         pub(super) VertexAttribI4i: FnPtr,
         pub(super) VertexAttribI4iEXT: FnPtr,
         pub(super) VertexAttribI4iv: FnPtr,
         pub(super) VertexAttribI4ivEXT: FnPtr,
         pub(super) VertexAttribI4sv: FnPtr,
         pub(super) VertexAttribI4svEXT: FnPtr,
         pub(super) VertexAttribI4ubv: FnPtr,
         pub(super) VertexAttribI4ubvEXT: FnPtr,
         pub(super) VertexAttribI4ui: FnPtr,
         pub(super) VertexAttribI4uiEXT: FnPtr,
         pub(super) VertexAttribI4uiv: FnPtr,
         pub(super) VertexAttribI4uivEXT: FnPtr,
         pub(super) VertexAttribI4usv: FnPtr,
         pub(super) VertexAttribI4usvEXT: FnPtr,
         pub(super) VertexAttribIFormat: FnPtr,
         pub(super) VertexAttribIPointer: FnPtr,
         pub(super) VertexAttribIPointerEXT: FnPtr,
         pub(super) VertexAttribL1d: FnPtr,
         pub(super) VertexAttribL1dEXT: FnPtr,
         pub(super) VertexAttribL1dv: FnPtr,
         pub(super) VertexAttribL1dvEXT: FnPtr,
         pub(super) VertexAttribL2d: FnPtr,
         pub(super) VertexAttribL2dEXT: FnPtr,
         pub(super) VertexAttribL2dv: FnPtr,
         pub(super) VertexAttribL2dvEXT: FnPtr,
         pub(super) VertexAttribL3d: FnPtr,
         pub(super) VertexAttribL3dEXT: FnPtr,
         pub(super) VertexAttribL3dv: FnPtr,
         pub(super) VertexAttribL3dvEXT: FnPtr,
         pub(super) VertexAttribL4d: FnPtr,
         pub(super) VertexAttribL4dEXT: FnPtr,
         pub(super) VertexAttribL4dv: FnPtr,
         pub(super) VertexAttribL4dvEXT: FnPtr,
         pub(super) VertexAttribLFormat: FnPtr,
         pub(super) VertexAttribLPointer: FnPtr,
         pub(super) VertexAttribLPointerEXT: FnPtr,
         pub(super) VertexAttribP1ui: FnPtr,
         pub(super) VertexAttribP1uiv: FnPtr,
         pub(super) VertexAttribP2ui: FnPtr,
         pub(super) VertexAttribP2uiv: FnPtr,
         pub(super) VertexAttribP3ui: FnPtr,
         pub(super) VertexAttribP3uiv: FnPtr,
         pub(super) VertexAttribP4ui: FnPtr,
         pub(super) VertexAttribP4uiv: FnPtr,
         pub(super) VertexAttribPointer: FnPtr,
         pub(super) VertexAttribPointerARB: FnPtr,
         pub(super) VertexAttribPointerNV: FnPtr,
         pub(super) VertexAttribs1dvNV: FnPtr,
         pub(super) VertexAttribs1fvNV: FnPtr,
         pub(super) VertexAttribs1svNV: FnPtr,
         pub(super) VertexAttribs2dvNV: FnPtr,
         pub(super) VertexAttribs2fvNV: FnPtr,
         pub(super) VertexAttribs2svNV: FnPtr,
         pub(super) VertexAttribs3dvNV: FnPtr,
         pub(super) VertexAttribs3fvNV: FnPtr,
         pub(super) VertexAttribs3svNV: FnPtr,
         pub(super) VertexAttribs4dvNV: FnPtr,
         pub(super) VertexAttribs4fvNV: FnPtr,
         pub(super) VertexAttribs4svNV: FnPtr,
         pub(super) VertexAttribs4ubvNV: FnPtr,
         pub(super) VertexBindingDivisor: FnPtr,
         pub(super) VertexPointerEXT: FnPtr,
         pub(super) Viewport: FnPtr,
         pub(super) ViewportArrayv: FnPtr,
         pub(super) ViewportIndexedf: FnPtr,
         pub(super) ViewportIndexedfv: FnPtr,
         pub(super) WaitSync: FnPtr,
    }


    impl Gl {

     func!(ActiveShaderProgram, (), pipeline: GLuint, program: GLuint);
     func!(ActiveTexture, (), texture: GLenum);
     func!(ActiveTextureARB, (), texture: GLenum);
     func!(ActiveVaryingNV, (), program: GLuint, name: *const GLchar);
     func!(AreProgramsResidentNV, GLboolean, n: GLsizei, programs: *const GLuint, residences: *mut GLboolean);
     func!(AreTexturesResidentEXT, GLboolean, n: GLsizei, textures: *const GLuint, residences: *mut GLboolean);
     func!(ArrayElementEXT, (), i: GLint);
     func!(AttachObjectARB, (), containerObj: GLhandleARB, obj: GLhandleARB);
     func!(AttachShader, (), program: GLuint, shader: GLuint);
     func!(BeginConditionalRender, (), id: GLuint, mode: GLenum);
     func!(BeginConditionalRenderNV, (), id: GLuint, mode: GLenum);
     func!(BeginConditionalRenderNVX, (), id: GLuint);
     func!(BeginQuery, (), target: GLenum, id: GLuint);
     func!(BeginQueryARB, (), target: GLenum, id: GLuint);
     func!(BeginQueryIndexed, (), target: GLenum, index: GLuint, id: GLuint);
     func!(BeginTransformFeedback, (), primitiveMode: GLenum);
     func!(BeginTransformFeedbackEXT, (), primitiveMode: GLenum);
     func!(BeginTransformFeedbackNV, (), primitiveMode: GLenum);
     func!(BindAttribLocation, (), program: GLuint, index: GLuint, name: *const GLchar);
     func!(BindAttribLocationARB, (), programObj: GLhandleARB, index: GLuint, name: *const GLcharARB);
     func!(BindBuffer, (), target: GLenum, buffer: GLuint);
     func!(BindBufferARB, (), target: GLenum, buffer: GLuint);
     func!(BindBufferBase, (), target: GLenum, index: GLuint, buffer: GLuint);
     func!(BindBufferBaseEXT, (), target: GLenum, index: GLuint, buffer: GLuint);
     func!(BindBufferBaseNV, (), target: GLenum, index: GLuint, buffer: GLuint);
     func!(BindBufferOffsetEXT, (), target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr);
     func!(BindBufferOffsetNV, (), target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr);
     func!(BindBufferRange, (), target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
     func!(BindBufferRangeEXT, (), target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
     func!(BindBufferRangeNV, (), target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
     func!(BindBuffersBase, (), target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint);
     func!(BindBuffersRange, (), target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr);
     func!(BindFragDataLocation, (), program: GLuint, color: GLuint, name: *const GLchar);
     func!(BindFragDataLocationEXT, (), program: GLuint, color: GLuint, name: *const GLchar);
     func!(BindFragDataLocationIndexed, (), program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar);
     func!(BindFramebuffer, (), target: GLenum, framebuffer: GLuint);
     func!(BindFramebufferEXT, (), target: GLenum, framebuffer: GLuint);
     func!(BindImageTexture, (), unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum);
     func!(BindImageTextureEXT, (), index: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLint);
     func!(BindImageTextures, (), first: GLuint, count: GLsizei, textures: *const GLuint);
     func!(BindMultiTextureEXT, (), texunit: GLenum, target: GLenum, texture: GLuint);
     func!(BindProgramARB, (), target: GLenum, program: GLuint);
     func!(BindProgramNV, (), target: GLenum, id: GLuint);
     func!(BindProgramPipeline, (), pipeline: GLuint);
     func!(BindRenderbuffer, (), target: GLenum, renderbuffer: GLuint);
     func!(BindRenderbufferEXT, (), target: GLenum, renderbuffer: GLuint);
     func!(BindSampler, (), unit: GLuint, sampler: GLuint);
     func!(BindSamplers, (), first: GLuint, count: GLsizei, samplers: *const GLuint);
     func!(BindTexture, (), target: GLenum, texture: GLuint);
     func!(BindTextureEXT, (), target: GLenum, texture: GLuint);
     func!(BindTextureUnit, (), unit: GLuint, texture: GLuint);
     func!(BindTextures, (), first: GLuint, count: GLsizei, textures: *const GLuint);
     func!(BindTransformFeedback, (), target: GLenum, id: GLuint);
     func!(BindTransformFeedbackNV, (), target: GLenum, id: GLuint);
     func!(BindVertexArray, (), array: GLuint);
     func!(BindVertexArrayAPPLE, (), array: GLuint);
     func!(BindVertexBuffer, (), bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);
     func!(BindVertexBuffers, (), first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei);
     func!(BlendColor, (), red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
     func!(BlendColorEXT, (), red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
     func!(BlendEquation, (), mode: GLenum);
     func!(BlendEquationEXT, (), mode: GLenum);
     func!(BlendEquationIndexedAMD, (), buf: GLuint, mode: GLenum);
     func!(BlendEquationSeparate, (), modeRGB: GLenum, modeAlpha: GLenum);
     func!(BlendEquationSeparateEXT, (), modeRGB: GLenum, modeAlpha: GLenum);
     func!(BlendEquationSeparateIndexedAMD, (), buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum);
     func!(BlendEquationSeparatei, (), buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum);
     func!(BlendEquationSeparateiARB, (), buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum);
     func!(BlendEquationi, (), buf: GLuint, mode: GLenum);
     func!(BlendEquationiARB, (), buf: GLuint, mode: GLenum);
     func!(BlendFunc, (), sfactor: GLenum, dfactor: GLenum);
     func!(BlendFuncIndexedAMD, (), buf: GLuint, src: GLenum, dst: GLenum);
     func!(BlendFuncSeparate, (), sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum);
     func!(BlendFuncSeparateEXT, (), sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum);
     func!(BlendFuncSeparateINGR, (), sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum);
     func!(BlendFuncSeparateIndexedAMD, (), buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);
     func!(BlendFuncSeparatei, (), buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);
     func!(BlendFuncSeparateiARB, (), buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);
     func!(BlendFunci, (), buf: GLuint, src: GLenum, dst: GLenum);
     func!(BlendFunciARB, (), buf: GLuint, src: GLenum, dst: GLenum);
     func!(BlitFramebuffer, (), srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);
     func!(BlitFramebufferEXT, (), srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);
     func!(BlitNamedFramebuffer, (), readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);
     func!(BufferData, (), target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum);
     func!(BufferDataARB, (), target: GLenum, size: GLsizeiptrARB, data: *const c_void, usage: GLenum);
     func!(BufferParameteriAPPLE, (), target: GLenum, pname: GLenum, param: GLint);
     func!(BufferStorage, (), target: GLenum, size: GLsizeiptr, data: *const c_void, flags: GLbitfield);
     func!(BufferSubData, (), target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void);
     func!(BufferSubDataARB, (), target: GLenum, offset: GLintptrARB, size: GLsizeiptrARB, data: *const c_void);
     func!(CheckFramebufferStatus, GLenum, target: GLenum);
     func!(CheckFramebufferStatusEXT, GLenum, target: GLenum);
     func!(CheckNamedFramebufferStatus, GLenum, framebuffer: GLuint, target: GLenum);
     func!(CheckNamedFramebufferStatusEXT, GLenum, framebuffer: GLuint, target: GLenum);
     func!(ClampColor, (), target: GLenum, clamp: GLenum);
     func!(ClampColorARB, (), target: GLenum, clamp: GLenum);
     func!(Clear, (), mask: GLbitfield);
     func!(ClearBufferData, (), target: GLenum, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void);
     func!(ClearBufferSubData, (), target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void);
     func!(ClearBufferfi, (), buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
     func!(ClearBufferfv, (), buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);
     func!(ClearBufferiv, (), buffer: GLenum, drawbuffer: GLint, value: *const GLint);
     func!(ClearBufferuiv, (), buffer: GLenum, drawbuffer: GLint, value: *const GLuint);
     func!(ClearColor, (), red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
     func!(ClearColorIiEXT, (), red: GLint, green: GLint, blue: GLint, alpha: GLint);
     func!(ClearColorIuiEXT, (), red: GLuint, green: GLuint, blue: GLuint, alpha: GLuint);
     func!(ClearDepth, (), depth: GLdouble);
     func!(ClearDepthf, (), d: GLfloat);
     func!(ClearDepthfOES, (), depth: GLclampf);
     func!(ClearNamedBufferData, (), buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void);
     func!(ClearNamedBufferDataEXT, (), buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void);
     func!(ClearNamedBufferSubData, (), buffer: GLuint, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void);
     func!(ClearNamedBufferSubDataEXT, (), buffer: GLuint, internalformat: GLenum, offset: GLsizeiptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void);
     func!(ClearNamedFramebufferfi, (), framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
     func!(ClearNamedFramebufferfv, (), framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);
     func!(ClearNamedFramebufferiv, (), framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLint);
     func!(ClearNamedFramebufferuiv, (), framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLuint);
     func!(ClearStencil, (), s: GLint);
     func!(ClearTexImage, (), texture: GLuint, level: GLint, format: GLenum, type_: GLenum, data: *const c_void);
     func!(ClearTexSubImage, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, data: *const c_void);
     func!(ClientActiveTextureARB, (), texture: GLenum);
     func!(ClientAttribDefaultEXT, (), mask: GLbitfield);
     func!(ClientWaitSync, GLenum, sync: GLsync, flags: GLbitfield, timeout: GLuint64);
     func!(ClipControl, (), origin: GLenum, depth: GLenum);
     func!(ClipPlanefOES, (), plane: GLenum, equation: *const GLfloat);
     func!(ColorMask, (), red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
     func!(ColorMaskIndexedEXT, (), index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);
     func!(ColorMaski, (), index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);
     func!(ColorPointerEXT, (), size: GLint, type_: GLenum, stride: GLsizei, count: GLsizei, pointer: *const c_void);
     func!(CompileShader, (), shader: GLuint);
     func!(CompileShaderARB, (), shaderObj: GLhandleARB);
     func!(CompressedMultiTexImage1DEXT, (), texunit: GLenum, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedMultiTexImage2DEXT, (), texunit: GLenum, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedMultiTexImage3DEXT, (), texunit: GLenum, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedMultiTexSubImage1DEXT, (), texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedMultiTexSubImage2DEXT, (), texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedMultiTexSubImage3DEXT, (), texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedTexImage1D, (), target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexImage1DARB, (), target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexImage2D, (), target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexImage2DARB, (), target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexImage3D, (), target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexImage3DARB, (), target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexSubImage1D, (), target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexSubImage1DARB, (), target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexSubImage2D, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexSubImage2DARB, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexSubImage3D, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTexSubImage3DARB, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTextureImage1DEXT, (), texture: GLuint, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedTextureImage2DEXT, (), texture: GLuint, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedTextureImage3DEXT, (), texture: GLuint, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedTextureSubImage1D, (), texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTextureSubImage1DEXT, (), texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedTextureSubImage2D, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTextureSubImage2DEXT, (), texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, bits: *const c_void);
     func!(CompressedTextureSubImage3D, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
     func!(CompressedTextureSubImage3DEXT, (), texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, bits: *const c_void);
     func!(CopyBufferSubData, (), readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);
     func!(CopyImageSubData, (), srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);
     func!(CopyMultiTexImage1DEXT, (), texunit: GLenum, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint);
     func!(CopyMultiTexImage2DEXT, (), texunit: GLenum, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);
     func!(CopyMultiTexSubImage1DEXT, (), texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);
     func!(CopyMultiTexSubImage2DEXT, (), texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyMultiTexSubImage3DEXT, (), texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyNamedBufferSubData, (), readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);
     func!(CopyTexImage1D, (), target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint);
     func!(CopyTexImage1DEXT, (), target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint);
     func!(CopyTexImage2D, (), target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);
     func!(CopyTexImage2DEXT, (), target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);
     func!(CopyTexSubImage1D, (), target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);
     func!(CopyTexSubImage1DEXT, (), target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);
     func!(CopyTexSubImage2D, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyTexSubImage2DEXT, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyTexSubImage3D, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyTexSubImage3DEXT, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyTextureImage1DEXT, (), texture: GLuint, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint);
     func!(CopyTextureImage2DEXT, (), texture: GLuint, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);
     func!(CopyTextureSubImage1D, (), texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);
     func!(CopyTextureSubImage1DEXT, (), texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);
     func!(CopyTextureSubImage2D, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyTextureSubImage2DEXT, (), texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyTextureSubImage3D, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CopyTextureSubImage3DEXT, (), texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(CreateBuffers, (), n: GLsizei, buffers: *mut GLuint);
     func!(CreateFramebuffers, (), n: GLsizei, framebuffers: *mut GLuint);
     func!(CreateProgram, GLuint, );
     func!(CreateProgramObjectARB, GLhandleARB, );
     func!(CreateProgramPipelines, (), n: GLsizei, pipelines: *mut GLuint);
     func!(CreateQueries, (), target: GLenum, n: GLsizei, ids: *mut GLuint);
     func!(CreateRenderbuffers, (), n: GLsizei, renderbuffers: *mut GLuint);
     func!(CreateSamplers, (), n: GLsizei, samplers: *mut GLuint);
     func!(CreateShader, GLuint, type_: GLenum);
     func!(CreateShaderObjectARB, GLhandleARB, shaderType: GLenum);
     func!(CreateShaderProgramv, GLuint, type_: GLenum, count: GLsizei, strings: *const *const GLchar);
     func!(CreateTextures, (), target: GLenum, n: GLsizei, textures: *mut GLuint);
     func!(CreateTransformFeedbacks, (), n: GLsizei, ids: *mut GLuint);
     func!(CreateVertexArrays, (), n: GLsizei, arrays: *mut GLuint);
     func!(CullFace, (), mode: GLenum);
     func!(DebugMessageCallback, (), callback: GLDEBUGPROC, userParam: *const c_void);
     func!(DebugMessageCallbackARB, (), callback: GLDEBUGPROCARB, userParam: *const c_void);
     func!(DebugMessageControl, (), source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean);
     func!(DebugMessageControlARB, (), source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean);
     func!(DebugMessageInsert, (), source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar);
     func!(DebugMessageInsertARB, (), source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar);
     func!(DeleteBuffers, (), n: GLsizei, buffers: *const GLuint);
     func!(DeleteBuffersARB, (), n: GLsizei, buffers: *const GLuint);
     func!(DeleteFramebuffers, (), n: GLsizei, framebuffers: *const GLuint);
     func!(DeleteFramebuffersEXT, (), n: GLsizei, framebuffers: *const GLuint);
     func!(DeleteObjectARB, (), obj: GLhandleARB);
     func!(DeleteProgram, (), program: GLuint);
     func!(DeleteProgramPipelines, (), n: GLsizei, pipelines: *const GLuint);
     func!(DeleteProgramsARB, (), n: GLsizei, programs: *const GLuint);
     func!(DeleteProgramsNV, (), n: GLsizei, programs: *const GLuint);
     func!(DeleteQueries, (), n: GLsizei, ids: *const GLuint);
     func!(DeleteQueriesARB, (), n: GLsizei, ids: *const GLuint);
     func!(DeleteRenderbuffers, (), n: GLsizei, renderbuffers: *const GLuint);
     func!(DeleteRenderbuffersEXT, (), n: GLsizei, renderbuffers: *const GLuint);
     func!(DeleteSamplers, (), count: GLsizei, samplers: *const GLuint);
     func!(DeleteShader, (), shader: GLuint);
     func!(DeleteSync, (), sync: GLsync);
     func!(DeleteTextures, (), n: GLsizei, textures: *const GLuint);
     func!(DeleteTexturesEXT, (), n: GLsizei, textures: *const GLuint);
     func!(DeleteTransformFeedbacks, (), n: GLsizei, ids: *const GLuint);
     func!(DeleteTransformFeedbacksNV, (), n: GLsizei, ids: *const GLuint);
     func!(DeleteVertexArrays, (), n: GLsizei, arrays: *const GLuint);
     func!(DeleteVertexArraysAPPLE, (), n: GLsizei, arrays: *const GLuint);
     func!(DepthFunc, (), func: GLenum);
     func!(DepthMask, (), flag: GLboolean);
     func!(DepthRange, (), n: GLdouble, f: GLdouble);
     func!(DepthRangeArraydvNV, (), first: GLuint, count: GLsizei, v: *const GLdouble);
     func!(DepthRangeArrayv, (), first: GLuint, count: GLsizei, v: *const GLdouble);
     func!(DepthRangeIndexed, (), index: GLuint, n: GLdouble, f: GLdouble);
     func!(DepthRangeIndexeddNV, (), index: GLuint, n: GLdouble, f: GLdouble);
     func!(DepthRangef, (), n: GLfloat, f: GLfloat);
     func!(DepthRangefOES, (), n: GLclampf, f: GLclampf);
     func!(DetachObjectARB, (), containerObj: GLhandleARB, attachedObj: GLhandleARB);
     func!(DetachShader, (), program: GLuint, shader: GLuint);
     func!(Disable, (), cap: GLenum);
     func!(DisableClientStateIndexedEXT, (), array: GLenum, index: GLuint);
     func!(DisableClientStateiEXT, (), array: GLenum, index: GLuint);
     func!(DisableIndexedEXT, (), target: GLenum, index: GLuint);
     func!(DisableVertexArrayAttrib, (), vaobj: GLuint, index: GLuint);
     func!(DisableVertexArrayAttribEXT, (), vaobj: GLuint, index: GLuint);
     func!(DisableVertexArrayEXT, (), vaobj: GLuint, array: GLenum);
     func!(DisableVertexAttribArray, (), index: GLuint);
     func!(DisableVertexAttribArrayARB, (), index: GLuint);
     func!(Disablei, (), target: GLenum, index: GLuint);
     func!(DispatchCompute, (), num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint);
     func!(DispatchComputeIndirect, (), indirect: GLintptr);
     func!(DrawArrays, (), mode: GLenum, first: GLint, count: GLsizei);
     func!(DrawArraysEXT, (), mode: GLenum, first: GLint, count: GLsizei);
     func!(DrawArraysIndirect, (), mode: GLenum, indirect: *const c_void);
     func!(DrawArraysInstanced, (), mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei);
     func!(DrawArraysInstancedARB, (), mode: GLenum, first: GLint, count: GLsizei, primcount: GLsizei);
     func!(DrawArraysInstancedBaseInstance, (), mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint);
     func!(DrawArraysInstancedEXT, (), mode: GLenum, start: GLint, count: GLsizei, primcount: GLsizei);
     func!(DrawBuffer, (), buf: GLenum);
     func!(DrawBuffers, (), n: GLsizei, bufs: *const GLenum);
     func!(DrawBuffersARB, (), n: GLsizei, bufs: *const GLenum);
     func!(DrawBuffersATI, (), n: GLsizei, bufs: *const GLenum);
     func!(DrawElements, (), mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void);
     func!(DrawElementsBaseVertex, (), mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint);
     func!(DrawElementsIndirect, (), mode: GLenum, type_: GLenum, indirect: *const c_void);
     func!(DrawElementsInstanced, (), mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei);
     func!(DrawElementsInstancedARB, (), mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, primcount: GLsizei);
     func!(DrawElementsInstancedBaseInstance, (), mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, baseinstance: GLuint);
     func!(DrawElementsInstancedBaseVertex, (), mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint);
     func!(DrawElementsInstancedBaseVertexBaseInstance, (), mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint);
     func!(DrawElementsInstancedEXT, (), mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, primcount: GLsizei);
     func!(DrawRangeElements, (), mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void);
     func!(DrawRangeElementsBaseVertex, (), mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint);
     func!(DrawRangeElementsEXT, (), mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void);
     func!(DrawTransformFeedback, (), mode: GLenum, id: GLuint);
     func!(DrawTransformFeedbackInstanced, (), mode: GLenum, id: GLuint, instancecount: GLsizei);
     func!(DrawTransformFeedbackNV, (), mode: GLenum, id: GLuint);
     func!(DrawTransformFeedbackStream, (), mode: GLenum, id: GLuint, stream: GLuint);
     func!(DrawTransformFeedbackStreamInstanced, (), mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei);
     func!(EdgeFlagPointerEXT, (), stride: GLsizei, count: GLsizei, pointer: *const GLboolean);
     func!(Enable, (), cap: GLenum);
     func!(EnableClientStateIndexedEXT, (), array: GLenum, index: GLuint);
     func!(EnableClientStateiEXT, (), array: GLenum, index: GLuint);
     func!(EnableIndexedEXT, (), target: GLenum, index: GLuint);
     func!(EnableVertexArrayAttrib, (), vaobj: GLuint, index: GLuint);
     func!(EnableVertexArrayAttribEXT, (), vaobj: GLuint, index: GLuint);
     func!(EnableVertexArrayEXT, (), vaobj: GLuint, array: GLenum);
     func!(EnableVertexAttribArray, (), index: GLuint);
     func!(EnableVertexAttribArrayARB, (), index: GLuint);
     func!(Enablei, (), target: GLenum, index: GLuint);
     func!(EndConditionalRender, (), );
     func!(EndConditionalRenderNV, (), );
     func!(EndConditionalRenderNVX, (), );
     func!(EndQuery, (), target: GLenum);
     func!(EndQueryARB, (), target: GLenum);
     func!(EndQueryIndexed, (), target: GLenum, index: GLuint);
     func!(EndTransformFeedback, (), );
     func!(EndTransformFeedbackEXT, (), );
     func!(EndTransformFeedbackNV, (), );
     func!(ExecuteProgramNV, (), target: GLenum, id: GLuint, params: *const GLfloat);
     func!(FenceSync, GLsync, condition: GLenum, flags: GLbitfield);
     func!(Finish, (), );
     func!(Flush, (), );
     func!(FlushMappedBufferRange, (), target: GLenum, offset: GLintptr, length: GLsizeiptr);
     func!(FlushMappedBufferRangeAPPLE, (), target: GLenum, offset: GLintptr, size: GLsizeiptr);
     func!(FlushMappedNamedBufferRange, (), buffer: GLuint, offset: GLintptr, length: GLsizeiptr);
     func!(FlushMappedNamedBufferRangeEXT, (), buffer: GLuint, offset: GLintptr, length: GLsizeiptr);
     func!(FramebufferDrawBufferEXT, (), framebuffer: GLuint, mode: GLenum);
     func!(FramebufferDrawBuffersEXT, (), framebuffer: GLuint, n: GLsizei, bufs: *const GLenum);
     func!(FramebufferParameteri, (), target: GLenum, pname: GLenum, param: GLint);
     func!(FramebufferReadBufferEXT, (), framebuffer: GLuint, mode: GLenum);
     func!(FramebufferRenderbuffer, (), target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);
     func!(FramebufferRenderbufferEXT, (), target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);
     func!(FramebufferTexture, (), target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);
     func!(FramebufferTexture1D, (), target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
     func!(FramebufferTexture1DEXT, (), target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
     func!(FramebufferTexture2D, (), target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
     func!(FramebufferTexture2DEXT, (), target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
     func!(FramebufferTexture3D, (), target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint);
     func!(FramebufferTexture3DEXT, (), target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint);
     func!(FramebufferTextureARB, (), target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);
     func!(FramebufferTextureEXT, (), target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);
     func!(FramebufferTextureFaceARB, (), target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, face: GLenum);
     func!(FramebufferTextureFaceEXT, (), target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, face: GLenum);
     func!(FramebufferTextureLayer, (), target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);
     func!(FramebufferTextureLayerARB, (), target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);
     func!(FramebufferTextureLayerEXT, (), target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);
     func!(FrontFace, (), mode: GLenum);
     func!(FrustumfOES, (), l: GLfloat, r: GLfloat, b: GLfloat, t: GLfloat, n: GLfloat, f: GLfloat);
     func!(GenBuffers, (), n: GLsizei, buffers: *mut GLuint);
     func!(GenBuffersARB, (), n: GLsizei, buffers: *mut GLuint);
     func!(GenFramebuffers, (), n: GLsizei, framebuffers: *mut GLuint);
     func!(GenFramebuffersEXT, (), n: GLsizei, framebuffers: *mut GLuint);
     func!(GenProgramPipelines, (), n: GLsizei, pipelines: *mut GLuint);
     func!(GenProgramsARB, (), n: GLsizei, programs: *mut GLuint);
     func!(GenProgramsNV, (), n: GLsizei, programs: *mut GLuint);
     func!(GenQueries, (), n: GLsizei, ids: *mut GLuint);
     func!(GenQueriesARB, (), n: GLsizei, ids: *mut GLuint);
     func!(GenRenderbuffers, (), n: GLsizei, renderbuffers: *mut GLuint);
     func!(GenRenderbuffersEXT, (), n: GLsizei, renderbuffers: *mut GLuint);
     func!(GenSamplers, (), count: GLsizei, samplers: *mut GLuint);
     func!(GenTextures, (), n: GLsizei, textures: *mut GLuint);
     func!(GenTexturesEXT, (), n: GLsizei, textures: *mut GLuint);
     func!(GenTransformFeedbacks, (), n: GLsizei, ids: *mut GLuint);
     func!(GenTransformFeedbacksNV, (), n: GLsizei, ids: *mut GLuint);
     func!(GenVertexArrays, (), n: GLsizei, arrays: *mut GLuint);
     func!(GenVertexArraysAPPLE, (), n: GLsizei, arrays: *mut GLuint);
     func!(GenerateMipmap, (), target: GLenum);
     func!(GenerateMipmapEXT, (), target: GLenum);
     func!(GenerateMultiTexMipmapEXT, (), texunit: GLenum, target: GLenum);
     func!(GenerateTextureMipmap, (), texture: GLuint);
     func!(GenerateTextureMipmapEXT, (), texture: GLuint, target: GLenum);
     func!(GetActiveAtomicCounterBufferiv, (), program: GLuint, bufferIndex: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetActiveAttrib, (), program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar);
     func!(GetActiveAttribARB, (), programObj: GLhandleARB, index: GLuint, maxLength: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLcharARB);
     func!(GetActiveSubroutineName, (), program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);
     func!(GetActiveSubroutineUniformName, (), program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);
     func!(GetActiveSubroutineUniformiv, (), program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *mut GLint);
     func!(GetActiveUniform, (), program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar);
     func!(GetActiveUniformARB, (), programObj: GLhandleARB, index: GLuint, maxLength: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLcharARB);
     func!(GetActiveUniformBlockName, (), program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar);
     func!(GetActiveUniformBlockiv, (), program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetActiveUniformName, (), program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar);
     func!(GetActiveUniformsiv, (), program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint);
     func!(GetActiveVaryingNV, (), program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar);
     func!(GetAttachedObjectsARB, (), containerObj: GLhandleARB, maxCount: GLsizei, count: *mut GLsizei, obj: *mut GLhandleARB);
     func!(GetAttachedShaders, (), program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint);
     func!(GetAttribLocation, GLint, program: GLuint, name: *const GLchar);
     func!(GetAttribLocationARB, GLint, programObj: GLhandleARB, name: *const GLcharARB);
     func!(GetBooleanIndexedvEXT, (), target: GLenum, index: GLuint, data: *mut GLboolean);
     func!(GetBooleani_v, (), target: GLenum, index: GLuint, data: *mut GLboolean);
     func!(GetBooleanv, (), pname: GLenum, data: *mut GLboolean);
     func!(GetBufferParameteri64v, (), target: GLenum, pname: GLenum, params: *mut GLint64);
     func!(GetBufferParameteriv, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetBufferParameterivARB, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetBufferPointerv, (), target: GLenum, pname: GLenum, params: *mut *mut c_void);
     func!(GetBufferPointervARB, (), target: GLenum, pname: GLenum, params: *mut *mut c_void);
     func!(GetBufferSubData, (), target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void);
     func!(GetBufferSubDataARB, (), target: GLenum, offset: GLintptrARB, size: GLsizeiptrARB, data: *mut c_void);
     func!(GetClipPlanefOES, (), plane: GLenum, equation: *mut GLfloat);
     func!(GetCompressedMultiTexImageEXT, (), texunit: GLenum, target: GLenum, lod: GLint, img: *mut c_void);
     func!(GetCompressedTexImage, (), target: GLenum, level: GLint, img: *mut c_void);
     func!(GetCompressedTexImageARB, (), target: GLenum, level: GLint, img: *mut c_void);
     func!(GetCompressedTextureImage, (), texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut c_void);
     func!(GetCompressedTextureImageEXT, (), texture: GLuint, target: GLenum, lod: GLint, img: *mut c_void);
     func!(GetCompressedTextureSubImage, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut c_void);
     func!(GetDebugMessageLog, GLuint, count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar);
     func!(GetDebugMessageLogARB, GLuint, count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar);
     func!(GetDoubleIndexedvEXT, (), target: GLenum, index: GLuint, data: *mut GLdouble);
     func!(GetDoublei_v, (), target: GLenum, index: GLuint, data: *mut GLdouble);
     func!(GetDoublei_vEXT, (), pname: GLenum, index: GLuint, params: *mut GLdouble);
     func!(GetDoublev, (), pname: GLenum, data: *mut GLdouble);
     func!(GetError, GLenum, );
     func!(GetFloatIndexedvEXT, (), target: GLenum, index: GLuint, data: *mut GLfloat);
     func!(GetFloati_v, (), target: GLenum, index: GLuint, data: *mut GLfloat);
     func!(GetFloati_vEXT, (), pname: GLenum, index: GLuint, params: *mut GLfloat);
     func!(GetFloatv, (), pname: GLenum, data: *mut GLfloat);
     func!(GetFragDataIndex, GLint, program: GLuint, name: *const GLchar);
     func!(GetFragDataLocation, GLint, program: GLuint, name: *const GLchar);
     func!(GetFragDataLocationEXT, GLint, program: GLuint, name: *const GLchar);
     func!(GetFramebufferAttachmentParameteriv, (), target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetFramebufferAttachmentParameterivEXT, (), target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetFramebufferParameteriv, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetFramebufferParameterivEXT, (), framebuffer: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetGraphicsResetStatus, GLenum, );
     func!(GetGraphicsResetStatusARB, GLenum, );
     func!(GetHandleARB, GLhandleARB, pname: GLenum);
     func!(GetInfoLogARB, (), obj: GLhandleARB, maxLength: GLsizei, length: *mut GLsizei, infoLog: *mut GLcharARB);
     func!(GetInteger64i_v, (), target: GLenum, index: GLuint, data: *mut GLint64);
     func!(GetInteger64v, (), pname: GLenum, data: *mut GLint64);
     func!(GetIntegerIndexedvEXT, (), target: GLenum, index: GLuint, data: *mut GLint);
     func!(GetIntegeri_v, (), target: GLenum, index: GLuint, data: *mut GLint);
     func!(GetIntegerv, (), pname: GLenum, data: *mut GLint);
     func!(GetInternalformati64v, (), target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint64);
     func!(GetInternalformativ, (), target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint);
     func!(GetMultiTexEnvfvEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetMultiTexEnvivEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetMultiTexGendvEXT, (), texunit: GLenum, coord: GLenum, pname: GLenum, params: *mut GLdouble);
     func!(GetMultiTexGenfvEXT, (), texunit: GLenum, coord: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetMultiTexGenivEXT, (), texunit: GLenum, coord: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetMultiTexImageEXT, (), texunit: GLenum, target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void);
     func!(GetMultiTexLevelParameterfvEXT, (), texunit: GLenum, target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat);
     func!(GetMultiTexLevelParameterivEXT, (), texunit: GLenum, target: GLenum, level: GLint, pname: GLenum, params: *mut GLint);
     func!(GetMultiTexParameterIivEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetMultiTexParameterIuivEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *mut GLuint);
     func!(GetMultiTexParameterfvEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetMultiTexParameterivEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetMultisamplefv, (), pname: GLenum, index: GLuint, val: *mut GLfloat);
     func!(GetMultisamplefvNV, (), pname: GLenum, index: GLuint, val: *mut GLfloat);
     func!(GetNamedBufferParameteri64v, (), buffer: GLuint, pname: GLenum, params: *mut GLint64);
     func!(GetNamedBufferParameteriv, (), buffer: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetNamedBufferParameterivEXT, (), buffer: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetNamedBufferPointerv, (), buffer: GLuint, pname: GLenum, params: *mut *mut c_void);
     func!(GetNamedBufferPointervEXT, (), buffer: GLuint, pname: GLenum, params: *mut *mut c_void);
     func!(GetNamedBufferSubData, (), buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut c_void);
     func!(GetNamedBufferSubDataEXT, (), buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut c_void);
     func!(GetNamedFramebufferAttachmentParameteriv, (), framebuffer: GLuint, attachment: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetNamedFramebufferAttachmentParameterivEXT, (), framebuffer: GLuint, attachment: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetNamedFramebufferParameteriv, (), framebuffer: GLuint, pname: GLenum, param: *mut GLint);
     func!(GetNamedFramebufferParameterivEXT, (), framebuffer: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetNamedProgramLocalParameterIivEXT, (), program: GLuint, target: GLenum, index: GLuint, params: *mut GLint);
     func!(GetNamedProgramLocalParameterIuivEXT, (), program: GLuint, target: GLenum, index: GLuint, params: *mut GLuint);
     func!(GetNamedProgramLocalParameterdvEXT, (), program: GLuint, target: GLenum, index: GLuint, params: *mut GLdouble);
     func!(GetNamedProgramLocalParameterfvEXT, (), program: GLuint, target: GLenum, index: GLuint, params: *mut GLfloat);
     func!(GetNamedProgramStringEXT, (), program: GLuint, target: GLenum, pname: GLenum, string: *mut c_void);
     func!(GetNamedProgramivEXT, (), program: GLuint, target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetNamedRenderbufferParameteriv, (), renderbuffer: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetNamedRenderbufferParameterivEXT, (), renderbuffer: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetObjectLabel, (), identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);
     func!(GetObjectParameterfvARB, (), obj: GLhandleARB, pname: GLenum, params: *mut GLfloat);
     func!(GetObjectParameterivARB, (), obj: GLhandleARB, pname: GLenum, params: *mut GLint);
     func!(GetObjectPtrLabel, (), ptr: *const c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);
     func!(GetPointerIndexedvEXT, (), target: GLenum, index: GLuint, data: *mut *mut c_void);
     func!(GetPointeri_vEXT, (), pname: GLenum, index: GLuint, params: *mut *mut c_void);
     func!(GetPointerv, (), pname: GLenum, params: *mut *mut c_void);
     func!(GetPointervEXT, (), pname: GLenum, params: *mut *mut c_void);
     func!(GetProgramBinary, (), program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut c_void);
     func!(GetProgramEnvParameterdvARB, (), target: GLenum, index: GLuint, params: *mut GLdouble);
     func!(GetProgramEnvParameterfvARB, (), target: GLenum, index: GLuint, params: *mut GLfloat);
     func!(GetProgramInfoLog, (), program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);
     func!(GetProgramInterfaceiv, (), program: GLuint, programInterface: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetProgramLocalParameterdvARB, (), target: GLenum, index: GLuint, params: *mut GLdouble);
     func!(GetProgramLocalParameterfvARB, (), target: GLenum, index: GLuint, params: *mut GLfloat);
     func!(GetProgramParameterdvNV, (), target: GLenum, index: GLuint, pname: GLenum, params: *mut GLdouble);
     func!(GetProgramParameterfvNV, (), target: GLenum, index: GLuint, pname: GLenum, params: *mut GLfloat);
     func!(GetProgramPipelineInfoLog, (), pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);
     func!(GetProgramPipelineiv, (), pipeline: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetProgramResourceIndex, GLuint, program: GLuint, programInterface: GLenum, name: *const GLchar);
     func!(GetProgramResourceLocation, GLint, program: GLuint, programInterface: GLenum, name: *const GLchar);
     func!(GetProgramResourceLocationIndex, GLint, program: GLuint, programInterface: GLenum, name: *const GLchar);
     func!(GetProgramResourceName, (), program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);
     func!(GetProgramResourceiv, (), program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *const GLenum, count: GLsizei, length: *mut GLsizei, params: *mut GLint);
     func!(GetProgramStageiv, (), program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint);
     func!(GetProgramStringARB, (), target: GLenum, pname: GLenum, string: *mut c_void);
     func!(GetProgramStringNV, (), id: GLuint, pname: GLenum, program: *mut GLubyte);
     func!(GetProgramiv, (), program: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetProgramivARB, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetProgramivNV, (), id: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetQueryBufferObjecti64v, (), id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
     func!(GetQueryBufferObjectiv, (), id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
     func!(GetQueryBufferObjectui64v, (), id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
     func!(GetQueryBufferObjectuiv, (), id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
     func!(GetQueryIndexediv, (), target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetQueryObjecti64v, (), id: GLuint, pname: GLenum, params: *mut GLint64);
     func!(GetQueryObjecti64vEXT, (), id: GLuint, pname: GLenum, params: *mut GLint64);
     func!(GetQueryObjectiv, (), id: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetQueryObjectivARB, (), id: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetQueryObjectui64v, (), id: GLuint, pname: GLenum, params: *mut GLuint64);
     func!(GetQueryObjectui64vEXT, (), id: GLuint, pname: GLenum, params: *mut GLuint64);
     func!(GetQueryObjectuiv, (), id: GLuint, pname: GLenum, params: *mut GLuint);
     func!(GetQueryObjectuivARB, (), id: GLuint, pname: GLenum, params: *mut GLuint);
     func!(GetQueryiv, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetQueryivARB, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetRenderbufferParameteriv, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetRenderbufferParameterivEXT, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetSamplerParameterIiv, (), sampler: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetSamplerParameterIuiv, (), sampler: GLuint, pname: GLenum, params: *mut GLuint);
     func!(GetSamplerParameterfv, (), sampler: GLuint, pname: GLenum, params: *mut GLfloat);
     func!(GetSamplerParameteriv, (), sampler: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetShaderInfoLog, (), shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);
     func!(GetShaderPrecisionFormat, (), shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint);
     func!(GetShaderSource, (), shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar);
     func!(GetShaderSourceARB, (), obj: GLhandleARB, maxLength: GLsizei, length: *mut GLsizei, source: *mut GLcharARB);
     func!(GetShaderiv, (), shader: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetString, *const GLubyte, name: GLenum);
     func!(GetStringi, *const GLubyte, name: GLenum, index: GLuint);
     func!(GetSubroutineIndex, GLuint, program: GLuint, shadertype: GLenum, name: *const GLchar);
     func!(GetSubroutineUniformLocation, GLint, program: GLuint, shadertype: GLenum, name: *const GLchar);
     func!(GetSynciv, (), sync: GLsync, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint);
     func!(GetTexImage, (), target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void);
     func!(GetTexLevelParameterfv, (), target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat);
     func!(GetTexLevelParameteriv, (), target: GLenum, level: GLint, pname: GLenum, params: *mut GLint);
     func!(GetTexParameterIiv, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetTexParameterIivEXT, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetTexParameterIuiv, (), target: GLenum, pname: GLenum, params: *mut GLuint);
     func!(GetTexParameterIuivEXT, (), target: GLenum, pname: GLenum, params: *mut GLuint);
     func!(GetTexParameterfv, (), target: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetTexParameteriv, (), target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetTextureImage, (), texture: GLuint, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void);
     func!(GetTextureImageEXT, (), texture: GLuint, target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void);
     func!(GetTextureLevelParameterfv, (), texture: GLuint, level: GLint, pname: GLenum, params: *mut GLfloat);
     func!(GetTextureLevelParameterfvEXT, (), texture: GLuint, target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat);
     func!(GetTextureLevelParameteriv, (), texture: GLuint, level: GLint, pname: GLenum, params: *mut GLint);
     func!(GetTextureLevelParameterivEXT, (), texture: GLuint, target: GLenum, level: GLint, pname: GLenum, params: *mut GLint);
     func!(GetTextureParameterIiv, (), texture: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetTextureParameterIivEXT, (), texture: GLuint, target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetTextureParameterIuiv, (), texture: GLuint, pname: GLenum, params: *mut GLuint);
     func!(GetTextureParameterIuivEXT, (), texture: GLuint, target: GLenum, pname: GLenum, params: *mut GLuint);
     func!(GetTextureParameterfv, (), texture: GLuint, pname: GLenum, params: *mut GLfloat);
     func!(GetTextureParameterfvEXT, (), texture: GLuint, target: GLenum, pname: GLenum, params: *mut GLfloat);
     func!(GetTextureParameteriv, (), texture: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetTextureParameterivEXT, (), texture: GLuint, target: GLenum, pname: GLenum, params: *mut GLint);
     func!(GetTextureSubImage, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void);
     func!(GetTrackMatrixivNV, (), target: GLenum, address: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetTransformFeedbackVarying, (), program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar);
     func!(GetTransformFeedbackVaryingEXT, (), program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar);
     func!(GetTransformFeedbackVaryingNV, (), program: GLuint, index: GLuint, location: *mut GLint);
     func!(GetTransformFeedbacki64_v, (), xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint64);
     func!(GetTransformFeedbacki_v, (), xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint);
     func!(GetTransformFeedbackiv, (), xfb: GLuint, pname: GLenum, param: *mut GLint);
     func!(GetUniformBlockIndex, GLuint, program: GLuint, uniformBlockName: *const GLchar);
     func!(GetUniformIndices, (), program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint);
     func!(GetUniformLocation, GLint, program: GLuint, name: *const GLchar);
     func!(GetUniformLocationARB, GLint, programObj: GLhandleARB, name: *const GLcharARB);
     func!(GetUniformSubroutineuiv, (), shadertype: GLenum, location: GLint, params: *mut GLuint);
     func!(GetUniformdv, (), program: GLuint, location: GLint, params: *mut GLdouble);
     func!(GetUniformfv, (), program: GLuint, location: GLint, params: *mut GLfloat);
     func!(GetUniformfvARB, (), programObj: GLhandleARB, location: GLint, params: *mut GLfloat);
     func!(GetUniformiv, (), program: GLuint, location: GLint, params: *mut GLint);
     func!(GetUniformivARB, (), programObj: GLhandleARB, location: GLint, params: *mut GLint);
     func!(GetUniformuiv, (), program: GLuint, location: GLint, params: *mut GLuint);
     func!(GetUniformuivEXT, (), program: GLuint, location: GLint, params: *mut GLuint);
     func!(GetVaryingLocationNV, GLint, program: GLuint, name: *const GLchar);
     func!(GetVertexArrayIndexed64iv, (), vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint64);
     func!(GetVertexArrayIndexediv, (), vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint);
     func!(GetVertexArrayIntegeri_vEXT, (), vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint);
     func!(GetVertexArrayIntegervEXT, (), vaobj: GLuint, pname: GLenum, param: *mut GLint);
     func!(GetVertexArrayPointeri_vEXT, (), vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut *mut c_void);
     func!(GetVertexArrayPointervEXT, (), vaobj: GLuint, pname: GLenum, param: *mut *mut c_void);
     func!(GetVertexArrayiv, (), vaobj: GLuint, pname: GLenum, param: *mut GLint);
     func!(GetVertexAttribIiv, (), index: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetVertexAttribIivEXT, (), index: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetVertexAttribIuiv, (), index: GLuint, pname: GLenum, params: *mut GLuint);
     func!(GetVertexAttribIuivEXT, (), index: GLuint, pname: GLenum, params: *mut GLuint);
     func!(GetVertexAttribLdv, (), index: GLuint, pname: GLenum, params: *mut GLdouble);
     func!(GetVertexAttribLdvEXT, (), index: GLuint, pname: GLenum, params: *mut GLdouble);
     func!(GetVertexAttribPointerv, (), index: GLuint, pname: GLenum, pointer: *mut *mut c_void);
     func!(GetVertexAttribPointervARB, (), index: GLuint, pname: GLenum, pointer: *mut *mut c_void);
     func!(GetVertexAttribPointervNV, (), index: GLuint, pname: GLenum, pointer: *mut *mut c_void);
     func!(GetVertexAttribdv, (), index: GLuint, pname: GLenum, params: *mut GLdouble);
     func!(GetVertexAttribdvARB, (), index: GLuint, pname: GLenum, params: *mut GLdouble);
     func!(GetVertexAttribdvNV, (), index: GLuint, pname: GLenum, params: *mut GLdouble);
     func!(GetVertexAttribfv, (), index: GLuint, pname: GLenum, params: *mut GLfloat);
     func!(GetVertexAttribfvARB, (), index: GLuint, pname: GLenum, params: *mut GLfloat);
     func!(GetVertexAttribfvNV, (), index: GLuint, pname: GLenum, params: *mut GLfloat);
     func!(GetVertexAttribiv, (), index: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetVertexAttribivARB, (), index: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetVertexAttribivNV, (), index: GLuint, pname: GLenum, params: *mut GLint);
     func!(GetnCompressedTexImage, (), target: GLenum, lod: GLint, bufSize: GLsizei, pixels: *mut c_void);
     func!(GetnCompressedTexImageARB, (), target: GLenum, lod: GLint, bufSize: GLsizei, img: *mut c_void);
     func!(GetnTexImage, (), target: GLenum, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void);
     func!(GetnTexImageARB, (), target: GLenum, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, img: *mut c_void);
     func!(GetnUniformdv, (), program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble);
     func!(GetnUniformdvARB, (), program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble);
     func!(GetnUniformfv, (), program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);
     func!(GetnUniformfvARB, (), program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);
     func!(GetnUniformiv, (), program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);
     func!(GetnUniformivARB, (), program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);
     func!(GetnUniformuiv, (), program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint);
     func!(GetnUniformuivARB, (), program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint);
     func!(Hint, (), target: GLenum, mode: GLenum);
     func!(IndexPointerEXT, (), type_: GLenum, stride: GLsizei, count: GLsizei, pointer: *const c_void);
     func!(InvalidateBufferData, (), buffer: GLuint);
     func!(InvalidateBufferSubData, (), buffer: GLuint, offset: GLintptr, length: GLsizeiptr);
     func!(InvalidateFramebuffer, (), target: GLenum, numAttachments: GLsizei, attachments: *const GLenum);
     func!(InvalidateNamedFramebufferData, (), framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum);
     func!(InvalidateNamedFramebufferSubData, (), framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(InvalidateSubFramebuffer, (), target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(InvalidateTexImage, (), texture: GLuint, level: GLint);
     func!(InvalidateTexSubImage, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);
     func!(IsBuffer, GLboolean, buffer: GLuint);
     func!(IsBufferARB, GLboolean, buffer: GLuint);
     func!(IsEnabled, GLboolean, cap: GLenum);
     func!(IsEnabledIndexedEXT, GLboolean, target: GLenum, index: GLuint);
     func!(IsEnabledi, GLboolean, target: GLenum, index: GLuint);
     func!(IsFramebuffer, GLboolean, framebuffer: GLuint);
     func!(IsFramebufferEXT, GLboolean, framebuffer: GLuint);
     func!(IsProgram, GLboolean, program: GLuint);
     func!(IsProgramARB, GLboolean, program: GLuint);
     func!(IsProgramNV, GLboolean, id: GLuint);
     func!(IsProgramPipeline, GLboolean, pipeline: GLuint);
     func!(IsQuery, GLboolean, id: GLuint);
     func!(IsQueryARB, GLboolean, id: GLuint);
     func!(IsRenderbuffer, GLboolean, renderbuffer: GLuint);
     func!(IsRenderbufferEXT, GLboolean, renderbuffer: GLuint);
     func!(IsSampler, GLboolean, sampler: GLuint);
     func!(IsShader, GLboolean, shader: GLuint);
     func!(IsSync, GLboolean, sync: GLsync);
     func!(IsTexture, GLboolean, texture: GLuint);
     func!(IsTextureEXT, GLboolean, texture: GLuint);
     func!(IsTransformFeedback, GLboolean, id: GLuint);
     func!(IsTransformFeedbackNV, GLboolean, id: GLuint);
     func!(IsVertexArray, GLboolean, array: GLuint);
     func!(IsVertexArrayAPPLE, GLboolean, array: GLuint);
     func!(LineWidth, (), width: GLfloat);
     func!(LinkProgram, (), program: GLuint);
     func!(LinkProgramARB, (), programObj: GLhandleARB);
     func!(LoadProgramNV, (), target: GLenum, id: GLuint, len: GLsizei, program: *const GLubyte);
     func!(LogicOp, (), opcode: GLenum);
     func!(MapBuffer, *mut c_void, target: GLenum, access: GLenum);
     func!(MapBufferARB, *mut c_void, target: GLenum, access: GLenum);
     func!(MapBufferRange, *mut c_void, target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield);
     func!(MapNamedBuffer, *mut c_void, buffer: GLuint, access: GLenum);
     func!(MapNamedBufferEXT, *mut c_void, buffer: GLuint, access: GLenum);
     func!(MapNamedBufferRange, *mut c_void, buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield);
     func!(MapNamedBufferRangeEXT, *mut c_void, buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield);
     func!(MatrixFrustumEXT, (), mode: GLenum, left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);
     func!(MatrixLoadIdentityEXT, (), mode: GLenum);
     func!(MatrixLoadTransposedEXT, (), mode: GLenum, m: *const GLdouble);
     func!(MatrixLoadTransposefEXT, (), mode: GLenum, m: *const GLfloat);
     func!(MatrixLoaddEXT, (), mode: GLenum, m: *const GLdouble);
     func!(MatrixLoadfEXT, (), mode: GLenum, m: *const GLfloat);
     func!(MatrixMultTransposedEXT, (), mode: GLenum, m: *const GLdouble);
     func!(MatrixMultTransposefEXT, (), mode: GLenum, m: *const GLfloat);
     func!(MatrixMultdEXT, (), mode: GLenum, m: *const GLdouble);
     func!(MatrixMultfEXT, (), mode: GLenum, m: *const GLfloat);
     func!(MatrixOrthoEXT, (), mode: GLenum, left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);
     func!(MatrixPopEXT, (), mode: GLenum);
     func!(MatrixPushEXT, (), mode: GLenum);
     func!(MatrixRotatedEXT, (), mode: GLenum, angle: GLdouble, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(MatrixRotatefEXT, (), mode: GLenum, angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(MatrixScaledEXT, (), mode: GLenum, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(MatrixScalefEXT, (), mode: GLenum, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(MatrixTranslatedEXT, (), mode: GLenum, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(MatrixTranslatefEXT, (), mode: GLenum, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(MemoryBarrier, (), barriers: GLbitfield);
     func!(MemoryBarrierByRegion, (), barriers: GLbitfield);
     func!(MemoryBarrierEXT, (), barriers: GLbitfield);
     func!(MinSampleShading, (), value: GLfloat);
     func!(MinSampleShadingARB, (), value: GLfloat);
     func!(MultiDrawArrays, (), mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei);
     func!(MultiDrawArraysEXT, (), mode: GLenum, first: *const GLint, count: *const GLsizei, primcount: GLsizei);
     func!(MultiDrawArraysIndirect, (), mode: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei);
     func!(MultiDrawArraysIndirectAMD, (), mode: GLenum, indirect: *const c_void, primcount: GLsizei, stride: GLsizei);
     func!(MultiDrawArraysIndirectCount, (), mode: GLenum, indirect: *const c_void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);
     func!(MultiDrawArraysIndirectCountARB, (), mode: GLenum, indirect: *const c_void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);
     func!(MultiDrawElements, (), mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei);
     func!(MultiDrawElementsBaseVertex, (), mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei, basevertex: *const GLint);
     func!(MultiDrawElementsEXT, (), mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, primcount: GLsizei);
     func!(MultiDrawElementsIndirect, (), mode: GLenum, type_: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei);
     func!(MultiDrawElementsIndirectAMD, (), mode: GLenum, type_: GLenum, indirect: *const c_void, primcount: GLsizei, stride: GLsizei);
     func!(MultiDrawElementsIndirectCount, (), mode: GLenum, type_: GLenum, indirect: *const c_void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);
     func!(MultiDrawElementsIndirectCountARB, (), mode: GLenum, type_: GLenum, indirect: *const c_void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);
     func!(MultiTexBufferEXT, (), texunit: GLenum, target: GLenum, internalformat: GLenum, buffer: GLuint);
     func!(MultiTexCoord1dARB, (), target: GLenum, s: GLdouble);
     func!(MultiTexCoord1dvARB, (), target: GLenum, v: *const GLdouble);
     func!(MultiTexCoord1fARB, (), target: GLenum, s: GLfloat);
     func!(MultiTexCoord1fvARB, (), target: GLenum, v: *const GLfloat);
     func!(MultiTexCoord1iARB, (), target: GLenum, s: GLint);
     func!(MultiTexCoord1ivARB, (), target: GLenum, v: *const GLint);
     func!(MultiTexCoord1sARB, (), target: GLenum, s: GLshort);
     func!(MultiTexCoord1svARB, (), target: GLenum, v: *const GLshort);
     func!(MultiTexCoord2dARB, (), target: GLenum, s: GLdouble, t: GLdouble);
     func!(MultiTexCoord2dvARB, (), target: GLenum, v: *const GLdouble);
     func!(MultiTexCoord2fARB, (), target: GLenum, s: GLfloat, t: GLfloat);
     func!(MultiTexCoord2fvARB, (), target: GLenum, v: *const GLfloat);
     func!(MultiTexCoord2iARB, (), target: GLenum, s: GLint, t: GLint);
     func!(MultiTexCoord2ivARB, (), target: GLenum, v: *const GLint);
     func!(MultiTexCoord2sARB, (), target: GLenum, s: GLshort, t: GLshort);
     func!(MultiTexCoord2svARB, (), target: GLenum, v: *const GLshort);
     func!(MultiTexCoord3dARB, (), target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble);
     func!(MultiTexCoord3dvARB, (), target: GLenum, v: *const GLdouble);
     func!(MultiTexCoord3fARB, (), target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat);
     func!(MultiTexCoord3fvARB, (), target: GLenum, v: *const GLfloat);
     func!(MultiTexCoord3iARB, (), target: GLenum, s: GLint, t: GLint, r: GLint);
     func!(MultiTexCoord3ivARB, (), target: GLenum, v: *const GLint);
     func!(MultiTexCoord3sARB, (), target: GLenum, s: GLshort, t: GLshort, r: GLshort);
     func!(MultiTexCoord3svARB, (), target: GLenum, v: *const GLshort);
     func!(MultiTexCoord4dARB, (), target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble);
     func!(MultiTexCoord4dvARB, (), target: GLenum, v: *const GLdouble);
     func!(MultiTexCoord4fARB, (), target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat);
     func!(MultiTexCoord4fvARB, (), target: GLenum, v: *const GLfloat);
     func!(MultiTexCoord4iARB, (), target: GLenum, s: GLint, t: GLint, r: GLint, q: GLint);
     func!(MultiTexCoord4ivARB, (), target: GLenum, v: *const GLint);
     func!(MultiTexCoord4sARB, (), target: GLenum, s: GLshort, t: GLshort, r: GLshort, q: GLshort);
     func!(MultiTexCoord4svARB, (), target: GLenum, v: *const GLshort);
     func!(MultiTexCoordPointerEXT, (), texunit: GLenum, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(MultiTexEnvfEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, param: GLfloat);
     func!(MultiTexEnvfvEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *const GLfloat);
     func!(MultiTexEnviEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, param: GLint);
     func!(MultiTexEnvivEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *const GLint);
     func!(MultiTexGendEXT, (), texunit: GLenum, coord: GLenum, pname: GLenum, param: GLdouble);
     func!(MultiTexGendvEXT, (), texunit: GLenum, coord: GLenum, pname: GLenum, params: *const GLdouble);
     func!(MultiTexGenfEXT, (), texunit: GLenum, coord: GLenum, pname: GLenum, param: GLfloat);
     func!(MultiTexGenfvEXT, (), texunit: GLenum, coord: GLenum, pname: GLenum, params: *const GLfloat);
     func!(MultiTexGeniEXT, (), texunit: GLenum, coord: GLenum, pname: GLenum, param: GLint);
     func!(MultiTexGenivEXT, (), texunit: GLenum, coord: GLenum, pname: GLenum, params: *const GLint);
     func!(MultiTexImage1DEXT, (), texunit: GLenum, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(MultiTexImage2DEXT, (), texunit: GLenum, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(MultiTexImage3DEXT, (), texunit: GLenum, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(MultiTexParameterIivEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *const GLint);
     func!(MultiTexParameterIuivEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *const GLuint);
     func!(MultiTexParameterfEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, param: GLfloat);
     func!(MultiTexParameterfvEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *const GLfloat);
     func!(MultiTexParameteriEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, param: GLint);
     func!(MultiTexParameterivEXT, (), texunit: GLenum, target: GLenum, pname: GLenum, params: *const GLint);
     func!(MultiTexRenderbufferEXT, (), texunit: GLenum, target: GLenum, renderbuffer: GLuint);
     func!(MultiTexSubImage1DEXT, (), texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(MultiTexSubImage2DEXT, (), texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(MultiTexSubImage3DEXT, (), texunit: GLenum, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(NamedBufferData, (), buffer: GLuint, size: GLsizeiptr, data: *const c_void, usage: GLenum);
     func!(NamedBufferDataEXT, (), buffer: GLuint, size: GLsizeiptr, data: *const c_void, usage: GLenum);
     func!(NamedBufferStorage, (), buffer: GLuint, size: GLsizeiptr, data: *const c_void, flags: GLbitfield);
     func!(NamedBufferStorageEXT, (), buffer: GLuint, size: GLsizeiptr, data: *const c_void, flags: GLbitfield);
     func!(NamedBufferSubData, (), buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const c_void);
     func!(NamedBufferSubDataEXT, (), buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const c_void);
     func!(NamedCopyBufferSubDataEXT, (), readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);
     func!(NamedFramebufferDrawBuffer, (), framebuffer: GLuint, buf: GLenum);
     func!(NamedFramebufferDrawBuffers, (), framebuffer: GLuint, n: GLsizei, bufs: *const GLenum);
     func!(NamedFramebufferParameteri, (), framebuffer: GLuint, pname: GLenum, param: GLint);
     func!(NamedFramebufferParameteriEXT, (), framebuffer: GLuint, pname: GLenum, param: GLint);
     func!(NamedFramebufferReadBuffer, (), framebuffer: GLuint, src: GLenum);
     func!(NamedFramebufferRenderbuffer, (), framebuffer: GLuint, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);
     func!(NamedFramebufferRenderbufferEXT, (), framebuffer: GLuint, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);
     func!(NamedFramebufferTexture, (), framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint);
     func!(NamedFramebufferTexture1DEXT, (), framebuffer: GLuint, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
     func!(NamedFramebufferTexture2DEXT, (), framebuffer: GLuint, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
     func!(NamedFramebufferTexture3DEXT, (), framebuffer: GLuint, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint);
     func!(NamedFramebufferTextureEXT, (), framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint);
     func!(NamedFramebufferTextureFaceEXT, (), framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, face: GLenum);
     func!(NamedFramebufferTextureLayer, (), framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);
     func!(NamedFramebufferTextureLayerEXT, (), framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);
     func!(NamedProgramLocalParameter4dEXT, (), program: GLuint, target: GLenum, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(NamedProgramLocalParameter4dvEXT, (), program: GLuint, target: GLenum, index: GLuint, params: *const GLdouble);
     func!(NamedProgramLocalParameter4fEXT, (), program: GLuint, target: GLenum, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(NamedProgramLocalParameter4fvEXT, (), program: GLuint, target: GLenum, index: GLuint, params: *const GLfloat);
     func!(NamedProgramLocalParameterI4iEXT, (), program: GLuint, target: GLenum, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
     func!(NamedProgramLocalParameterI4ivEXT, (), program: GLuint, target: GLenum, index: GLuint, params: *const GLint);
     func!(NamedProgramLocalParameterI4uiEXT, (), program: GLuint, target: GLenum, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
     func!(NamedProgramLocalParameterI4uivEXT, (), program: GLuint, target: GLenum, index: GLuint, params: *const GLuint);
     func!(NamedProgramLocalParameters4fvEXT, (), program: GLuint, target: GLenum, index: GLuint, count: GLsizei, params: *const GLfloat);
     func!(NamedProgramLocalParametersI4ivEXT, (), program: GLuint, target: GLenum, index: GLuint, count: GLsizei, params: *const GLint);
     func!(NamedProgramLocalParametersI4uivEXT, (), program: GLuint, target: GLenum, index: GLuint, count: GLsizei, params: *const GLuint);
     func!(NamedProgramStringEXT, (), program: GLuint, target: GLenum, format: GLenum, len: GLsizei, string: *const c_void);
     func!(NamedRenderbufferStorage, (), renderbuffer: GLuint, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(NamedRenderbufferStorageEXT, (), renderbuffer: GLuint, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(NamedRenderbufferStorageMultisample, (), renderbuffer: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(NamedRenderbufferStorageMultisampleCoverageEXT, (), renderbuffer: GLuint, coverageSamples: GLsizei, colorSamples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(NamedRenderbufferStorageMultisampleEXT, (), renderbuffer: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(NormalPointerEXT, (), type_: GLenum, stride: GLsizei, count: GLsizei, pointer: *const c_void);
     func!(ObjectLabel, (), identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar);
     func!(ObjectPtrLabel, (), ptr: *const c_void, length: GLsizei, label: *const GLchar);
     func!(OrthofOES, (), l: GLfloat, r: GLfloat, b: GLfloat, t: GLfloat, n: GLfloat, f: GLfloat);
     func!(PatchParameterfv, (), pname: GLenum, values: *const GLfloat);
     func!(PatchParameteri, (), pname: GLenum, value: GLint);
     func!(PauseTransformFeedback, (), );
     func!(PauseTransformFeedbackNV, (), );
     func!(PixelStoref, (), pname: GLenum, param: GLfloat);
     func!(PixelStorei, (), pname: GLenum, param: GLint);
     func!(PointParameterf, (), pname: GLenum, param: GLfloat);
     func!(PointParameterfARB, (), pname: GLenum, param: GLfloat);
     func!(PointParameterfEXT, (), pname: GLenum, param: GLfloat);
     func!(PointParameterfSGIS, (), pname: GLenum, param: GLfloat);
     func!(PointParameterfv, (), pname: GLenum, params: *const GLfloat);
     func!(PointParameterfvARB, (), pname: GLenum, params: *const GLfloat);
     func!(PointParameterfvEXT, (), pname: GLenum, params: *const GLfloat);
     func!(PointParameterfvSGIS, (), pname: GLenum, params: *const GLfloat);
     func!(PointParameteri, (), pname: GLenum, param: GLint);
     func!(PointParameteriNV, (), pname: GLenum, param: GLint);
     func!(PointParameteriv, (), pname: GLenum, params: *const GLint);
     func!(PointParameterivNV, (), pname: GLenum, params: *const GLint);
     func!(PointSize, (), size: GLfloat);
     func!(PolygonMode, (), face: GLenum, mode: GLenum);
     func!(PolygonOffset, (), factor: GLfloat, units: GLfloat);
     func!(PolygonOffsetClamp, (), factor: GLfloat, units: GLfloat, clamp: GLfloat);
     func!(PolygonOffsetClampEXT, (), factor: GLfloat, units: GLfloat, clamp: GLfloat);
     func!(PopDebugGroup, (), );
     func!(PrimitiveRestartIndex, (), index: GLuint);
     func!(PrioritizeTexturesEXT, (), n: GLsizei, textures: *const GLuint, priorities: *const GLclampf);
     func!(ProgramBinary, (), program: GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei);
     func!(ProgramEnvParameter4dARB, (), target: GLenum, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(ProgramEnvParameter4dvARB, (), target: GLenum, index: GLuint, params: *const GLdouble);
     func!(ProgramEnvParameter4fARB, (), target: GLenum, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(ProgramEnvParameter4fvARB, (), target: GLenum, index: GLuint, params: *const GLfloat);
     func!(ProgramLocalParameter4dARB, (), target: GLenum, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(ProgramLocalParameter4dvARB, (), target: GLenum, index: GLuint, params: *const GLdouble);
     func!(ProgramLocalParameter4fARB, (), target: GLenum, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(ProgramLocalParameter4fvARB, (), target: GLenum, index: GLuint, params: *const GLfloat);
     func!(ProgramParameter4dNV, (), target: GLenum, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(ProgramParameter4dvNV, (), target: GLenum, index: GLuint, v: *const GLdouble);
     func!(ProgramParameter4fNV, (), target: GLenum, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(ProgramParameter4fvNV, (), target: GLenum, index: GLuint, v: *const GLfloat);
     func!(ProgramParameteri, (), program: GLuint, pname: GLenum, value: GLint);
     func!(ProgramParameteriARB, (), program: GLuint, pname: GLenum, value: GLint);
     func!(ProgramParameteriEXT, (), program: GLuint, pname: GLenum, value: GLint);
     func!(ProgramParameters4dvNV, (), target: GLenum, index: GLuint, count: GLsizei, v: *const GLdouble);
     func!(ProgramParameters4fvNV, (), target: GLenum, index: GLuint, count: GLsizei, v: *const GLfloat);
     func!(ProgramStringARB, (), target: GLenum, format: GLenum, len: GLsizei, string: *const c_void);
     func!(ProgramUniform1d, (), program: GLuint, location: GLint, v0: GLdouble);
     func!(ProgramUniform1dEXT, (), program: GLuint, location: GLint, x: GLdouble);
     func!(ProgramUniform1dv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
     func!(ProgramUniform1dvEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
     func!(ProgramUniform1f, (), program: GLuint, location: GLint, v0: GLfloat);
     func!(ProgramUniform1fEXT, (), program: GLuint, location: GLint, v0: GLfloat);
     func!(ProgramUniform1fv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
     func!(ProgramUniform1fvEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
     func!(ProgramUniform1i, (), program: GLuint, location: GLint, v0: GLint);
     func!(ProgramUniform1iEXT, (), program: GLuint, location: GLint, v0: GLint);
     func!(ProgramUniform1iv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
     func!(ProgramUniform1ivEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
     func!(ProgramUniform1ui, (), program: GLuint, location: GLint, v0: GLuint);
     func!(ProgramUniform1uiEXT, (), program: GLuint, location: GLint, v0: GLuint);
     func!(ProgramUniform1uiv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
     func!(ProgramUniform1uivEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
     func!(ProgramUniform2d, (), program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble);
     func!(ProgramUniform2dEXT, (), program: GLuint, location: GLint, x: GLdouble, y: GLdouble);
     func!(ProgramUniform2dv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
     func!(ProgramUniform2dvEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
     func!(ProgramUniform2f, (), program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat);
     func!(ProgramUniform2fEXT, (), program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat);
     func!(ProgramUniform2fv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
     func!(ProgramUniform2fvEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
     func!(ProgramUniform2i, (), program: GLuint, location: GLint, v0: GLint, v1: GLint);
     func!(ProgramUniform2iEXT, (), program: GLuint, location: GLint, v0: GLint, v1: GLint);
     func!(ProgramUniform2iv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
     func!(ProgramUniform2ivEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
     func!(ProgramUniform2ui, (), program: GLuint, location: GLint, v0: GLuint, v1: GLuint);
     func!(ProgramUniform2uiEXT, (), program: GLuint, location: GLint, v0: GLuint, v1: GLuint);
     func!(ProgramUniform2uiv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
     func!(ProgramUniform2uivEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
     func!(ProgramUniform3d, (), program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble);
     func!(ProgramUniform3dEXT, (), program: GLuint, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(ProgramUniform3dv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
     func!(ProgramUniform3dvEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
     func!(ProgramUniform3f, (), program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
     func!(ProgramUniform3fEXT, (), program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
     func!(ProgramUniform3fv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
     func!(ProgramUniform3fvEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
     func!(ProgramUniform3i, (), program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint);
     func!(ProgramUniform3iEXT, (), program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint);
     func!(ProgramUniform3iv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
     func!(ProgramUniform3ivEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
     func!(ProgramUniform3ui, (), program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
     func!(ProgramUniform3uiEXT, (), program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
     func!(ProgramUniform3uiv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
     func!(ProgramUniform3uivEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
     func!(ProgramUniform4d, (), program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble);
     func!(ProgramUniform4dEXT, (), program: GLuint, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(ProgramUniform4dv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
     func!(ProgramUniform4dvEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
     func!(ProgramUniform4f, (), program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
     func!(ProgramUniform4fEXT, (), program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
     func!(ProgramUniform4fv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
     func!(ProgramUniform4fvEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
     func!(ProgramUniform4i, (), program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
     func!(ProgramUniform4iEXT, (), program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
     func!(ProgramUniform4iv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
     func!(ProgramUniform4ivEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
     func!(ProgramUniform4ui, (), program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
     func!(ProgramUniform4uiEXT, (), program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
     func!(ProgramUniform4uiv, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
     func!(ProgramUniform4uivEXT, (), program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
     func!(ProgramUniformMatrix2dv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix2dvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix2fv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix2fvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix2x3dv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix2x3dvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix2x3fv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix2x3fvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix2x4dv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix2x4dvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix2x4fv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix2x4fvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix3dv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix3dvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix3fv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix3fvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix3x2dv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix3x2dvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix3x2fv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix3x2fvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix3x4dv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix3x4dvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix3x4fv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix3x4fvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix4dv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix4dvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix4fv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix4fvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix4x2dv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix4x2dvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix4x2fv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix4x2fvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix4x3dv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix4x3dvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(ProgramUniformMatrix4x3fv, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramUniformMatrix4x3fvEXT, (), program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(ProgramVertexLimitNV, (), target: GLenum, limit: GLint);
     func!(ProvokingVertex, (), mode: GLenum);
     func!(ProvokingVertexEXT, (), mode: GLenum);
     func!(PushClientAttribDefaultEXT, (), mask: GLbitfield);
     func!(PushDebugGroup, (), source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar);
     func!(QueryCounter, (), id: GLuint, target: GLenum);
     func!(ReadBuffer, (), src: GLenum);
     func!(ReadPixels, (), x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void);
     func!(ReadnPixels, (), x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, data: *mut c_void);
     func!(ReadnPixelsARB, (), x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, data: *mut c_void);
     func!(ReleaseShaderCompiler, (), );
     func!(RenderbufferStorage, (), target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(RenderbufferStorageEXT, (), target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(RenderbufferStorageMultisample, (), target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(RenderbufferStorageMultisampleEXT, (), target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(RequestResidentProgramsNV, (), n: GLsizei, programs: *const GLuint);
     func!(ResumeTransformFeedback, (), );
     func!(ResumeTransformFeedbackNV, (), );
     func!(SampleCoverage, (), value: GLfloat, invert: GLboolean);
     func!(SampleCoverageARB, (), value: GLfloat, invert: GLboolean);
     func!(SampleMaskIndexedNV, (), index: GLuint, mask: GLbitfield);
     func!(SampleMaski, (), maskNumber: GLuint, mask: GLbitfield);
     func!(SamplerParameterIiv, (), sampler: GLuint, pname: GLenum, param: *const GLint);
     func!(SamplerParameterIuiv, (), sampler: GLuint, pname: GLenum, param: *const GLuint);
     func!(SamplerParameterf, (), sampler: GLuint, pname: GLenum, param: GLfloat);
     func!(SamplerParameterfv, (), sampler: GLuint, pname: GLenum, param: *const GLfloat);
     func!(SamplerParameteri, (), sampler: GLuint, pname: GLenum, param: GLint);
     func!(SamplerParameteriv, (), sampler: GLuint, pname: GLenum, param: *const GLint);
     func!(Scissor, (), x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(ScissorArrayv, (), first: GLuint, count: GLsizei, v: *const GLint);
     func!(ScissorIndexed, (), index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei);
     func!(ScissorIndexedv, (), index: GLuint, v: *const GLint);
     func!(ShaderBinary, (), count: GLsizei, shaders: *const GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei);
     func!(ShaderSource, (), shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint);
     func!(ShaderSourceARB, (), shaderObj: GLhandleARB, count: GLsizei, string: *const *const GLcharARB, length: *const GLint);
     func!(ShaderStorageBlockBinding, (), program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint);
     func!(SpecializeShader, (), shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint);
     func!(SpecializeShaderARB, (), shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint);
     func!(StencilFunc, (), func: GLenum, ref_: GLint, mask: GLuint);
     func!(StencilFuncSeparate, (), face: GLenum, func: GLenum, ref_: GLint, mask: GLuint);
     func!(StencilFuncSeparateATI, (), frontfunc: GLenum, backfunc: GLenum, ref_: GLint, mask: GLuint);
     func!(StencilMask, (), mask: GLuint);
     func!(StencilMaskSeparate, (), face: GLenum, mask: GLuint);
     func!(StencilOp, (), fail: GLenum, zfail: GLenum, zpass: GLenum);
     func!(StencilOpSeparate, (), face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
     func!(StencilOpSeparateATI, (), face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
     func!(TexBuffer, (), target: GLenum, internalformat: GLenum, buffer: GLuint);
     func!(TexBufferARB, (), target: GLenum, internalformat: GLenum, buffer: GLuint);
     func!(TexBufferEXT, (), target: GLenum, internalformat: GLenum, buffer: GLuint);
     func!(TexBufferRange, (), target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
     func!(TexCoordPointerEXT, (), size: GLint, type_: GLenum, stride: GLsizei, count: GLsizei, pointer: *const c_void);
     func!(TexImage1D, (), target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexImage2D, (), target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexImage2DMultisample, (), target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);
     func!(TexImage3D, (), target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexImage3DEXT, (), target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexImage3DMultisample, (), target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);
     func!(TexParameterIiv, (), target: GLenum, pname: GLenum, params: *const GLint);
     func!(TexParameterIivEXT, (), target: GLenum, pname: GLenum, params: *const GLint);
     func!(TexParameterIuiv, (), target: GLenum, pname: GLenum, params: *const GLuint);
     func!(TexParameterIuivEXT, (), target: GLenum, pname: GLenum, params: *const GLuint);
     func!(TexParameterf, (), target: GLenum, pname: GLenum, param: GLfloat);
     func!(TexParameterfv, (), target: GLenum, pname: GLenum, params: *const GLfloat);
     func!(TexParameteri, (), target: GLenum, pname: GLenum, param: GLint);
     func!(TexParameteriv, (), target: GLenum, pname: GLenum, params: *const GLint);
     func!(TexRenderbufferNV, (), target: GLenum, renderbuffer: GLuint);
     func!(TexStorage1D, (), target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei);
     func!(TexStorage1DEXT, (), target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei);
     func!(TexStorage2D, (), target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(TexStorage2DEXT, (), target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(TexStorage2DMultisample, (), target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);
     func!(TexStorage3D, (), target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei);
     func!(TexStorage3DEXT, (), target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei);
     func!(TexStorage3DMultisample, (), target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);
     func!(TexSubImage1D, (), target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexSubImage1DEXT, (), target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexSubImage2D, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexSubImage2DEXT, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexSubImage3D, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexSubImage3DEXT, (), target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureBarrier, (), );
     func!(TextureBuffer, (), texture: GLuint, internalformat: GLenum, buffer: GLuint);
     func!(TextureBufferEXT, (), texture: GLuint, target: GLenum, internalformat: GLenum, buffer: GLuint);
     func!(TextureBufferRange, (), texture: GLuint, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
     func!(TextureBufferRangeEXT, (), texture: GLuint, target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
     func!(TextureImage1DEXT, (), texture: GLuint, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureImage2DEXT, (), texture: GLuint, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureImage3DEXT, (), texture: GLuint, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TexturePageCommitmentEXT, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, commit: GLboolean);
     func!(TextureParameterIiv, (), texture: GLuint, pname: GLenum, params: *const GLint);
     func!(TextureParameterIivEXT, (), texture: GLuint, target: GLenum, pname: GLenum, params: *const GLint);
     func!(TextureParameterIuiv, (), texture: GLuint, pname: GLenum, params: *const GLuint);
     func!(TextureParameterIuivEXT, (), texture: GLuint, target: GLenum, pname: GLenum, params: *const GLuint);
     func!(TextureParameterf, (), texture: GLuint, pname: GLenum, param: GLfloat);
     func!(TextureParameterfEXT, (), texture: GLuint, target: GLenum, pname: GLenum, param: GLfloat);
     func!(TextureParameterfv, (), texture: GLuint, pname: GLenum, param: *const GLfloat);
     func!(TextureParameterfvEXT, (), texture: GLuint, target: GLenum, pname: GLenum, params: *const GLfloat);
     func!(TextureParameteri, (), texture: GLuint, pname: GLenum, param: GLint);
     func!(TextureParameteriEXT, (), texture: GLuint, target: GLenum, pname: GLenum, param: GLint);
     func!(TextureParameteriv, (), texture: GLuint, pname: GLenum, param: *const GLint);
     func!(TextureParameterivEXT, (), texture: GLuint, target: GLenum, pname: GLenum, params: *const GLint);
     func!(TextureRenderbufferEXT, (), texture: GLuint, target: GLenum, renderbuffer: GLuint);
     func!(TextureStorage1D, (), texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei);
     func!(TextureStorage1DEXT, (), texture: GLuint, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei);
     func!(TextureStorage2D, (), texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(TextureStorage2DEXT, (), texture: GLuint, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
     func!(TextureStorage2DMultisample, (), texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);
     func!(TextureStorage2DMultisampleEXT, (), texture: GLuint, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);
     func!(TextureStorage3D, (), texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei);
     func!(TextureStorage3DEXT, (), texture: GLuint, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei);
     func!(TextureStorage3DMultisample, (), texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);
     func!(TextureStorage3DMultisampleEXT, (), texture: GLuint, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);
     func!(TextureSubImage1D, (), texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureSubImage1DEXT, (), texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureSubImage2D, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureSubImage2DEXT, (), texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureSubImage3D, (), texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureSubImage3DEXT, (), texture: GLuint, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
     func!(TextureView, (), texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint);
     func!(TrackMatrixNV, (), target: GLenum, address: GLuint, matrix: GLenum, transform: GLenum);
     func!(TransformFeedbackAttribsNV, (), count: GLsizei, attribs: *const GLint, bufferMode: GLenum);
     func!(TransformFeedbackBufferBase, (), xfb: GLuint, index: GLuint, buffer: GLuint);
     func!(TransformFeedbackBufferRange, (), xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
     func!(TransformFeedbackStreamAttribsNV, (), count: GLsizei, attribs: *const GLint, nbuffers: GLsizei, bufstreams: *const GLint, bufferMode: GLenum);
     func!(TransformFeedbackVaryings, (), program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum);
     func!(TransformFeedbackVaryingsEXT, (), program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum);
     func!(TransformFeedbackVaryingsNV, (), program: GLuint, count: GLsizei, locations: *const GLint, bufferMode: GLenum);
     func!(Uniform1d, (), location: GLint, x: GLdouble);
     func!(Uniform1dv, (), location: GLint, count: GLsizei, value: *const GLdouble);
     func!(Uniform1f, (), location: GLint, v0: GLfloat);
     func!(Uniform1fARB, (), location: GLint, v0: GLfloat);
     func!(Uniform1fv, (), location: GLint, count: GLsizei, value: *const GLfloat);
     func!(Uniform1fvARB, (), location: GLint, count: GLsizei, value: *const GLfloat);
     func!(Uniform1i, (), location: GLint, v0: GLint);
     func!(Uniform1iARB, (), location: GLint, v0: GLint);
     func!(Uniform1iv, (), location: GLint, count: GLsizei, value: *const GLint);
     func!(Uniform1ivARB, (), location: GLint, count: GLsizei, value: *const GLint);
     func!(Uniform1ui, (), location: GLint, v0: GLuint);
     func!(Uniform1uiEXT, (), location: GLint, v0: GLuint);
     func!(Uniform1uiv, (), location: GLint, count: GLsizei, value: *const GLuint);
     func!(Uniform1uivEXT, (), location: GLint, count: GLsizei, value: *const GLuint);
     func!(Uniform2d, (), location: GLint, x: GLdouble, y: GLdouble);
     func!(Uniform2dv, (), location: GLint, count: GLsizei, value: *const GLdouble);
     func!(Uniform2f, (), location: GLint, v0: GLfloat, v1: GLfloat);
     func!(Uniform2fARB, (), location: GLint, v0: GLfloat, v1: GLfloat);
     func!(Uniform2fv, (), location: GLint, count: GLsizei, value: *const GLfloat);
     func!(Uniform2fvARB, (), location: GLint, count: GLsizei, value: *const GLfloat);
     func!(Uniform2i, (), location: GLint, v0: GLint, v1: GLint);
     func!(Uniform2iARB, (), location: GLint, v0: GLint, v1: GLint);
     func!(Uniform2iv, (), location: GLint, count: GLsizei, value: *const GLint);
     func!(Uniform2ivARB, (), location: GLint, count: GLsizei, value: *const GLint);
     func!(Uniform2ui, (), location: GLint, v0: GLuint, v1: GLuint);
     func!(Uniform2uiEXT, (), location: GLint, v0: GLuint, v1: GLuint);
     func!(Uniform2uiv, (), location: GLint, count: GLsizei, value: *const GLuint);
     func!(Uniform2uivEXT, (), location: GLint, count: GLsizei, value: *const GLuint);
     func!(Uniform3d, (), location: GLint, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(Uniform3dv, (), location: GLint, count: GLsizei, value: *const GLdouble);
     func!(Uniform3f, (), location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
     func!(Uniform3fARB, (), location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
     func!(Uniform3fv, (), location: GLint, count: GLsizei, value: *const GLfloat);
     func!(Uniform3fvARB, (), location: GLint, count: GLsizei, value: *const GLfloat);
     func!(Uniform3i, (), location: GLint, v0: GLint, v1: GLint, v2: GLint);
     func!(Uniform3iARB, (), location: GLint, v0: GLint, v1: GLint, v2: GLint);
     func!(Uniform3iv, (), location: GLint, count: GLsizei, value: *const GLint);
     func!(Uniform3ivARB, (), location: GLint, count: GLsizei, value: *const GLint);
     func!(Uniform3ui, (), location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
     func!(Uniform3uiEXT, (), location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
     func!(Uniform3uiv, (), location: GLint, count: GLsizei, value: *const GLuint);
     func!(Uniform3uivEXT, (), location: GLint, count: GLsizei, value: *const GLuint);
     func!(Uniform4d, (), location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(Uniform4dv, (), location: GLint, count: GLsizei, value: *const GLdouble);
     func!(Uniform4f, (), location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
     func!(Uniform4fARB, (), location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
     func!(Uniform4fv, (), location: GLint, count: GLsizei, value: *const GLfloat);
     func!(Uniform4fvARB, (), location: GLint, count: GLsizei, value: *const GLfloat);
     func!(Uniform4i, (), location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
     func!(Uniform4iARB, (), location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
     func!(Uniform4iv, (), location: GLint, count: GLsizei, value: *const GLint);
     func!(Uniform4ivARB, (), location: GLint, count: GLsizei, value: *const GLint);
     func!(Uniform4ui, (), location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
     func!(Uniform4uiEXT, (), location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
     func!(Uniform4uiv, (), location: GLint, count: GLsizei, value: *const GLuint);
     func!(Uniform4uivEXT, (), location: GLint, count: GLsizei, value: *const GLuint);
     func!(UniformBlockBinding, (), program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint);
     func!(UniformMatrix2dv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(UniformMatrix2fv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix2fvARB, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix2x3dv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(UniformMatrix2x3fv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix2x4dv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(UniformMatrix2x4fv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix3dv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(UniformMatrix3fv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix3fvARB, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix3x2dv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(UniformMatrix3x2fv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix3x4dv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(UniformMatrix3x4fv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix4dv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(UniformMatrix4fv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix4fvARB, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix4x2dv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(UniformMatrix4x2fv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformMatrix4x3dv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
     func!(UniformMatrix4x3fv, (), location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
     func!(UniformSubroutinesuiv, (), shadertype: GLenum, count: GLsizei, indices: *const GLuint);
     func!(UnmapBuffer, GLboolean, target: GLenum);
     func!(UnmapBufferARB, GLboolean, target: GLenum);
     func!(UnmapNamedBuffer, GLboolean, buffer: GLuint);
     func!(UnmapNamedBufferEXT, GLboolean, buffer: GLuint);
     func!(UseProgram, (), program: GLuint);
     func!(UseProgramObjectARB, (), programObj: GLhandleARB);
     func!(UseProgramStages, (), pipeline: GLuint, stages: GLbitfield, program: GLuint);
     func!(ValidateProgram, (), program: GLuint);
     func!(ValidateProgramARB, (), programObj: GLhandleARB);
     func!(ValidateProgramPipeline, (), pipeline: GLuint);
     func!(VertexArrayAttribBinding, (), vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint);
     func!(VertexArrayAttribFormat, (), vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint);
     func!(VertexArrayAttribIFormat, (), vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);
     func!(VertexArrayAttribLFormat, (), vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);
     func!(VertexArrayBindVertexBufferEXT, (), vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);
     func!(VertexArrayBindingDivisor, (), vaobj: GLuint, bindingindex: GLuint, divisor: GLuint);
     func!(VertexArrayColorOffsetEXT, (), vaobj: GLuint, buffer: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayEdgeFlagOffsetEXT, (), vaobj: GLuint, buffer: GLuint, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayElementBuffer, (), vaobj: GLuint, buffer: GLuint);
     func!(VertexArrayFogCoordOffsetEXT, (), vaobj: GLuint, buffer: GLuint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayIndexOffsetEXT, (), vaobj: GLuint, buffer: GLuint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayMultiTexCoordOffsetEXT, (), vaobj: GLuint, buffer: GLuint, texunit: GLenum, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayNormalOffsetEXT, (), vaobj: GLuint, buffer: GLuint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexArraySecondaryColorOffsetEXT, (), vaobj: GLuint, buffer: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayTexCoordOffsetEXT, (), vaobj: GLuint, buffer: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayVertexAttribBindingEXT, (), vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint);
     func!(VertexArrayVertexAttribDivisorEXT, (), vaobj: GLuint, index: GLuint, divisor: GLuint);
     func!(VertexArrayVertexAttribFormatEXT, (), vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint);
     func!(VertexArrayVertexAttribIFormatEXT, (), vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);
     func!(VertexArrayVertexAttribIOffsetEXT, (), vaobj: GLuint, buffer: GLuint, index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayVertexAttribLFormatEXT, (), vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);
     func!(VertexArrayVertexAttribLOffsetEXT, (), vaobj: GLuint, buffer: GLuint, index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayVertexAttribOffsetEXT, (), vaobj: GLuint, buffer: GLuint, index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, offset: GLintptr);
     func!(VertexArrayVertexBindingDivisorEXT, (), vaobj: GLuint, bindingindex: GLuint, divisor: GLuint);
     func!(VertexArrayVertexBuffer, (), vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);
     func!(VertexArrayVertexBuffers, (), vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei);
     func!(VertexArrayVertexOffsetEXT, (), vaobj: GLuint, buffer: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr);
     func!(VertexAttrib1d, (), index: GLuint, x: GLdouble);
     func!(VertexAttrib1dARB, (), index: GLuint, x: GLdouble);
     func!(VertexAttrib1dNV, (), index: GLuint, x: GLdouble);
     func!(VertexAttrib1dv, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib1dvARB, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib1dvNV, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib1f, (), index: GLuint, x: GLfloat);
     func!(VertexAttrib1fARB, (), index: GLuint, x: GLfloat);
     func!(VertexAttrib1fNV, (), index: GLuint, x: GLfloat);
     func!(VertexAttrib1fv, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib1fvARB, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib1fvNV, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib1s, (), index: GLuint, x: GLshort);
     func!(VertexAttrib1sARB, (), index: GLuint, x: GLshort);
     func!(VertexAttrib1sNV, (), index: GLuint, x: GLshort);
     func!(VertexAttrib1sv, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib1svARB, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib1svNV, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib2d, (), index: GLuint, x: GLdouble, y: GLdouble);
     func!(VertexAttrib2dARB, (), index: GLuint, x: GLdouble, y: GLdouble);
     func!(VertexAttrib2dNV, (), index: GLuint, x: GLdouble, y: GLdouble);
     func!(VertexAttrib2dv, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib2dvARB, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib2dvNV, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib2f, (), index: GLuint, x: GLfloat, y: GLfloat);
     func!(VertexAttrib2fARB, (), index: GLuint, x: GLfloat, y: GLfloat);
     func!(VertexAttrib2fNV, (), index: GLuint, x: GLfloat, y: GLfloat);
     func!(VertexAttrib2fv, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib2fvARB, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib2fvNV, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib2s, (), index: GLuint, x: GLshort, y: GLshort);
     func!(VertexAttrib2sARB, (), index: GLuint, x: GLshort, y: GLshort);
     func!(VertexAttrib2sNV, (), index: GLuint, x: GLshort, y: GLshort);
     func!(VertexAttrib2sv, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib2svARB, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib2svNV, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib3d, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(VertexAttrib3dARB, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(VertexAttrib3dNV, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(VertexAttrib3dv, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib3dvARB, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib3dvNV, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib3f, (), index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(VertexAttrib3fARB, (), index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(VertexAttrib3fNV, (), index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
     func!(VertexAttrib3fv, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib3fvARB, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib3fvNV, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib3s, (), index: GLuint, x: GLshort, y: GLshort, z: GLshort);
     func!(VertexAttrib3sARB, (), index: GLuint, x: GLshort, y: GLshort, z: GLshort);
     func!(VertexAttrib3sNV, (), index: GLuint, x: GLshort, y: GLshort, z: GLshort);
     func!(VertexAttrib3sv, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib3svARB, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib3svNV, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib4Nbv, (), index: GLuint, v: *const GLbyte);
     func!(VertexAttrib4NbvARB, (), index: GLuint, v: *const GLbyte);
     func!(VertexAttrib4Niv, (), index: GLuint, v: *const GLint);
     func!(VertexAttrib4NivARB, (), index: GLuint, v: *const GLint);
     func!(VertexAttrib4Nsv, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib4NsvARB, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib4Nub, (), index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);
     func!(VertexAttrib4NubARB, (), index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);
     func!(VertexAttrib4Nubv, (), index: GLuint, v: *const GLubyte);
     func!(VertexAttrib4NubvARB, (), index: GLuint, v: *const GLubyte);
     func!(VertexAttrib4Nuiv, (), index: GLuint, v: *const GLuint);
     func!(VertexAttrib4NuivARB, (), index: GLuint, v: *const GLuint);
     func!(VertexAttrib4Nusv, (), index: GLuint, v: *const GLushort);
     func!(VertexAttrib4NusvARB, (), index: GLuint, v: *const GLushort);
     func!(VertexAttrib4bv, (), index: GLuint, v: *const GLbyte);
     func!(VertexAttrib4bvARB, (), index: GLuint, v: *const GLbyte);
     func!(VertexAttrib4d, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(VertexAttrib4dARB, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(VertexAttrib4dNV, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(VertexAttrib4dv, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib4dvARB, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib4dvNV, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttrib4f, (), index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(VertexAttrib4fARB, (), index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(VertexAttrib4fNV, (), index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
     func!(VertexAttrib4fv, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib4fvARB, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib4fvNV, (), index: GLuint, v: *const GLfloat);
     func!(VertexAttrib4iv, (), index: GLuint, v: *const GLint);
     func!(VertexAttrib4ivARB, (), index: GLuint, v: *const GLint);
     func!(VertexAttrib4s, (), index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);
     func!(VertexAttrib4sARB, (), index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);
     func!(VertexAttrib4sNV, (), index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);
     func!(VertexAttrib4sv, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib4svARB, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib4svNV, (), index: GLuint, v: *const GLshort);
     func!(VertexAttrib4ubNV, (), index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);
     func!(VertexAttrib4ubv, (), index: GLuint, v: *const GLubyte);
     func!(VertexAttrib4ubvARB, (), index: GLuint, v: *const GLubyte);
     func!(VertexAttrib4ubvNV, (), index: GLuint, v: *const GLubyte);
     func!(VertexAttrib4uiv, (), index: GLuint, v: *const GLuint);
     func!(VertexAttrib4uivARB, (), index: GLuint, v: *const GLuint);
     func!(VertexAttrib4usv, (), index: GLuint, v: *const GLushort);
     func!(VertexAttrib4usvARB, (), index: GLuint, v: *const GLushort);
     func!(VertexAttribBinding, (), attribindex: GLuint, bindingindex: GLuint);
     func!(VertexAttribDivisor, (), index: GLuint, divisor: GLuint);
     func!(VertexAttribDivisorARB, (), index: GLuint, divisor: GLuint);
     func!(VertexAttribFormat, (), attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint);
     func!(VertexAttribI1i, (), index: GLuint, x: GLint);
     func!(VertexAttribI1iEXT, (), index: GLuint, x: GLint);
     func!(VertexAttribI1iv, (), index: GLuint, v: *const GLint);
     func!(VertexAttribI1ivEXT, (), index: GLuint, v: *const GLint);
     func!(VertexAttribI1ui, (), index: GLuint, x: GLuint);
     func!(VertexAttribI1uiEXT, (), index: GLuint, x: GLuint);
     func!(VertexAttribI1uiv, (), index: GLuint, v: *const GLuint);
     func!(VertexAttribI1uivEXT, (), index: GLuint, v: *const GLuint);
     func!(VertexAttribI2i, (), index: GLuint, x: GLint, y: GLint);
     func!(VertexAttribI2iEXT, (), index: GLuint, x: GLint, y: GLint);
     func!(VertexAttribI2iv, (), index: GLuint, v: *const GLint);
     func!(VertexAttribI2ivEXT, (), index: GLuint, v: *const GLint);
     func!(VertexAttribI2ui, (), index: GLuint, x: GLuint, y: GLuint);
     func!(VertexAttribI2uiEXT, (), index: GLuint, x: GLuint, y: GLuint);
     func!(VertexAttribI2uiv, (), index: GLuint, v: *const GLuint);
     func!(VertexAttribI2uivEXT, (), index: GLuint, v: *const GLuint);
     func!(VertexAttribI3i, (), index: GLuint, x: GLint, y: GLint, z: GLint);
     func!(VertexAttribI3iEXT, (), index: GLuint, x: GLint, y: GLint, z: GLint);
     func!(VertexAttribI3iv, (), index: GLuint, v: *const GLint);
     func!(VertexAttribI3ivEXT, (), index: GLuint, v: *const GLint);
     func!(VertexAttribI3ui, (), index: GLuint, x: GLuint, y: GLuint, z: GLuint);
     func!(VertexAttribI3uiEXT, (), index: GLuint, x: GLuint, y: GLuint, z: GLuint);
     func!(VertexAttribI3uiv, (), index: GLuint, v: *const GLuint);
     func!(VertexAttribI3uivEXT, (), index: GLuint, v: *const GLuint);
     func!(VertexAttribI4bv, (), index: GLuint, v: *const GLbyte);
     func!(VertexAttribI4bvEXT, (), index: GLuint, v: *const GLbyte);
     func!(VertexAttribI4i, (), index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
     func!(VertexAttribI4iEXT, (), index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
     func!(VertexAttribI4iv, (), index: GLuint, v: *const GLint);
     func!(VertexAttribI4ivEXT, (), index: GLuint, v: *const GLint);
     func!(VertexAttribI4sv, (), index: GLuint, v: *const GLshort);
     func!(VertexAttribI4svEXT, (), index: GLuint, v: *const GLshort);
     func!(VertexAttribI4ubv, (), index: GLuint, v: *const GLubyte);
     func!(VertexAttribI4ubvEXT, (), index: GLuint, v: *const GLubyte);
     func!(VertexAttribI4ui, (), index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
     func!(VertexAttribI4uiEXT, (), index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
     func!(VertexAttribI4uiv, (), index: GLuint, v: *const GLuint);
     func!(VertexAttribI4uivEXT, (), index: GLuint, v: *const GLuint);
     func!(VertexAttribI4usv, (), index: GLuint, v: *const GLushort);
     func!(VertexAttribI4usvEXT, (), index: GLuint, v: *const GLushort);
     func!(VertexAttribIFormat, (), attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);
     func!(VertexAttribIPointer, (), index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(VertexAttribIPointerEXT, (), index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(VertexAttribL1d, (), index: GLuint, x: GLdouble);
     func!(VertexAttribL1dEXT, (), index: GLuint, x: GLdouble);
     func!(VertexAttribL1dv, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttribL1dvEXT, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttribL2d, (), index: GLuint, x: GLdouble, y: GLdouble);
     func!(VertexAttribL2dEXT, (), index: GLuint, x: GLdouble, y: GLdouble);
     func!(VertexAttribL2dv, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttribL2dvEXT, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttribL3d, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(VertexAttribL3dEXT, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
     func!(VertexAttribL3dv, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttribL3dvEXT, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttribL4d, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(VertexAttribL4dEXT, (), index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
     func!(VertexAttribL4dv, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttribL4dvEXT, (), index: GLuint, v: *const GLdouble);
     func!(VertexAttribLFormat, (), attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);
     func!(VertexAttribLPointer, (), index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(VertexAttribLPointerEXT, (), index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(VertexAttribP1ui, (), index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
     func!(VertexAttribP1uiv, (), index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint);
     func!(VertexAttribP2ui, (), index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
     func!(VertexAttribP2uiv, (), index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint);
     func!(VertexAttribP3ui, (), index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
     func!(VertexAttribP3uiv, (), index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint);
     func!(VertexAttribP4ui, (), index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
     func!(VertexAttribP4uiv, (), index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint);
     func!(VertexAttribPointer, (), index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void);
     func!(VertexAttribPointerARB, (), index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void);
     func!(VertexAttribPointerNV, (), index: GLuint, fsize: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
     func!(VertexAttribs1dvNV, (), index: GLuint, count: GLsizei, v: *const GLdouble);
     func!(VertexAttribs1fvNV, (), index: GLuint, count: GLsizei, v: *const GLfloat);
     func!(VertexAttribs1svNV, (), index: GLuint, count: GLsizei, v: *const GLshort);
     func!(VertexAttribs2dvNV, (), index: GLuint, count: GLsizei, v: *const GLdouble);
     func!(VertexAttribs2fvNV, (), index: GLuint, count: GLsizei, v: *const GLfloat);
     func!(VertexAttribs2svNV, (), index: GLuint, count: GLsizei, v: *const GLshort);
     func!(VertexAttribs3dvNV, (), index: GLuint, count: GLsizei, v: *const GLdouble);
     func!(VertexAttribs3fvNV, (), index: GLuint, count: GLsizei, v: *const GLfloat);
     func!(VertexAttribs3svNV, (), index: GLuint, count: GLsizei, v: *const GLshort);
     func!(VertexAttribs4dvNV, (), index: GLuint, count: GLsizei, v: *const GLdouble);
     func!(VertexAttribs4fvNV, (), index: GLuint, count: GLsizei, v: *const GLfloat);
     func!(VertexAttribs4svNV, (), index: GLuint, count: GLsizei, v: *const GLshort);
     func!(VertexAttribs4ubvNV, (), index: GLuint, count: GLsizei, v: *const GLubyte);
     func!(VertexBindingDivisor, (), bindingindex: GLuint, divisor: GLuint);
     func!(VertexPointerEXT, (), size: GLint, type_: GLenum, stride: GLsizei, count: GLsizei, pointer: *const c_void);
     func!(Viewport, (), x: GLint, y: GLint, width: GLsizei, height: GLsizei);
     func!(ViewportArrayv, (), first: GLuint, count: GLsizei, v: *const GLfloat);
     func!(ViewportIndexedf, (), index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat);
     func!(ViewportIndexedfv, (), index: GLuint, v: *const GLfloat);
     func!(WaitSync, (), sync: GLsync, flags: GLbitfield, timeout: GLuint64);

    }
}


pub fn load<F>(mut loadfn: F) -> functions::Gl where F: FnMut(&'static str) -> *const c_void {
    #[allow(unused_mut)]
    let mut ctx = Gl {
         ActiveShaderProgram: FnPtr::new(loadfn("glActiveShaderProgram")),
         ActiveTexture: FnPtr::new(loadfn("glActiveTexture")),
         ActiveTextureARB: FnPtr::new(loadfn("glActiveTextureARB")),
         ActiveVaryingNV: FnPtr::new(loadfn("glActiveVaryingNV")),
         AreProgramsResidentNV: FnPtr::new(loadfn("glAreProgramsResidentNV")),
         AreTexturesResidentEXT: FnPtr::new(loadfn("glAreTexturesResidentEXT")),
         ArrayElementEXT: FnPtr::new(loadfn("glArrayElementEXT")),
         AttachObjectARB: FnPtr::new(loadfn("glAttachObjectARB")),
         AttachShader: FnPtr::new(loadfn("glAttachShader")),
         BeginConditionalRender: FnPtr::new(loadfn("glBeginConditionalRender")),
         BeginConditionalRenderNV: FnPtr::new(loadfn("glBeginConditionalRenderNV")),
         BeginConditionalRenderNVX: FnPtr::new(loadfn("glBeginConditionalRenderNVX")),
         BeginQuery: FnPtr::new(loadfn("glBeginQuery")),
         BeginQueryARB: FnPtr::new(loadfn("glBeginQueryARB")),
         BeginQueryIndexed: FnPtr::new(loadfn("glBeginQueryIndexed")),
         BeginTransformFeedback: FnPtr::new(loadfn("glBeginTransformFeedback")),
         BeginTransformFeedbackEXT: FnPtr::new(loadfn("glBeginTransformFeedbackEXT")),
         BeginTransformFeedbackNV: FnPtr::new(loadfn("glBeginTransformFeedbackNV")),
         BindAttribLocation: FnPtr::new(loadfn("glBindAttribLocation")),
         BindAttribLocationARB: FnPtr::new(loadfn("glBindAttribLocationARB")),
         BindBuffer: FnPtr::new(loadfn("glBindBuffer")),
         BindBufferARB: FnPtr::new(loadfn("glBindBufferARB")),
         BindBufferBase: FnPtr::new(loadfn("glBindBufferBase")),
         BindBufferBaseEXT: FnPtr::new(loadfn("glBindBufferBaseEXT")),
         BindBufferBaseNV: FnPtr::new(loadfn("glBindBufferBaseNV")),
         BindBufferOffsetEXT: FnPtr::new(loadfn("glBindBufferOffsetEXT")),
         BindBufferOffsetNV: FnPtr::new(loadfn("glBindBufferOffsetNV")),
         BindBufferRange: FnPtr::new(loadfn("glBindBufferRange")),
         BindBufferRangeEXT: FnPtr::new(loadfn("glBindBufferRangeEXT")),
         BindBufferRangeNV: FnPtr::new(loadfn("glBindBufferRangeNV")),
         BindBuffersBase: FnPtr::new(loadfn("glBindBuffersBase")),
         BindBuffersRange: FnPtr::new(loadfn("glBindBuffersRange")),
         BindFragDataLocation: FnPtr::new(loadfn("glBindFragDataLocation")),
         BindFragDataLocationEXT: FnPtr::new(loadfn("glBindFragDataLocationEXT")),
         BindFragDataLocationIndexed: FnPtr::new(loadfn("glBindFragDataLocationIndexed")),
         BindFramebuffer: FnPtr::new(loadfn("glBindFramebuffer")),
         BindFramebufferEXT: FnPtr::new(loadfn("glBindFramebufferEXT")),
         BindImageTexture: FnPtr::new(loadfn("glBindImageTexture")),
         BindImageTextureEXT: FnPtr::new(loadfn("glBindImageTextureEXT")),
         BindImageTextures: FnPtr::new(loadfn("glBindImageTextures")),
         BindMultiTextureEXT: FnPtr::new(loadfn("glBindMultiTextureEXT")),
         BindProgramARB: FnPtr::new(loadfn("glBindProgramARB")),
         BindProgramNV: FnPtr::new(loadfn("glBindProgramNV")),
         BindProgramPipeline: FnPtr::new(loadfn("glBindProgramPipeline")),
         BindRenderbuffer: FnPtr::new(loadfn("glBindRenderbuffer")),
         BindRenderbufferEXT: FnPtr::new(loadfn("glBindRenderbufferEXT")),
         BindSampler: FnPtr::new(loadfn("glBindSampler")),
         BindSamplers: FnPtr::new(loadfn("glBindSamplers")),
         BindTexture: FnPtr::new(loadfn("glBindTexture")),
         BindTextureEXT: FnPtr::new(loadfn("glBindTextureEXT")),
         BindTextureUnit: FnPtr::new(loadfn("glBindTextureUnit")),
         BindTextures: FnPtr::new(loadfn("glBindTextures")),
         BindTransformFeedback: FnPtr::new(loadfn("glBindTransformFeedback")),
         BindTransformFeedbackNV: FnPtr::new(loadfn("glBindTransformFeedbackNV")),
         BindVertexArray: FnPtr::new(loadfn("glBindVertexArray")),
         BindVertexArrayAPPLE: FnPtr::new(loadfn("glBindVertexArrayAPPLE")),
         BindVertexBuffer: FnPtr::new(loadfn("glBindVertexBuffer")),
         BindVertexBuffers: FnPtr::new(loadfn("glBindVertexBuffers")),
         BlendColor: FnPtr::new(loadfn("glBlendColor")),
         BlendColorEXT: FnPtr::new(loadfn("glBlendColorEXT")),
         BlendEquation: FnPtr::new(loadfn("glBlendEquation")),
         BlendEquationEXT: FnPtr::new(loadfn("glBlendEquationEXT")),
         BlendEquationIndexedAMD: FnPtr::new(loadfn("glBlendEquationIndexedAMD")),
         BlendEquationSeparate: FnPtr::new(loadfn("glBlendEquationSeparate")),
         BlendEquationSeparateEXT: FnPtr::new(loadfn("glBlendEquationSeparateEXT")),
         BlendEquationSeparateIndexedAMD: FnPtr::new(loadfn("glBlendEquationSeparateIndexedAMD")),
         BlendEquationSeparatei: FnPtr::new(loadfn("glBlendEquationSeparatei")),
         BlendEquationSeparateiARB: FnPtr::new(loadfn("glBlendEquationSeparateiARB")),
         BlendEquationi: FnPtr::new(loadfn("glBlendEquationi")),
         BlendEquationiARB: FnPtr::new(loadfn("glBlendEquationiARB")),
         BlendFunc: FnPtr::new(loadfn("glBlendFunc")),
         BlendFuncIndexedAMD: FnPtr::new(loadfn("glBlendFuncIndexedAMD")),
         BlendFuncSeparate: FnPtr::new(loadfn("glBlendFuncSeparate")),
         BlendFuncSeparateEXT: FnPtr::new(loadfn("glBlendFuncSeparateEXT")),
         BlendFuncSeparateINGR: FnPtr::new(loadfn("glBlendFuncSeparateINGR")),
         BlendFuncSeparateIndexedAMD: FnPtr::new(loadfn("glBlendFuncSeparateIndexedAMD")),
         BlendFuncSeparatei: FnPtr::new(loadfn("glBlendFuncSeparatei")),
         BlendFuncSeparateiARB: FnPtr::new(loadfn("glBlendFuncSeparateiARB")),
         BlendFunci: FnPtr::new(loadfn("glBlendFunci")),
         BlendFunciARB: FnPtr::new(loadfn("glBlendFunciARB")),
         BlitFramebuffer: FnPtr::new(loadfn("glBlitFramebuffer")),
         BlitFramebufferEXT: FnPtr::new(loadfn("glBlitFramebufferEXT")),
         BlitNamedFramebuffer: FnPtr::new(loadfn("glBlitNamedFramebuffer")),
         BufferData: FnPtr::new(loadfn("glBufferData")),
         BufferDataARB: FnPtr::new(loadfn("glBufferDataARB")),
         BufferParameteriAPPLE: FnPtr::new(loadfn("glBufferParameteriAPPLE")),
         BufferStorage: FnPtr::new(loadfn("glBufferStorage")),
         BufferSubData: FnPtr::new(loadfn("glBufferSubData")),
         BufferSubDataARB: FnPtr::new(loadfn("glBufferSubDataARB")),
         CheckFramebufferStatus: FnPtr::new(loadfn("glCheckFramebufferStatus")),
         CheckFramebufferStatusEXT: FnPtr::new(loadfn("glCheckFramebufferStatusEXT")),
         CheckNamedFramebufferStatus: FnPtr::new(loadfn("glCheckNamedFramebufferStatus")),
         CheckNamedFramebufferStatusEXT: FnPtr::new(loadfn("glCheckNamedFramebufferStatusEXT")),
         ClampColor: FnPtr::new(loadfn("glClampColor")),
         ClampColorARB: FnPtr::new(loadfn("glClampColorARB")),
         Clear: FnPtr::new(loadfn("glClear")),
         ClearBufferData: FnPtr::new(loadfn("glClearBufferData")),
         ClearBufferSubData: FnPtr::new(loadfn("glClearBufferSubData")),
         ClearBufferfi: FnPtr::new(loadfn("glClearBufferfi")),
         ClearBufferfv: FnPtr::new(loadfn("glClearBufferfv")),
         ClearBufferiv: FnPtr::new(loadfn("glClearBufferiv")),
         ClearBufferuiv: FnPtr::new(loadfn("glClearBufferuiv")),
         ClearColor: FnPtr::new(loadfn("glClearColor")),
         ClearColorIiEXT: FnPtr::new(loadfn("glClearColorIiEXT")),
         ClearColorIuiEXT: FnPtr::new(loadfn("glClearColorIuiEXT")),
         ClearDepth: FnPtr::new(loadfn("glClearDepth")),
         ClearDepthf: FnPtr::new(loadfn("glClearDepthf")),
         ClearDepthfOES: FnPtr::new(loadfn("glClearDepthfOES")),
         ClearNamedBufferData: FnPtr::new(loadfn("glClearNamedBufferData")),
         ClearNamedBufferDataEXT: FnPtr::new(loadfn("glClearNamedBufferDataEXT")),
         ClearNamedBufferSubData: FnPtr::new(loadfn("glClearNamedBufferSubData")),
         ClearNamedBufferSubDataEXT: FnPtr::new(loadfn("glClearNamedBufferSubDataEXT")),
         ClearNamedFramebufferfi: FnPtr::new(loadfn("glClearNamedFramebufferfi")),
         ClearNamedFramebufferfv: FnPtr::new(loadfn("glClearNamedFramebufferfv")),
         ClearNamedFramebufferiv: FnPtr::new(loadfn("glClearNamedFramebufferiv")),
         ClearNamedFramebufferuiv: FnPtr::new(loadfn("glClearNamedFramebufferuiv")),
         ClearStencil: FnPtr::new(loadfn("glClearStencil")),
         ClearTexImage: FnPtr::new(loadfn("glClearTexImage")),
         ClearTexSubImage: FnPtr::new(loadfn("glClearTexSubImage")),
         ClientActiveTextureARB: FnPtr::new(loadfn("glClientActiveTextureARB")),
         ClientAttribDefaultEXT: FnPtr::new(loadfn("glClientAttribDefaultEXT")),
         ClientWaitSync: FnPtr::new(loadfn("glClientWaitSync")),
         ClipControl: FnPtr::new(loadfn("glClipControl")),
         ClipPlanefOES: FnPtr::new(loadfn("glClipPlanefOES")),
         ColorMask: FnPtr::new(loadfn("glColorMask")),
         ColorMaskIndexedEXT: FnPtr::new(loadfn("glColorMaskIndexedEXT")),
         ColorMaski: FnPtr::new(loadfn("glColorMaski")),
         ColorPointerEXT: FnPtr::new(loadfn("glColorPointerEXT")),
         CompileShader: FnPtr::new(loadfn("glCompileShader")),
         CompileShaderARB: FnPtr::new(loadfn("glCompileShaderARB")),
         CompressedMultiTexImage1DEXT: FnPtr::new(loadfn("glCompressedMultiTexImage1DEXT")),
         CompressedMultiTexImage2DEXT: FnPtr::new(loadfn("glCompressedMultiTexImage2DEXT")),
         CompressedMultiTexImage3DEXT: FnPtr::new(loadfn("glCompressedMultiTexImage3DEXT")),
         CompressedMultiTexSubImage1DEXT: FnPtr::new(loadfn("glCompressedMultiTexSubImage1DEXT")),
         CompressedMultiTexSubImage2DEXT: FnPtr::new(loadfn("glCompressedMultiTexSubImage2DEXT")),
         CompressedMultiTexSubImage3DEXT: FnPtr::new(loadfn("glCompressedMultiTexSubImage3DEXT")),
         CompressedTexImage1D: FnPtr::new(loadfn("glCompressedTexImage1D")),
         CompressedTexImage1DARB: FnPtr::new(loadfn("glCompressedTexImage1DARB")),
         CompressedTexImage2D: FnPtr::new(loadfn("glCompressedTexImage2D")),
         CompressedTexImage2DARB: FnPtr::new(loadfn("glCompressedTexImage2DARB")),
         CompressedTexImage3D: FnPtr::new(loadfn("glCompressedTexImage3D")),
         CompressedTexImage3DARB: FnPtr::new(loadfn("glCompressedTexImage3DARB")),
         CompressedTexSubImage1D: FnPtr::new(loadfn("glCompressedTexSubImage1D")),
         CompressedTexSubImage1DARB: FnPtr::new(loadfn("glCompressedTexSubImage1DARB")),
         CompressedTexSubImage2D: FnPtr::new(loadfn("glCompressedTexSubImage2D")),
         CompressedTexSubImage2DARB: FnPtr::new(loadfn("glCompressedTexSubImage2DARB")),
         CompressedTexSubImage3D: FnPtr::new(loadfn("glCompressedTexSubImage3D")),
         CompressedTexSubImage3DARB: FnPtr::new(loadfn("glCompressedTexSubImage3DARB")),
         CompressedTextureImage1DEXT: FnPtr::new(loadfn("glCompressedTextureImage1DEXT")),
         CompressedTextureImage2DEXT: FnPtr::new(loadfn("glCompressedTextureImage2DEXT")),
         CompressedTextureImage3DEXT: FnPtr::new(loadfn("glCompressedTextureImage3DEXT")),
         CompressedTextureSubImage1D: FnPtr::new(loadfn("glCompressedTextureSubImage1D")),
         CompressedTextureSubImage1DEXT: FnPtr::new(loadfn("glCompressedTextureSubImage1DEXT")),
         CompressedTextureSubImage2D: FnPtr::new(loadfn("glCompressedTextureSubImage2D")),
         CompressedTextureSubImage2DEXT: FnPtr::new(loadfn("glCompressedTextureSubImage2DEXT")),
         CompressedTextureSubImage3D: FnPtr::new(loadfn("glCompressedTextureSubImage3D")),
         CompressedTextureSubImage3DEXT: FnPtr::new(loadfn("glCompressedTextureSubImage3DEXT")),
         CopyBufferSubData: FnPtr::new(loadfn("glCopyBufferSubData")),
         CopyImageSubData: FnPtr::new(loadfn("glCopyImageSubData")),
         CopyMultiTexImage1DEXT: FnPtr::new(loadfn("glCopyMultiTexImage1DEXT")),
         CopyMultiTexImage2DEXT: FnPtr::new(loadfn("glCopyMultiTexImage2DEXT")),
         CopyMultiTexSubImage1DEXT: FnPtr::new(loadfn("glCopyMultiTexSubImage1DEXT")),
         CopyMultiTexSubImage2DEXT: FnPtr::new(loadfn("glCopyMultiTexSubImage2DEXT")),
         CopyMultiTexSubImage3DEXT: FnPtr::new(loadfn("glCopyMultiTexSubImage3DEXT")),
         CopyNamedBufferSubData: FnPtr::new(loadfn("glCopyNamedBufferSubData")),
         CopyTexImage1D: FnPtr::new(loadfn("glCopyTexImage1D")),
         CopyTexImage1DEXT: FnPtr::new(loadfn("glCopyTexImage1DEXT")),
         CopyTexImage2D: FnPtr::new(loadfn("glCopyTexImage2D")),
         CopyTexImage2DEXT: FnPtr::new(loadfn("glCopyTexImage2DEXT")),
         CopyTexSubImage1D: FnPtr::new(loadfn("glCopyTexSubImage1D")),
         CopyTexSubImage1DEXT: FnPtr::new(loadfn("glCopyTexSubImage1DEXT")),
         CopyTexSubImage2D: FnPtr::new(loadfn("glCopyTexSubImage2D")),
         CopyTexSubImage2DEXT: FnPtr::new(loadfn("glCopyTexSubImage2DEXT")),
         CopyTexSubImage3D: FnPtr::new(loadfn("glCopyTexSubImage3D")),
         CopyTexSubImage3DEXT: FnPtr::new(loadfn("glCopyTexSubImage3DEXT")),
         CopyTextureImage1DEXT: FnPtr::new(loadfn("glCopyTextureImage1DEXT")),
         CopyTextureImage2DEXT: FnPtr::new(loadfn("glCopyTextureImage2DEXT")),
         CopyTextureSubImage1D: FnPtr::new(loadfn("glCopyTextureSubImage1D")),
         CopyTextureSubImage1DEXT: FnPtr::new(loadfn("glCopyTextureSubImage1DEXT")),
         CopyTextureSubImage2D: FnPtr::new(loadfn("glCopyTextureSubImage2D")),
         CopyTextureSubImage2DEXT: FnPtr::new(loadfn("glCopyTextureSubImage2DEXT")),
         CopyTextureSubImage3D: FnPtr::new(loadfn("glCopyTextureSubImage3D")),
         CopyTextureSubImage3DEXT: FnPtr::new(loadfn("glCopyTextureSubImage3DEXT")),
         CreateBuffers: FnPtr::new(loadfn("glCreateBuffers")),
         CreateFramebuffers: FnPtr::new(loadfn("glCreateFramebuffers")),
         CreateProgram: FnPtr::new(loadfn("glCreateProgram")),
         CreateProgramObjectARB: FnPtr::new(loadfn("glCreateProgramObjectARB")),
         CreateProgramPipelines: FnPtr::new(loadfn("glCreateProgramPipelines")),
         CreateQueries: FnPtr::new(loadfn("glCreateQueries")),
         CreateRenderbuffers: FnPtr::new(loadfn("glCreateRenderbuffers")),
         CreateSamplers: FnPtr::new(loadfn("glCreateSamplers")),
         CreateShader: FnPtr::new(loadfn("glCreateShader")),
         CreateShaderObjectARB: FnPtr::new(loadfn("glCreateShaderObjectARB")),
         CreateShaderProgramv: FnPtr::new(loadfn("glCreateShaderProgramv")),
         CreateTextures: FnPtr::new(loadfn("glCreateTextures")),
         CreateTransformFeedbacks: FnPtr::new(loadfn("glCreateTransformFeedbacks")),
         CreateVertexArrays: FnPtr::new(loadfn("glCreateVertexArrays")),
         CullFace: FnPtr::new(loadfn("glCullFace")),
         DebugMessageCallback: FnPtr::new(loadfn("glDebugMessageCallback")),
         DebugMessageCallbackARB: FnPtr::new(loadfn("glDebugMessageCallbackARB")),
         DebugMessageControl: FnPtr::new(loadfn("glDebugMessageControl")),
         DebugMessageControlARB: FnPtr::new(loadfn("glDebugMessageControlARB")),
         DebugMessageInsert: FnPtr::new(loadfn("glDebugMessageInsert")),
         DebugMessageInsertARB: FnPtr::new(loadfn("glDebugMessageInsertARB")),
         DeleteBuffers: FnPtr::new(loadfn("glDeleteBuffers")),
         DeleteBuffersARB: FnPtr::new(loadfn("glDeleteBuffersARB")),
         DeleteFramebuffers: FnPtr::new(loadfn("glDeleteFramebuffers")),
         DeleteFramebuffersEXT: FnPtr::new(loadfn("glDeleteFramebuffersEXT")),
         DeleteObjectARB: FnPtr::new(loadfn("glDeleteObjectARB")),
         DeleteProgram: FnPtr::new(loadfn("glDeleteProgram")),
         DeleteProgramPipelines: FnPtr::new(loadfn("glDeleteProgramPipelines")),
         DeleteProgramsARB: FnPtr::new(loadfn("glDeleteProgramsARB")),
         DeleteProgramsNV: FnPtr::new(loadfn("glDeleteProgramsNV")),
         DeleteQueries: FnPtr::new(loadfn("glDeleteQueries")),
         DeleteQueriesARB: FnPtr::new(loadfn("glDeleteQueriesARB")),
         DeleteRenderbuffers: FnPtr::new(loadfn("glDeleteRenderbuffers")),
         DeleteRenderbuffersEXT: FnPtr::new(loadfn("glDeleteRenderbuffersEXT")),
         DeleteSamplers: FnPtr::new(loadfn("glDeleteSamplers")),
         DeleteShader: FnPtr::new(loadfn("glDeleteShader")),
         DeleteSync: FnPtr::new(loadfn("glDeleteSync")),
         DeleteTextures: FnPtr::new(loadfn("glDeleteTextures")),
         DeleteTexturesEXT: FnPtr::new(loadfn("glDeleteTexturesEXT")),
         DeleteTransformFeedbacks: FnPtr::new(loadfn("glDeleteTransformFeedbacks")),
         DeleteTransformFeedbacksNV: FnPtr::new(loadfn("glDeleteTransformFeedbacksNV")),
         DeleteVertexArrays: FnPtr::new(loadfn("glDeleteVertexArrays")),
         DeleteVertexArraysAPPLE: FnPtr::new(loadfn("glDeleteVertexArraysAPPLE")),
         DepthFunc: FnPtr::new(loadfn("glDepthFunc")),
         DepthMask: FnPtr::new(loadfn("glDepthMask")),
         DepthRange: FnPtr::new(loadfn("glDepthRange")),
         DepthRangeArraydvNV: FnPtr::new(loadfn("glDepthRangeArraydvNV")),
         DepthRangeArrayv: FnPtr::new(loadfn("glDepthRangeArrayv")),
         DepthRangeIndexed: FnPtr::new(loadfn("glDepthRangeIndexed")),
         DepthRangeIndexeddNV: FnPtr::new(loadfn("glDepthRangeIndexeddNV")),
         DepthRangef: FnPtr::new(loadfn("glDepthRangef")),
         DepthRangefOES: FnPtr::new(loadfn("glDepthRangefOES")),
         DetachObjectARB: FnPtr::new(loadfn("glDetachObjectARB")),
         DetachShader: FnPtr::new(loadfn("glDetachShader")),
         Disable: FnPtr::new(loadfn("glDisable")),
         DisableClientStateIndexedEXT: FnPtr::new(loadfn("glDisableClientStateIndexedEXT")),
         DisableClientStateiEXT: FnPtr::new(loadfn("glDisableClientStateiEXT")),
         DisableIndexedEXT: FnPtr::new(loadfn("glDisableIndexedEXT")),
         DisableVertexArrayAttrib: FnPtr::new(loadfn("glDisableVertexArrayAttrib")),
         DisableVertexArrayAttribEXT: FnPtr::new(loadfn("glDisableVertexArrayAttribEXT")),
         DisableVertexArrayEXT: FnPtr::new(loadfn("glDisableVertexArrayEXT")),
         DisableVertexAttribArray: FnPtr::new(loadfn("glDisableVertexAttribArray")),
         DisableVertexAttribArrayARB: FnPtr::new(loadfn("glDisableVertexAttribArrayARB")),
         Disablei: FnPtr::new(loadfn("glDisablei")),
         DispatchCompute: FnPtr::new(loadfn("glDispatchCompute")),
         DispatchComputeIndirect: FnPtr::new(loadfn("glDispatchComputeIndirect")),
         DrawArrays: FnPtr::new(loadfn("glDrawArrays")),
         DrawArraysEXT: FnPtr::new(loadfn("glDrawArraysEXT")),
         DrawArraysIndirect: FnPtr::new(loadfn("glDrawArraysIndirect")),
         DrawArraysInstanced: FnPtr::new(loadfn("glDrawArraysInstanced")),
         DrawArraysInstancedARB: FnPtr::new(loadfn("glDrawArraysInstancedARB")),
         DrawArraysInstancedBaseInstance: FnPtr::new(loadfn("glDrawArraysInstancedBaseInstance")),
         DrawArraysInstancedEXT: FnPtr::new(loadfn("glDrawArraysInstancedEXT")),
         DrawBuffer: FnPtr::new(loadfn("glDrawBuffer")),
         DrawBuffers: FnPtr::new(loadfn("glDrawBuffers")),
         DrawBuffersARB: FnPtr::new(loadfn("glDrawBuffersARB")),
         DrawBuffersATI: FnPtr::new(loadfn("glDrawBuffersATI")),
         DrawElements: FnPtr::new(loadfn("glDrawElements")),
         DrawElementsBaseVertex: FnPtr::new(loadfn("glDrawElementsBaseVertex")),
         DrawElementsIndirect: FnPtr::new(loadfn("glDrawElementsIndirect")),
         DrawElementsInstanced: FnPtr::new(loadfn("glDrawElementsInstanced")),
         DrawElementsInstancedARB: FnPtr::new(loadfn("glDrawElementsInstancedARB")),
         DrawElementsInstancedBaseInstance: FnPtr::new(loadfn("glDrawElementsInstancedBaseInstance")),
         DrawElementsInstancedBaseVertex: FnPtr::new(loadfn("glDrawElementsInstancedBaseVertex")),
         DrawElementsInstancedBaseVertexBaseInstance: FnPtr::new(loadfn("glDrawElementsInstancedBaseVertexBaseInstance")),
         DrawElementsInstancedEXT: FnPtr::new(loadfn("glDrawElementsInstancedEXT")),
         DrawRangeElements: FnPtr::new(loadfn("glDrawRangeElements")),
         DrawRangeElementsBaseVertex: FnPtr::new(loadfn("glDrawRangeElementsBaseVertex")),
         DrawRangeElementsEXT: FnPtr::new(loadfn("glDrawRangeElementsEXT")),
         DrawTransformFeedback: FnPtr::new(loadfn("glDrawTransformFeedback")),
         DrawTransformFeedbackInstanced: FnPtr::new(loadfn("glDrawTransformFeedbackInstanced")),
         DrawTransformFeedbackNV: FnPtr::new(loadfn("glDrawTransformFeedbackNV")),
         DrawTransformFeedbackStream: FnPtr::new(loadfn("glDrawTransformFeedbackStream")),
         DrawTransformFeedbackStreamInstanced: FnPtr::new(loadfn("glDrawTransformFeedbackStreamInstanced")),
         EdgeFlagPointerEXT: FnPtr::new(loadfn("glEdgeFlagPointerEXT")),
         Enable: FnPtr::new(loadfn("glEnable")),
         EnableClientStateIndexedEXT: FnPtr::new(loadfn("glEnableClientStateIndexedEXT")),
         EnableClientStateiEXT: FnPtr::new(loadfn("glEnableClientStateiEXT")),
         EnableIndexedEXT: FnPtr::new(loadfn("glEnableIndexedEXT")),
         EnableVertexArrayAttrib: FnPtr::new(loadfn("glEnableVertexArrayAttrib")),
         EnableVertexArrayAttribEXT: FnPtr::new(loadfn("glEnableVertexArrayAttribEXT")),
         EnableVertexArrayEXT: FnPtr::new(loadfn("glEnableVertexArrayEXT")),
         EnableVertexAttribArray: FnPtr::new(loadfn("glEnableVertexAttribArray")),
         EnableVertexAttribArrayARB: FnPtr::new(loadfn("glEnableVertexAttribArrayARB")),
         Enablei: FnPtr::new(loadfn("glEnablei")),
         EndConditionalRender: FnPtr::new(loadfn("glEndConditionalRender")),
         EndConditionalRenderNV: FnPtr::new(loadfn("glEndConditionalRenderNV")),
         EndConditionalRenderNVX: FnPtr::new(loadfn("glEndConditionalRenderNVX")),
         EndQuery: FnPtr::new(loadfn("glEndQuery")),
         EndQueryARB: FnPtr::new(loadfn("glEndQueryARB")),
         EndQueryIndexed: FnPtr::new(loadfn("glEndQueryIndexed")),
         EndTransformFeedback: FnPtr::new(loadfn("glEndTransformFeedback")),
         EndTransformFeedbackEXT: FnPtr::new(loadfn("glEndTransformFeedbackEXT")),
         EndTransformFeedbackNV: FnPtr::new(loadfn("glEndTransformFeedbackNV")),
         ExecuteProgramNV: FnPtr::new(loadfn("glExecuteProgramNV")),
         FenceSync: FnPtr::new(loadfn("glFenceSync")),
         Finish: FnPtr::new(loadfn("glFinish")),
         Flush: FnPtr::new(loadfn("glFlush")),
         FlushMappedBufferRange: FnPtr::new(loadfn("glFlushMappedBufferRange")),
         FlushMappedBufferRangeAPPLE: FnPtr::new(loadfn("glFlushMappedBufferRangeAPPLE")),
         FlushMappedNamedBufferRange: FnPtr::new(loadfn("glFlushMappedNamedBufferRange")),
         FlushMappedNamedBufferRangeEXT: FnPtr::new(loadfn("glFlushMappedNamedBufferRangeEXT")),
         FramebufferDrawBufferEXT: FnPtr::new(loadfn("glFramebufferDrawBufferEXT")),
         FramebufferDrawBuffersEXT: FnPtr::new(loadfn("glFramebufferDrawBuffersEXT")),
         FramebufferParameteri: FnPtr::new(loadfn("glFramebufferParameteri")),
         FramebufferReadBufferEXT: FnPtr::new(loadfn("glFramebufferReadBufferEXT")),
         FramebufferRenderbuffer: FnPtr::new(loadfn("glFramebufferRenderbuffer")),
         FramebufferRenderbufferEXT: FnPtr::new(loadfn("glFramebufferRenderbufferEXT")),
         FramebufferTexture: FnPtr::new(loadfn("glFramebufferTexture")),
         FramebufferTexture1D: FnPtr::new(loadfn("glFramebufferTexture1D")),
         FramebufferTexture1DEXT: FnPtr::new(loadfn("glFramebufferTexture1DEXT")),
         FramebufferTexture2D: FnPtr::new(loadfn("glFramebufferTexture2D")),
         FramebufferTexture2DEXT: FnPtr::new(loadfn("glFramebufferTexture2DEXT")),
         FramebufferTexture3D: FnPtr::new(loadfn("glFramebufferTexture3D")),
         FramebufferTexture3DEXT: FnPtr::new(loadfn("glFramebufferTexture3DEXT")),
         FramebufferTextureARB: FnPtr::new(loadfn("glFramebufferTextureARB")),
         FramebufferTextureEXT: FnPtr::new(loadfn("glFramebufferTextureEXT")),
         FramebufferTextureFaceARB: FnPtr::new(loadfn("glFramebufferTextureFaceARB")),
         FramebufferTextureFaceEXT: FnPtr::new(loadfn("glFramebufferTextureFaceEXT")),
         FramebufferTextureLayer: FnPtr::new(loadfn("glFramebufferTextureLayer")),
         FramebufferTextureLayerARB: FnPtr::new(loadfn("glFramebufferTextureLayerARB")),
         FramebufferTextureLayerEXT: FnPtr::new(loadfn("glFramebufferTextureLayerEXT")),
         FrontFace: FnPtr::new(loadfn("glFrontFace")),
         FrustumfOES: FnPtr::new(loadfn("glFrustumfOES")),
         GenBuffers: FnPtr::new(loadfn("glGenBuffers")),
         GenBuffersARB: FnPtr::new(loadfn("glGenBuffersARB")),
         GenFramebuffers: FnPtr::new(loadfn("glGenFramebuffers")),
         GenFramebuffersEXT: FnPtr::new(loadfn("glGenFramebuffersEXT")),
         GenProgramPipelines: FnPtr::new(loadfn("glGenProgramPipelines")),
         GenProgramsARB: FnPtr::new(loadfn("glGenProgramsARB")),
         GenProgramsNV: FnPtr::new(loadfn("glGenProgramsNV")),
         GenQueries: FnPtr::new(loadfn("glGenQueries")),
         GenQueriesARB: FnPtr::new(loadfn("glGenQueriesARB")),
         GenRenderbuffers: FnPtr::new(loadfn("glGenRenderbuffers")),
         GenRenderbuffersEXT: FnPtr::new(loadfn("glGenRenderbuffersEXT")),
         GenSamplers: FnPtr::new(loadfn("glGenSamplers")),
         GenTextures: FnPtr::new(loadfn("glGenTextures")),
         GenTexturesEXT: FnPtr::new(loadfn("glGenTexturesEXT")),
         GenTransformFeedbacks: FnPtr::new(loadfn("glGenTransformFeedbacks")),
         GenTransformFeedbacksNV: FnPtr::new(loadfn("glGenTransformFeedbacksNV")),
         GenVertexArrays: FnPtr::new(loadfn("glGenVertexArrays")),
         GenVertexArraysAPPLE: FnPtr::new(loadfn("glGenVertexArraysAPPLE")),
         GenerateMipmap: FnPtr::new(loadfn("glGenerateMipmap")),
         GenerateMipmapEXT: FnPtr::new(loadfn("glGenerateMipmapEXT")),
         GenerateMultiTexMipmapEXT: FnPtr::new(loadfn("glGenerateMultiTexMipmapEXT")),
         GenerateTextureMipmap: FnPtr::new(loadfn("glGenerateTextureMipmap")),
         GenerateTextureMipmapEXT: FnPtr::new(loadfn("glGenerateTextureMipmapEXT")),
         GetActiveAtomicCounterBufferiv: FnPtr::new(loadfn("glGetActiveAtomicCounterBufferiv")),
         GetActiveAttrib: FnPtr::new(loadfn("glGetActiveAttrib")),
         GetActiveAttribARB: FnPtr::new(loadfn("glGetActiveAttribARB")),
         GetActiveSubroutineName: FnPtr::new(loadfn("glGetActiveSubroutineName")),
         GetActiveSubroutineUniformName: FnPtr::new(loadfn("glGetActiveSubroutineUniformName")),
         GetActiveSubroutineUniformiv: FnPtr::new(loadfn("glGetActiveSubroutineUniformiv")),
         GetActiveUniform: FnPtr::new(loadfn("glGetActiveUniform")),
         GetActiveUniformARB: FnPtr::new(loadfn("glGetActiveUniformARB")),
         GetActiveUniformBlockName: FnPtr::new(loadfn("glGetActiveUniformBlockName")),
         GetActiveUniformBlockiv: FnPtr::new(loadfn("glGetActiveUniformBlockiv")),
         GetActiveUniformName: FnPtr::new(loadfn("glGetActiveUniformName")),
         GetActiveUniformsiv: FnPtr::new(loadfn("glGetActiveUniformsiv")),
         GetActiveVaryingNV: FnPtr::new(loadfn("glGetActiveVaryingNV")),
         GetAttachedObjectsARB: FnPtr::new(loadfn("glGetAttachedObjectsARB")),
         GetAttachedShaders: FnPtr::new(loadfn("glGetAttachedShaders")),
         GetAttribLocation: FnPtr::new(loadfn("glGetAttribLocation")),
         GetAttribLocationARB: FnPtr::new(loadfn("glGetAttribLocationARB")),
         GetBooleanIndexedvEXT: FnPtr::new(loadfn("glGetBooleanIndexedvEXT")),
         GetBooleani_v: FnPtr::new(loadfn("glGetBooleani_v")),
         GetBooleanv: FnPtr::new(loadfn("glGetBooleanv")),
         GetBufferParameteri64v: FnPtr::new(loadfn("glGetBufferParameteri64v")),
         GetBufferParameteriv: FnPtr::new(loadfn("glGetBufferParameteriv")),
         GetBufferParameterivARB: FnPtr::new(loadfn("glGetBufferParameterivARB")),
         GetBufferPointerv: FnPtr::new(loadfn("glGetBufferPointerv")),
         GetBufferPointervARB: FnPtr::new(loadfn("glGetBufferPointervARB")),
         GetBufferSubData: FnPtr::new(loadfn("glGetBufferSubData")),
         GetBufferSubDataARB: FnPtr::new(loadfn("glGetBufferSubDataARB")),
         GetClipPlanefOES: FnPtr::new(loadfn("glGetClipPlanefOES")),
         GetCompressedMultiTexImageEXT: FnPtr::new(loadfn("glGetCompressedMultiTexImageEXT")),
         GetCompressedTexImage: FnPtr::new(loadfn("glGetCompressedTexImage")),
         GetCompressedTexImageARB: FnPtr::new(loadfn("glGetCompressedTexImageARB")),
         GetCompressedTextureImage: FnPtr::new(loadfn("glGetCompressedTextureImage")),
         GetCompressedTextureImageEXT: FnPtr::new(loadfn("glGetCompressedTextureImageEXT")),
         GetCompressedTextureSubImage: FnPtr::new(loadfn("glGetCompressedTextureSubImage")),
         GetDebugMessageLog: FnPtr::new(loadfn("glGetDebugMessageLog")),
         GetDebugMessageLogARB: FnPtr::new(loadfn("glGetDebugMessageLogARB")),
         GetDoubleIndexedvEXT: FnPtr::new(loadfn("glGetDoubleIndexedvEXT")),
         GetDoublei_v: FnPtr::new(loadfn("glGetDoublei_v")),
         GetDoublei_vEXT: FnPtr::new(loadfn("glGetDoublei_vEXT")),
         GetDoublev: FnPtr::new(loadfn("glGetDoublev")),
         GetError: FnPtr::new(loadfn("glGetError")),
         GetFloatIndexedvEXT: FnPtr::new(loadfn("glGetFloatIndexedvEXT")),
         GetFloati_v: FnPtr::new(loadfn("glGetFloati_v")),
         GetFloati_vEXT: FnPtr::new(loadfn("glGetFloati_vEXT")),
         GetFloatv: FnPtr::new(loadfn("glGetFloatv")),
         GetFragDataIndex: FnPtr::new(loadfn("glGetFragDataIndex")),
         GetFragDataLocation: FnPtr::new(loadfn("glGetFragDataLocation")),
         GetFragDataLocationEXT: FnPtr::new(loadfn("glGetFragDataLocationEXT")),
         GetFramebufferAttachmentParameteriv: FnPtr::new(loadfn("glGetFramebufferAttachmentParameteriv")),
         GetFramebufferAttachmentParameterivEXT: FnPtr::new(loadfn("glGetFramebufferAttachmentParameterivEXT")),
         GetFramebufferParameteriv: FnPtr::new(loadfn("glGetFramebufferParameteriv")),
         GetFramebufferParameterivEXT: FnPtr::new(loadfn("glGetFramebufferParameterivEXT")),
         GetGraphicsResetStatus: FnPtr::new(loadfn("glGetGraphicsResetStatus")),
         GetGraphicsResetStatusARB: FnPtr::new(loadfn("glGetGraphicsResetStatusARB")),
         GetHandleARB: FnPtr::new(loadfn("glGetHandleARB")),
         GetInfoLogARB: FnPtr::new(loadfn("glGetInfoLogARB")),
         GetInteger64i_v: FnPtr::new(loadfn("glGetInteger64i_v")),
         GetInteger64v: FnPtr::new(loadfn("glGetInteger64v")),
         GetIntegerIndexedvEXT: FnPtr::new(loadfn("glGetIntegerIndexedvEXT")),
         GetIntegeri_v: FnPtr::new(loadfn("glGetIntegeri_v")),
         GetIntegerv: FnPtr::new(loadfn("glGetIntegerv")),
         GetInternalformati64v: FnPtr::new(loadfn("glGetInternalformati64v")),
         GetInternalformativ: FnPtr::new(loadfn("glGetInternalformativ")),
         GetMultiTexEnvfvEXT: FnPtr::new(loadfn("glGetMultiTexEnvfvEXT")),
         GetMultiTexEnvivEXT: FnPtr::new(loadfn("glGetMultiTexEnvivEXT")),
         GetMultiTexGendvEXT: FnPtr::new(loadfn("glGetMultiTexGendvEXT")),
         GetMultiTexGenfvEXT: FnPtr::new(loadfn("glGetMultiTexGenfvEXT")),
         GetMultiTexGenivEXT: FnPtr::new(loadfn("glGetMultiTexGenivEXT")),
         GetMultiTexImageEXT: FnPtr::new(loadfn("glGetMultiTexImageEXT")),
         GetMultiTexLevelParameterfvEXT: FnPtr::new(loadfn("glGetMultiTexLevelParameterfvEXT")),
         GetMultiTexLevelParameterivEXT: FnPtr::new(loadfn("glGetMultiTexLevelParameterivEXT")),
         GetMultiTexParameterIivEXT: FnPtr::new(loadfn("glGetMultiTexParameterIivEXT")),
         GetMultiTexParameterIuivEXT: FnPtr::new(loadfn("glGetMultiTexParameterIuivEXT")),
         GetMultiTexParameterfvEXT: FnPtr::new(loadfn("glGetMultiTexParameterfvEXT")),
         GetMultiTexParameterivEXT: FnPtr::new(loadfn("glGetMultiTexParameterivEXT")),
         GetMultisamplefv: FnPtr::new(loadfn("glGetMultisamplefv")),
         GetMultisamplefvNV: FnPtr::new(loadfn("glGetMultisamplefvNV")),
         GetNamedBufferParameteri64v: FnPtr::new(loadfn("glGetNamedBufferParameteri64v")),
         GetNamedBufferParameteriv: FnPtr::new(loadfn("glGetNamedBufferParameteriv")),
         GetNamedBufferParameterivEXT: FnPtr::new(loadfn("glGetNamedBufferParameterivEXT")),
         GetNamedBufferPointerv: FnPtr::new(loadfn("glGetNamedBufferPointerv")),
         GetNamedBufferPointervEXT: FnPtr::new(loadfn("glGetNamedBufferPointervEXT")),
         GetNamedBufferSubData: FnPtr::new(loadfn("glGetNamedBufferSubData")),
         GetNamedBufferSubDataEXT: FnPtr::new(loadfn("glGetNamedBufferSubDataEXT")),
         GetNamedFramebufferAttachmentParameteriv: FnPtr::new(loadfn("glGetNamedFramebufferAttachmentParameteriv")),
         GetNamedFramebufferAttachmentParameterivEXT: FnPtr::new(loadfn("glGetNamedFramebufferAttachmentParameterivEXT")),
         GetNamedFramebufferParameteriv: FnPtr::new(loadfn("glGetNamedFramebufferParameteriv")),
         GetNamedFramebufferParameterivEXT: FnPtr::new(loadfn("glGetNamedFramebufferParameterivEXT")),
         GetNamedProgramLocalParameterIivEXT: FnPtr::new(loadfn("glGetNamedProgramLocalParameterIivEXT")),
         GetNamedProgramLocalParameterIuivEXT: FnPtr::new(loadfn("glGetNamedProgramLocalParameterIuivEXT")),
         GetNamedProgramLocalParameterdvEXT: FnPtr::new(loadfn("glGetNamedProgramLocalParameterdvEXT")),
         GetNamedProgramLocalParameterfvEXT: FnPtr::new(loadfn("glGetNamedProgramLocalParameterfvEXT")),
         GetNamedProgramStringEXT: FnPtr::new(loadfn("glGetNamedProgramStringEXT")),
         GetNamedProgramivEXT: FnPtr::new(loadfn("glGetNamedProgramivEXT")),
         GetNamedRenderbufferParameteriv: FnPtr::new(loadfn("glGetNamedRenderbufferParameteriv")),
         GetNamedRenderbufferParameterivEXT: FnPtr::new(loadfn("glGetNamedRenderbufferParameterivEXT")),
         GetObjectLabel: FnPtr::new(loadfn("glGetObjectLabel")),
         GetObjectParameterfvARB: FnPtr::new(loadfn("glGetObjectParameterfvARB")),
         GetObjectParameterivARB: FnPtr::new(loadfn("glGetObjectParameterivARB")),
         GetObjectPtrLabel: FnPtr::new(loadfn("glGetObjectPtrLabel")),
         GetPointerIndexedvEXT: FnPtr::new(loadfn("glGetPointerIndexedvEXT")),
         GetPointeri_vEXT: FnPtr::new(loadfn("glGetPointeri_vEXT")),
         GetPointerv: FnPtr::new(loadfn("glGetPointerv")),
         GetPointervEXT: FnPtr::new(loadfn("glGetPointervEXT")),
         GetProgramBinary: FnPtr::new(loadfn("glGetProgramBinary")),
         GetProgramEnvParameterdvARB: FnPtr::new(loadfn("glGetProgramEnvParameterdvARB")),
         GetProgramEnvParameterfvARB: FnPtr::new(loadfn("glGetProgramEnvParameterfvARB")),
         GetProgramInfoLog: FnPtr::new(loadfn("glGetProgramInfoLog")),
         GetProgramInterfaceiv: FnPtr::new(loadfn("glGetProgramInterfaceiv")),
         GetProgramLocalParameterdvARB: FnPtr::new(loadfn("glGetProgramLocalParameterdvARB")),
         GetProgramLocalParameterfvARB: FnPtr::new(loadfn("glGetProgramLocalParameterfvARB")),
         GetProgramParameterdvNV: FnPtr::new(loadfn("glGetProgramParameterdvNV")),
         GetProgramParameterfvNV: FnPtr::new(loadfn("glGetProgramParameterfvNV")),
         GetProgramPipelineInfoLog: FnPtr::new(loadfn("glGetProgramPipelineInfoLog")),
         GetProgramPipelineiv: FnPtr::new(loadfn("glGetProgramPipelineiv")),
         GetProgramResourceIndex: FnPtr::new(loadfn("glGetProgramResourceIndex")),
         GetProgramResourceLocation: FnPtr::new(loadfn("glGetProgramResourceLocation")),
         GetProgramResourceLocationIndex: FnPtr::new(loadfn("glGetProgramResourceLocationIndex")),
         GetProgramResourceName: FnPtr::new(loadfn("glGetProgramResourceName")),
         GetProgramResourceiv: FnPtr::new(loadfn("glGetProgramResourceiv")),
         GetProgramStageiv: FnPtr::new(loadfn("glGetProgramStageiv")),
         GetProgramStringARB: FnPtr::new(loadfn("glGetProgramStringARB")),
         GetProgramStringNV: FnPtr::new(loadfn("glGetProgramStringNV")),
         GetProgramiv: FnPtr::new(loadfn("glGetProgramiv")),
         GetProgramivARB: FnPtr::new(loadfn("glGetProgramivARB")),
         GetProgramivNV: FnPtr::new(loadfn("glGetProgramivNV")),
         GetQueryBufferObjecti64v: FnPtr::new(loadfn("glGetQueryBufferObjecti64v")),
         GetQueryBufferObjectiv: FnPtr::new(loadfn("glGetQueryBufferObjectiv")),
         GetQueryBufferObjectui64v: FnPtr::new(loadfn("glGetQueryBufferObjectui64v")),
         GetQueryBufferObjectuiv: FnPtr::new(loadfn("glGetQueryBufferObjectuiv")),
         GetQueryIndexediv: FnPtr::new(loadfn("glGetQueryIndexediv")),
         GetQueryObjecti64v: FnPtr::new(loadfn("glGetQueryObjecti64v")),
         GetQueryObjecti64vEXT: FnPtr::new(loadfn("glGetQueryObjecti64vEXT")),
         GetQueryObjectiv: FnPtr::new(loadfn("glGetQueryObjectiv")),
         GetQueryObjectivARB: FnPtr::new(loadfn("glGetQueryObjectivARB")),
         GetQueryObjectui64v: FnPtr::new(loadfn("glGetQueryObjectui64v")),
         GetQueryObjectui64vEXT: FnPtr::new(loadfn("glGetQueryObjectui64vEXT")),
         GetQueryObjectuiv: FnPtr::new(loadfn("glGetQueryObjectuiv")),
         GetQueryObjectuivARB: FnPtr::new(loadfn("glGetQueryObjectuivARB")),
         GetQueryiv: FnPtr::new(loadfn("glGetQueryiv")),
         GetQueryivARB: FnPtr::new(loadfn("glGetQueryivARB")),
         GetRenderbufferParameteriv: FnPtr::new(loadfn("glGetRenderbufferParameteriv")),
         GetRenderbufferParameterivEXT: FnPtr::new(loadfn("glGetRenderbufferParameterivEXT")),
         GetSamplerParameterIiv: FnPtr::new(loadfn("glGetSamplerParameterIiv")),
         GetSamplerParameterIuiv: FnPtr::new(loadfn("glGetSamplerParameterIuiv")),
         GetSamplerParameterfv: FnPtr::new(loadfn("glGetSamplerParameterfv")),
         GetSamplerParameteriv: FnPtr::new(loadfn("glGetSamplerParameteriv")),
         GetShaderInfoLog: FnPtr::new(loadfn("glGetShaderInfoLog")),
         GetShaderPrecisionFormat: FnPtr::new(loadfn("glGetShaderPrecisionFormat")),
         GetShaderSource: FnPtr::new(loadfn("glGetShaderSource")),
         GetShaderSourceARB: FnPtr::new(loadfn("glGetShaderSourceARB")),
         GetShaderiv: FnPtr::new(loadfn("glGetShaderiv")),
         GetString: FnPtr::new(loadfn("glGetString")),
         GetStringi: FnPtr::new(loadfn("glGetStringi")),
         GetSubroutineIndex: FnPtr::new(loadfn("glGetSubroutineIndex")),
         GetSubroutineUniformLocation: FnPtr::new(loadfn("glGetSubroutineUniformLocation")),
         GetSynciv: FnPtr::new(loadfn("glGetSynciv")),
         GetTexImage: FnPtr::new(loadfn("glGetTexImage")),
         GetTexLevelParameterfv: FnPtr::new(loadfn("glGetTexLevelParameterfv")),
         GetTexLevelParameteriv: FnPtr::new(loadfn("glGetTexLevelParameteriv")),
         GetTexParameterIiv: FnPtr::new(loadfn("glGetTexParameterIiv")),
         GetTexParameterIivEXT: FnPtr::new(loadfn("glGetTexParameterIivEXT")),
         GetTexParameterIuiv: FnPtr::new(loadfn("glGetTexParameterIuiv")),
         GetTexParameterIuivEXT: FnPtr::new(loadfn("glGetTexParameterIuivEXT")),
         GetTexParameterfv: FnPtr::new(loadfn("glGetTexParameterfv")),
         GetTexParameteriv: FnPtr::new(loadfn("glGetTexParameteriv")),
         GetTextureImage: FnPtr::new(loadfn("glGetTextureImage")),
         GetTextureImageEXT: FnPtr::new(loadfn("glGetTextureImageEXT")),
         GetTextureLevelParameterfv: FnPtr::new(loadfn("glGetTextureLevelParameterfv")),
         GetTextureLevelParameterfvEXT: FnPtr::new(loadfn("glGetTextureLevelParameterfvEXT")),
         GetTextureLevelParameteriv: FnPtr::new(loadfn("glGetTextureLevelParameteriv")),
         GetTextureLevelParameterivEXT: FnPtr::new(loadfn("glGetTextureLevelParameterivEXT")),
         GetTextureParameterIiv: FnPtr::new(loadfn("glGetTextureParameterIiv")),
         GetTextureParameterIivEXT: FnPtr::new(loadfn("glGetTextureParameterIivEXT")),
         GetTextureParameterIuiv: FnPtr::new(loadfn("glGetTextureParameterIuiv")),
         GetTextureParameterIuivEXT: FnPtr::new(loadfn("glGetTextureParameterIuivEXT")),
         GetTextureParameterfv: FnPtr::new(loadfn("glGetTextureParameterfv")),
         GetTextureParameterfvEXT: FnPtr::new(loadfn("glGetTextureParameterfvEXT")),
         GetTextureParameteriv: FnPtr::new(loadfn("glGetTextureParameteriv")),
         GetTextureParameterivEXT: FnPtr::new(loadfn("glGetTextureParameterivEXT")),
         GetTextureSubImage: FnPtr::new(loadfn("glGetTextureSubImage")),
         GetTrackMatrixivNV: FnPtr::new(loadfn("glGetTrackMatrixivNV")),
         GetTransformFeedbackVarying: FnPtr::new(loadfn("glGetTransformFeedbackVarying")),
         GetTransformFeedbackVaryingEXT: FnPtr::new(loadfn("glGetTransformFeedbackVaryingEXT")),
         GetTransformFeedbackVaryingNV: FnPtr::new(loadfn("glGetTransformFeedbackVaryingNV")),
         GetTransformFeedbacki64_v: FnPtr::new(loadfn("glGetTransformFeedbacki64_v")),
         GetTransformFeedbacki_v: FnPtr::new(loadfn("glGetTransformFeedbacki_v")),
         GetTransformFeedbackiv: FnPtr::new(loadfn("glGetTransformFeedbackiv")),
         GetUniformBlockIndex: FnPtr::new(loadfn("glGetUniformBlockIndex")),
         GetUniformIndices: FnPtr::new(loadfn("glGetUniformIndices")),
         GetUniformLocation: FnPtr::new(loadfn("glGetUniformLocation")),
         GetUniformLocationARB: FnPtr::new(loadfn("glGetUniformLocationARB")),
         GetUniformSubroutineuiv: FnPtr::new(loadfn("glGetUniformSubroutineuiv")),
         GetUniformdv: FnPtr::new(loadfn("glGetUniformdv")),
         GetUniformfv: FnPtr::new(loadfn("glGetUniformfv")),
         GetUniformfvARB: FnPtr::new(loadfn("glGetUniformfvARB")),
         GetUniformiv: FnPtr::new(loadfn("glGetUniformiv")),
         GetUniformivARB: FnPtr::new(loadfn("glGetUniformivARB")),
         GetUniformuiv: FnPtr::new(loadfn("glGetUniformuiv")),
         GetUniformuivEXT: FnPtr::new(loadfn("glGetUniformuivEXT")),
         GetVaryingLocationNV: FnPtr::new(loadfn("glGetVaryingLocationNV")),
         GetVertexArrayIndexed64iv: FnPtr::new(loadfn("glGetVertexArrayIndexed64iv")),
         GetVertexArrayIndexediv: FnPtr::new(loadfn("glGetVertexArrayIndexediv")),
         GetVertexArrayIntegeri_vEXT: FnPtr::new(loadfn("glGetVertexArrayIntegeri_vEXT")),
         GetVertexArrayIntegervEXT: FnPtr::new(loadfn("glGetVertexArrayIntegervEXT")),
         GetVertexArrayPointeri_vEXT: FnPtr::new(loadfn("glGetVertexArrayPointeri_vEXT")),
         GetVertexArrayPointervEXT: FnPtr::new(loadfn("glGetVertexArrayPointervEXT")),
         GetVertexArrayiv: FnPtr::new(loadfn("glGetVertexArrayiv")),
         GetVertexAttribIiv: FnPtr::new(loadfn("glGetVertexAttribIiv")),
         GetVertexAttribIivEXT: FnPtr::new(loadfn("glGetVertexAttribIivEXT")),
         GetVertexAttribIuiv: FnPtr::new(loadfn("glGetVertexAttribIuiv")),
         GetVertexAttribIuivEXT: FnPtr::new(loadfn("glGetVertexAttribIuivEXT")),
         GetVertexAttribLdv: FnPtr::new(loadfn("glGetVertexAttribLdv")),
         GetVertexAttribLdvEXT: FnPtr::new(loadfn("glGetVertexAttribLdvEXT")),
         GetVertexAttribPointerv: FnPtr::new(loadfn("glGetVertexAttribPointerv")),
         GetVertexAttribPointervARB: FnPtr::new(loadfn("glGetVertexAttribPointervARB")),
         GetVertexAttribPointervNV: FnPtr::new(loadfn("glGetVertexAttribPointervNV")),
         GetVertexAttribdv: FnPtr::new(loadfn("glGetVertexAttribdv")),
         GetVertexAttribdvARB: FnPtr::new(loadfn("glGetVertexAttribdvARB")),
         GetVertexAttribdvNV: FnPtr::new(loadfn("glGetVertexAttribdvNV")),
         GetVertexAttribfv: FnPtr::new(loadfn("glGetVertexAttribfv")),
         GetVertexAttribfvARB: FnPtr::new(loadfn("glGetVertexAttribfvARB")),
         GetVertexAttribfvNV: FnPtr::new(loadfn("glGetVertexAttribfvNV")),
         GetVertexAttribiv: FnPtr::new(loadfn("glGetVertexAttribiv")),
         GetVertexAttribivARB: FnPtr::new(loadfn("glGetVertexAttribivARB")),
         GetVertexAttribivNV: FnPtr::new(loadfn("glGetVertexAttribivNV")),
         GetnCompressedTexImage: FnPtr::new(loadfn("glGetnCompressedTexImage")),
         GetnCompressedTexImageARB: FnPtr::new(loadfn("glGetnCompressedTexImageARB")),
         GetnTexImage: FnPtr::new(loadfn("glGetnTexImage")),
         GetnTexImageARB: FnPtr::new(loadfn("glGetnTexImageARB")),
         GetnUniformdv: FnPtr::new(loadfn("glGetnUniformdv")),
         GetnUniformdvARB: FnPtr::new(loadfn("glGetnUniformdvARB")),
         GetnUniformfv: FnPtr::new(loadfn("glGetnUniformfv")),
         GetnUniformfvARB: FnPtr::new(loadfn("glGetnUniformfvARB")),
         GetnUniformiv: FnPtr::new(loadfn("glGetnUniformiv")),
         GetnUniformivARB: FnPtr::new(loadfn("glGetnUniformivARB")),
         GetnUniformuiv: FnPtr::new(loadfn("glGetnUniformuiv")),
         GetnUniformuivARB: FnPtr::new(loadfn("glGetnUniformuivARB")),
         Hint: FnPtr::new(loadfn("glHint")),
         IndexPointerEXT: FnPtr::new(loadfn("glIndexPointerEXT")),
         InvalidateBufferData: FnPtr::new(loadfn("glInvalidateBufferData")),
         InvalidateBufferSubData: FnPtr::new(loadfn("glInvalidateBufferSubData")),
         InvalidateFramebuffer: FnPtr::new(loadfn("glInvalidateFramebuffer")),
         InvalidateNamedFramebufferData: FnPtr::new(loadfn("glInvalidateNamedFramebufferData")),
         InvalidateNamedFramebufferSubData: FnPtr::new(loadfn("glInvalidateNamedFramebufferSubData")),
         InvalidateSubFramebuffer: FnPtr::new(loadfn("glInvalidateSubFramebuffer")),
         InvalidateTexImage: FnPtr::new(loadfn("glInvalidateTexImage")),
         InvalidateTexSubImage: FnPtr::new(loadfn("glInvalidateTexSubImage")),
         IsBuffer: FnPtr::new(loadfn("glIsBuffer")),
         IsBufferARB: FnPtr::new(loadfn("glIsBufferARB")),
         IsEnabled: FnPtr::new(loadfn("glIsEnabled")),
         IsEnabledIndexedEXT: FnPtr::new(loadfn("glIsEnabledIndexedEXT")),
         IsEnabledi: FnPtr::new(loadfn("glIsEnabledi")),
         IsFramebuffer: FnPtr::new(loadfn("glIsFramebuffer")),
         IsFramebufferEXT: FnPtr::new(loadfn("glIsFramebufferEXT")),
         IsProgram: FnPtr::new(loadfn("glIsProgram")),
         IsProgramARB: FnPtr::new(loadfn("glIsProgramARB")),
         IsProgramNV: FnPtr::new(loadfn("glIsProgramNV")),
         IsProgramPipeline: FnPtr::new(loadfn("glIsProgramPipeline")),
         IsQuery: FnPtr::new(loadfn("glIsQuery")),
         IsQueryARB: FnPtr::new(loadfn("glIsQueryARB")),
         IsRenderbuffer: FnPtr::new(loadfn("glIsRenderbuffer")),
         IsRenderbufferEXT: FnPtr::new(loadfn("glIsRenderbufferEXT")),
         IsSampler: FnPtr::new(loadfn("glIsSampler")),
         IsShader: FnPtr::new(loadfn("glIsShader")),
         IsSync: FnPtr::new(loadfn("glIsSync")),
         IsTexture: FnPtr::new(loadfn("glIsTexture")),
         IsTextureEXT: FnPtr::new(loadfn("glIsTextureEXT")),
         IsTransformFeedback: FnPtr::new(loadfn("glIsTransformFeedback")),
         IsTransformFeedbackNV: FnPtr::new(loadfn("glIsTransformFeedbackNV")),
         IsVertexArray: FnPtr::new(loadfn("glIsVertexArray")),
         IsVertexArrayAPPLE: FnPtr::new(loadfn("glIsVertexArrayAPPLE")),
         LineWidth: FnPtr::new(loadfn("glLineWidth")),
         LinkProgram: FnPtr::new(loadfn("glLinkProgram")),
         LinkProgramARB: FnPtr::new(loadfn("glLinkProgramARB")),
         LoadProgramNV: FnPtr::new(loadfn("glLoadProgramNV")),
         LogicOp: FnPtr::new(loadfn("glLogicOp")),
         MapBuffer: FnPtr::new(loadfn("glMapBuffer")),
         MapBufferARB: FnPtr::new(loadfn("glMapBufferARB")),
         MapBufferRange: FnPtr::new(loadfn("glMapBufferRange")),
         MapNamedBuffer: FnPtr::new(loadfn("glMapNamedBuffer")),
         MapNamedBufferEXT: FnPtr::new(loadfn("glMapNamedBufferEXT")),
         MapNamedBufferRange: FnPtr::new(loadfn("glMapNamedBufferRange")),
         MapNamedBufferRangeEXT: FnPtr::new(loadfn("glMapNamedBufferRangeEXT")),
         MatrixFrustumEXT: FnPtr::new(loadfn("glMatrixFrustumEXT")),
         MatrixLoadIdentityEXT: FnPtr::new(loadfn("glMatrixLoadIdentityEXT")),
         MatrixLoadTransposedEXT: FnPtr::new(loadfn("glMatrixLoadTransposedEXT")),
         MatrixLoadTransposefEXT: FnPtr::new(loadfn("glMatrixLoadTransposefEXT")),
         MatrixLoaddEXT: FnPtr::new(loadfn("glMatrixLoaddEXT")),
         MatrixLoadfEXT: FnPtr::new(loadfn("glMatrixLoadfEXT")),
         MatrixMultTransposedEXT: FnPtr::new(loadfn("glMatrixMultTransposedEXT")),
         MatrixMultTransposefEXT: FnPtr::new(loadfn("glMatrixMultTransposefEXT")),
         MatrixMultdEXT: FnPtr::new(loadfn("glMatrixMultdEXT")),
         MatrixMultfEXT: FnPtr::new(loadfn("glMatrixMultfEXT")),
         MatrixOrthoEXT: FnPtr::new(loadfn("glMatrixOrthoEXT")),
         MatrixPopEXT: FnPtr::new(loadfn("glMatrixPopEXT")),
         MatrixPushEXT: FnPtr::new(loadfn("glMatrixPushEXT")),
         MatrixRotatedEXT: FnPtr::new(loadfn("glMatrixRotatedEXT")),
         MatrixRotatefEXT: FnPtr::new(loadfn("glMatrixRotatefEXT")),
         MatrixScaledEXT: FnPtr::new(loadfn("glMatrixScaledEXT")),
         MatrixScalefEXT: FnPtr::new(loadfn("glMatrixScalefEXT")),
         MatrixTranslatedEXT: FnPtr::new(loadfn("glMatrixTranslatedEXT")),
         MatrixTranslatefEXT: FnPtr::new(loadfn("glMatrixTranslatefEXT")),
         MemoryBarrier: FnPtr::new(loadfn("glMemoryBarrier")),
         MemoryBarrierByRegion: FnPtr::new(loadfn("glMemoryBarrierByRegion")),
         MemoryBarrierEXT: FnPtr::new(loadfn("glMemoryBarrierEXT")),
         MinSampleShading: FnPtr::new(loadfn("glMinSampleShading")),
         MinSampleShadingARB: FnPtr::new(loadfn("glMinSampleShadingARB")),
         MultiDrawArrays: FnPtr::new(loadfn("glMultiDrawArrays")),
         MultiDrawArraysEXT: FnPtr::new(loadfn("glMultiDrawArraysEXT")),
         MultiDrawArraysIndirect: FnPtr::new(loadfn("glMultiDrawArraysIndirect")),
         MultiDrawArraysIndirectAMD: FnPtr::new(loadfn("glMultiDrawArraysIndirectAMD")),
         MultiDrawArraysIndirectCount: FnPtr::new(loadfn("glMultiDrawArraysIndirectCount")),
         MultiDrawArraysIndirectCountARB: FnPtr::new(loadfn("glMultiDrawArraysIndirectCountARB")),
         MultiDrawElements: FnPtr::new(loadfn("glMultiDrawElements")),
         MultiDrawElementsBaseVertex: FnPtr::new(loadfn("glMultiDrawElementsBaseVertex")),
         MultiDrawElementsEXT: FnPtr::new(loadfn("glMultiDrawElementsEXT")),
         MultiDrawElementsIndirect: FnPtr::new(loadfn("glMultiDrawElementsIndirect")),
         MultiDrawElementsIndirectAMD: FnPtr::new(loadfn("glMultiDrawElementsIndirectAMD")),
         MultiDrawElementsIndirectCount: FnPtr::new(loadfn("glMultiDrawElementsIndirectCount")),
         MultiDrawElementsIndirectCountARB: FnPtr::new(loadfn("glMultiDrawElementsIndirectCountARB")),
         MultiTexBufferEXT: FnPtr::new(loadfn("glMultiTexBufferEXT")),
         MultiTexCoord1dARB: FnPtr::new(loadfn("glMultiTexCoord1dARB")),
         MultiTexCoord1dvARB: FnPtr::new(loadfn("glMultiTexCoord1dvARB")),
         MultiTexCoord1fARB: FnPtr::new(loadfn("glMultiTexCoord1fARB")),
         MultiTexCoord1fvARB: FnPtr::new(loadfn("glMultiTexCoord1fvARB")),
         MultiTexCoord1iARB: FnPtr::new(loadfn("glMultiTexCoord1iARB")),
         MultiTexCoord1ivARB: FnPtr::new(loadfn("glMultiTexCoord1ivARB")),
         MultiTexCoord1sARB: FnPtr::new(loadfn("glMultiTexCoord1sARB")),
         MultiTexCoord1svARB: FnPtr::new(loadfn("glMultiTexCoord1svARB")),
         MultiTexCoord2dARB: FnPtr::new(loadfn("glMultiTexCoord2dARB")),
         MultiTexCoord2dvARB: FnPtr::new(loadfn("glMultiTexCoord2dvARB")),
         MultiTexCoord2fARB: FnPtr::new(loadfn("glMultiTexCoord2fARB")),
         MultiTexCoord2fvARB: FnPtr::new(loadfn("glMultiTexCoord2fvARB")),
         MultiTexCoord2iARB: FnPtr::new(loadfn("glMultiTexCoord2iARB")),
         MultiTexCoord2ivARB: FnPtr::new(loadfn("glMultiTexCoord2ivARB")),
         MultiTexCoord2sARB: FnPtr::new(loadfn("glMultiTexCoord2sARB")),
         MultiTexCoord2svARB: FnPtr::new(loadfn("glMultiTexCoord2svARB")),
         MultiTexCoord3dARB: FnPtr::new(loadfn("glMultiTexCoord3dARB")),
         MultiTexCoord3dvARB: FnPtr::new(loadfn("glMultiTexCoord3dvARB")),
         MultiTexCoord3fARB: FnPtr::new(loadfn("glMultiTexCoord3fARB")),
         MultiTexCoord3fvARB: FnPtr::new(loadfn("glMultiTexCoord3fvARB")),
         MultiTexCoord3iARB: FnPtr::new(loadfn("glMultiTexCoord3iARB")),
         MultiTexCoord3ivARB: FnPtr::new(loadfn("glMultiTexCoord3ivARB")),
         MultiTexCoord3sARB: FnPtr::new(loadfn("glMultiTexCoord3sARB")),
         MultiTexCoord3svARB: FnPtr::new(loadfn("glMultiTexCoord3svARB")),
         MultiTexCoord4dARB: FnPtr::new(loadfn("glMultiTexCoord4dARB")),
         MultiTexCoord4dvARB: FnPtr::new(loadfn("glMultiTexCoord4dvARB")),
         MultiTexCoord4fARB: FnPtr::new(loadfn("glMultiTexCoord4fARB")),
         MultiTexCoord4fvARB: FnPtr::new(loadfn("glMultiTexCoord4fvARB")),
         MultiTexCoord4iARB: FnPtr::new(loadfn("glMultiTexCoord4iARB")),
         MultiTexCoord4ivARB: FnPtr::new(loadfn("glMultiTexCoord4ivARB")),
         MultiTexCoord4sARB: FnPtr::new(loadfn("glMultiTexCoord4sARB")),
         MultiTexCoord4svARB: FnPtr::new(loadfn("glMultiTexCoord4svARB")),
         MultiTexCoordPointerEXT: FnPtr::new(loadfn("glMultiTexCoordPointerEXT")),
         MultiTexEnvfEXT: FnPtr::new(loadfn("glMultiTexEnvfEXT")),
         MultiTexEnvfvEXT: FnPtr::new(loadfn("glMultiTexEnvfvEXT")),
         MultiTexEnviEXT: FnPtr::new(loadfn("glMultiTexEnviEXT")),
         MultiTexEnvivEXT: FnPtr::new(loadfn("glMultiTexEnvivEXT")),
         MultiTexGendEXT: FnPtr::new(loadfn("glMultiTexGendEXT")),
         MultiTexGendvEXT: FnPtr::new(loadfn("glMultiTexGendvEXT")),
         MultiTexGenfEXT: FnPtr::new(loadfn("glMultiTexGenfEXT")),
         MultiTexGenfvEXT: FnPtr::new(loadfn("glMultiTexGenfvEXT")),
         MultiTexGeniEXT: FnPtr::new(loadfn("glMultiTexGeniEXT")),
         MultiTexGenivEXT: FnPtr::new(loadfn("glMultiTexGenivEXT")),
         MultiTexImage1DEXT: FnPtr::new(loadfn("glMultiTexImage1DEXT")),
         MultiTexImage2DEXT: FnPtr::new(loadfn("glMultiTexImage2DEXT")),
         MultiTexImage3DEXT: FnPtr::new(loadfn("glMultiTexImage3DEXT")),
         MultiTexParameterIivEXT: FnPtr::new(loadfn("glMultiTexParameterIivEXT")),
         MultiTexParameterIuivEXT: FnPtr::new(loadfn("glMultiTexParameterIuivEXT")),
         MultiTexParameterfEXT: FnPtr::new(loadfn("glMultiTexParameterfEXT")),
         MultiTexParameterfvEXT: FnPtr::new(loadfn("glMultiTexParameterfvEXT")),
         MultiTexParameteriEXT: FnPtr::new(loadfn("glMultiTexParameteriEXT")),
         MultiTexParameterivEXT: FnPtr::new(loadfn("glMultiTexParameterivEXT")),
         MultiTexRenderbufferEXT: FnPtr::new(loadfn("glMultiTexRenderbufferEXT")),
         MultiTexSubImage1DEXT: FnPtr::new(loadfn("glMultiTexSubImage1DEXT")),
         MultiTexSubImage2DEXT: FnPtr::new(loadfn("glMultiTexSubImage2DEXT")),
         MultiTexSubImage3DEXT: FnPtr::new(loadfn("glMultiTexSubImage3DEXT")),
         NamedBufferData: FnPtr::new(loadfn("glNamedBufferData")),
         NamedBufferDataEXT: FnPtr::new(loadfn("glNamedBufferDataEXT")),
         NamedBufferStorage: FnPtr::new(loadfn("glNamedBufferStorage")),
         NamedBufferStorageEXT: FnPtr::new(loadfn("glNamedBufferStorageEXT")),
         NamedBufferSubData: FnPtr::new(loadfn("glNamedBufferSubData")),
         NamedBufferSubDataEXT: FnPtr::new(loadfn("glNamedBufferSubDataEXT")),
         NamedCopyBufferSubDataEXT: FnPtr::new(loadfn("glNamedCopyBufferSubDataEXT")),
         NamedFramebufferDrawBuffer: FnPtr::new(loadfn("glNamedFramebufferDrawBuffer")),
         NamedFramebufferDrawBuffers: FnPtr::new(loadfn("glNamedFramebufferDrawBuffers")),
         NamedFramebufferParameteri: FnPtr::new(loadfn("glNamedFramebufferParameteri")),
         NamedFramebufferParameteriEXT: FnPtr::new(loadfn("glNamedFramebufferParameteriEXT")),
         NamedFramebufferReadBuffer: FnPtr::new(loadfn("glNamedFramebufferReadBuffer")),
         NamedFramebufferRenderbuffer: FnPtr::new(loadfn("glNamedFramebufferRenderbuffer")),
         NamedFramebufferRenderbufferEXT: FnPtr::new(loadfn("glNamedFramebufferRenderbufferEXT")),
         NamedFramebufferTexture: FnPtr::new(loadfn("glNamedFramebufferTexture")),
         NamedFramebufferTexture1DEXT: FnPtr::new(loadfn("glNamedFramebufferTexture1DEXT")),
         NamedFramebufferTexture2DEXT: FnPtr::new(loadfn("glNamedFramebufferTexture2DEXT")),
         NamedFramebufferTexture3DEXT: FnPtr::new(loadfn("glNamedFramebufferTexture3DEXT")),
         NamedFramebufferTextureEXT: FnPtr::new(loadfn("glNamedFramebufferTextureEXT")),
         NamedFramebufferTextureFaceEXT: FnPtr::new(loadfn("glNamedFramebufferTextureFaceEXT")),
         NamedFramebufferTextureLayer: FnPtr::new(loadfn("glNamedFramebufferTextureLayer")),
         NamedFramebufferTextureLayerEXT: FnPtr::new(loadfn("glNamedFramebufferTextureLayerEXT")),
         NamedProgramLocalParameter4dEXT: FnPtr::new(loadfn("glNamedProgramLocalParameter4dEXT")),
         NamedProgramLocalParameter4dvEXT: FnPtr::new(loadfn("glNamedProgramLocalParameter4dvEXT")),
         NamedProgramLocalParameter4fEXT: FnPtr::new(loadfn("glNamedProgramLocalParameter4fEXT")),
         NamedProgramLocalParameter4fvEXT: FnPtr::new(loadfn("glNamedProgramLocalParameter4fvEXT")),
         NamedProgramLocalParameterI4iEXT: FnPtr::new(loadfn("glNamedProgramLocalParameterI4iEXT")),
         NamedProgramLocalParameterI4ivEXT: FnPtr::new(loadfn("glNamedProgramLocalParameterI4ivEXT")),
         NamedProgramLocalParameterI4uiEXT: FnPtr::new(loadfn("glNamedProgramLocalParameterI4uiEXT")),
         NamedProgramLocalParameterI4uivEXT: FnPtr::new(loadfn("glNamedProgramLocalParameterI4uivEXT")),
         NamedProgramLocalParameters4fvEXT: FnPtr::new(loadfn("glNamedProgramLocalParameters4fvEXT")),
         NamedProgramLocalParametersI4ivEXT: FnPtr::new(loadfn("glNamedProgramLocalParametersI4ivEXT")),
         NamedProgramLocalParametersI4uivEXT: FnPtr::new(loadfn("glNamedProgramLocalParametersI4uivEXT")),
         NamedProgramStringEXT: FnPtr::new(loadfn("glNamedProgramStringEXT")),
         NamedRenderbufferStorage: FnPtr::new(loadfn("glNamedRenderbufferStorage")),
         NamedRenderbufferStorageEXT: FnPtr::new(loadfn("glNamedRenderbufferStorageEXT")),
         NamedRenderbufferStorageMultisample: FnPtr::new(loadfn("glNamedRenderbufferStorageMultisample")),
         NamedRenderbufferStorageMultisampleCoverageEXT: FnPtr::new(loadfn("glNamedRenderbufferStorageMultisampleCoverageEXT")),
         NamedRenderbufferStorageMultisampleEXT: FnPtr::new(loadfn("glNamedRenderbufferStorageMultisampleEXT")),
         NormalPointerEXT: FnPtr::new(loadfn("glNormalPointerEXT")),
         ObjectLabel: FnPtr::new(loadfn("glObjectLabel")),
         ObjectPtrLabel: FnPtr::new(loadfn("glObjectPtrLabel")),
         OrthofOES: FnPtr::new(loadfn("glOrthofOES")),
         PatchParameterfv: FnPtr::new(loadfn("glPatchParameterfv")),
         PatchParameteri: FnPtr::new(loadfn("glPatchParameteri")),
         PauseTransformFeedback: FnPtr::new(loadfn("glPauseTransformFeedback")),
         PauseTransformFeedbackNV: FnPtr::new(loadfn("glPauseTransformFeedbackNV")),
         PixelStoref: FnPtr::new(loadfn("glPixelStoref")),
         PixelStorei: FnPtr::new(loadfn("glPixelStorei")),
         PointParameterf: FnPtr::new(loadfn("glPointParameterf")),
         PointParameterfARB: FnPtr::new(loadfn("glPointParameterfARB")),
         PointParameterfEXT: FnPtr::new(loadfn("glPointParameterfEXT")),
         PointParameterfSGIS: FnPtr::new(loadfn("glPointParameterfSGIS")),
         PointParameterfv: FnPtr::new(loadfn("glPointParameterfv")),
         PointParameterfvARB: FnPtr::new(loadfn("glPointParameterfvARB")),
         PointParameterfvEXT: FnPtr::new(loadfn("glPointParameterfvEXT")),
         PointParameterfvSGIS: FnPtr::new(loadfn("glPointParameterfvSGIS")),
         PointParameteri: FnPtr::new(loadfn("glPointParameteri")),
         PointParameteriNV: FnPtr::new(loadfn("glPointParameteriNV")),
         PointParameteriv: FnPtr::new(loadfn("glPointParameteriv")),
         PointParameterivNV: FnPtr::new(loadfn("glPointParameterivNV")),
         PointSize: FnPtr::new(loadfn("glPointSize")),
         PolygonMode: FnPtr::new(loadfn("glPolygonMode")),
         PolygonOffset: FnPtr::new(loadfn("glPolygonOffset")),
         PolygonOffsetClamp: FnPtr::new(loadfn("glPolygonOffsetClamp")),
         PolygonOffsetClampEXT: FnPtr::new(loadfn("glPolygonOffsetClampEXT")),
         PopDebugGroup: FnPtr::new(loadfn("glPopDebugGroup")),
         PrimitiveRestartIndex: FnPtr::new(loadfn("glPrimitiveRestartIndex")),
         PrioritizeTexturesEXT: FnPtr::new(loadfn("glPrioritizeTexturesEXT")),
         ProgramBinary: FnPtr::new(loadfn("glProgramBinary")),
         ProgramEnvParameter4dARB: FnPtr::new(loadfn("glProgramEnvParameter4dARB")),
         ProgramEnvParameter4dvARB: FnPtr::new(loadfn("glProgramEnvParameter4dvARB")),
         ProgramEnvParameter4fARB: FnPtr::new(loadfn("glProgramEnvParameter4fARB")),
         ProgramEnvParameter4fvARB: FnPtr::new(loadfn("glProgramEnvParameter4fvARB")),
         ProgramLocalParameter4dARB: FnPtr::new(loadfn("glProgramLocalParameter4dARB")),
         ProgramLocalParameter4dvARB: FnPtr::new(loadfn("glProgramLocalParameter4dvARB")),
         ProgramLocalParameter4fARB: FnPtr::new(loadfn("glProgramLocalParameter4fARB")),
         ProgramLocalParameter4fvARB: FnPtr::new(loadfn("glProgramLocalParameter4fvARB")),
         ProgramParameter4dNV: FnPtr::new(loadfn("glProgramParameter4dNV")),
         ProgramParameter4dvNV: FnPtr::new(loadfn("glProgramParameter4dvNV")),
         ProgramParameter4fNV: FnPtr::new(loadfn("glProgramParameter4fNV")),
         ProgramParameter4fvNV: FnPtr::new(loadfn("glProgramParameter4fvNV")),
         ProgramParameteri: FnPtr::new(loadfn("glProgramParameteri")),
         ProgramParameteriARB: FnPtr::new(loadfn("glProgramParameteriARB")),
         ProgramParameteriEXT: FnPtr::new(loadfn("glProgramParameteriEXT")),
         ProgramParameters4dvNV: FnPtr::new(loadfn("glProgramParameters4dvNV")),
         ProgramParameters4fvNV: FnPtr::new(loadfn("glProgramParameters4fvNV")),
         ProgramStringARB: FnPtr::new(loadfn("glProgramStringARB")),
         ProgramUniform1d: FnPtr::new(loadfn("glProgramUniform1d")),
         ProgramUniform1dEXT: FnPtr::new(loadfn("glProgramUniform1dEXT")),
         ProgramUniform1dv: FnPtr::new(loadfn("glProgramUniform1dv")),
         ProgramUniform1dvEXT: FnPtr::new(loadfn("glProgramUniform1dvEXT")),
         ProgramUniform1f: FnPtr::new(loadfn("glProgramUniform1f")),
         ProgramUniform1fEXT: FnPtr::new(loadfn("glProgramUniform1fEXT")),
         ProgramUniform1fv: FnPtr::new(loadfn("glProgramUniform1fv")),
         ProgramUniform1fvEXT: FnPtr::new(loadfn("glProgramUniform1fvEXT")),
         ProgramUniform1i: FnPtr::new(loadfn("glProgramUniform1i")),
         ProgramUniform1iEXT: FnPtr::new(loadfn("glProgramUniform1iEXT")),
         ProgramUniform1iv: FnPtr::new(loadfn("glProgramUniform1iv")),
         ProgramUniform1ivEXT: FnPtr::new(loadfn("glProgramUniform1ivEXT")),
         ProgramUniform1ui: FnPtr::new(loadfn("glProgramUniform1ui")),
         ProgramUniform1uiEXT: FnPtr::new(loadfn("glProgramUniform1uiEXT")),
         ProgramUniform1uiv: FnPtr::new(loadfn("glProgramUniform1uiv")),
         ProgramUniform1uivEXT: FnPtr::new(loadfn("glProgramUniform1uivEXT")),
         ProgramUniform2d: FnPtr::new(loadfn("glProgramUniform2d")),
         ProgramUniform2dEXT: FnPtr::new(loadfn("glProgramUniform2dEXT")),
         ProgramUniform2dv: FnPtr::new(loadfn("glProgramUniform2dv")),
         ProgramUniform2dvEXT: FnPtr::new(loadfn("glProgramUniform2dvEXT")),
         ProgramUniform2f: FnPtr::new(loadfn("glProgramUniform2f")),
         ProgramUniform2fEXT: FnPtr::new(loadfn("glProgramUniform2fEXT")),
         ProgramUniform2fv: FnPtr::new(loadfn("glProgramUniform2fv")),
         ProgramUniform2fvEXT: FnPtr::new(loadfn("glProgramUniform2fvEXT")),
         ProgramUniform2i: FnPtr::new(loadfn("glProgramUniform2i")),
         ProgramUniform2iEXT: FnPtr::new(loadfn("glProgramUniform2iEXT")),
         ProgramUniform2iv: FnPtr::new(loadfn("glProgramUniform2iv")),
         ProgramUniform2ivEXT: FnPtr::new(loadfn("glProgramUniform2ivEXT")),
         ProgramUniform2ui: FnPtr::new(loadfn("glProgramUniform2ui")),
         ProgramUniform2uiEXT: FnPtr::new(loadfn("glProgramUniform2uiEXT")),
         ProgramUniform2uiv: FnPtr::new(loadfn("glProgramUniform2uiv")),
         ProgramUniform2uivEXT: FnPtr::new(loadfn("glProgramUniform2uivEXT")),
         ProgramUniform3d: FnPtr::new(loadfn("glProgramUniform3d")),
         ProgramUniform3dEXT: FnPtr::new(loadfn("glProgramUniform3dEXT")),
         ProgramUniform3dv: FnPtr::new(loadfn("glProgramUniform3dv")),
         ProgramUniform3dvEXT: FnPtr::new(loadfn("glProgramUniform3dvEXT")),
         ProgramUniform3f: FnPtr::new(loadfn("glProgramUniform3f")),
         ProgramUniform3fEXT: FnPtr::new(loadfn("glProgramUniform3fEXT")),
         ProgramUniform3fv: FnPtr::new(loadfn("glProgramUniform3fv")),
         ProgramUniform3fvEXT: FnPtr::new(loadfn("glProgramUniform3fvEXT")),
         ProgramUniform3i: FnPtr::new(loadfn("glProgramUniform3i")),
         ProgramUniform3iEXT: FnPtr::new(loadfn("glProgramUniform3iEXT")),
         ProgramUniform3iv: FnPtr::new(loadfn("glProgramUniform3iv")),
         ProgramUniform3ivEXT: FnPtr::new(loadfn("glProgramUniform3ivEXT")),
         ProgramUniform3ui: FnPtr::new(loadfn("glProgramUniform3ui")),
         ProgramUniform3uiEXT: FnPtr::new(loadfn("glProgramUniform3uiEXT")),
         ProgramUniform3uiv: FnPtr::new(loadfn("glProgramUniform3uiv")),
         ProgramUniform3uivEXT: FnPtr::new(loadfn("glProgramUniform3uivEXT")),
         ProgramUniform4d: FnPtr::new(loadfn("glProgramUniform4d")),
         ProgramUniform4dEXT: FnPtr::new(loadfn("glProgramUniform4dEXT")),
         ProgramUniform4dv: FnPtr::new(loadfn("glProgramUniform4dv")),
         ProgramUniform4dvEXT: FnPtr::new(loadfn("glProgramUniform4dvEXT")),
         ProgramUniform4f: FnPtr::new(loadfn("glProgramUniform4f")),
         ProgramUniform4fEXT: FnPtr::new(loadfn("glProgramUniform4fEXT")),
         ProgramUniform4fv: FnPtr::new(loadfn("glProgramUniform4fv")),
         ProgramUniform4fvEXT: FnPtr::new(loadfn("glProgramUniform4fvEXT")),
         ProgramUniform4i: FnPtr::new(loadfn("glProgramUniform4i")),
         ProgramUniform4iEXT: FnPtr::new(loadfn("glProgramUniform4iEXT")),
         ProgramUniform4iv: FnPtr::new(loadfn("glProgramUniform4iv")),
         ProgramUniform4ivEXT: FnPtr::new(loadfn("glProgramUniform4ivEXT")),
         ProgramUniform4ui: FnPtr::new(loadfn("glProgramUniform4ui")),
         ProgramUniform4uiEXT: FnPtr::new(loadfn("glProgramUniform4uiEXT")),
         ProgramUniform4uiv: FnPtr::new(loadfn("glProgramUniform4uiv")),
         ProgramUniform4uivEXT: FnPtr::new(loadfn("glProgramUniform4uivEXT")),
         ProgramUniformMatrix2dv: FnPtr::new(loadfn("glProgramUniformMatrix2dv")),
         ProgramUniformMatrix2dvEXT: FnPtr::new(loadfn("glProgramUniformMatrix2dvEXT")),
         ProgramUniformMatrix2fv: FnPtr::new(loadfn("glProgramUniformMatrix2fv")),
         ProgramUniformMatrix2fvEXT: FnPtr::new(loadfn("glProgramUniformMatrix2fvEXT")),
         ProgramUniformMatrix2x3dv: FnPtr::new(loadfn("glProgramUniformMatrix2x3dv")),
         ProgramUniformMatrix2x3dvEXT: FnPtr::new(loadfn("glProgramUniformMatrix2x3dvEXT")),
         ProgramUniformMatrix2x3fv: FnPtr::new(loadfn("glProgramUniformMatrix2x3fv")),
         ProgramUniformMatrix2x3fvEXT: FnPtr::new(loadfn("glProgramUniformMatrix2x3fvEXT")),
         ProgramUniformMatrix2x4dv: FnPtr::new(loadfn("glProgramUniformMatrix2x4dv")),
         ProgramUniformMatrix2x4dvEXT: FnPtr::new(loadfn("glProgramUniformMatrix2x4dvEXT")),
         ProgramUniformMatrix2x4fv: FnPtr::new(loadfn("glProgramUniformMatrix2x4fv")),
         ProgramUniformMatrix2x4fvEXT: FnPtr::new(loadfn("glProgramUniformMatrix2x4fvEXT")),
         ProgramUniformMatrix3dv: FnPtr::new(loadfn("glProgramUniformMatrix3dv")),
         ProgramUniformMatrix3dvEXT: FnPtr::new(loadfn("glProgramUniformMatrix3dvEXT")),
         ProgramUniformMatrix3fv: FnPtr::new(loadfn("glProgramUniformMatrix3fv")),
         ProgramUniformMatrix3fvEXT: FnPtr::new(loadfn("glProgramUniformMatrix3fvEXT")),
         ProgramUniformMatrix3x2dv: FnPtr::new(loadfn("glProgramUniformMatrix3x2dv")),
         ProgramUniformMatrix3x2dvEXT: FnPtr::new(loadfn("glProgramUniformMatrix3x2dvEXT")),
         ProgramUniformMatrix3x2fv: FnPtr::new(loadfn("glProgramUniformMatrix3x2fv")),
         ProgramUniformMatrix3x2fvEXT: FnPtr::new(loadfn("glProgramUniformMatrix3x2fvEXT")),
         ProgramUniformMatrix3x4dv: FnPtr::new(loadfn("glProgramUniformMatrix3x4dv")),
         ProgramUniformMatrix3x4dvEXT: FnPtr::new(loadfn("glProgramUniformMatrix3x4dvEXT")),
         ProgramUniformMatrix3x4fv: FnPtr::new(loadfn("glProgramUniformMatrix3x4fv")),
         ProgramUniformMatrix3x4fvEXT: FnPtr::new(loadfn("glProgramUniformMatrix3x4fvEXT")),
         ProgramUniformMatrix4dv: FnPtr::new(loadfn("glProgramUniformMatrix4dv")),
         ProgramUniformMatrix4dvEXT: FnPtr::new(loadfn("glProgramUniformMatrix4dvEXT")),
         ProgramUniformMatrix4fv: FnPtr::new(loadfn("glProgramUniformMatrix4fv")),
         ProgramUniformMatrix4fvEXT: FnPtr::new(loadfn("glProgramUniformMatrix4fvEXT")),
         ProgramUniformMatrix4x2dv: FnPtr::new(loadfn("glProgramUniformMatrix4x2dv")),
         ProgramUniformMatrix4x2dvEXT: FnPtr::new(loadfn("glProgramUniformMatrix4x2dvEXT")),
         ProgramUniformMatrix4x2fv: FnPtr::new(loadfn("glProgramUniformMatrix4x2fv")),
         ProgramUniformMatrix4x2fvEXT: FnPtr::new(loadfn("glProgramUniformMatrix4x2fvEXT")),
         ProgramUniformMatrix4x3dv: FnPtr::new(loadfn("glProgramUniformMatrix4x3dv")),
         ProgramUniformMatrix4x3dvEXT: FnPtr::new(loadfn("glProgramUniformMatrix4x3dvEXT")),
         ProgramUniformMatrix4x3fv: FnPtr::new(loadfn("glProgramUniformMatrix4x3fv")),
         ProgramUniformMatrix4x3fvEXT: FnPtr::new(loadfn("glProgramUniformMatrix4x3fvEXT")),
         ProgramVertexLimitNV: FnPtr::new(loadfn("glProgramVertexLimitNV")),
         ProvokingVertex: FnPtr::new(loadfn("glProvokingVertex")),
         ProvokingVertexEXT: FnPtr::new(loadfn("glProvokingVertexEXT")),
         PushClientAttribDefaultEXT: FnPtr::new(loadfn("glPushClientAttribDefaultEXT")),
         PushDebugGroup: FnPtr::new(loadfn("glPushDebugGroup")),
         QueryCounter: FnPtr::new(loadfn("glQueryCounter")),
         ReadBuffer: FnPtr::new(loadfn("glReadBuffer")),
         ReadPixels: FnPtr::new(loadfn("glReadPixels")),
         ReadnPixels: FnPtr::new(loadfn("glReadnPixels")),
         ReadnPixelsARB: FnPtr::new(loadfn("glReadnPixelsARB")),
         ReleaseShaderCompiler: FnPtr::new(loadfn("glReleaseShaderCompiler")),
         RenderbufferStorage: FnPtr::new(loadfn("glRenderbufferStorage")),
         RenderbufferStorageEXT: FnPtr::new(loadfn("glRenderbufferStorageEXT")),
         RenderbufferStorageMultisample: FnPtr::new(loadfn("glRenderbufferStorageMultisample")),
         RenderbufferStorageMultisampleEXT: FnPtr::new(loadfn("glRenderbufferStorageMultisampleEXT")),
         RequestResidentProgramsNV: FnPtr::new(loadfn("glRequestResidentProgramsNV")),
         ResumeTransformFeedback: FnPtr::new(loadfn("glResumeTransformFeedback")),
         ResumeTransformFeedbackNV: FnPtr::new(loadfn("glResumeTransformFeedbackNV")),
         SampleCoverage: FnPtr::new(loadfn("glSampleCoverage")),
         SampleCoverageARB: FnPtr::new(loadfn("glSampleCoverageARB")),
         SampleMaskIndexedNV: FnPtr::new(loadfn("glSampleMaskIndexedNV")),
         SampleMaski: FnPtr::new(loadfn("glSampleMaski")),
         SamplerParameterIiv: FnPtr::new(loadfn("glSamplerParameterIiv")),
         SamplerParameterIuiv: FnPtr::new(loadfn("glSamplerParameterIuiv")),
         SamplerParameterf: FnPtr::new(loadfn("glSamplerParameterf")),
         SamplerParameterfv: FnPtr::new(loadfn("glSamplerParameterfv")),
         SamplerParameteri: FnPtr::new(loadfn("glSamplerParameteri")),
         SamplerParameteriv: FnPtr::new(loadfn("glSamplerParameteriv")),
         Scissor: FnPtr::new(loadfn("glScissor")),
         ScissorArrayv: FnPtr::new(loadfn("glScissorArrayv")),
         ScissorIndexed: FnPtr::new(loadfn("glScissorIndexed")),
         ScissorIndexedv: FnPtr::new(loadfn("glScissorIndexedv")),
         ShaderBinary: FnPtr::new(loadfn("glShaderBinary")),
         ShaderSource: FnPtr::new(loadfn("glShaderSource")),
         ShaderSourceARB: FnPtr::new(loadfn("glShaderSourceARB")),
         ShaderStorageBlockBinding: FnPtr::new(loadfn("glShaderStorageBlockBinding")),
         SpecializeShader: FnPtr::new(loadfn("glSpecializeShader")),
         SpecializeShaderARB: FnPtr::new(loadfn("glSpecializeShaderARB")),
         StencilFunc: FnPtr::new(loadfn("glStencilFunc")),
         StencilFuncSeparate: FnPtr::new(loadfn("glStencilFuncSeparate")),
         StencilFuncSeparateATI: FnPtr::new(loadfn("glStencilFuncSeparateATI")),
         StencilMask: FnPtr::new(loadfn("glStencilMask")),
         StencilMaskSeparate: FnPtr::new(loadfn("glStencilMaskSeparate")),
         StencilOp: FnPtr::new(loadfn("glStencilOp")),
         StencilOpSeparate: FnPtr::new(loadfn("glStencilOpSeparate")),
         StencilOpSeparateATI: FnPtr::new(loadfn("glStencilOpSeparateATI")),
         TexBuffer: FnPtr::new(loadfn("glTexBuffer")),
         TexBufferARB: FnPtr::new(loadfn("glTexBufferARB")),
         TexBufferEXT: FnPtr::new(loadfn("glTexBufferEXT")),
         TexBufferRange: FnPtr::new(loadfn("glTexBufferRange")),
         TexCoordPointerEXT: FnPtr::new(loadfn("glTexCoordPointerEXT")),
         TexImage1D: FnPtr::new(loadfn("glTexImage1D")),
         TexImage2D: FnPtr::new(loadfn("glTexImage2D")),
         TexImage2DMultisample: FnPtr::new(loadfn("glTexImage2DMultisample")),
         TexImage3D: FnPtr::new(loadfn("glTexImage3D")),
         TexImage3DEXT: FnPtr::new(loadfn("glTexImage3DEXT")),
         TexImage3DMultisample: FnPtr::new(loadfn("glTexImage3DMultisample")),
         TexParameterIiv: FnPtr::new(loadfn("glTexParameterIiv")),
         TexParameterIivEXT: FnPtr::new(loadfn("glTexParameterIivEXT")),
         TexParameterIuiv: FnPtr::new(loadfn("glTexParameterIuiv")),
         TexParameterIuivEXT: FnPtr::new(loadfn("glTexParameterIuivEXT")),
         TexParameterf: FnPtr::new(loadfn("glTexParameterf")),
         TexParameterfv: FnPtr::new(loadfn("glTexParameterfv")),
         TexParameteri: FnPtr::new(loadfn("glTexParameteri")),
         TexParameteriv: FnPtr::new(loadfn("glTexParameteriv")),
         TexRenderbufferNV: FnPtr::new(loadfn("glTexRenderbufferNV")),
         TexStorage1D: FnPtr::new(loadfn("glTexStorage1D")),
         TexStorage1DEXT: FnPtr::new(loadfn("glTexStorage1DEXT")),
         TexStorage2D: FnPtr::new(loadfn("glTexStorage2D")),
         TexStorage2DEXT: FnPtr::new(loadfn("glTexStorage2DEXT")),
         TexStorage2DMultisample: FnPtr::new(loadfn("glTexStorage2DMultisample")),
         TexStorage3D: FnPtr::new(loadfn("glTexStorage3D")),
         TexStorage3DEXT: FnPtr::new(loadfn("glTexStorage3DEXT")),
         TexStorage3DMultisample: FnPtr::new(loadfn("glTexStorage3DMultisample")),
         TexSubImage1D: FnPtr::new(loadfn("glTexSubImage1D")),
         TexSubImage1DEXT: FnPtr::new(loadfn("glTexSubImage1DEXT")),
         TexSubImage2D: FnPtr::new(loadfn("glTexSubImage2D")),
         TexSubImage2DEXT: FnPtr::new(loadfn("glTexSubImage2DEXT")),
         TexSubImage3D: FnPtr::new(loadfn("glTexSubImage3D")),
         TexSubImage3DEXT: FnPtr::new(loadfn("glTexSubImage3DEXT")),
         TextureBarrier: FnPtr::new(loadfn("glTextureBarrier")),
         TextureBuffer: FnPtr::new(loadfn("glTextureBuffer")),
         TextureBufferEXT: FnPtr::new(loadfn("glTextureBufferEXT")),
         TextureBufferRange: FnPtr::new(loadfn("glTextureBufferRange")),
         TextureBufferRangeEXT: FnPtr::new(loadfn("glTextureBufferRangeEXT")),
         TextureImage1DEXT: FnPtr::new(loadfn("glTextureImage1DEXT")),
         TextureImage2DEXT: FnPtr::new(loadfn("glTextureImage2DEXT")),
         TextureImage3DEXT: FnPtr::new(loadfn("glTextureImage3DEXT")),
         TexturePageCommitmentEXT: FnPtr::new(loadfn("glTexturePageCommitmentEXT")),
         TextureParameterIiv: FnPtr::new(loadfn("glTextureParameterIiv")),
         TextureParameterIivEXT: FnPtr::new(loadfn("glTextureParameterIivEXT")),
         TextureParameterIuiv: FnPtr::new(loadfn("glTextureParameterIuiv")),
         TextureParameterIuivEXT: FnPtr::new(loadfn("glTextureParameterIuivEXT")),
         TextureParameterf: FnPtr::new(loadfn("glTextureParameterf")),
         TextureParameterfEXT: FnPtr::new(loadfn("glTextureParameterfEXT")),
         TextureParameterfv: FnPtr::new(loadfn("glTextureParameterfv")),
         TextureParameterfvEXT: FnPtr::new(loadfn("glTextureParameterfvEXT")),
         TextureParameteri: FnPtr::new(loadfn("glTextureParameteri")),
         TextureParameteriEXT: FnPtr::new(loadfn("glTextureParameteriEXT")),
         TextureParameteriv: FnPtr::new(loadfn("glTextureParameteriv")),
         TextureParameterivEXT: FnPtr::new(loadfn("glTextureParameterivEXT")),
         TextureRenderbufferEXT: FnPtr::new(loadfn("glTextureRenderbufferEXT")),
         TextureStorage1D: FnPtr::new(loadfn("glTextureStorage1D")),
         TextureStorage1DEXT: FnPtr::new(loadfn("glTextureStorage1DEXT")),
         TextureStorage2D: FnPtr::new(loadfn("glTextureStorage2D")),
         TextureStorage2DEXT: FnPtr::new(loadfn("glTextureStorage2DEXT")),
         TextureStorage2DMultisample: FnPtr::new(loadfn("glTextureStorage2DMultisample")),
         TextureStorage2DMultisampleEXT: FnPtr::new(loadfn("glTextureStorage2DMultisampleEXT")),
         TextureStorage3D: FnPtr::new(loadfn("glTextureStorage3D")),
         TextureStorage3DEXT: FnPtr::new(loadfn("glTextureStorage3DEXT")),
         TextureStorage3DMultisample: FnPtr::new(loadfn("glTextureStorage3DMultisample")),
         TextureStorage3DMultisampleEXT: FnPtr::new(loadfn("glTextureStorage3DMultisampleEXT")),
         TextureSubImage1D: FnPtr::new(loadfn("glTextureSubImage1D")),
         TextureSubImage1DEXT: FnPtr::new(loadfn("glTextureSubImage1DEXT")),
         TextureSubImage2D: FnPtr::new(loadfn("glTextureSubImage2D")),
         TextureSubImage2DEXT: FnPtr::new(loadfn("glTextureSubImage2DEXT")),
         TextureSubImage3D: FnPtr::new(loadfn("glTextureSubImage3D")),
         TextureSubImage3DEXT: FnPtr::new(loadfn("glTextureSubImage3DEXT")),
         TextureView: FnPtr::new(loadfn("glTextureView")),
         TrackMatrixNV: FnPtr::new(loadfn("glTrackMatrixNV")),
         TransformFeedbackAttribsNV: FnPtr::new(loadfn("glTransformFeedbackAttribsNV")),
         TransformFeedbackBufferBase: FnPtr::new(loadfn("glTransformFeedbackBufferBase")),
         TransformFeedbackBufferRange: FnPtr::new(loadfn("glTransformFeedbackBufferRange")),
         TransformFeedbackStreamAttribsNV: FnPtr::new(loadfn("glTransformFeedbackStreamAttribsNV")),
         TransformFeedbackVaryings: FnPtr::new(loadfn("glTransformFeedbackVaryings")),
         TransformFeedbackVaryingsEXT: FnPtr::new(loadfn("glTransformFeedbackVaryingsEXT")),
         TransformFeedbackVaryingsNV: FnPtr::new(loadfn("glTransformFeedbackVaryingsNV")),
         Uniform1d: FnPtr::new(loadfn("glUniform1d")),
         Uniform1dv: FnPtr::new(loadfn("glUniform1dv")),
         Uniform1f: FnPtr::new(loadfn("glUniform1f")),
         Uniform1fARB: FnPtr::new(loadfn("glUniform1fARB")),
         Uniform1fv: FnPtr::new(loadfn("glUniform1fv")),
         Uniform1fvARB: FnPtr::new(loadfn("glUniform1fvARB")),
         Uniform1i: FnPtr::new(loadfn("glUniform1i")),
         Uniform1iARB: FnPtr::new(loadfn("glUniform1iARB")),
         Uniform1iv: FnPtr::new(loadfn("glUniform1iv")),
         Uniform1ivARB: FnPtr::new(loadfn("glUniform1ivARB")),
         Uniform1ui: FnPtr::new(loadfn("glUniform1ui")),
         Uniform1uiEXT: FnPtr::new(loadfn("glUniform1uiEXT")),
         Uniform1uiv: FnPtr::new(loadfn("glUniform1uiv")),
         Uniform1uivEXT: FnPtr::new(loadfn("glUniform1uivEXT")),
         Uniform2d: FnPtr::new(loadfn("glUniform2d")),
         Uniform2dv: FnPtr::new(loadfn("glUniform2dv")),
         Uniform2f: FnPtr::new(loadfn("glUniform2f")),
         Uniform2fARB: FnPtr::new(loadfn("glUniform2fARB")),
         Uniform2fv: FnPtr::new(loadfn("glUniform2fv")),
         Uniform2fvARB: FnPtr::new(loadfn("glUniform2fvARB")),
         Uniform2i: FnPtr::new(loadfn("glUniform2i")),
         Uniform2iARB: FnPtr::new(loadfn("glUniform2iARB")),
         Uniform2iv: FnPtr::new(loadfn("glUniform2iv")),
         Uniform2ivARB: FnPtr::new(loadfn("glUniform2ivARB")),
         Uniform2ui: FnPtr::new(loadfn("glUniform2ui")),
         Uniform2uiEXT: FnPtr::new(loadfn("glUniform2uiEXT")),
         Uniform2uiv: FnPtr::new(loadfn("glUniform2uiv")),
         Uniform2uivEXT: FnPtr::new(loadfn("glUniform2uivEXT")),
         Uniform3d: FnPtr::new(loadfn("glUniform3d")),
         Uniform3dv: FnPtr::new(loadfn("glUniform3dv")),
         Uniform3f: FnPtr::new(loadfn("glUniform3f")),
         Uniform3fARB: FnPtr::new(loadfn("glUniform3fARB")),
         Uniform3fv: FnPtr::new(loadfn("glUniform3fv")),
         Uniform3fvARB: FnPtr::new(loadfn("glUniform3fvARB")),
         Uniform3i: FnPtr::new(loadfn("glUniform3i")),
         Uniform3iARB: FnPtr::new(loadfn("glUniform3iARB")),
         Uniform3iv: FnPtr::new(loadfn("glUniform3iv")),
         Uniform3ivARB: FnPtr::new(loadfn("glUniform3ivARB")),
         Uniform3ui: FnPtr::new(loadfn("glUniform3ui")),
         Uniform3uiEXT: FnPtr::new(loadfn("glUniform3uiEXT")),
         Uniform3uiv: FnPtr::new(loadfn("glUniform3uiv")),
         Uniform3uivEXT: FnPtr::new(loadfn("glUniform3uivEXT")),
         Uniform4d: FnPtr::new(loadfn("glUniform4d")),
         Uniform4dv: FnPtr::new(loadfn("glUniform4dv")),
         Uniform4f: FnPtr::new(loadfn("glUniform4f")),
         Uniform4fARB: FnPtr::new(loadfn("glUniform4fARB")),
         Uniform4fv: FnPtr::new(loadfn("glUniform4fv")),
         Uniform4fvARB: FnPtr::new(loadfn("glUniform4fvARB")),
         Uniform4i: FnPtr::new(loadfn("glUniform4i")),
         Uniform4iARB: FnPtr::new(loadfn("glUniform4iARB")),
         Uniform4iv: FnPtr::new(loadfn("glUniform4iv")),
         Uniform4ivARB: FnPtr::new(loadfn("glUniform4ivARB")),
         Uniform4ui: FnPtr::new(loadfn("glUniform4ui")),
         Uniform4uiEXT: FnPtr::new(loadfn("glUniform4uiEXT")),
         Uniform4uiv: FnPtr::new(loadfn("glUniform4uiv")),
         Uniform4uivEXT: FnPtr::new(loadfn("glUniform4uivEXT")),
         UniformBlockBinding: FnPtr::new(loadfn("glUniformBlockBinding")),
         UniformMatrix2dv: FnPtr::new(loadfn("glUniformMatrix2dv")),
         UniformMatrix2fv: FnPtr::new(loadfn("glUniformMatrix2fv")),
         UniformMatrix2fvARB: FnPtr::new(loadfn("glUniformMatrix2fvARB")),
         UniformMatrix2x3dv: FnPtr::new(loadfn("glUniformMatrix2x3dv")),
         UniformMatrix2x3fv: FnPtr::new(loadfn("glUniformMatrix2x3fv")),
         UniformMatrix2x4dv: FnPtr::new(loadfn("glUniformMatrix2x4dv")),
         UniformMatrix2x4fv: FnPtr::new(loadfn("glUniformMatrix2x4fv")),
         UniformMatrix3dv: FnPtr::new(loadfn("glUniformMatrix3dv")),
         UniformMatrix3fv: FnPtr::new(loadfn("glUniformMatrix3fv")),
         UniformMatrix3fvARB: FnPtr::new(loadfn("glUniformMatrix3fvARB")),
         UniformMatrix3x2dv: FnPtr::new(loadfn("glUniformMatrix3x2dv")),
         UniformMatrix3x2fv: FnPtr::new(loadfn("glUniformMatrix3x2fv")),
         UniformMatrix3x4dv: FnPtr::new(loadfn("glUniformMatrix3x4dv")),
         UniformMatrix3x4fv: FnPtr::new(loadfn("glUniformMatrix3x4fv")),
         UniformMatrix4dv: FnPtr::new(loadfn("glUniformMatrix4dv")),
         UniformMatrix4fv: FnPtr::new(loadfn("glUniformMatrix4fv")),
         UniformMatrix4fvARB: FnPtr::new(loadfn("glUniformMatrix4fvARB")),
         UniformMatrix4x2dv: FnPtr::new(loadfn("glUniformMatrix4x2dv")),
         UniformMatrix4x2fv: FnPtr::new(loadfn("glUniformMatrix4x2fv")),
         UniformMatrix4x3dv: FnPtr::new(loadfn("glUniformMatrix4x3dv")),
         UniformMatrix4x3fv: FnPtr::new(loadfn("glUniformMatrix4x3fv")),
         UniformSubroutinesuiv: FnPtr::new(loadfn("glUniformSubroutinesuiv")),
         UnmapBuffer: FnPtr::new(loadfn("glUnmapBuffer")),
         UnmapBufferARB: FnPtr::new(loadfn("glUnmapBufferARB")),
         UnmapNamedBuffer: FnPtr::new(loadfn("glUnmapNamedBuffer")),
         UnmapNamedBufferEXT: FnPtr::new(loadfn("glUnmapNamedBufferEXT")),
         UseProgram: FnPtr::new(loadfn("glUseProgram")),
         UseProgramObjectARB: FnPtr::new(loadfn("glUseProgramObjectARB")),
         UseProgramStages: FnPtr::new(loadfn("glUseProgramStages")),
         ValidateProgram: FnPtr::new(loadfn("glValidateProgram")),
         ValidateProgramARB: FnPtr::new(loadfn("glValidateProgramARB")),
         ValidateProgramPipeline: FnPtr::new(loadfn("glValidateProgramPipeline")),
         VertexArrayAttribBinding: FnPtr::new(loadfn("glVertexArrayAttribBinding")),
         VertexArrayAttribFormat: FnPtr::new(loadfn("glVertexArrayAttribFormat")),
         VertexArrayAttribIFormat: FnPtr::new(loadfn("glVertexArrayAttribIFormat")),
         VertexArrayAttribLFormat: FnPtr::new(loadfn("glVertexArrayAttribLFormat")),
         VertexArrayBindVertexBufferEXT: FnPtr::new(loadfn("glVertexArrayBindVertexBufferEXT")),
         VertexArrayBindingDivisor: FnPtr::new(loadfn("glVertexArrayBindingDivisor")),
         VertexArrayColorOffsetEXT: FnPtr::new(loadfn("glVertexArrayColorOffsetEXT")),
         VertexArrayEdgeFlagOffsetEXT: FnPtr::new(loadfn("glVertexArrayEdgeFlagOffsetEXT")),
         VertexArrayElementBuffer: FnPtr::new(loadfn("glVertexArrayElementBuffer")),
         VertexArrayFogCoordOffsetEXT: FnPtr::new(loadfn("glVertexArrayFogCoordOffsetEXT")),
         VertexArrayIndexOffsetEXT: FnPtr::new(loadfn("glVertexArrayIndexOffsetEXT")),
         VertexArrayMultiTexCoordOffsetEXT: FnPtr::new(loadfn("glVertexArrayMultiTexCoordOffsetEXT")),
         VertexArrayNormalOffsetEXT: FnPtr::new(loadfn("glVertexArrayNormalOffsetEXT")),
         VertexArraySecondaryColorOffsetEXT: FnPtr::new(loadfn("glVertexArraySecondaryColorOffsetEXT")),
         VertexArrayTexCoordOffsetEXT: FnPtr::new(loadfn("glVertexArrayTexCoordOffsetEXT")),
         VertexArrayVertexAttribBindingEXT: FnPtr::new(loadfn("glVertexArrayVertexAttribBindingEXT")),
         VertexArrayVertexAttribDivisorEXT: FnPtr::new(loadfn("glVertexArrayVertexAttribDivisorEXT")),
         VertexArrayVertexAttribFormatEXT: FnPtr::new(loadfn("glVertexArrayVertexAttribFormatEXT")),
         VertexArrayVertexAttribIFormatEXT: FnPtr::new(loadfn("glVertexArrayVertexAttribIFormatEXT")),
         VertexArrayVertexAttribIOffsetEXT: FnPtr::new(loadfn("glVertexArrayVertexAttribIOffsetEXT")),
         VertexArrayVertexAttribLFormatEXT: FnPtr::new(loadfn("glVertexArrayVertexAttribLFormatEXT")),
         VertexArrayVertexAttribLOffsetEXT: FnPtr::new(loadfn("glVertexArrayVertexAttribLOffsetEXT")),
         VertexArrayVertexAttribOffsetEXT: FnPtr::new(loadfn("glVertexArrayVertexAttribOffsetEXT")),
         VertexArrayVertexBindingDivisorEXT: FnPtr::new(loadfn("glVertexArrayVertexBindingDivisorEXT")),
         VertexArrayVertexBuffer: FnPtr::new(loadfn("glVertexArrayVertexBuffer")),
         VertexArrayVertexBuffers: FnPtr::new(loadfn("glVertexArrayVertexBuffers")),
         VertexArrayVertexOffsetEXT: FnPtr::new(loadfn("glVertexArrayVertexOffsetEXT")),
         VertexAttrib1d: FnPtr::new(loadfn("glVertexAttrib1d")),
         VertexAttrib1dARB: FnPtr::new(loadfn("glVertexAttrib1dARB")),
         VertexAttrib1dNV: FnPtr::new(loadfn("glVertexAttrib1dNV")),
         VertexAttrib1dv: FnPtr::new(loadfn("glVertexAttrib1dv")),
         VertexAttrib1dvARB: FnPtr::new(loadfn("glVertexAttrib1dvARB")),
         VertexAttrib1dvNV: FnPtr::new(loadfn("glVertexAttrib1dvNV")),
         VertexAttrib1f: FnPtr::new(loadfn("glVertexAttrib1f")),
         VertexAttrib1fARB: FnPtr::new(loadfn("glVertexAttrib1fARB")),
         VertexAttrib1fNV: FnPtr::new(loadfn("glVertexAttrib1fNV")),
         VertexAttrib1fv: FnPtr::new(loadfn("glVertexAttrib1fv")),
         VertexAttrib1fvARB: FnPtr::new(loadfn("glVertexAttrib1fvARB")),
         VertexAttrib1fvNV: FnPtr::new(loadfn("glVertexAttrib1fvNV")),
         VertexAttrib1s: FnPtr::new(loadfn("glVertexAttrib1s")),
         VertexAttrib1sARB: FnPtr::new(loadfn("glVertexAttrib1sARB")),
         VertexAttrib1sNV: FnPtr::new(loadfn("glVertexAttrib1sNV")),
         VertexAttrib1sv: FnPtr::new(loadfn("glVertexAttrib1sv")),
         VertexAttrib1svARB: FnPtr::new(loadfn("glVertexAttrib1svARB")),
         VertexAttrib1svNV: FnPtr::new(loadfn("glVertexAttrib1svNV")),
         VertexAttrib2d: FnPtr::new(loadfn("glVertexAttrib2d")),
         VertexAttrib2dARB: FnPtr::new(loadfn("glVertexAttrib2dARB")),
         VertexAttrib2dNV: FnPtr::new(loadfn("glVertexAttrib2dNV")),
         VertexAttrib2dv: FnPtr::new(loadfn("glVertexAttrib2dv")),
         VertexAttrib2dvARB: FnPtr::new(loadfn("glVertexAttrib2dvARB")),
         VertexAttrib2dvNV: FnPtr::new(loadfn("glVertexAttrib2dvNV")),
         VertexAttrib2f: FnPtr::new(loadfn("glVertexAttrib2f")),
         VertexAttrib2fARB: FnPtr::new(loadfn("glVertexAttrib2fARB")),
         VertexAttrib2fNV: FnPtr::new(loadfn("glVertexAttrib2fNV")),
         VertexAttrib2fv: FnPtr::new(loadfn("glVertexAttrib2fv")),
         VertexAttrib2fvARB: FnPtr::new(loadfn("glVertexAttrib2fvARB")),
         VertexAttrib2fvNV: FnPtr::new(loadfn("glVertexAttrib2fvNV")),
         VertexAttrib2s: FnPtr::new(loadfn("glVertexAttrib2s")),
         VertexAttrib2sARB: FnPtr::new(loadfn("glVertexAttrib2sARB")),
         VertexAttrib2sNV: FnPtr::new(loadfn("glVertexAttrib2sNV")),
         VertexAttrib2sv: FnPtr::new(loadfn("glVertexAttrib2sv")),
         VertexAttrib2svARB: FnPtr::new(loadfn("glVertexAttrib2svARB")),
         VertexAttrib2svNV: FnPtr::new(loadfn("glVertexAttrib2svNV")),
         VertexAttrib3d: FnPtr::new(loadfn("glVertexAttrib3d")),
         VertexAttrib3dARB: FnPtr::new(loadfn("glVertexAttrib3dARB")),
         VertexAttrib3dNV: FnPtr::new(loadfn("glVertexAttrib3dNV")),
         VertexAttrib3dv: FnPtr::new(loadfn("glVertexAttrib3dv")),
         VertexAttrib3dvARB: FnPtr::new(loadfn("glVertexAttrib3dvARB")),
         VertexAttrib3dvNV: FnPtr::new(loadfn("glVertexAttrib3dvNV")),
         VertexAttrib3f: FnPtr::new(loadfn("glVertexAttrib3f")),
         VertexAttrib3fARB: FnPtr::new(loadfn("glVertexAttrib3fARB")),
         VertexAttrib3fNV: FnPtr::new(loadfn("glVertexAttrib3fNV")),
         VertexAttrib3fv: FnPtr::new(loadfn("glVertexAttrib3fv")),
         VertexAttrib3fvARB: FnPtr::new(loadfn("glVertexAttrib3fvARB")),
         VertexAttrib3fvNV: FnPtr::new(loadfn("glVertexAttrib3fvNV")),
         VertexAttrib3s: FnPtr::new(loadfn("glVertexAttrib3s")),
         VertexAttrib3sARB: FnPtr::new(loadfn("glVertexAttrib3sARB")),
         VertexAttrib3sNV: FnPtr::new(loadfn("glVertexAttrib3sNV")),
         VertexAttrib3sv: FnPtr::new(loadfn("glVertexAttrib3sv")),
         VertexAttrib3svARB: FnPtr::new(loadfn("glVertexAttrib3svARB")),
         VertexAttrib3svNV: FnPtr::new(loadfn("glVertexAttrib3svNV")),
         VertexAttrib4Nbv: FnPtr::new(loadfn("glVertexAttrib4Nbv")),
         VertexAttrib4NbvARB: FnPtr::new(loadfn("glVertexAttrib4NbvARB")),
         VertexAttrib4Niv: FnPtr::new(loadfn("glVertexAttrib4Niv")),
         VertexAttrib4NivARB: FnPtr::new(loadfn("glVertexAttrib4NivARB")),
         VertexAttrib4Nsv: FnPtr::new(loadfn("glVertexAttrib4Nsv")),
         VertexAttrib4NsvARB: FnPtr::new(loadfn("glVertexAttrib4NsvARB")),
         VertexAttrib4Nub: FnPtr::new(loadfn("glVertexAttrib4Nub")),
         VertexAttrib4NubARB: FnPtr::new(loadfn("glVertexAttrib4NubARB")),
         VertexAttrib4Nubv: FnPtr::new(loadfn("glVertexAttrib4Nubv")),
         VertexAttrib4NubvARB: FnPtr::new(loadfn("glVertexAttrib4NubvARB")),
         VertexAttrib4Nuiv: FnPtr::new(loadfn("glVertexAttrib4Nuiv")),
         VertexAttrib4NuivARB: FnPtr::new(loadfn("glVertexAttrib4NuivARB")),
         VertexAttrib4Nusv: FnPtr::new(loadfn("glVertexAttrib4Nusv")),
         VertexAttrib4NusvARB: FnPtr::new(loadfn("glVertexAttrib4NusvARB")),
         VertexAttrib4bv: FnPtr::new(loadfn("glVertexAttrib4bv")),
         VertexAttrib4bvARB: FnPtr::new(loadfn("glVertexAttrib4bvARB")),
         VertexAttrib4d: FnPtr::new(loadfn("glVertexAttrib4d")),
         VertexAttrib4dARB: FnPtr::new(loadfn("glVertexAttrib4dARB")),
         VertexAttrib4dNV: FnPtr::new(loadfn("glVertexAttrib4dNV")),
         VertexAttrib4dv: FnPtr::new(loadfn("glVertexAttrib4dv")),
         VertexAttrib4dvARB: FnPtr::new(loadfn("glVertexAttrib4dvARB")),
         VertexAttrib4dvNV: FnPtr::new(loadfn("glVertexAttrib4dvNV")),
         VertexAttrib4f: FnPtr::new(loadfn("glVertexAttrib4f")),
         VertexAttrib4fARB: FnPtr::new(loadfn("glVertexAttrib4fARB")),
         VertexAttrib4fNV: FnPtr::new(loadfn("glVertexAttrib4fNV")),
         VertexAttrib4fv: FnPtr::new(loadfn("glVertexAttrib4fv")),
         VertexAttrib4fvARB: FnPtr::new(loadfn("glVertexAttrib4fvARB")),
         VertexAttrib4fvNV: FnPtr::new(loadfn("glVertexAttrib4fvNV")),
         VertexAttrib4iv: FnPtr::new(loadfn("glVertexAttrib4iv")),
         VertexAttrib4ivARB: FnPtr::new(loadfn("glVertexAttrib4ivARB")),
         VertexAttrib4s: FnPtr::new(loadfn("glVertexAttrib4s")),
         VertexAttrib4sARB: FnPtr::new(loadfn("glVertexAttrib4sARB")),
         VertexAttrib4sNV: FnPtr::new(loadfn("glVertexAttrib4sNV")),
         VertexAttrib4sv: FnPtr::new(loadfn("glVertexAttrib4sv")),
         VertexAttrib4svARB: FnPtr::new(loadfn("glVertexAttrib4svARB")),
         VertexAttrib4svNV: FnPtr::new(loadfn("glVertexAttrib4svNV")),
         VertexAttrib4ubNV: FnPtr::new(loadfn("glVertexAttrib4ubNV")),
         VertexAttrib4ubv: FnPtr::new(loadfn("glVertexAttrib4ubv")),
         VertexAttrib4ubvARB: FnPtr::new(loadfn("glVertexAttrib4ubvARB")),
         VertexAttrib4ubvNV: FnPtr::new(loadfn("glVertexAttrib4ubvNV")),
         VertexAttrib4uiv: FnPtr::new(loadfn("glVertexAttrib4uiv")),
         VertexAttrib4uivARB: FnPtr::new(loadfn("glVertexAttrib4uivARB")),
         VertexAttrib4usv: FnPtr::new(loadfn("glVertexAttrib4usv")),
         VertexAttrib4usvARB: FnPtr::new(loadfn("glVertexAttrib4usvARB")),
         VertexAttribBinding: FnPtr::new(loadfn("glVertexAttribBinding")),
         VertexAttribDivisor: FnPtr::new(loadfn("glVertexAttribDivisor")),
         VertexAttribDivisorARB: FnPtr::new(loadfn("glVertexAttribDivisorARB")),
         VertexAttribFormat: FnPtr::new(loadfn("glVertexAttribFormat")),
         VertexAttribI1i: FnPtr::new(loadfn("glVertexAttribI1i")),
         VertexAttribI1iEXT: FnPtr::new(loadfn("glVertexAttribI1iEXT")),
         VertexAttribI1iv: FnPtr::new(loadfn("glVertexAttribI1iv")),
         VertexAttribI1ivEXT: FnPtr::new(loadfn("glVertexAttribI1ivEXT")),
         VertexAttribI1ui: FnPtr::new(loadfn("glVertexAttribI1ui")),
         VertexAttribI1uiEXT: FnPtr::new(loadfn("glVertexAttribI1uiEXT")),
         VertexAttribI1uiv: FnPtr::new(loadfn("glVertexAttribI1uiv")),
         VertexAttribI1uivEXT: FnPtr::new(loadfn("glVertexAttribI1uivEXT")),
         VertexAttribI2i: FnPtr::new(loadfn("glVertexAttribI2i")),
         VertexAttribI2iEXT: FnPtr::new(loadfn("glVertexAttribI2iEXT")),
         VertexAttribI2iv: FnPtr::new(loadfn("glVertexAttribI2iv")),
         VertexAttribI2ivEXT: FnPtr::new(loadfn("glVertexAttribI2ivEXT")),
         VertexAttribI2ui: FnPtr::new(loadfn("glVertexAttribI2ui")),
         VertexAttribI2uiEXT: FnPtr::new(loadfn("glVertexAttribI2uiEXT")),
         VertexAttribI2uiv: FnPtr::new(loadfn("glVertexAttribI2uiv")),
         VertexAttribI2uivEXT: FnPtr::new(loadfn("glVertexAttribI2uivEXT")),
         VertexAttribI3i: FnPtr::new(loadfn("glVertexAttribI3i")),
         VertexAttribI3iEXT: FnPtr::new(loadfn("glVertexAttribI3iEXT")),
         VertexAttribI3iv: FnPtr::new(loadfn("glVertexAttribI3iv")),
         VertexAttribI3ivEXT: FnPtr::new(loadfn("glVertexAttribI3ivEXT")),
         VertexAttribI3ui: FnPtr::new(loadfn("glVertexAttribI3ui")),
         VertexAttribI3uiEXT: FnPtr::new(loadfn("glVertexAttribI3uiEXT")),
         VertexAttribI3uiv: FnPtr::new(loadfn("glVertexAttribI3uiv")),
         VertexAttribI3uivEXT: FnPtr::new(loadfn("glVertexAttribI3uivEXT")),
         VertexAttribI4bv: FnPtr::new(loadfn("glVertexAttribI4bv")),
         VertexAttribI4bvEXT: FnPtr::new(loadfn("glVertexAttribI4bvEXT")),
         VertexAttribI4i: FnPtr::new(loadfn("glVertexAttribI4i")),
         VertexAttribI4iEXT: FnPtr::new(loadfn("glVertexAttribI4iEXT")),
         VertexAttribI4iv: FnPtr::new(loadfn("glVertexAttribI4iv")),
         VertexAttribI4ivEXT: FnPtr::new(loadfn("glVertexAttribI4ivEXT")),
         VertexAttribI4sv: FnPtr::new(loadfn("glVertexAttribI4sv")),
         VertexAttribI4svEXT: FnPtr::new(loadfn("glVertexAttribI4svEXT")),
         VertexAttribI4ubv: FnPtr::new(loadfn("glVertexAttribI4ubv")),
         VertexAttribI4ubvEXT: FnPtr::new(loadfn("glVertexAttribI4ubvEXT")),
         VertexAttribI4ui: FnPtr::new(loadfn("glVertexAttribI4ui")),
         VertexAttribI4uiEXT: FnPtr::new(loadfn("glVertexAttribI4uiEXT")),
         VertexAttribI4uiv: FnPtr::new(loadfn("glVertexAttribI4uiv")),
         VertexAttribI4uivEXT: FnPtr::new(loadfn("glVertexAttribI4uivEXT")),
         VertexAttribI4usv: FnPtr::new(loadfn("glVertexAttribI4usv")),
         VertexAttribI4usvEXT: FnPtr::new(loadfn("glVertexAttribI4usvEXT")),
         VertexAttribIFormat: FnPtr::new(loadfn("glVertexAttribIFormat")),
         VertexAttribIPointer: FnPtr::new(loadfn("glVertexAttribIPointer")),
         VertexAttribIPointerEXT: FnPtr::new(loadfn("glVertexAttribIPointerEXT")),
         VertexAttribL1d: FnPtr::new(loadfn("glVertexAttribL1d")),
         VertexAttribL1dEXT: FnPtr::new(loadfn("glVertexAttribL1dEXT")),
         VertexAttribL1dv: FnPtr::new(loadfn("glVertexAttribL1dv")),
         VertexAttribL1dvEXT: FnPtr::new(loadfn("glVertexAttribL1dvEXT")),
         VertexAttribL2d: FnPtr::new(loadfn("glVertexAttribL2d")),
         VertexAttribL2dEXT: FnPtr::new(loadfn("glVertexAttribL2dEXT")),
         VertexAttribL2dv: FnPtr::new(loadfn("glVertexAttribL2dv")),
         VertexAttribL2dvEXT: FnPtr::new(loadfn("glVertexAttribL2dvEXT")),
         VertexAttribL3d: FnPtr::new(loadfn("glVertexAttribL3d")),
         VertexAttribL3dEXT: FnPtr::new(loadfn("glVertexAttribL3dEXT")),
         VertexAttribL3dv: FnPtr::new(loadfn("glVertexAttribL3dv")),
         VertexAttribL3dvEXT: FnPtr::new(loadfn("glVertexAttribL3dvEXT")),
         VertexAttribL4d: FnPtr::new(loadfn("glVertexAttribL4d")),
         VertexAttribL4dEXT: FnPtr::new(loadfn("glVertexAttribL4dEXT")),
         VertexAttribL4dv: FnPtr::new(loadfn("glVertexAttribL4dv")),
         VertexAttribL4dvEXT: FnPtr::new(loadfn("glVertexAttribL4dvEXT")),
         VertexAttribLFormat: FnPtr::new(loadfn("glVertexAttribLFormat")),
         VertexAttribLPointer: FnPtr::new(loadfn("glVertexAttribLPointer")),
         VertexAttribLPointerEXT: FnPtr::new(loadfn("glVertexAttribLPointerEXT")),
         VertexAttribP1ui: FnPtr::new(loadfn("glVertexAttribP1ui")),
         VertexAttribP1uiv: FnPtr::new(loadfn("glVertexAttribP1uiv")),
         VertexAttribP2ui: FnPtr::new(loadfn("glVertexAttribP2ui")),
         VertexAttribP2uiv: FnPtr::new(loadfn("glVertexAttribP2uiv")),
         VertexAttribP3ui: FnPtr::new(loadfn("glVertexAttribP3ui")),
         VertexAttribP3uiv: FnPtr::new(loadfn("glVertexAttribP3uiv")),
         VertexAttribP4ui: FnPtr::new(loadfn("glVertexAttribP4ui")),
         VertexAttribP4uiv: FnPtr::new(loadfn("glVertexAttribP4uiv")),
         VertexAttribPointer: FnPtr::new(loadfn("glVertexAttribPointer")),
         VertexAttribPointerARB: FnPtr::new(loadfn("glVertexAttribPointerARB")),
         VertexAttribPointerNV: FnPtr::new(loadfn("glVertexAttribPointerNV")),
         VertexAttribs1dvNV: FnPtr::new(loadfn("glVertexAttribs1dvNV")),
         VertexAttribs1fvNV: FnPtr::new(loadfn("glVertexAttribs1fvNV")),
         VertexAttribs1svNV: FnPtr::new(loadfn("glVertexAttribs1svNV")),
         VertexAttribs2dvNV: FnPtr::new(loadfn("glVertexAttribs2dvNV")),
         VertexAttribs2fvNV: FnPtr::new(loadfn("glVertexAttribs2fvNV")),
         VertexAttribs2svNV: FnPtr::new(loadfn("glVertexAttribs2svNV")),
         VertexAttribs3dvNV: FnPtr::new(loadfn("glVertexAttribs3dvNV")),
         VertexAttribs3fvNV: FnPtr::new(loadfn("glVertexAttribs3fvNV")),
         VertexAttribs3svNV: FnPtr::new(loadfn("glVertexAttribs3svNV")),
         VertexAttribs4dvNV: FnPtr::new(loadfn("glVertexAttribs4dvNV")),
         VertexAttribs4fvNV: FnPtr::new(loadfn("glVertexAttribs4fvNV")),
         VertexAttribs4svNV: FnPtr::new(loadfn("glVertexAttribs4svNV")),
         VertexAttribs4ubvNV: FnPtr::new(loadfn("glVertexAttribs4ubvNV")),
         VertexBindingDivisor: FnPtr::new(loadfn("glVertexBindingDivisor")),
         VertexPointerEXT: FnPtr::new(loadfn("glVertexPointerEXT")),
         Viewport: FnPtr::new(loadfn("glViewport")),
         ViewportArrayv: FnPtr::new(loadfn("glViewportArrayv")),
         ViewportIndexedf: FnPtr::new(loadfn("glViewportIndexedf")),
         ViewportIndexedfv: FnPtr::new(loadfn("glViewportIndexedfv")),
         WaitSync: FnPtr::new(loadfn("glWaitSync")),
    };

     ctx.ActiveTexture.aliased(&ctx.ActiveTextureARB);
     ctx.ActiveTextureARB.aliased(&ctx.ActiveTexture);
     ctx.AttachObjectARB.aliased(&ctx.AttachShader);
     ctx.AttachShader.aliased(&ctx.AttachObjectARB);
     ctx.BeginConditionalRender.aliased(&ctx.BeginConditionalRenderNV);
     ctx.BeginConditionalRenderNV.aliased(&ctx.BeginConditionalRender);
     ctx.BeginQuery.aliased(&ctx.BeginQueryARB);
     ctx.BeginQueryARB.aliased(&ctx.BeginQuery);
     ctx.BeginTransformFeedback.aliased(&ctx.BeginTransformFeedbackEXT);
     ctx.BeginTransformFeedback.aliased(&ctx.BeginTransformFeedbackNV);
     ctx.BeginTransformFeedbackEXT.aliased(&ctx.BeginTransformFeedback);
     ctx.BeginTransformFeedbackEXT.aliased(&ctx.BeginTransformFeedbackNV);
     ctx.BeginTransformFeedbackNV.aliased(&ctx.BeginTransformFeedback);
     ctx.BeginTransformFeedbackNV.aliased(&ctx.BeginTransformFeedbackEXT);
     ctx.BindAttribLocation.aliased(&ctx.BindAttribLocationARB);
     ctx.BindAttribLocationARB.aliased(&ctx.BindAttribLocation);
     ctx.BindBuffer.aliased(&ctx.BindBufferARB);
     ctx.BindBufferARB.aliased(&ctx.BindBuffer);
     ctx.BindBufferBase.aliased(&ctx.BindBufferBaseEXT);
     ctx.BindBufferBase.aliased(&ctx.BindBufferBaseNV);
     ctx.BindBufferBaseEXT.aliased(&ctx.BindBufferBase);
     ctx.BindBufferBaseEXT.aliased(&ctx.BindBufferBaseNV);
     ctx.BindBufferBaseNV.aliased(&ctx.BindBufferBase);
     ctx.BindBufferBaseNV.aliased(&ctx.BindBufferBaseEXT);
     ctx.BindBufferOffsetEXT.aliased(&ctx.BindBufferOffsetNV);
     ctx.BindBufferOffsetNV.aliased(&ctx.BindBufferOffsetEXT);
     ctx.BindBufferRange.aliased(&ctx.BindBufferRangeEXT);
     ctx.BindBufferRange.aliased(&ctx.BindBufferRangeNV);
     ctx.BindBufferRangeEXT.aliased(&ctx.BindBufferRange);
     ctx.BindBufferRangeEXT.aliased(&ctx.BindBufferRangeNV);
     ctx.BindBufferRangeNV.aliased(&ctx.BindBufferRange);
     ctx.BindBufferRangeNV.aliased(&ctx.BindBufferRangeEXT);
     ctx.BindFragDataLocation.aliased(&ctx.BindFragDataLocationEXT);
     ctx.BindFragDataLocationEXT.aliased(&ctx.BindFragDataLocation);
     ctx.BindProgramARB.aliased(&ctx.BindProgramNV);
     ctx.BindProgramNV.aliased(&ctx.BindProgramARB);
     ctx.BindTexture.aliased(&ctx.BindTextureEXT);
     ctx.BindTextureEXT.aliased(&ctx.BindTexture);
     ctx.BlendColor.aliased(&ctx.BlendColorEXT);
     ctx.BlendColorEXT.aliased(&ctx.BlendColor);
     ctx.BlendEquation.aliased(&ctx.BlendEquationEXT);
     ctx.BlendEquationEXT.aliased(&ctx.BlendEquation);
     ctx.BlendEquationi.aliased(&ctx.BlendEquationIndexedAMD);
     ctx.BlendEquationi.aliased(&ctx.BlendEquationiARB);
     ctx.BlendEquationiARB.aliased(&ctx.BlendEquationIndexedAMD);
     ctx.BlendEquationiARB.aliased(&ctx.BlendEquationi);
     ctx.BlendEquationIndexedAMD.aliased(&ctx.BlendEquationi);
     ctx.BlendEquationIndexedAMD.aliased(&ctx.BlendEquationiARB);
     ctx.BlendEquationSeparate.aliased(&ctx.BlendEquationSeparateEXT);
     ctx.BlendEquationSeparateEXT.aliased(&ctx.BlendEquationSeparate);
     ctx.BlendEquationSeparatei.aliased(&ctx.BlendEquationSeparateIndexedAMD);
     ctx.BlendEquationSeparatei.aliased(&ctx.BlendEquationSeparateiARB);
     ctx.BlendEquationSeparateiARB.aliased(&ctx.BlendEquationSeparateIndexedAMD);
     ctx.BlendEquationSeparateiARB.aliased(&ctx.BlendEquationSeparatei);
     ctx.BlendEquationSeparateIndexedAMD.aliased(&ctx.BlendEquationSeparatei);
     ctx.BlendEquationSeparateIndexedAMD.aliased(&ctx.BlendEquationSeparateiARB);
     ctx.BlendFunci.aliased(&ctx.BlendFuncIndexedAMD);
     ctx.BlendFunci.aliased(&ctx.BlendFunciARB);
     ctx.BlendFunciARB.aliased(&ctx.BlendFuncIndexedAMD);
     ctx.BlendFunciARB.aliased(&ctx.BlendFunci);
     ctx.BlendFuncIndexedAMD.aliased(&ctx.BlendFunci);
     ctx.BlendFuncIndexedAMD.aliased(&ctx.BlendFunciARB);
     ctx.BlendFuncSeparate.aliased(&ctx.BlendFuncSeparateEXT);
     ctx.BlendFuncSeparate.aliased(&ctx.BlendFuncSeparateINGR);
     ctx.BlendFuncSeparateEXT.aliased(&ctx.BlendFuncSeparate);
     ctx.BlendFuncSeparateEXT.aliased(&ctx.BlendFuncSeparateINGR);
     ctx.BlendFuncSeparatei.aliased(&ctx.BlendFuncSeparateIndexedAMD);
     ctx.BlendFuncSeparatei.aliased(&ctx.BlendFuncSeparateiARB);
     ctx.BlendFuncSeparateiARB.aliased(&ctx.BlendFuncSeparateIndexedAMD);
     ctx.BlendFuncSeparateiARB.aliased(&ctx.BlendFuncSeparatei);
     ctx.BlendFuncSeparateIndexedAMD.aliased(&ctx.BlendFuncSeparatei);
     ctx.BlendFuncSeparateIndexedAMD.aliased(&ctx.BlendFuncSeparateiARB);
     ctx.BlendFuncSeparateINGR.aliased(&ctx.BlendFuncSeparate);
     ctx.BlendFuncSeparateINGR.aliased(&ctx.BlendFuncSeparateEXT);
     ctx.BlitFramebuffer.aliased(&ctx.BlitFramebufferEXT);
     ctx.BlitFramebufferEXT.aliased(&ctx.BlitFramebuffer);
     ctx.BufferData.aliased(&ctx.BufferDataARB);
     ctx.BufferDataARB.aliased(&ctx.BufferData);
     ctx.BufferSubData.aliased(&ctx.BufferSubDataARB);
     ctx.BufferSubDataARB.aliased(&ctx.BufferSubData);
     ctx.CheckFramebufferStatus.aliased(&ctx.CheckFramebufferStatusEXT);
     ctx.CheckFramebufferStatusEXT.aliased(&ctx.CheckFramebufferStatus);
     ctx.ClampColor.aliased(&ctx.ClampColorARB);
     ctx.ClampColorARB.aliased(&ctx.ClampColor);
     ctx.ClearDepthf.aliased(&ctx.ClearDepthfOES);
     ctx.ClearDepthfOES.aliased(&ctx.ClearDepthf);
     ctx.ColorMaski.aliased(&ctx.ColorMaskIndexedEXT);
     ctx.ColorMaskIndexedEXT.aliased(&ctx.ColorMaski);
     ctx.CompileShader.aliased(&ctx.CompileShaderARB);
     ctx.CompileShaderARB.aliased(&ctx.CompileShader);
     ctx.CompressedTexImage1D.aliased(&ctx.CompressedTexImage1DARB);
     ctx.CompressedTexImage1DARB.aliased(&ctx.CompressedTexImage1D);
     ctx.CompressedTexImage2D.aliased(&ctx.CompressedTexImage2DARB);
     ctx.CompressedTexImage2DARB.aliased(&ctx.CompressedTexImage2D);
     ctx.CompressedTexImage3D.aliased(&ctx.CompressedTexImage3DARB);
     ctx.CompressedTexImage3DARB.aliased(&ctx.CompressedTexImage3D);
     ctx.CompressedTexSubImage1D.aliased(&ctx.CompressedTexSubImage1DARB);
     ctx.CompressedTexSubImage1DARB.aliased(&ctx.CompressedTexSubImage1D);
     ctx.CompressedTexSubImage2D.aliased(&ctx.CompressedTexSubImage2DARB);
     ctx.CompressedTexSubImage2DARB.aliased(&ctx.CompressedTexSubImage2D);
     ctx.CompressedTexSubImage3D.aliased(&ctx.CompressedTexSubImage3DARB);
     ctx.CompressedTexSubImage3DARB.aliased(&ctx.CompressedTexSubImage3D);
     ctx.CopyTexImage1D.aliased(&ctx.CopyTexImage1DEXT);
     ctx.CopyTexImage1DEXT.aliased(&ctx.CopyTexImage1D);
     ctx.CopyTexImage2D.aliased(&ctx.CopyTexImage2DEXT);
     ctx.CopyTexImage2DEXT.aliased(&ctx.CopyTexImage2D);
     ctx.CopyTexSubImage1D.aliased(&ctx.CopyTexSubImage1DEXT);
     ctx.CopyTexSubImage1DEXT.aliased(&ctx.CopyTexSubImage1D);
     ctx.CopyTexSubImage2D.aliased(&ctx.CopyTexSubImage2DEXT);
     ctx.CopyTexSubImage2DEXT.aliased(&ctx.CopyTexSubImage2D);
     ctx.CopyTexSubImage3D.aliased(&ctx.CopyTexSubImage3DEXT);
     ctx.CopyTexSubImage3DEXT.aliased(&ctx.CopyTexSubImage3D);
     ctx.CreateProgram.aliased(&ctx.CreateProgramObjectARB);
     ctx.CreateProgramObjectARB.aliased(&ctx.CreateProgram);
     ctx.CreateShader.aliased(&ctx.CreateShaderObjectARB);
     ctx.CreateShaderObjectARB.aliased(&ctx.CreateShader);
     ctx.DebugMessageCallback.aliased(&ctx.DebugMessageCallbackARB);
     ctx.DebugMessageCallbackARB.aliased(&ctx.DebugMessageCallback);
     ctx.DebugMessageControl.aliased(&ctx.DebugMessageControlARB);
     ctx.DebugMessageControlARB.aliased(&ctx.DebugMessageControl);
     ctx.DebugMessageInsert.aliased(&ctx.DebugMessageInsertARB);
     ctx.DebugMessageInsertARB.aliased(&ctx.DebugMessageInsert);
     ctx.DeleteBuffers.aliased(&ctx.DeleteBuffersARB);
     ctx.DeleteBuffersARB.aliased(&ctx.DeleteBuffers);
     ctx.DeleteFramebuffers.aliased(&ctx.DeleteFramebuffersEXT);
     ctx.DeleteFramebuffersEXT.aliased(&ctx.DeleteFramebuffers);
     ctx.DeleteProgramsARB.aliased(&ctx.DeleteProgramsNV);
     ctx.DeleteProgramsNV.aliased(&ctx.DeleteProgramsARB);
     ctx.DeleteQueries.aliased(&ctx.DeleteQueriesARB);
     ctx.DeleteQueriesARB.aliased(&ctx.DeleteQueries);
     ctx.DeleteRenderbuffers.aliased(&ctx.DeleteRenderbuffersEXT);
     ctx.DeleteRenderbuffersEXT.aliased(&ctx.DeleteRenderbuffers);
     ctx.DeleteTransformFeedbacks.aliased(&ctx.DeleteTransformFeedbacksNV);
     ctx.DeleteTransformFeedbacksNV.aliased(&ctx.DeleteTransformFeedbacks);
     ctx.DeleteVertexArrays.aliased(&ctx.DeleteVertexArraysAPPLE);
     ctx.DeleteVertexArraysAPPLE.aliased(&ctx.DeleteVertexArrays);
     ctx.DepthRangef.aliased(&ctx.DepthRangefOES);
     ctx.DepthRangefOES.aliased(&ctx.DepthRangef);
     ctx.DetachObjectARB.aliased(&ctx.DetachShader);
     ctx.DetachShader.aliased(&ctx.DetachObjectARB);
     ctx.Disablei.aliased(&ctx.DisableIndexedEXT);
     ctx.DisableIndexedEXT.aliased(&ctx.Disablei);
     ctx.DisableVertexAttribArray.aliased(&ctx.DisableVertexAttribArrayARB);
     ctx.DisableVertexAttribArrayARB.aliased(&ctx.DisableVertexAttribArray);
     ctx.DrawArrays.aliased(&ctx.DrawArraysEXT);
     ctx.DrawArraysEXT.aliased(&ctx.DrawArrays);
     ctx.DrawArraysInstanced.aliased(&ctx.DrawArraysInstancedARB);
     ctx.DrawArraysInstanced.aliased(&ctx.DrawArraysInstancedEXT);
     ctx.DrawArraysInstancedARB.aliased(&ctx.DrawArraysInstanced);
     ctx.DrawArraysInstancedARB.aliased(&ctx.DrawArraysInstancedEXT);
     ctx.DrawArraysInstancedEXT.aliased(&ctx.DrawArraysInstanced);
     ctx.DrawArraysInstancedEXT.aliased(&ctx.DrawArraysInstancedARB);
     ctx.DrawBuffers.aliased(&ctx.DrawBuffersARB);
     ctx.DrawBuffers.aliased(&ctx.DrawBuffersATI);
     ctx.DrawBuffersARB.aliased(&ctx.DrawBuffers);
     ctx.DrawBuffersARB.aliased(&ctx.DrawBuffersATI);
     ctx.DrawBuffersATI.aliased(&ctx.DrawBuffers);
     ctx.DrawBuffersATI.aliased(&ctx.DrawBuffersARB);
     ctx.DrawElementsInstanced.aliased(&ctx.DrawElementsInstancedARB);
     ctx.DrawElementsInstanced.aliased(&ctx.DrawElementsInstancedEXT);
     ctx.DrawElementsInstancedARB.aliased(&ctx.DrawElementsInstanced);
     ctx.DrawElementsInstancedARB.aliased(&ctx.DrawElementsInstancedEXT);
     ctx.DrawElementsInstancedEXT.aliased(&ctx.DrawElementsInstanced);
     ctx.DrawElementsInstancedEXT.aliased(&ctx.DrawElementsInstancedARB);
     ctx.DrawRangeElements.aliased(&ctx.DrawRangeElementsEXT);
     ctx.DrawRangeElementsEXT.aliased(&ctx.DrawRangeElements);
     ctx.DrawTransformFeedback.aliased(&ctx.DrawTransformFeedbackNV);
     ctx.DrawTransformFeedbackNV.aliased(&ctx.DrawTransformFeedback);
     ctx.Enablei.aliased(&ctx.EnableIndexedEXT);
     ctx.EnableIndexedEXT.aliased(&ctx.Enablei);
     ctx.EnableVertexAttribArray.aliased(&ctx.EnableVertexAttribArrayARB);
     ctx.EnableVertexAttribArrayARB.aliased(&ctx.EnableVertexAttribArray);
     ctx.EndConditionalRender.aliased(&ctx.EndConditionalRenderNV);
     ctx.EndConditionalRender.aliased(&ctx.EndConditionalRenderNVX);
     ctx.EndConditionalRenderNV.aliased(&ctx.EndConditionalRender);
     ctx.EndConditionalRenderNV.aliased(&ctx.EndConditionalRenderNVX);
     ctx.EndConditionalRenderNVX.aliased(&ctx.EndConditionalRender);
     ctx.EndConditionalRenderNVX.aliased(&ctx.EndConditionalRenderNV);
     ctx.EndQuery.aliased(&ctx.EndQueryARB);
     ctx.EndQueryARB.aliased(&ctx.EndQuery);
     ctx.EndTransformFeedback.aliased(&ctx.EndTransformFeedbackEXT);
     ctx.EndTransformFeedback.aliased(&ctx.EndTransformFeedbackNV);
     ctx.EndTransformFeedbackEXT.aliased(&ctx.EndTransformFeedback);
     ctx.EndTransformFeedbackEXT.aliased(&ctx.EndTransformFeedbackNV);
     ctx.EndTransformFeedbackNV.aliased(&ctx.EndTransformFeedback);
     ctx.EndTransformFeedbackNV.aliased(&ctx.EndTransformFeedbackEXT);
     ctx.FlushMappedBufferRange.aliased(&ctx.FlushMappedBufferRangeAPPLE);
     ctx.FlushMappedBufferRangeAPPLE.aliased(&ctx.FlushMappedBufferRange);
     ctx.FramebufferRenderbuffer.aliased(&ctx.FramebufferRenderbufferEXT);
     ctx.FramebufferRenderbufferEXT.aliased(&ctx.FramebufferRenderbuffer);
     ctx.FramebufferTexture.aliased(&ctx.FramebufferTextureARB);
     ctx.FramebufferTexture.aliased(&ctx.FramebufferTextureEXT);
     ctx.FramebufferTexture1D.aliased(&ctx.FramebufferTexture1DEXT);
     ctx.FramebufferTexture1DEXT.aliased(&ctx.FramebufferTexture1D);
     ctx.FramebufferTexture2D.aliased(&ctx.FramebufferTexture2DEXT);
     ctx.FramebufferTexture2DEXT.aliased(&ctx.FramebufferTexture2D);
     ctx.FramebufferTexture3D.aliased(&ctx.FramebufferTexture3DEXT);
     ctx.FramebufferTexture3DEXT.aliased(&ctx.FramebufferTexture3D);
     ctx.FramebufferTextureARB.aliased(&ctx.FramebufferTexture);
     ctx.FramebufferTextureARB.aliased(&ctx.FramebufferTextureEXT);
     ctx.FramebufferTextureEXT.aliased(&ctx.FramebufferTexture);
     ctx.FramebufferTextureEXT.aliased(&ctx.FramebufferTextureARB);
     ctx.FramebufferTextureFaceARB.aliased(&ctx.FramebufferTextureFaceEXT);
     ctx.FramebufferTextureFaceEXT.aliased(&ctx.FramebufferTextureFaceARB);
     ctx.FramebufferTextureLayer.aliased(&ctx.FramebufferTextureLayerARB);
     ctx.FramebufferTextureLayer.aliased(&ctx.FramebufferTextureLayerEXT);
     ctx.FramebufferTextureLayerARB.aliased(&ctx.FramebufferTextureLayer);
     ctx.FramebufferTextureLayerARB.aliased(&ctx.FramebufferTextureLayerEXT);
     ctx.FramebufferTextureLayerEXT.aliased(&ctx.FramebufferTextureLayer);
     ctx.FramebufferTextureLayerEXT.aliased(&ctx.FramebufferTextureLayerARB);
     ctx.GenBuffers.aliased(&ctx.GenBuffersARB);
     ctx.GenBuffersARB.aliased(&ctx.GenBuffers);
     ctx.GenerateMipmap.aliased(&ctx.GenerateMipmapEXT);
     ctx.GenerateMipmapEXT.aliased(&ctx.GenerateMipmap);
     ctx.GenFramebuffers.aliased(&ctx.GenFramebuffersEXT);
     ctx.GenFramebuffersEXT.aliased(&ctx.GenFramebuffers);
     ctx.GenProgramsARB.aliased(&ctx.GenProgramsNV);
     ctx.GenProgramsNV.aliased(&ctx.GenProgramsARB);
     ctx.GenQueries.aliased(&ctx.GenQueriesARB);
     ctx.GenQueriesARB.aliased(&ctx.GenQueries);
     ctx.GenRenderbuffers.aliased(&ctx.GenRenderbuffersEXT);
     ctx.GenRenderbuffersEXT.aliased(&ctx.GenRenderbuffers);
     ctx.GenTransformFeedbacks.aliased(&ctx.GenTransformFeedbacksNV);
     ctx.GenTransformFeedbacksNV.aliased(&ctx.GenTransformFeedbacks);
     ctx.GenVertexArrays.aliased(&ctx.GenVertexArraysAPPLE);
     ctx.GenVertexArraysAPPLE.aliased(&ctx.GenVertexArrays);
     ctx.GetActiveAttrib.aliased(&ctx.GetActiveAttribARB);
     ctx.GetActiveAttribARB.aliased(&ctx.GetActiveAttrib);
     ctx.GetActiveUniform.aliased(&ctx.GetActiveUniformARB);
     ctx.GetActiveUniformARB.aliased(&ctx.GetActiveUniform);
     ctx.GetAttribLocation.aliased(&ctx.GetAttribLocationARB);
     ctx.GetAttribLocationARB.aliased(&ctx.GetAttribLocation);
     ctx.GetBooleani_v.aliased(&ctx.GetBooleanIndexedvEXT);
     ctx.GetBooleanIndexedvEXT.aliased(&ctx.GetBooleani_v);
     ctx.GetBufferParameteriv.aliased(&ctx.GetBufferParameterivARB);
     ctx.GetBufferParameterivARB.aliased(&ctx.GetBufferParameteriv);
     ctx.GetBufferPointerv.aliased(&ctx.GetBufferPointervARB);
     ctx.GetBufferPointervARB.aliased(&ctx.GetBufferPointerv);
     ctx.GetBufferSubData.aliased(&ctx.GetBufferSubDataARB);
     ctx.GetBufferSubDataARB.aliased(&ctx.GetBufferSubData);
     ctx.GetCompressedTexImage.aliased(&ctx.GetCompressedTexImageARB);
     ctx.GetCompressedTexImageARB.aliased(&ctx.GetCompressedTexImage);
     ctx.GetDebugMessageLog.aliased(&ctx.GetDebugMessageLogARB);
     ctx.GetDebugMessageLogARB.aliased(&ctx.GetDebugMessageLog);
     ctx.GetDoublei_v.aliased(&ctx.GetDoubleIndexedvEXT);
     ctx.GetDoublei_v.aliased(&ctx.GetDoublei_vEXT);
     ctx.GetDoublei_vEXT.aliased(&ctx.GetDoubleIndexedvEXT);
     ctx.GetDoublei_vEXT.aliased(&ctx.GetDoublei_v);
     ctx.GetDoubleIndexedvEXT.aliased(&ctx.GetDoublei_v);
     ctx.GetDoubleIndexedvEXT.aliased(&ctx.GetDoublei_vEXT);
     ctx.GetFloati_v.aliased(&ctx.GetFloatIndexedvEXT);
     ctx.GetFloati_v.aliased(&ctx.GetFloati_vEXT);
     ctx.GetFloati_vEXT.aliased(&ctx.GetFloatIndexedvEXT);
     ctx.GetFloati_vEXT.aliased(&ctx.GetFloati_v);
     ctx.GetFloatIndexedvEXT.aliased(&ctx.GetFloati_v);
     ctx.GetFloatIndexedvEXT.aliased(&ctx.GetFloati_vEXT);
     ctx.GetFragDataLocation.aliased(&ctx.GetFragDataLocationEXT);
     ctx.GetFragDataLocationEXT.aliased(&ctx.GetFragDataLocation);
     ctx.GetFramebufferAttachmentParameteriv.aliased(&ctx.GetFramebufferAttachmentParameterivEXT);
     ctx.GetFramebufferAttachmentParameterivEXT.aliased(&ctx.GetFramebufferAttachmentParameteriv);
     ctx.GetIntegeri_v.aliased(&ctx.GetIntegerIndexedvEXT);
     ctx.GetIntegerIndexedvEXT.aliased(&ctx.GetIntegeri_v);
     ctx.GetMultisamplefv.aliased(&ctx.GetMultisamplefvNV);
     ctx.GetMultisamplefvNV.aliased(&ctx.GetMultisamplefv);
     ctx.GetPointerv.aliased(&ctx.GetPointervEXT);
     ctx.GetPointervEXT.aliased(&ctx.GetPointerv);
     ctx.GetQueryiv.aliased(&ctx.GetQueryivARB);
     ctx.GetQueryivARB.aliased(&ctx.GetQueryiv);
     ctx.GetQueryObjecti64v.aliased(&ctx.GetQueryObjecti64vEXT);
     ctx.GetQueryObjecti64vEXT.aliased(&ctx.GetQueryObjecti64v);
     ctx.GetQueryObjectiv.aliased(&ctx.GetQueryObjectivARB);
     ctx.GetQueryObjectivARB.aliased(&ctx.GetQueryObjectiv);
     ctx.GetQueryObjectui64v.aliased(&ctx.GetQueryObjectui64vEXT);
     ctx.GetQueryObjectui64vEXT.aliased(&ctx.GetQueryObjectui64v);
     ctx.GetQueryObjectuiv.aliased(&ctx.GetQueryObjectuivARB);
     ctx.GetQueryObjectuivARB.aliased(&ctx.GetQueryObjectuiv);
     ctx.GetRenderbufferParameteriv.aliased(&ctx.GetRenderbufferParameterivEXT);
     ctx.GetRenderbufferParameterivEXT.aliased(&ctx.GetRenderbufferParameteriv);
     ctx.GetShaderSource.aliased(&ctx.GetShaderSourceARB);
     ctx.GetShaderSourceARB.aliased(&ctx.GetShaderSource);
     ctx.GetTexParameterIiv.aliased(&ctx.GetTexParameterIivEXT);
     ctx.GetTexParameterIivEXT.aliased(&ctx.GetTexParameterIiv);
     ctx.GetTexParameterIuiv.aliased(&ctx.GetTexParameterIuivEXT);
     ctx.GetTexParameterIuivEXT.aliased(&ctx.GetTexParameterIuiv);
     ctx.GetTransformFeedbackVarying.aliased(&ctx.GetTransformFeedbackVaryingEXT);
     ctx.GetTransformFeedbackVaryingEXT.aliased(&ctx.GetTransformFeedbackVarying);
     ctx.GetUniformfv.aliased(&ctx.GetUniformfvARB);
     ctx.GetUniformfvARB.aliased(&ctx.GetUniformfv);
     ctx.GetUniformiv.aliased(&ctx.GetUniformivARB);
     ctx.GetUniformivARB.aliased(&ctx.GetUniformiv);
     ctx.GetUniformLocation.aliased(&ctx.GetUniformLocationARB);
     ctx.GetUniformLocationARB.aliased(&ctx.GetUniformLocation);
     ctx.GetUniformuiv.aliased(&ctx.GetUniformuivEXT);
     ctx.GetUniformuivEXT.aliased(&ctx.GetUniformuiv);
     ctx.GetVertexAttribdv.aliased(&ctx.GetVertexAttribdvARB);
     ctx.GetVertexAttribdv.aliased(&ctx.GetVertexAttribdvNV);
     ctx.GetVertexAttribdvARB.aliased(&ctx.GetVertexAttribdv);
     ctx.GetVertexAttribdvARB.aliased(&ctx.GetVertexAttribdvNV);
     ctx.GetVertexAttribdvNV.aliased(&ctx.GetVertexAttribdv);
     ctx.GetVertexAttribdvNV.aliased(&ctx.GetVertexAttribdvARB);
     ctx.GetVertexAttribfv.aliased(&ctx.GetVertexAttribfvARB);
     ctx.GetVertexAttribfv.aliased(&ctx.GetVertexAttribfvNV);
     ctx.GetVertexAttribfvARB.aliased(&ctx.GetVertexAttribfv);
     ctx.GetVertexAttribfvARB.aliased(&ctx.GetVertexAttribfvNV);
     ctx.GetVertexAttribfvNV.aliased(&ctx.GetVertexAttribfv);
     ctx.GetVertexAttribfvNV.aliased(&ctx.GetVertexAttribfvARB);
     ctx.GetVertexAttribIiv.aliased(&ctx.GetVertexAttribIivEXT);
     ctx.GetVertexAttribIivEXT.aliased(&ctx.GetVertexAttribIiv);
     ctx.GetVertexAttribIuiv.aliased(&ctx.GetVertexAttribIuivEXT);
     ctx.GetVertexAttribIuivEXT.aliased(&ctx.GetVertexAttribIuiv);
     ctx.GetVertexAttribiv.aliased(&ctx.GetVertexAttribivARB);
     ctx.GetVertexAttribiv.aliased(&ctx.GetVertexAttribivNV);
     ctx.GetVertexAttribivARB.aliased(&ctx.GetVertexAttribiv);
     ctx.GetVertexAttribivARB.aliased(&ctx.GetVertexAttribivNV);
     ctx.GetVertexAttribivNV.aliased(&ctx.GetVertexAttribiv);
     ctx.GetVertexAttribivNV.aliased(&ctx.GetVertexAttribivARB);
     ctx.GetVertexAttribLdv.aliased(&ctx.GetVertexAttribLdvEXT);
     ctx.GetVertexAttribLdvEXT.aliased(&ctx.GetVertexAttribLdv);
     ctx.GetVertexAttribPointerv.aliased(&ctx.GetVertexAttribPointervARB);
     ctx.GetVertexAttribPointerv.aliased(&ctx.GetVertexAttribPointervNV);
     ctx.GetVertexAttribPointervARB.aliased(&ctx.GetVertexAttribPointerv);
     ctx.GetVertexAttribPointervARB.aliased(&ctx.GetVertexAttribPointervNV);
     ctx.GetVertexAttribPointervNV.aliased(&ctx.GetVertexAttribPointerv);
     ctx.GetVertexAttribPointervNV.aliased(&ctx.GetVertexAttribPointervARB);
     ctx.IsBuffer.aliased(&ctx.IsBufferARB);
     ctx.IsBufferARB.aliased(&ctx.IsBuffer);
     ctx.IsEnabledi.aliased(&ctx.IsEnabledIndexedEXT);
     ctx.IsEnabledIndexedEXT.aliased(&ctx.IsEnabledi);
     ctx.IsFramebuffer.aliased(&ctx.IsFramebufferEXT);
     ctx.IsFramebufferEXT.aliased(&ctx.IsFramebuffer);
     ctx.IsProgramARB.aliased(&ctx.IsProgramNV);
     ctx.IsProgramNV.aliased(&ctx.IsProgramARB);
     ctx.IsQuery.aliased(&ctx.IsQueryARB);
     ctx.IsQueryARB.aliased(&ctx.IsQuery);
     ctx.IsRenderbuffer.aliased(&ctx.IsRenderbufferEXT);
     ctx.IsRenderbufferEXT.aliased(&ctx.IsRenderbuffer);
     ctx.IsTransformFeedback.aliased(&ctx.IsTransformFeedbackNV);
     ctx.IsTransformFeedbackNV.aliased(&ctx.IsTransformFeedback);
     ctx.IsVertexArray.aliased(&ctx.IsVertexArrayAPPLE);
     ctx.IsVertexArrayAPPLE.aliased(&ctx.IsVertexArray);
     ctx.LinkProgram.aliased(&ctx.LinkProgramARB);
     ctx.LinkProgramARB.aliased(&ctx.LinkProgram);
     ctx.MapBuffer.aliased(&ctx.MapBufferARB);
     ctx.MapBufferARB.aliased(&ctx.MapBuffer);
     ctx.MemoryBarrier.aliased(&ctx.MemoryBarrierEXT);
     ctx.MemoryBarrierEXT.aliased(&ctx.MemoryBarrier);
     ctx.MinSampleShading.aliased(&ctx.MinSampleShadingARB);
     ctx.MinSampleShadingARB.aliased(&ctx.MinSampleShading);
     ctx.MultiDrawArrays.aliased(&ctx.MultiDrawArraysEXT);
     ctx.MultiDrawArraysEXT.aliased(&ctx.MultiDrawArrays);
     ctx.MultiDrawArraysIndirect.aliased(&ctx.MultiDrawArraysIndirectAMD);
     ctx.MultiDrawArraysIndirectAMD.aliased(&ctx.MultiDrawArraysIndirect);
     ctx.MultiDrawArraysIndirectCount.aliased(&ctx.MultiDrawArraysIndirectCountARB);
     ctx.MultiDrawArraysIndirectCountARB.aliased(&ctx.MultiDrawArraysIndirectCount);
     ctx.MultiDrawElements.aliased(&ctx.MultiDrawElementsEXT);
     ctx.MultiDrawElementsEXT.aliased(&ctx.MultiDrawElements);
     ctx.MultiDrawElementsIndirect.aliased(&ctx.MultiDrawElementsIndirectAMD);
     ctx.MultiDrawElementsIndirectAMD.aliased(&ctx.MultiDrawElementsIndirect);
     ctx.MultiDrawElementsIndirectCount.aliased(&ctx.MultiDrawElementsIndirectCountARB);
     ctx.MultiDrawElementsIndirectCountARB.aliased(&ctx.MultiDrawElementsIndirectCount);
     ctx.NamedBufferStorage.aliased(&ctx.NamedBufferStorageEXT);
     ctx.NamedBufferStorageEXT.aliased(&ctx.NamedBufferStorage);
     ctx.NamedBufferSubData.aliased(&ctx.NamedBufferSubDataEXT);
     ctx.NamedBufferSubDataEXT.aliased(&ctx.NamedBufferSubData);
     ctx.PauseTransformFeedback.aliased(&ctx.PauseTransformFeedbackNV);
     ctx.PauseTransformFeedbackNV.aliased(&ctx.PauseTransformFeedback);
     ctx.PointParameterf.aliased(&ctx.PointParameterfARB);
     ctx.PointParameterf.aliased(&ctx.PointParameterfEXT);
     ctx.PointParameterf.aliased(&ctx.PointParameterfSGIS);
     ctx.PointParameterfARB.aliased(&ctx.PointParameterf);
     ctx.PointParameterfARB.aliased(&ctx.PointParameterfEXT);
     ctx.PointParameterfARB.aliased(&ctx.PointParameterfSGIS);
     ctx.PointParameterfEXT.aliased(&ctx.PointParameterf);
     ctx.PointParameterfEXT.aliased(&ctx.PointParameterfARB);
     ctx.PointParameterfEXT.aliased(&ctx.PointParameterfSGIS);
     ctx.PointParameterfSGIS.aliased(&ctx.PointParameterf);
     ctx.PointParameterfSGIS.aliased(&ctx.PointParameterfARB);
     ctx.PointParameterfSGIS.aliased(&ctx.PointParameterfEXT);
     ctx.PointParameterfv.aliased(&ctx.PointParameterfvARB);
     ctx.PointParameterfv.aliased(&ctx.PointParameterfvEXT);
     ctx.PointParameterfv.aliased(&ctx.PointParameterfvSGIS);
     ctx.PointParameterfvARB.aliased(&ctx.PointParameterfv);
     ctx.PointParameterfvARB.aliased(&ctx.PointParameterfvEXT);
     ctx.PointParameterfvARB.aliased(&ctx.PointParameterfvSGIS);
     ctx.PointParameterfvEXT.aliased(&ctx.PointParameterfv);
     ctx.PointParameterfvEXT.aliased(&ctx.PointParameterfvARB);
     ctx.PointParameterfvEXT.aliased(&ctx.PointParameterfvSGIS);
     ctx.PointParameterfvSGIS.aliased(&ctx.PointParameterfv);
     ctx.PointParameterfvSGIS.aliased(&ctx.PointParameterfvARB);
     ctx.PointParameterfvSGIS.aliased(&ctx.PointParameterfvEXT);
     ctx.PointParameteri.aliased(&ctx.PointParameteriNV);
     ctx.PointParameteriNV.aliased(&ctx.PointParameteri);
     ctx.PointParameteriv.aliased(&ctx.PointParameterivNV);
     ctx.PointParameterivNV.aliased(&ctx.PointParameteriv);
     ctx.PolygonOffsetClamp.aliased(&ctx.PolygonOffsetClampEXT);
     ctx.PolygonOffsetClampEXT.aliased(&ctx.PolygonOffsetClamp);
     ctx.ProgramParameteri.aliased(&ctx.ProgramParameteriARB);
     ctx.ProgramParameteri.aliased(&ctx.ProgramParameteriEXT);
     ctx.ProgramParameteriARB.aliased(&ctx.ProgramParameteri);
     ctx.ProgramParameteriARB.aliased(&ctx.ProgramParameteriEXT);
     ctx.ProgramParameteriEXT.aliased(&ctx.ProgramParameteri);
     ctx.ProgramParameteriEXT.aliased(&ctx.ProgramParameteriARB);
     ctx.ProgramUniform1f.aliased(&ctx.ProgramUniform1fEXT);
     ctx.ProgramUniform1fEXT.aliased(&ctx.ProgramUniform1f);
     ctx.ProgramUniform1fv.aliased(&ctx.ProgramUniform1fvEXT);
     ctx.ProgramUniform1fvEXT.aliased(&ctx.ProgramUniform1fv);
     ctx.ProgramUniform1i.aliased(&ctx.ProgramUniform1iEXT);
     ctx.ProgramUniform1iEXT.aliased(&ctx.ProgramUniform1i);
     ctx.ProgramUniform1iv.aliased(&ctx.ProgramUniform1ivEXT);
     ctx.ProgramUniform1ivEXT.aliased(&ctx.ProgramUniform1iv);
     ctx.ProgramUniform1ui.aliased(&ctx.ProgramUniform1uiEXT);
     ctx.ProgramUniform1uiEXT.aliased(&ctx.ProgramUniform1ui);
     ctx.ProgramUniform1uiv.aliased(&ctx.ProgramUniform1uivEXT);
     ctx.ProgramUniform1uivEXT.aliased(&ctx.ProgramUniform1uiv);
     ctx.ProgramUniform2f.aliased(&ctx.ProgramUniform2fEXT);
     ctx.ProgramUniform2fEXT.aliased(&ctx.ProgramUniform2f);
     ctx.ProgramUniform2fv.aliased(&ctx.ProgramUniform2fvEXT);
     ctx.ProgramUniform2fvEXT.aliased(&ctx.ProgramUniform2fv);
     ctx.ProgramUniform2i.aliased(&ctx.ProgramUniform2iEXT);
     ctx.ProgramUniform2iEXT.aliased(&ctx.ProgramUniform2i);
     ctx.ProgramUniform2iv.aliased(&ctx.ProgramUniform2ivEXT);
     ctx.ProgramUniform2ivEXT.aliased(&ctx.ProgramUniform2iv);
     ctx.ProgramUniform2ui.aliased(&ctx.ProgramUniform2uiEXT);
     ctx.ProgramUniform2uiEXT.aliased(&ctx.ProgramUniform2ui);
     ctx.ProgramUniform2uiv.aliased(&ctx.ProgramUniform2uivEXT);
     ctx.ProgramUniform2uivEXT.aliased(&ctx.ProgramUniform2uiv);
     ctx.ProgramUniform3f.aliased(&ctx.ProgramUniform3fEXT);
     ctx.ProgramUniform3fEXT.aliased(&ctx.ProgramUniform3f);
     ctx.ProgramUniform3fv.aliased(&ctx.ProgramUniform3fvEXT);
     ctx.ProgramUniform3fvEXT.aliased(&ctx.ProgramUniform3fv);
     ctx.ProgramUniform3i.aliased(&ctx.ProgramUniform3iEXT);
     ctx.ProgramUniform3iEXT.aliased(&ctx.ProgramUniform3i);
     ctx.ProgramUniform3iv.aliased(&ctx.ProgramUniform3ivEXT);
     ctx.ProgramUniform3ivEXT.aliased(&ctx.ProgramUniform3iv);
     ctx.ProgramUniform3ui.aliased(&ctx.ProgramUniform3uiEXT);
     ctx.ProgramUniform3uiEXT.aliased(&ctx.ProgramUniform3ui);
     ctx.ProgramUniform3uiv.aliased(&ctx.ProgramUniform3uivEXT);
     ctx.ProgramUniform3uivEXT.aliased(&ctx.ProgramUniform3uiv);
     ctx.ProgramUniform4f.aliased(&ctx.ProgramUniform4fEXT);
     ctx.ProgramUniform4fEXT.aliased(&ctx.ProgramUniform4f);
     ctx.ProgramUniform4fv.aliased(&ctx.ProgramUniform4fvEXT);
     ctx.ProgramUniform4fvEXT.aliased(&ctx.ProgramUniform4fv);
     ctx.ProgramUniform4i.aliased(&ctx.ProgramUniform4iEXT);
     ctx.ProgramUniform4iEXT.aliased(&ctx.ProgramUniform4i);
     ctx.ProgramUniform4iv.aliased(&ctx.ProgramUniform4ivEXT);
     ctx.ProgramUniform4ivEXT.aliased(&ctx.ProgramUniform4iv);
     ctx.ProgramUniform4ui.aliased(&ctx.ProgramUniform4uiEXT);
     ctx.ProgramUniform4uiEXT.aliased(&ctx.ProgramUniform4ui);
     ctx.ProgramUniform4uiv.aliased(&ctx.ProgramUniform4uivEXT);
     ctx.ProgramUniform4uivEXT.aliased(&ctx.ProgramUniform4uiv);
     ctx.ProgramUniformMatrix2fv.aliased(&ctx.ProgramUniformMatrix2fvEXT);
     ctx.ProgramUniformMatrix2fvEXT.aliased(&ctx.ProgramUniformMatrix2fv);
     ctx.ProgramUniformMatrix2x3fv.aliased(&ctx.ProgramUniformMatrix2x3fvEXT);
     ctx.ProgramUniformMatrix2x3fvEXT.aliased(&ctx.ProgramUniformMatrix2x3fv);
     ctx.ProgramUniformMatrix2x4fv.aliased(&ctx.ProgramUniformMatrix2x4fvEXT);
     ctx.ProgramUniformMatrix2x4fvEXT.aliased(&ctx.ProgramUniformMatrix2x4fv);
     ctx.ProgramUniformMatrix3fv.aliased(&ctx.ProgramUniformMatrix3fvEXT);
     ctx.ProgramUniformMatrix3fvEXT.aliased(&ctx.ProgramUniformMatrix3fv);
     ctx.ProgramUniformMatrix3x2fv.aliased(&ctx.ProgramUniformMatrix3x2fvEXT);
     ctx.ProgramUniformMatrix3x2fvEXT.aliased(&ctx.ProgramUniformMatrix3x2fv);
     ctx.ProgramUniformMatrix3x4fv.aliased(&ctx.ProgramUniformMatrix3x4fvEXT);
     ctx.ProgramUniformMatrix3x4fvEXT.aliased(&ctx.ProgramUniformMatrix3x4fv);
     ctx.ProgramUniformMatrix4fv.aliased(&ctx.ProgramUniformMatrix4fvEXT);
     ctx.ProgramUniformMatrix4fvEXT.aliased(&ctx.ProgramUniformMatrix4fv);
     ctx.ProgramUniformMatrix4x2fv.aliased(&ctx.ProgramUniformMatrix4x2fvEXT);
     ctx.ProgramUniformMatrix4x2fvEXT.aliased(&ctx.ProgramUniformMatrix4x2fv);
     ctx.ProgramUniformMatrix4x3fv.aliased(&ctx.ProgramUniformMatrix4x3fvEXT);
     ctx.ProgramUniformMatrix4x3fvEXT.aliased(&ctx.ProgramUniformMatrix4x3fv);
     ctx.ProvokingVertex.aliased(&ctx.ProvokingVertexEXT);
     ctx.ProvokingVertexEXT.aliased(&ctx.ProvokingVertex);
     ctx.ReadnPixels.aliased(&ctx.ReadnPixelsARB);
     ctx.ReadnPixelsARB.aliased(&ctx.ReadnPixels);
     ctx.RenderbufferStorage.aliased(&ctx.RenderbufferStorageEXT);
     ctx.RenderbufferStorageEXT.aliased(&ctx.RenderbufferStorage);
     ctx.RenderbufferStorageMultisample.aliased(&ctx.RenderbufferStorageMultisampleEXT);
     ctx.RenderbufferStorageMultisampleEXT.aliased(&ctx.RenderbufferStorageMultisample);
     ctx.ResumeTransformFeedback.aliased(&ctx.ResumeTransformFeedbackNV);
     ctx.ResumeTransformFeedbackNV.aliased(&ctx.ResumeTransformFeedback);
     ctx.SampleCoverage.aliased(&ctx.SampleCoverageARB);
     ctx.SampleCoverageARB.aliased(&ctx.SampleCoverage);
     ctx.ShaderSource.aliased(&ctx.ShaderSourceARB);
     ctx.ShaderSourceARB.aliased(&ctx.ShaderSource);
     ctx.SpecializeShader.aliased(&ctx.SpecializeShaderARB);
     ctx.SpecializeShaderARB.aliased(&ctx.SpecializeShader);
     ctx.StencilOpSeparate.aliased(&ctx.StencilOpSeparateATI);
     ctx.StencilOpSeparateATI.aliased(&ctx.StencilOpSeparate);
     ctx.TexBuffer.aliased(&ctx.TexBufferARB);
     ctx.TexBuffer.aliased(&ctx.TexBufferEXT);
     ctx.TexBufferARB.aliased(&ctx.TexBuffer);
     ctx.TexBufferARB.aliased(&ctx.TexBufferEXT);
     ctx.TexBufferEXT.aliased(&ctx.TexBuffer);
     ctx.TexBufferEXT.aliased(&ctx.TexBufferARB);
     ctx.TexImage3D.aliased(&ctx.TexImage3DEXT);
     ctx.TexImage3DEXT.aliased(&ctx.TexImage3D);
     ctx.TexParameterIiv.aliased(&ctx.TexParameterIivEXT);
     ctx.TexParameterIivEXT.aliased(&ctx.TexParameterIiv);
     ctx.TexParameterIuiv.aliased(&ctx.TexParameterIuivEXT);
     ctx.TexParameterIuivEXT.aliased(&ctx.TexParameterIuiv);
     ctx.TexStorage1D.aliased(&ctx.TexStorage1DEXT);
     ctx.TexStorage1DEXT.aliased(&ctx.TexStorage1D);
     ctx.TexStorage2D.aliased(&ctx.TexStorage2DEXT);
     ctx.TexStorage2DEXT.aliased(&ctx.TexStorage2D);
     ctx.TexStorage3D.aliased(&ctx.TexStorage3DEXT);
     ctx.TexStorage3DEXT.aliased(&ctx.TexStorage3D);
     ctx.TexSubImage1D.aliased(&ctx.TexSubImage1DEXT);
     ctx.TexSubImage1DEXT.aliased(&ctx.TexSubImage1D);
     ctx.TexSubImage2D.aliased(&ctx.TexSubImage2DEXT);
     ctx.TexSubImage2DEXT.aliased(&ctx.TexSubImage2D);
     ctx.TexSubImage3D.aliased(&ctx.TexSubImage3DEXT);
     ctx.TexSubImage3DEXT.aliased(&ctx.TexSubImage3D);
     ctx.TransformFeedbackVaryings.aliased(&ctx.TransformFeedbackVaryingsEXT);
     ctx.TransformFeedbackVaryingsEXT.aliased(&ctx.TransformFeedbackVaryings);
     ctx.Uniform1f.aliased(&ctx.Uniform1fARB);
     ctx.Uniform1fARB.aliased(&ctx.Uniform1f);
     ctx.Uniform1fv.aliased(&ctx.Uniform1fvARB);
     ctx.Uniform1fvARB.aliased(&ctx.Uniform1fv);
     ctx.Uniform1i.aliased(&ctx.Uniform1iARB);
     ctx.Uniform1iARB.aliased(&ctx.Uniform1i);
     ctx.Uniform1iv.aliased(&ctx.Uniform1ivARB);
     ctx.Uniform1ivARB.aliased(&ctx.Uniform1iv);
     ctx.Uniform1ui.aliased(&ctx.Uniform1uiEXT);
     ctx.Uniform1uiEXT.aliased(&ctx.Uniform1ui);
     ctx.Uniform1uiv.aliased(&ctx.Uniform1uivEXT);
     ctx.Uniform1uivEXT.aliased(&ctx.Uniform1uiv);
     ctx.Uniform2f.aliased(&ctx.Uniform2fARB);
     ctx.Uniform2fARB.aliased(&ctx.Uniform2f);
     ctx.Uniform2fv.aliased(&ctx.Uniform2fvARB);
     ctx.Uniform2fvARB.aliased(&ctx.Uniform2fv);
     ctx.Uniform2i.aliased(&ctx.Uniform2iARB);
     ctx.Uniform2iARB.aliased(&ctx.Uniform2i);
     ctx.Uniform2iv.aliased(&ctx.Uniform2ivARB);
     ctx.Uniform2ivARB.aliased(&ctx.Uniform2iv);
     ctx.Uniform2ui.aliased(&ctx.Uniform2uiEXT);
     ctx.Uniform2uiEXT.aliased(&ctx.Uniform2ui);
     ctx.Uniform2uiv.aliased(&ctx.Uniform2uivEXT);
     ctx.Uniform2uivEXT.aliased(&ctx.Uniform2uiv);
     ctx.Uniform3f.aliased(&ctx.Uniform3fARB);
     ctx.Uniform3fARB.aliased(&ctx.Uniform3f);
     ctx.Uniform3fv.aliased(&ctx.Uniform3fvARB);
     ctx.Uniform3fvARB.aliased(&ctx.Uniform3fv);
     ctx.Uniform3i.aliased(&ctx.Uniform3iARB);
     ctx.Uniform3iARB.aliased(&ctx.Uniform3i);
     ctx.Uniform3iv.aliased(&ctx.Uniform3ivARB);
     ctx.Uniform3ivARB.aliased(&ctx.Uniform3iv);
     ctx.Uniform3ui.aliased(&ctx.Uniform3uiEXT);
     ctx.Uniform3uiEXT.aliased(&ctx.Uniform3ui);
     ctx.Uniform3uiv.aliased(&ctx.Uniform3uivEXT);
     ctx.Uniform3uivEXT.aliased(&ctx.Uniform3uiv);
     ctx.Uniform4f.aliased(&ctx.Uniform4fARB);
     ctx.Uniform4fARB.aliased(&ctx.Uniform4f);
     ctx.Uniform4fv.aliased(&ctx.Uniform4fvARB);
     ctx.Uniform4fvARB.aliased(&ctx.Uniform4fv);
     ctx.Uniform4i.aliased(&ctx.Uniform4iARB);
     ctx.Uniform4iARB.aliased(&ctx.Uniform4i);
     ctx.Uniform4iv.aliased(&ctx.Uniform4ivARB);
     ctx.Uniform4ivARB.aliased(&ctx.Uniform4iv);
     ctx.Uniform4ui.aliased(&ctx.Uniform4uiEXT);
     ctx.Uniform4uiEXT.aliased(&ctx.Uniform4ui);
     ctx.Uniform4uiv.aliased(&ctx.Uniform4uivEXT);
     ctx.Uniform4uivEXT.aliased(&ctx.Uniform4uiv);
     ctx.UniformMatrix2fv.aliased(&ctx.UniformMatrix2fvARB);
     ctx.UniformMatrix2fvARB.aliased(&ctx.UniformMatrix2fv);
     ctx.UniformMatrix3fv.aliased(&ctx.UniformMatrix3fvARB);
     ctx.UniformMatrix3fvARB.aliased(&ctx.UniformMatrix3fv);
     ctx.UniformMatrix4fv.aliased(&ctx.UniformMatrix4fvARB);
     ctx.UniformMatrix4fvARB.aliased(&ctx.UniformMatrix4fv);
     ctx.UnmapBuffer.aliased(&ctx.UnmapBufferARB);
     ctx.UnmapBufferARB.aliased(&ctx.UnmapBuffer);
     ctx.UseProgram.aliased(&ctx.UseProgramObjectARB);
     ctx.UseProgramObjectARB.aliased(&ctx.UseProgram);
     ctx.ValidateProgram.aliased(&ctx.ValidateProgramARB);
     ctx.ValidateProgramARB.aliased(&ctx.ValidateProgram);
     ctx.VertexAttrib1d.aliased(&ctx.VertexAttrib1dARB);
     ctx.VertexAttrib1d.aliased(&ctx.VertexAttrib1dNV);
     ctx.VertexAttrib1dARB.aliased(&ctx.VertexAttrib1d);
     ctx.VertexAttrib1dARB.aliased(&ctx.VertexAttrib1dNV);
     ctx.VertexAttrib1dNV.aliased(&ctx.VertexAttrib1d);
     ctx.VertexAttrib1dNV.aliased(&ctx.VertexAttrib1dARB);
     ctx.VertexAttrib1dv.aliased(&ctx.VertexAttrib1dvARB);
     ctx.VertexAttrib1dv.aliased(&ctx.VertexAttrib1dvNV);
     ctx.VertexAttrib1dvARB.aliased(&ctx.VertexAttrib1dv);
     ctx.VertexAttrib1dvARB.aliased(&ctx.VertexAttrib1dvNV);
     ctx.VertexAttrib1dvNV.aliased(&ctx.VertexAttrib1dv);
     ctx.VertexAttrib1dvNV.aliased(&ctx.VertexAttrib1dvARB);
     ctx.VertexAttrib1f.aliased(&ctx.VertexAttrib1fARB);
     ctx.VertexAttrib1f.aliased(&ctx.VertexAttrib1fNV);
     ctx.VertexAttrib1fARB.aliased(&ctx.VertexAttrib1f);
     ctx.VertexAttrib1fARB.aliased(&ctx.VertexAttrib1fNV);
     ctx.VertexAttrib1fNV.aliased(&ctx.VertexAttrib1f);
     ctx.VertexAttrib1fNV.aliased(&ctx.VertexAttrib1fARB);
     ctx.VertexAttrib1fv.aliased(&ctx.VertexAttrib1fvARB);
     ctx.VertexAttrib1fv.aliased(&ctx.VertexAttrib1fvNV);
     ctx.VertexAttrib1fvARB.aliased(&ctx.VertexAttrib1fv);
     ctx.VertexAttrib1fvARB.aliased(&ctx.VertexAttrib1fvNV);
     ctx.VertexAttrib1fvNV.aliased(&ctx.VertexAttrib1fv);
     ctx.VertexAttrib1fvNV.aliased(&ctx.VertexAttrib1fvARB);
     ctx.VertexAttrib1s.aliased(&ctx.VertexAttrib1sARB);
     ctx.VertexAttrib1s.aliased(&ctx.VertexAttrib1sNV);
     ctx.VertexAttrib1sARB.aliased(&ctx.VertexAttrib1s);
     ctx.VertexAttrib1sARB.aliased(&ctx.VertexAttrib1sNV);
     ctx.VertexAttrib1sNV.aliased(&ctx.VertexAttrib1s);
     ctx.VertexAttrib1sNV.aliased(&ctx.VertexAttrib1sARB);
     ctx.VertexAttrib1sv.aliased(&ctx.VertexAttrib1svARB);
     ctx.VertexAttrib1sv.aliased(&ctx.VertexAttrib1svNV);
     ctx.VertexAttrib1svARB.aliased(&ctx.VertexAttrib1sv);
     ctx.VertexAttrib1svARB.aliased(&ctx.VertexAttrib1svNV);
     ctx.VertexAttrib1svNV.aliased(&ctx.VertexAttrib1sv);
     ctx.VertexAttrib1svNV.aliased(&ctx.VertexAttrib1svARB);
     ctx.VertexAttrib2d.aliased(&ctx.VertexAttrib2dARB);
     ctx.VertexAttrib2d.aliased(&ctx.VertexAttrib2dNV);
     ctx.VertexAttrib2dARB.aliased(&ctx.VertexAttrib2d);
     ctx.VertexAttrib2dARB.aliased(&ctx.VertexAttrib2dNV);
     ctx.VertexAttrib2dNV.aliased(&ctx.VertexAttrib2d);
     ctx.VertexAttrib2dNV.aliased(&ctx.VertexAttrib2dARB);
     ctx.VertexAttrib2dv.aliased(&ctx.VertexAttrib2dvARB);
     ctx.VertexAttrib2dv.aliased(&ctx.VertexAttrib2dvNV);
     ctx.VertexAttrib2dvARB.aliased(&ctx.VertexAttrib2dv);
     ctx.VertexAttrib2dvARB.aliased(&ctx.VertexAttrib2dvNV);
     ctx.VertexAttrib2dvNV.aliased(&ctx.VertexAttrib2dv);
     ctx.VertexAttrib2dvNV.aliased(&ctx.VertexAttrib2dvARB);
     ctx.VertexAttrib2f.aliased(&ctx.VertexAttrib2fARB);
     ctx.VertexAttrib2f.aliased(&ctx.VertexAttrib2fNV);
     ctx.VertexAttrib2fARB.aliased(&ctx.VertexAttrib2f);
     ctx.VertexAttrib2fARB.aliased(&ctx.VertexAttrib2fNV);
     ctx.VertexAttrib2fNV.aliased(&ctx.VertexAttrib2f);
     ctx.VertexAttrib2fNV.aliased(&ctx.VertexAttrib2fARB);
     ctx.VertexAttrib2fv.aliased(&ctx.VertexAttrib2fvARB);
     ctx.VertexAttrib2fv.aliased(&ctx.VertexAttrib2fvNV);
     ctx.VertexAttrib2fvARB.aliased(&ctx.VertexAttrib2fv);
     ctx.VertexAttrib2fvARB.aliased(&ctx.VertexAttrib2fvNV);
     ctx.VertexAttrib2fvNV.aliased(&ctx.VertexAttrib2fv);
     ctx.VertexAttrib2fvNV.aliased(&ctx.VertexAttrib2fvARB);
     ctx.VertexAttrib2s.aliased(&ctx.VertexAttrib2sARB);
     ctx.VertexAttrib2s.aliased(&ctx.VertexAttrib2sNV);
     ctx.VertexAttrib2sARB.aliased(&ctx.VertexAttrib2s);
     ctx.VertexAttrib2sARB.aliased(&ctx.VertexAttrib2sNV);
     ctx.VertexAttrib2sNV.aliased(&ctx.VertexAttrib2s);
     ctx.VertexAttrib2sNV.aliased(&ctx.VertexAttrib2sARB);
     ctx.VertexAttrib2sv.aliased(&ctx.VertexAttrib2svARB);
     ctx.VertexAttrib2sv.aliased(&ctx.VertexAttrib2svNV);
     ctx.VertexAttrib2svARB.aliased(&ctx.VertexAttrib2sv);
     ctx.VertexAttrib2svARB.aliased(&ctx.VertexAttrib2svNV);
     ctx.VertexAttrib2svNV.aliased(&ctx.VertexAttrib2sv);
     ctx.VertexAttrib2svNV.aliased(&ctx.VertexAttrib2svARB);
     ctx.VertexAttrib3d.aliased(&ctx.VertexAttrib3dARB);
     ctx.VertexAttrib3d.aliased(&ctx.VertexAttrib3dNV);
     ctx.VertexAttrib3dARB.aliased(&ctx.VertexAttrib3d);
     ctx.VertexAttrib3dARB.aliased(&ctx.VertexAttrib3dNV);
     ctx.VertexAttrib3dNV.aliased(&ctx.VertexAttrib3d);
     ctx.VertexAttrib3dNV.aliased(&ctx.VertexAttrib3dARB);
     ctx.VertexAttrib3dv.aliased(&ctx.VertexAttrib3dvARB);
     ctx.VertexAttrib3dv.aliased(&ctx.VertexAttrib3dvNV);
     ctx.VertexAttrib3dvARB.aliased(&ctx.VertexAttrib3dv);
     ctx.VertexAttrib3dvARB.aliased(&ctx.VertexAttrib3dvNV);
     ctx.VertexAttrib3dvNV.aliased(&ctx.VertexAttrib3dv);
     ctx.VertexAttrib3dvNV.aliased(&ctx.VertexAttrib3dvARB);
     ctx.VertexAttrib3f.aliased(&ctx.VertexAttrib3fARB);
     ctx.VertexAttrib3f.aliased(&ctx.VertexAttrib3fNV);
     ctx.VertexAttrib3fARB.aliased(&ctx.VertexAttrib3f);
     ctx.VertexAttrib3fARB.aliased(&ctx.VertexAttrib3fNV);
     ctx.VertexAttrib3fNV.aliased(&ctx.VertexAttrib3f);
     ctx.VertexAttrib3fNV.aliased(&ctx.VertexAttrib3fARB);
     ctx.VertexAttrib3fv.aliased(&ctx.VertexAttrib3fvARB);
     ctx.VertexAttrib3fv.aliased(&ctx.VertexAttrib3fvNV);
     ctx.VertexAttrib3fvARB.aliased(&ctx.VertexAttrib3fv);
     ctx.VertexAttrib3fvARB.aliased(&ctx.VertexAttrib3fvNV);
     ctx.VertexAttrib3fvNV.aliased(&ctx.VertexAttrib3fv);
     ctx.VertexAttrib3fvNV.aliased(&ctx.VertexAttrib3fvARB);
     ctx.VertexAttrib3s.aliased(&ctx.VertexAttrib3sARB);
     ctx.VertexAttrib3s.aliased(&ctx.VertexAttrib3sNV);
     ctx.VertexAttrib3sARB.aliased(&ctx.VertexAttrib3s);
     ctx.VertexAttrib3sARB.aliased(&ctx.VertexAttrib3sNV);
     ctx.VertexAttrib3sNV.aliased(&ctx.VertexAttrib3s);
     ctx.VertexAttrib3sNV.aliased(&ctx.VertexAttrib3sARB);
     ctx.VertexAttrib3sv.aliased(&ctx.VertexAttrib3svARB);
     ctx.VertexAttrib3sv.aliased(&ctx.VertexAttrib3svNV);
     ctx.VertexAttrib3svARB.aliased(&ctx.VertexAttrib3sv);
     ctx.VertexAttrib3svARB.aliased(&ctx.VertexAttrib3svNV);
     ctx.VertexAttrib3svNV.aliased(&ctx.VertexAttrib3sv);
     ctx.VertexAttrib3svNV.aliased(&ctx.VertexAttrib3svARB);
     ctx.VertexAttrib4bv.aliased(&ctx.VertexAttrib4bvARB);
     ctx.VertexAttrib4bvARB.aliased(&ctx.VertexAttrib4bv);
     ctx.VertexAttrib4d.aliased(&ctx.VertexAttrib4dARB);
     ctx.VertexAttrib4d.aliased(&ctx.VertexAttrib4dNV);
     ctx.VertexAttrib4dARB.aliased(&ctx.VertexAttrib4d);
     ctx.VertexAttrib4dARB.aliased(&ctx.VertexAttrib4dNV);
     ctx.VertexAttrib4dNV.aliased(&ctx.VertexAttrib4d);
     ctx.VertexAttrib4dNV.aliased(&ctx.VertexAttrib4dARB);
     ctx.VertexAttrib4dv.aliased(&ctx.VertexAttrib4dvARB);
     ctx.VertexAttrib4dv.aliased(&ctx.VertexAttrib4dvNV);
     ctx.VertexAttrib4dvARB.aliased(&ctx.VertexAttrib4dv);
     ctx.VertexAttrib4dvARB.aliased(&ctx.VertexAttrib4dvNV);
     ctx.VertexAttrib4dvNV.aliased(&ctx.VertexAttrib4dv);
     ctx.VertexAttrib4dvNV.aliased(&ctx.VertexAttrib4dvARB);
     ctx.VertexAttrib4f.aliased(&ctx.VertexAttrib4fARB);
     ctx.VertexAttrib4f.aliased(&ctx.VertexAttrib4fNV);
     ctx.VertexAttrib4fARB.aliased(&ctx.VertexAttrib4f);
     ctx.VertexAttrib4fARB.aliased(&ctx.VertexAttrib4fNV);
     ctx.VertexAttrib4fNV.aliased(&ctx.VertexAttrib4f);
     ctx.VertexAttrib4fNV.aliased(&ctx.VertexAttrib4fARB);
     ctx.VertexAttrib4fv.aliased(&ctx.VertexAttrib4fvARB);
     ctx.VertexAttrib4fv.aliased(&ctx.VertexAttrib4fvNV);
     ctx.VertexAttrib4fvARB.aliased(&ctx.VertexAttrib4fv);
     ctx.VertexAttrib4fvARB.aliased(&ctx.VertexAttrib4fvNV);
     ctx.VertexAttrib4fvNV.aliased(&ctx.VertexAttrib4fv);
     ctx.VertexAttrib4fvNV.aliased(&ctx.VertexAttrib4fvARB);
     ctx.VertexAttrib4iv.aliased(&ctx.VertexAttrib4ivARB);
     ctx.VertexAttrib4ivARB.aliased(&ctx.VertexAttrib4iv);
     ctx.VertexAttrib4Nbv.aliased(&ctx.VertexAttrib4NbvARB);
     ctx.VertexAttrib4NbvARB.aliased(&ctx.VertexAttrib4Nbv);
     ctx.VertexAttrib4Niv.aliased(&ctx.VertexAttrib4NivARB);
     ctx.VertexAttrib4NivARB.aliased(&ctx.VertexAttrib4Niv);
     ctx.VertexAttrib4Nsv.aliased(&ctx.VertexAttrib4NsvARB);
     ctx.VertexAttrib4NsvARB.aliased(&ctx.VertexAttrib4Nsv);
     ctx.VertexAttrib4Nub.aliased(&ctx.VertexAttrib4NubARB);
     ctx.VertexAttrib4Nub.aliased(&ctx.VertexAttrib4ubNV);
     ctx.VertexAttrib4NubARB.aliased(&ctx.VertexAttrib4Nub);
     ctx.VertexAttrib4NubARB.aliased(&ctx.VertexAttrib4ubNV);
     ctx.VertexAttrib4Nubv.aliased(&ctx.VertexAttrib4NubvARB);
     ctx.VertexAttrib4Nubv.aliased(&ctx.VertexAttrib4ubvNV);
     ctx.VertexAttrib4NubvARB.aliased(&ctx.VertexAttrib4Nubv);
     ctx.VertexAttrib4NubvARB.aliased(&ctx.VertexAttrib4ubvNV);
     ctx.VertexAttrib4Nuiv.aliased(&ctx.VertexAttrib4NuivARB);
     ctx.VertexAttrib4NuivARB.aliased(&ctx.VertexAttrib4Nuiv);
     ctx.VertexAttrib4Nusv.aliased(&ctx.VertexAttrib4NusvARB);
     ctx.VertexAttrib4NusvARB.aliased(&ctx.VertexAttrib4Nusv);
     ctx.VertexAttrib4s.aliased(&ctx.VertexAttrib4sARB);
     ctx.VertexAttrib4s.aliased(&ctx.VertexAttrib4sNV);
     ctx.VertexAttrib4sARB.aliased(&ctx.VertexAttrib4s);
     ctx.VertexAttrib4sARB.aliased(&ctx.VertexAttrib4sNV);
     ctx.VertexAttrib4sNV.aliased(&ctx.VertexAttrib4s);
     ctx.VertexAttrib4sNV.aliased(&ctx.VertexAttrib4sARB);
     ctx.VertexAttrib4sv.aliased(&ctx.VertexAttrib4svARB);
     ctx.VertexAttrib4sv.aliased(&ctx.VertexAttrib4svNV);
     ctx.VertexAttrib4svARB.aliased(&ctx.VertexAttrib4sv);
     ctx.VertexAttrib4svARB.aliased(&ctx.VertexAttrib4svNV);
     ctx.VertexAttrib4svNV.aliased(&ctx.VertexAttrib4sv);
     ctx.VertexAttrib4svNV.aliased(&ctx.VertexAttrib4svARB);
     ctx.VertexAttrib4ubNV.aliased(&ctx.VertexAttrib4Nub);
     ctx.VertexAttrib4ubNV.aliased(&ctx.VertexAttrib4NubARB);
     ctx.VertexAttrib4ubv.aliased(&ctx.VertexAttrib4ubvARB);
     ctx.VertexAttrib4ubvARB.aliased(&ctx.VertexAttrib4ubv);
     ctx.VertexAttrib4ubvNV.aliased(&ctx.VertexAttrib4Nubv);
     ctx.VertexAttrib4ubvNV.aliased(&ctx.VertexAttrib4NubvARB);
     ctx.VertexAttrib4uiv.aliased(&ctx.VertexAttrib4uivARB);
     ctx.VertexAttrib4uivARB.aliased(&ctx.VertexAttrib4uiv);
     ctx.VertexAttrib4usv.aliased(&ctx.VertexAttrib4usvARB);
     ctx.VertexAttrib4usvARB.aliased(&ctx.VertexAttrib4usv);
     ctx.VertexAttribDivisor.aliased(&ctx.VertexAttribDivisorARB);
     ctx.VertexAttribDivisorARB.aliased(&ctx.VertexAttribDivisor);
     ctx.VertexAttribI1i.aliased(&ctx.VertexAttribI1iEXT);
     ctx.VertexAttribI1iEXT.aliased(&ctx.VertexAttribI1i);
     ctx.VertexAttribI1iv.aliased(&ctx.VertexAttribI1ivEXT);
     ctx.VertexAttribI1ivEXT.aliased(&ctx.VertexAttribI1iv);
     ctx.VertexAttribI1ui.aliased(&ctx.VertexAttribI1uiEXT);
     ctx.VertexAttribI1uiEXT.aliased(&ctx.VertexAttribI1ui);
     ctx.VertexAttribI1uiv.aliased(&ctx.VertexAttribI1uivEXT);
     ctx.VertexAttribI1uivEXT.aliased(&ctx.VertexAttribI1uiv);
     ctx.VertexAttribI2i.aliased(&ctx.VertexAttribI2iEXT);
     ctx.VertexAttribI2iEXT.aliased(&ctx.VertexAttribI2i);
     ctx.VertexAttribI2iv.aliased(&ctx.VertexAttribI2ivEXT);
     ctx.VertexAttribI2ivEXT.aliased(&ctx.VertexAttribI2iv);
     ctx.VertexAttribI2ui.aliased(&ctx.VertexAttribI2uiEXT);
     ctx.VertexAttribI2uiEXT.aliased(&ctx.VertexAttribI2ui);
     ctx.VertexAttribI2uiv.aliased(&ctx.VertexAttribI2uivEXT);
     ctx.VertexAttribI2uivEXT.aliased(&ctx.VertexAttribI2uiv);
     ctx.VertexAttribI3i.aliased(&ctx.VertexAttribI3iEXT);
     ctx.VertexAttribI3iEXT.aliased(&ctx.VertexAttribI3i);
     ctx.VertexAttribI3iv.aliased(&ctx.VertexAttribI3ivEXT);
     ctx.VertexAttribI3ivEXT.aliased(&ctx.VertexAttribI3iv);
     ctx.VertexAttribI3ui.aliased(&ctx.VertexAttribI3uiEXT);
     ctx.VertexAttribI3uiEXT.aliased(&ctx.VertexAttribI3ui);
     ctx.VertexAttribI3uiv.aliased(&ctx.VertexAttribI3uivEXT);
     ctx.VertexAttribI3uivEXT.aliased(&ctx.VertexAttribI3uiv);
     ctx.VertexAttribI4bv.aliased(&ctx.VertexAttribI4bvEXT);
     ctx.VertexAttribI4bvEXT.aliased(&ctx.VertexAttribI4bv);
     ctx.VertexAttribI4i.aliased(&ctx.VertexAttribI4iEXT);
     ctx.VertexAttribI4iEXT.aliased(&ctx.VertexAttribI4i);
     ctx.VertexAttribI4iv.aliased(&ctx.VertexAttribI4ivEXT);
     ctx.VertexAttribI4ivEXT.aliased(&ctx.VertexAttribI4iv);
     ctx.VertexAttribI4sv.aliased(&ctx.VertexAttribI4svEXT);
     ctx.VertexAttribI4svEXT.aliased(&ctx.VertexAttribI4sv);
     ctx.VertexAttribI4ubv.aliased(&ctx.VertexAttribI4ubvEXT);
     ctx.VertexAttribI4ubvEXT.aliased(&ctx.VertexAttribI4ubv);
     ctx.VertexAttribI4ui.aliased(&ctx.VertexAttribI4uiEXT);
     ctx.VertexAttribI4uiEXT.aliased(&ctx.VertexAttribI4ui);
     ctx.VertexAttribI4uiv.aliased(&ctx.VertexAttribI4uivEXT);
     ctx.VertexAttribI4uivEXT.aliased(&ctx.VertexAttribI4uiv);
     ctx.VertexAttribI4usv.aliased(&ctx.VertexAttribI4usvEXT);
     ctx.VertexAttribI4usvEXT.aliased(&ctx.VertexAttribI4usv);
     ctx.VertexAttribIPointer.aliased(&ctx.VertexAttribIPointerEXT);
     ctx.VertexAttribIPointerEXT.aliased(&ctx.VertexAttribIPointer);
     ctx.VertexAttribL1d.aliased(&ctx.VertexAttribL1dEXT);
     ctx.VertexAttribL1dEXT.aliased(&ctx.VertexAttribL1d);
     ctx.VertexAttribL1dv.aliased(&ctx.VertexAttribL1dvEXT);
     ctx.VertexAttribL1dvEXT.aliased(&ctx.VertexAttribL1dv);
     ctx.VertexAttribL2d.aliased(&ctx.VertexAttribL2dEXT);
     ctx.VertexAttribL2dEXT.aliased(&ctx.VertexAttribL2d);
     ctx.VertexAttribL2dv.aliased(&ctx.VertexAttribL2dvEXT);
     ctx.VertexAttribL2dvEXT.aliased(&ctx.VertexAttribL2dv);
     ctx.VertexAttribL3d.aliased(&ctx.VertexAttribL3dEXT);
     ctx.VertexAttribL3dEXT.aliased(&ctx.VertexAttribL3d);
     ctx.VertexAttribL3dv.aliased(&ctx.VertexAttribL3dvEXT);
     ctx.VertexAttribL3dvEXT.aliased(&ctx.VertexAttribL3dv);
     ctx.VertexAttribL4d.aliased(&ctx.VertexAttribL4dEXT);
     ctx.VertexAttribL4dEXT.aliased(&ctx.VertexAttribL4d);
     ctx.VertexAttribL4dv.aliased(&ctx.VertexAttribL4dvEXT);
     ctx.VertexAttribL4dvEXT.aliased(&ctx.VertexAttribL4dv);
     ctx.VertexAttribLPointer.aliased(&ctx.VertexAttribLPointerEXT);
     ctx.VertexAttribLPointerEXT.aliased(&ctx.VertexAttribLPointer);
     ctx.VertexAttribPointer.aliased(&ctx.VertexAttribPointerARB);
     ctx.VertexAttribPointerARB.aliased(&ctx.VertexAttribPointer);

     ctx
}

