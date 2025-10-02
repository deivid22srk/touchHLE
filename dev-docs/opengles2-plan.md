# OpenGL ES 2.0 support plan

## Goals
- Allow iOS apps requesting `EAGLRenderingAPIOpenGLES2` (and 3) to obtain a working `EAGLContext`.
- Provide full coverage of the OpenGL ES 2.0 core profile functions plus common extensions used by 2010-era games (framebuffers, vertex buffers, shaders, etc.).
- Integrate with existing GLES 1.1 pipeline (fixed function) without breaking current titles.
- Avoid regressions on platforms without native GLES 2.0 by layering over desktop GL (via EGL, ANGLE or glow).

## High-level architecture

```
+-------------------+      +-------------------+
| ObjC runtime      |      | dyld symbol linker |
| EAGLContext       |----->| _gl* symbol table  |
+-------------------+      +-------------------+
         |                          |
         v                          v
+-------------------+      +-------------------+
| framework state   |      | GLES2 trait impl  |
| opengles::State   |<---->| (host backend)    |
+-------------------+      +-------------------+
```

1. **Backend selection**
   - Introduce `gles::backend` module with two implementations: `Gles1Backend` (existing) and new `Gles2Backend`.
   - Backend chosen at `EAGLContext::initWithAPI` time and stored inside `EAGLContextHostObject`.
   - `opengles::sync_context` updated to hand out an enum/trait object that knows whether programmable pipeline is active.

2. **Trait definition**
   - Split current `GLES` trait into:
     - `GlesBase` (common operations: context management, buffer objects, renderbuffers, etc.).
     - `Gles1` (fixed function extras) and `Gles2` (programmable pipeline functions: shaders, uniforms, vertex attribs).
   - Provide `dyn GlesAny` wrapper that exposes dispatch points used by high-level code.

3. **Symbol resolution**
   - Extend `dyld::FUNCTION_LISTS` with `_gl*` entries mapping to new host shims.
   - Shims delegate to current context via `framework_state.opengles` and call either GLES1 or GLES2 interface.
   - Introduce fast path for ES2-only functions (e.g. `glCreateShader`) to avoid runtime panics.

4. **Context management**
   - `EAGLContextHostObject` stores:
     ```rust
     enum GlesContext {
         Gles1(Box<dyn Gles1>),
         Gles2(Box<dyn Gles2>),
     }
     ```
   - `setCurrentContext:` puts enum into thread-local storage; shims branch on variant.
   - `renderbufferStorage:fromDrawable:` chooses correct attachment behavior for ES2 FBO pipeline (color + depth FBOs).

5. **Backend implementation strategy**
   - Short term: re-use desktop OpenGL via `glutin`/`raw-gl-context` when host supports GL 2.0+. Map ES2 calls to desktop GL equivalents (consider ANGLE translation on Windows).
   - Long term: optional feature to use `angle` static libs or `wgpu` for portability.

6. **Resource lifetime & sharegroups**
   - Update sharegroup logic to share `renderbuffer_drawable_bindings` and program caches between contexts.
   - Ensure `retain`/`release` new objects for ES2 like shaders, programs, and FBOs.

7. **Testing**
   - Unit tests for symbol resolution verifying that previously missing functions now exist.
   - Integration smoke test with simple GLES2 sample app (add to `tests` folder) to render a triangle.
   - Manual validation on Assassin's Creed HD build and other ES2 titles.

## Implementation phases (mirrors todo list)
1. Skeleton + trait refactor.
2. Bindings and backend scaffolding.
3. EAGLContext + runtime wiring.
4. Implement function shims (buffer, framebuffer, shader pipeline).
5. Testing/build validation.
