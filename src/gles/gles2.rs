/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub use touchHLE_gl_bindings::gles20 as gles20_raw;

use crate::gles::gles11_raw::types::{GLclampx, GLfixed};
use crate::gles::GLES;
use crate::window::{GLContext, GLVersion, Window};
use gles20::types::*;
use gles20_raw as gles20;
use std::ffi::CStr;

macro_rules! forward_gles20_ret {
    ($(unsafe fn $name:ident(&mut self $(, $arg:ident : $ty:ty)*) -> $ret:ty;)+) => {
        $(
            unsafe fn $name(&mut self, $( $arg: $ty ),*) -> $ret {
                gles20::$name($( $arg ),*)
            }
        )+
    };
}

macro_rules! forward_gles20_void {
    ($(unsafe fn $name:ident(&mut self $(, $arg:ident : $ty:ty)*);)+) => {
        $(
            unsafe fn $name(&mut self, $( $arg: $ty ),*) {
                gles20::$name($( $arg ),*)
            }
        )+
    };
}

macro_rules! unsupported_gles2 {
    ($(unsafe fn $name:ident(&mut self $(, $arg:ident : $ty:ty)*) $(-> $ret:ty)?;)+) => {
        $(
            unsafe fn $name(&mut self, $( $arg: $ty ),*) $(-> $ret)? {
                panic!(concat!(stringify!($name), " is unsupported on OpenGL ES 2.0 backend"))
            }
        )+
    };
}

pub trait GLES2: GLES {
    unsafe fn AttachShader(&mut self, program: GLuint, shader: GLuint);
    unsafe fn BindAttribLocation(&mut self, program: GLuint, index: GLuint, name: *const GLchar);
    unsafe fn BlendColor(&mut self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
    unsafe fn BlendEquationSeparate(&mut self, mode_rgb: GLenum, mode_alpha: GLenum);
    unsafe fn BlendFuncSeparate(
        &mut self,
        src_rgb: GLenum,
        dst_rgb: GLenum,
        src_alpha: GLenum,
        dst_alpha: GLenum,
    );
    unsafe fn CompileShader(&mut self, shader: GLuint);
    unsafe fn CreateProgram(&mut self) -> GLuint;
    unsafe fn CreateShader(&mut self, type_: GLenum) -> GLuint;
    unsafe fn DeleteProgram(&mut self, program: GLuint);
    unsafe fn DeleteShader(&mut self, shader: GLuint);
    unsafe fn DetachShader(&mut self, program: GLuint, shader: GLuint);
    unsafe fn DisableVertexAttribArray(&mut self, index: GLuint);
    unsafe fn EnableVertexAttribArray(&mut self, index: GLuint);
    unsafe fn GetActiveAttrib(
        &mut self,
        program: GLuint,
        index: GLuint,
        bufsize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        type_: *mut GLenum,
        name: *mut GLchar,
    );
    unsafe fn GetActiveUniform(
        &mut self,
        program: GLuint,
        index: GLuint,
        bufsize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        type_: *mut GLenum,
        name: *mut GLchar,
    );
    unsafe fn GetAttachedShaders(
        &mut self,
        program: GLuint,
        maxcount: GLsizei,
        count: *mut GLsizei,
        shaders: *mut GLuint,
    );
    unsafe fn GetAttribLocation(&mut self, program: GLuint, name: *const GLchar) -> GLint;
    unsafe fn GetProgramInfoLog(
        &mut self,
        program: GLuint,
        bufsize: GLsizei,
        length: *mut GLsizei,
        infolog: *mut GLchar,
    );
    unsafe fn GetProgramiv(&mut self, program: GLuint, pname: GLenum, params: *mut GLint);
    unsafe fn GetShaderInfoLog(
        &mut self,
        shader: GLuint,
        bufsize: GLsizei,
        length: *mut GLsizei,
        infolog: *mut GLchar,
    );
    unsafe fn GetShaderPrecisionFormat(
        &mut self,
        shadertype: GLenum,
        precisiontype: GLenum,
        range: *mut GLint,
        precision: *mut GLint,
    );
    unsafe fn GetShaderSource(
        &mut self,
        shader: GLuint,
        bufsize: GLsizei,
        length: *mut GLsizei,
        source: *mut GLchar,
    );
    unsafe fn GetShaderiv(&mut self, shader: GLuint, pname: GLenum, params: *mut GLint);
    unsafe fn GetUniformfv(&mut self, program: GLuint, location: GLint, params: *mut GLfloat);
    unsafe fn GetUniformiv(&mut self, program: GLuint, location: GLint, params: *mut GLint);
    unsafe fn GetUniformLocation(&mut self, program: GLuint, name: *const GLchar) -> GLint;
    unsafe fn GetVertexAttribPointerv(
        &mut self,
        index: GLuint,
        pname: GLenum,
        pointer: *mut *mut GLvoid,
    );
    unsafe fn GetVertexAttribfv(&mut self, index: GLuint, pname: GLenum, params: *mut GLfloat);
    unsafe fn GetVertexAttribiv(&mut self, index: GLuint, pname: GLenum, params: *mut GLint);
    unsafe fn IsProgram(&mut self, program: GLuint) -> GLboolean;
    unsafe fn IsShader(&mut self, shader: GLuint) -> GLboolean;
    unsafe fn LinkProgram(&mut self, program: GLuint);
    unsafe fn ReleaseShaderCompiler(&mut self);
    unsafe fn ShaderBinary(
        &mut self,
        n: GLsizei,
        shaders: *const GLuint,
        binaryformat: GLenum,
        binary: *const GLvoid,
        length: GLsizei,
    );
    unsafe fn ShaderSource(
        &mut self,
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    );
    unsafe fn StencilFuncSeparate(&mut self, face: GLenum, func: GLenum, ref_: GLint, mask: GLuint);
    unsafe fn StencilMaskSeparate(&mut self, face: GLenum, mask: GLuint);
    unsafe fn StencilOpSeparate(
        &mut self,
        face: GLenum,
        sfail: GLenum,
        dpfail: GLenum,
        dppass: GLenum,
    );
    unsafe fn Uniform1f(&mut self, location: GLint, v0: GLfloat);
    unsafe fn Uniform1fv(&mut self, location: GLint, count: GLsizei, value: *const GLfloat);
    unsafe fn Uniform1i(&mut self, location: GLint, v0: GLint);
    unsafe fn Uniform1iv(&mut self, location: GLint, count: GLsizei, value: *const GLint);
    unsafe fn Uniform2f(&mut self, location: GLint, v0: GLfloat, v1: GLfloat);
    unsafe fn Uniform2fv(&mut self, location: GLint, count: GLsizei, value: *const GLfloat);
    unsafe fn Uniform2i(&mut self, location: GLint, v0: GLint, v1: GLint);
    unsafe fn Uniform2iv(&mut self, location: GLint, count: GLsizei, value: *const GLint);
    unsafe fn Uniform3f(&mut self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
    unsafe fn Uniform3fv(&mut self, location: GLint, count: GLsizei, value: *const GLfloat);
    unsafe fn Uniform3i(&mut self, location: GLint, v0: GLint, v1: GLint, v2: GLint);
    unsafe fn Uniform3iv(&mut self, location: GLint, count: GLsizei, value: *const GLint);
    unsafe fn Uniform4f(
        &mut self,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
        v3: GLfloat,
    );
    unsafe fn Uniform4fv(&mut self, location: GLint, count: GLsizei, value: *const GLfloat);
    unsafe fn Uniform4i(&mut self, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
    unsafe fn Uniform4iv(&mut self, location: GLint, count: GLsizei, value: *const GLint);
    unsafe fn UniformMatrix2fv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
    unsafe fn UniformMatrix3fv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
    unsafe fn UniformMatrix4fv(
        &mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
    unsafe fn UseProgram(&mut self, program: GLuint);
    unsafe fn ValidateProgram(&mut self, program: GLuint);
    unsafe fn VertexAttrib1f(&mut self, index: GLuint, x: GLfloat);
    unsafe fn VertexAttrib1fv(&mut self, index: GLuint, v: *const GLfloat);
    unsafe fn VertexAttrib2f(&mut self, index: GLuint, x: GLfloat, y: GLfloat);
    unsafe fn VertexAttrib2fv(&mut self, index: GLuint, v: *const GLfloat);
    unsafe fn VertexAttrib3f(&mut self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
    unsafe fn VertexAttrib3fv(&mut self, index: GLuint, v: *const GLfloat);
    unsafe fn VertexAttrib4f(
        &mut self,
        index: GLuint,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
        w: GLfloat,
    );
    unsafe fn VertexAttrib4fv(&mut self, index: GLuint, v: *const GLfloat);
    unsafe fn VertexAttribPointer(
        &mut self,
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const GLvoid,
    );
}

pub struct GLES2Native {
    gl_ctx: GLContext,
}

impl GLES for GLES2Native {
    fn description() -> &'static str {
        "Native OpenGL ES 2.0"
    }

    fn new(window: &mut Window) -> Result<Self, String>
    where
        Self: Sized,
    {
        Ok(Self {
            gl_ctx: window.create_gl_context(GLVersion::GLES20)?,
        })
    }

    fn make_current(&self, window: &Window) {
        unsafe { window.make_gl_context_current(&self.gl_ctx) };
        gles20::load_with(|s| window.gl_get_proc_address(s));
    }

    unsafe fn driver_description(&self) -> String {
        let version = CStr::from_ptr(gles20::GetString(gles20::VERSION) as *const _);
        let vendor = CStr::from_ptr(gles20::GetString(gles20::VENDOR) as *const _);
        let renderer = CStr::from_ptr(gles20::GetString(gles20::RENDERER) as *const _);
        format!(
            "{} / {} / {}",
            version.to_string_lossy(),
            vendor.to_string_lossy(),
            renderer.to_string_lossy()
        )
    }

    forward_gles20_ret! {
        unsafe fn GetError(&mut self) -> GLenum;
        unsafe fn IsEnabled(&mut self, cap: GLenum) -> GLboolean;
        unsafe fn GetString(&mut self, name: GLenum) -> *const GLubyte;
        unsafe fn IsTexture(&mut self, texture: GLuint) -> GLboolean;
        unsafe fn IsBuffer(&mut self, buffer: GLuint) -> GLboolean;
    }

    forward_gles20_void! {
        unsafe fn Enable(&mut self, cap: GLenum);
        unsafe fn Disable(&mut self, cap: GLenum);
        unsafe fn BlendFunc(&mut self, sfactor: GLenum, dfactor: GLenum);
        unsafe fn ColorMask(&mut self, red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
        unsafe fn CullFace(&mut self, mode: GLenum);
        unsafe fn DepthFunc(&mut self, func: GLenum);
        unsafe fn DepthMask(&mut self, flag: GLboolean);
        unsafe fn FrontFace(&mut self, mode: GLenum);
        unsafe fn DepthRangef(&mut self, near: GLclampf, far: GLclampf);
        unsafe fn PolygonOffset(&mut self, factor: GLfloat, units: GLfloat);
        unsafe fn SampleCoverage(&mut self, value: GLclampf, invert: GLboolean);
        unsafe fn Scissor(&mut self, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
        unsafe fn Viewport(&mut self, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
        unsafe fn LineWidth(&mut self, val: GLfloat);
        unsafe fn StencilFunc(&mut self, func: GLenum, ref_: GLint, mask: GLuint);
        unsafe fn StencilOp(&mut self, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
        unsafe fn StencilMask(&mut self, mask: GLuint);
        unsafe fn GenBuffers(&mut self, n: GLsizei, buffers: *mut GLuint);
        unsafe fn DeleteBuffers(&mut self, n: GLsizei, buffers: *const GLuint);
        unsafe fn BindBuffer(&mut self, target: GLenum, buffer: GLuint);
        unsafe fn BufferData(&mut self, target: GLenum, size: GLsizeiptr, data: *const GLvoid, usage: GLenum);
        unsafe fn BufferSubData(&mut self, target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const GLvoid);
        unsafe fn DrawArrays(&mut self, mode: GLenum, first: GLint, count: GLsizei);
        unsafe fn DrawElements(&mut self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const GLvoid);
        unsafe fn Clear(&mut self, mask: GLbitfield);
        unsafe fn ClearColor(&mut self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
        unsafe fn ClearDepthf(&mut self, depth: GLclampf);
        unsafe fn ClearStencil(&mut self, s: GLint);
        unsafe fn PixelStorei(&mut self, pname: GLenum, param: GLint);
        unsafe fn ReadPixels(&mut self, x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut GLvoid);
        unsafe fn GenTextures(&mut self, n: GLsizei, textures: *mut GLuint);
        unsafe fn DeleteTextures(&mut self, n: GLsizei, textures: *const GLuint);
        unsafe fn ActiveTexture(&mut self, texture: GLenum);
        unsafe fn BindTexture(&mut self, target: GLenum, texture: GLuint);
        unsafe fn TexParameteri(&mut self, target: GLenum, pname: GLenum, param: GLint);
        unsafe fn TexParameterf(&mut self, target: GLenum, pname: GLenum, param: GLfloat);
        unsafe fn TexParameteriv(&mut self, target: GLenum, pname: GLenum, params: *const GLint);
        unsafe fn TexParameterfv(&mut self, target: GLenum, pname: GLenum, params: *const GLfloat);
        unsafe fn TexImage2D(&mut self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const GLvoid);
        unsafe fn TexSubImage2D(&mut self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const GLvoid);
        unsafe fn CompressedTexImage2D(&mut self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, image_size: GLsizei, data: *const GLvoid);
        unsafe fn CopyTexImage2D(&mut self, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);
        unsafe fn CopyTexSubImage2D(&mut self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
        unsafe fn GetBooleanv(&mut self, pname: GLenum, params: *mut GLboolean);
        unsafe fn GetFloatv(&mut self, pname: GLenum, params: *mut GLfloat);
        unsafe fn GetIntegerv(&mut self, pname: GLenum, params: *mut GLint);
        unsafe fn GetBufferParameteriv(&mut self, target: GLenum, pname: GLenum, params: *mut GLint);
        unsafe fn Flush(&mut self);
        unsafe fn Finish(&mut self);
        unsafe fn Hint(&mut self, target: GLenum, mode: GLenum);
    }

    unsafe fn BlendEquationOES(&mut self, mode: GLenum) {
        gles20::BlendEquation(mode);
    }

    unsafe fn GenFramebuffersOES(&mut self, n: GLsizei, framebuffers: *mut GLuint) {
        gles20::GenFramebuffers(n, framebuffers);
    }

    unsafe fn GenRenderbuffersOES(&mut self, n: GLsizei, renderbuffers: *mut GLuint) {
        gles20::GenRenderbuffers(n, renderbuffers);
    }

    unsafe fn IsFramebufferOES(&mut self, framebuffer: GLuint) -> GLboolean {
        gles20::IsFramebuffer(framebuffer)
    }

    unsafe fn IsRenderbufferOES(&mut self, renderbuffer: GLuint) -> GLboolean {
        gles20::IsRenderbuffer(renderbuffer)
    }

    unsafe fn BindFramebufferOES(&mut self, target: GLenum, framebuffer: GLuint) {
        gles20::BindFramebuffer(target, framebuffer);
    }

    unsafe fn BindRenderbufferOES(&mut self, target: GLenum, renderbuffer: GLuint) {
        gles20::BindRenderbuffer(target, renderbuffer);
    }

    unsafe fn RenderbufferStorageOES(
        &mut self,
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        gles20::RenderbufferStorage(target, internalformat, width, height);
    }

    unsafe fn FramebufferRenderbufferOES(
        &mut self,
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint,
    ) {
        gles20::FramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer);
    }

    unsafe fn FramebufferTexture2DOES(
        &mut self,
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
    ) {
        gles20::FramebufferTexture2D(target, attachment, textarget, texture, level);
    }

    unsafe fn GetFramebufferAttachmentParameterivOES(
        &mut self,
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        gles20::GetFramebufferAttachmentParameteriv(target, attachment, pname, params);
    }

    unsafe fn GetRenderbufferParameterivOES(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        gles20::GetRenderbufferParameteriv(target, pname, params);
    }

    unsafe fn CheckFramebufferStatusOES(&mut self, target: GLenum) -> GLenum {
        gles20::CheckFramebufferStatus(target)
    }

    unsafe fn DeleteFramebuffersOES(&mut self, n: GLsizei, framebuffers: *const GLuint) {
        gles20::DeleteFramebuffers(n, framebuffers);
    }

    unsafe fn DeleteRenderbuffersOES(&mut self, n: GLsizei, renderbuffers: *const GLuint) {
        gles20::DeleteRenderbuffers(n, renderbuffers);
    }

    unsafe fn GenerateMipmapOES(&mut self, target: GLenum) {
        gles20::GenerateMipmap(target);
    }

    unsupported_gles2! {
        unsafe fn ClientActiveTexture(&mut self, texture: GLenum);
        unsafe fn EnableClientState(&mut self, array: GLenum);
        unsafe fn DisableClientState(&mut self, array: GLenum);
        unsafe fn GetTexEnviv(&mut self, target: GLenum, pname: GLenum, params: *mut GLint);
        unsafe fn GetTexEnvfv(&mut self, target: GLenum, pname: GLenum, params: *mut GLfloat);
        unsafe fn GetPointerv(&mut self, pname: GLenum, params: *mut *const GLvoid);
        unsafe fn AlphaFunc(&mut self, func: GLenum, ref_: GLclampf);
        unsafe fn AlphaFuncx(&mut self, func: GLenum, ref_: GLclampx);
        unsafe fn ClipPlanef(&mut self, plane: GLenum, equation: *const GLfloat);
        unsafe fn ClipPlanex(&mut self, plane: GLenum, equation: *const GLfixed);
        unsafe fn DepthRangex(&mut self, near: GLclampx, far: GLclampx);
        unsafe fn PolygonOffsetx(&mut self, factor: GLfixed, units: GLfixed);
        unsafe fn SampleCoveragex(&mut self, value: GLclampx, invert: GLboolean);
        unsafe fn ShadeModel(&mut self, mode: GLenum);
        unsafe fn LineWidthx(&mut self, val: GLfixed);
        unsafe fn PointSize(&mut self, size: GLfloat);
        unsafe fn PointSizex(&mut self, size: GLfixed);
        unsafe fn PointParameterf(&mut self, pname: GLenum, param: GLfloat);
        unsafe fn PointParameterx(&mut self, pname: GLenum, param: GLfixed);
        unsafe fn PointParameterfv(&mut self, pname: GLenum, params: *const GLfloat);
        unsafe fn PointParameterxv(&mut self, pname: GLenum, params: *const GLfixed);
        unsafe fn Fogf(&mut self, pname: GLenum, param: GLfloat);
        unsafe fn Fogx(&mut self, pname: GLenum, param: GLfixed);
        unsafe fn Fogfv(&mut self, pname: GLenum, params: *const GLfloat);
        unsafe fn Fogxv(&mut self, pname: GLenum, params: *const GLfixed);
        unsafe fn Lightf(&mut self, light: GLenum, pname: GLenum, param: GLfloat);
        unsafe fn Lightx(&mut self, light: GLenum, pname: GLenum, param: GLfixed);
        unsafe fn Lightfv(&mut self, light: GLenum, pname: GLenum, params: *const GLfloat);
        unsafe fn Lightxv(&mut self, light: GLenum, pname: GLenum, params: *const GLfixed);
        unsafe fn LightModelf(&mut self, pname: GLenum, param: GLfloat);
        unsafe fn LightModelx(&mut self, pname: GLenum, param: GLfixed);
        unsafe fn LightModelfv(&mut self, pname: GLenum, params: *const GLfloat);
        unsafe fn LightModelxv(&mut self, pname: GLenum, params: *const GLfixed);
        unsafe fn Materialf(&mut self, face: GLenum, pname: GLenum, param: GLfloat);
        unsafe fn Materialx(&mut self, face: GLenum, pname: GLenum, param: GLfixed);
        unsafe fn Materialfv(&mut self, face: GLenum, pname: GLenum, params: *const GLfloat);
        unsafe fn Materialxv(&mut self, face: GLenum, pname: GLenum, params: *const GLfixed);
        unsafe fn Color4f(&mut self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
        unsafe fn Color4x(&mut self, red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);
        unsafe fn Color4ub(&mut self, red: GLubyte, green: GLubyte, blue: GLubyte, alpha: GLubyte);
        unsafe fn Normal3f(&mut self, nx: GLfloat, ny: GLfloat, nz: GLfloat);
        unsafe fn Normal3x(&mut self, nx: GLfixed, ny: GLfixed, nz: GLfixed);
        unsafe fn ColorPointer(&mut self, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const GLvoid);
        unsafe fn NormalPointer(&mut self, type_: GLenum, stride: GLsizei, pointer: *const GLvoid);
        unsafe fn TexCoordPointer(&mut self, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const GLvoid);
        unsafe fn VertexPointer(&mut self, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const GLvoid);
        unsafe fn ClearColorx(&mut self, red: GLclampx, green: GLclampx, blue: GLclampx, alpha: GLclampx);
        unsafe fn ClearDepthx(&mut self, depth: GLclampx);
        unsafe fn TexParameterx(&mut self, target: GLenum, pname: GLenum, param: GLfixed);
        unsafe fn TexParameterxv(&mut self, target: GLenum, pname: GLenum, params: *const GLfixed);
        unsafe fn TexEnvf(&mut self, target: GLenum, pname: GLenum, param: GLfloat);
        unsafe fn TexEnvx(&mut self, target: GLenum, pname: GLenum, param: GLfixed);
        unsafe fn TexEnvi(&mut self, target: GLenum, pname: GLenum, param: GLint);
        unsafe fn TexEnvfv(&mut self, target: GLenum, pname: GLenum, params: *const GLfloat);
        unsafe fn TexEnvxv(&mut self, target: GLenum, pname: GLenum, params: *const GLfixed);
        unsafe fn TexEnviv(&mut self, target: GLenum, pname: GLenum, params: *const GLint);
        unsafe fn MatrixMode(&mut self, mode: GLenum);
        unsafe fn LoadIdentity(&mut self);
        unsafe fn LoadMatrixf(&mut self, m: *const GLfloat);
        unsafe fn LoadMatrixx(&mut self, m: *const GLfixed);
        unsafe fn MultMatrixf(&mut self, m: *const GLfloat);
        unsafe fn MultMatrixx(&mut self, m: *const GLfixed);
        unsafe fn PushMatrix(&mut self);
        unsafe fn PopMatrix(&mut self);
        unsafe fn Orthof(&mut self, left: GLfloat, right: GLfloat, bottom: GLfloat, top: GLfloat, near: GLfloat, far: GLfloat);
        unsafe fn Orthox(&mut self, left: GLfixed, right: GLfixed, bottom: GLfixed, top: GLfixed, near: GLfixed, far: GLfixed);
        unsafe fn Frustumf(&mut self, left: GLfloat, right: GLfloat, bottom: GLfloat, top: GLfloat, near: GLfloat, far: GLfloat);
        unsafe fn Frustumx(&mut self, left: GLfixed, right: GLfixed, bottom: GLfixed, top: GLfixed, near: GLfixed, far: GLfixed);
        unsafe fn Rotatef(&mut self, angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
        unsafe fn Rotatex(&mut self, angle: GLfixed, x: GLfixed, y: GLfixed, z: GLfixed);
        unsafe fn Scalef(&mut self, x: GLfloat, y: GLfloat, z: GLfloat);
        unsafe fn Scalex(&mut self, x: GLfixed, y: GLfixed, z: GLfixed);
        unsafe fn Translatef(&mut self, x: GLfloat, y: GLfloat, z: GLfloat);
        unsafe fn Translatex(&mut self, x: GLfixed, y: GLfixed, z: GLfixed);
        unsafe fn MapBufferOES(&mut self, target: GLenum, access: GLenum) -> *mut GLvoid;
        unsafe fn UnmapBufferOES(&mut self, target: GLenum) -> GLboolean;
    }
}

impl GLES2 for GLES2Native {
    forward_gles20_ret! {
        unsafe fn CreateProgram(&mut self) -> GLuint;
        unsafe fn CreateShader(&mut self, type_: GLenum) -> GLuint;
        unsafe fn GetAttribLocation(&mut self, program: GLuint, name: *const GLchar) -> GLint;
        unsafe fn IsProgram(&mut self, program: GLuint) -> GLboolean;
        unsafe fn IsShader(&mut self, shader: GLuint) -> GLboolean;
        unsafe fn GetUniformLocation(&mut self, program: GLuint, name: *const GLchar) -> GLint;
    }

    forward_gles20_void! {
        unsafe fn AttachShader(&mut self, program: GLuint, shader: GLuint);
        unsafe fn BindAttribLocation(&mut self, program: GLuint, index: GLuint, name: *const GLchar);
        unsafe fn BlendColor(&mut self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
        unsafe fn BlendEquationSeparate(&mut self, mode_rgb: GLenum, mode_alpha: GLenum);
        unsafe fn BlendFuncSeparate(&mut self, src_rgb: GLenum, dst_rgb: GLenum, src_alpha: GLenum, dst_alpha: GLenum);
        unsafe fn CompileShader(&mut self, shader: GLuint);
        unsafe fn DeleteProgram(&mut self, program: GLuint);
        unsafe fn DeleteShader(&mut self, shader: GLuint);
        unsafe fn DetachShader(&mut self, program: GLuint, shader: GLuint);
        unsafe fn DisableVertexAttribArray(&mut self, index: GLuint);
        unsafe fn EnableVertexAttribArray(&mut self, index: GLuint);
        unsafe fn GetActiveAttrib(&mut self, program: GLuint, index: GLuint, bufsize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar);
        unsafe fn GetActiveUniform(&mut self, program: GLuint, index: GLuint, bufsize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar);
        unsafe fn GetAttachedShaders(&mut self, program: GLuint, maxcount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint);
        unsafe fn GetProgramInfoLog(&mut self, program: GLuint, bufsize: GLsizei, length: *mut GLsizei, infolog: *mut GLchar);
        unsafe fn GetProgramiv(&mut self, program: GLuint, pname: GLenum, params: *mut GLint);
        unsafe fn GetShaderInfoLog(&mut self, shader: GLuint, bufsize: GLsizei, length: *mut GLsizei, infolog: *mut GLchar);
        unsafe fn GetShaderPrecisionFormat(&mut self, shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint);
        unsafe fn GetShaderSource(&mut self, shader: GLuint, bufsize: GLsizei, length: *mut GLsizei, source: *mut GLchar);
        unsafe fn GetShaderiv(&mut self, shader: GLuint, pname: GLenum, params: *mut GLint);
        unsafe fn GetUniformfv(&mut self, program: GLuint, location: GLint, params: *mut GLfloat);
        unsafe fn GetUniformiv(&mut self, program: GLuint, location: GLint, params: *mut GLint);
        unsafe fn GetVertexAttribPointerv(&mut self, index: GLuint, pname: GLenum, pointer: *mut *mut GLvoid);
        unsafe fn GetVertexAttribfv(&mut self, index: GLuint, pname: GLenum, params: *mut GLfloat);
        unsafe fn GetVertexAttribiv(&mut self, index: GLuint, pname: GLenum, params: *mut GLint);
        unsafe fn LinkProgram(&mut self, program: GLuint);
        unsafe fn ReleaseShaderCompiler(&mut self);
        unsafe fn ShaderBinary(&mut self, n: GLsizei, shaders: *const GLuint, binaryformat: GLenum, binary: *const GLvoid, length: GLsizei);
        unsafe fn ShaderSource(&mut self, shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint);
        unsafe fn StencilFuncSeparate(&mut self, face: GLenum, func: GLenum, ref_: GLint, mask: GLuint);
        unsafe fn StencilMaskSeparate(&mut self, face: GLenum, mask: GLuint);
        unsafe fn StencilOpSeparate(&mut self, face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
        unsafe fn Uniform1f(&mut self, location: GLint, v0: GLfloat);
        unsafe fn Uniform1fv(&mut self, location: GLint, count: GLsizei, value: *const GLfloat);
        unsafe fn Uniform1i(&mut self, location: GLint, v0: GLint);
        unsafe fn Uniform1iv(&mut self, location: GLint, count: GLsizei, value: *const GLint);
        unsafe fn Uniform2f(&mut self, location: GLint, v0: GLfloat, v1: GLfloat);
        unsafe fn Uniform2fv(&mut self, location: GLint, count: GLsizei, value: *const GLfloat);
        unsafe fn Uniform2i(&mut self, location: GLint, v0: GLint, v1: GLint);
        unsafe fn Uniform2iv(&mut self, location: GLint, count: GLsizei, value: *const GLint);
        unsafe fn Uniform3f(&mut self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
        unsafe fn Uniform3fv(&mut self, location: GLint, count: GLsizei, value: *const GLfloat);
        unsafe fn Uniform3i(&mut self, location: GLint, v0: GLint, v1: GLint, v2: GLint);
        unsafe fn Uniform3iv(&mut self, location: GLint, count: GLsizei, value: *const GLint);
        unsafe fn Uniform4f(&mut self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
        unsafe fn Uniform4fv(&mut self, location: GLint, count: GLsizei, value: *const GLfloat);
        unsafe fn Uniform4i(&mut self, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
        unsafe fn Uniform4iv(&mut self, location: GLint, count: GLsizei, value: *const GLint);
        unsafe fn UniformMatrix2fv(&mut self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
        unsafe fn UniformMatrix3fv(&mut self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
        unsafe fn UniformMatrix4fv(&mut self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
        unsafe fn UseProgram(&mut self, program: GLuint);
        unsafe fn ValidateProgram(&mut self, program: GLuint);
        unsafe fn VertexAttrib1f(&mut self, index: GLuint, x: GLfloat);
        unsafe fn VertexAttrib1fv(&mut self, index: GLuint, v: *const GLfloat);
        unsafe fn VertexAttrib2f(&mut self, index: GLuint, x: GLfloat, y: GLfloat);
        unsafe fn VertexAttrib2fv(&mut self, index: GLuint, v: *const GLfloat);
        unsafe fn VertexAttrib3f(&mut self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
        unsafe fn VertexAttrib3fv(&mut self, index: GLuint, v: *const GLfloat);
        unsafe fn VertexAttrib4f(&mut self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
        unsafe fn VertexAttrib4fv(&mut self, index: GLuint, v: *const GLfloat);
        unsafe fn VertexAttribPointer(&mut self, index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const GLvoid);
    }
}
