use glium::backend::surface_texture::{Display, SurfaceTextureContext as GliumSurfaceTextureContext, SurfaceTextureContext};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use glium::{implement_vertex, program, Surface, uniform};
use glium::debug::DebugCallbackBehavior;
use glium::index::PrimitiveType;
use jni::{
    objects::{JClass},
    sys::{jboolean, jobject, jint},
    JNIEnv,
};
use log::info;
use ndk::{trace};
use ndk::surface_texture::SurfaceTexture as ASurfaceTexture;

lazy_static! {
    static ref SURFACE_TEXTURE: Arc<Mutex<Option<GliumSurfaceTextureContext>>> = Arc::new(Mutex::new(None));
}

#[no_mangle]
pub extern "system" fn Java_com_example_gliumsurfacetexturepoc_MainActivity_00024Companion_registerSurfaceTextureNativeHandler(
    env: JNIEnv,
    _class: JClass,
    surface_texture: jobject,
    width: jint,
    height: jint
) -> jboolean {
    //TODO: no unwrap and verify non null for surface texture
    let surface_texture = unsafe {
        ASurfaceTexture::from_surface_texture(env.get_native_interface(), surface_texture)
    };
    let surface_texture_context = GliumSurfaceTextureContext::new(
        Arc::new(Mutex::new(Some(surface_texture.unwrap()))),
        (width as u32, height as u32),
        false,
        0
    );
    let _ = SURFACE_TEXTURE.lock().unwrap().insert(surface_texture_context.clone());
    poc_main(surface_texture_context);
    0
}

fn poc_main(surface_texture_context: SurfaceTextureContext) {
    let _trace;
    if trace::is_trace_enabled() {
        _trace = trace::Section::new("lib.rs").unwrap();
    }
    info!("poc_main");
    let display = Display::new(surface_texture_context, DebugCallbackBehavior::PrintAll, true).unwrap();
    //--------------------------------
    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            color: [f32; 3],
        }

        implement_vertex!(Vertex, position, color);

        glium::VertexBuffer::new(&display,
                                 &[
                                     Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                                     Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
                                     Vertex { position: [ 0.5, -0.5], color: [1.0, 0.0, 0.0] },
                                 ]
        ).unwrap()
    };

    // building the index buffer
    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
                                               &[0u16, 1, 2]).unwrap();

    // compiling shaders and linking them together
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 matrix;
                in vec2 position;
                in vec3 color;
                out vec3 vColor;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 140
                in vec3 vColor;
                out vec4 f_color;
                void main() {
                    f_color = vec4(vColor, 1.0);
                }
            "
        },

        110 => {
            vertex: "
                #version 110
                uniform mat4 matrix;
                attribute vec2 position;
                attribute vec3 color;
                varying vec3 vColor;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 110
                varying vec3 vColor;
                void main() {
                    gl_FragColor = vec4(vColor, 1.0);
                }
            ",
        },

        100 => {
            vertex: "
                #version 100
                uniform lowp mat4 matrix;
                attribute lowp vec2 position;
                attribute lowp vec3 color;
                varying lowp vec3 vColor;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 100
                varying lowp vec3 vColor;
                void main() {
                    gl_FragColor = vec4(vColor, 1.0);
                }
            ",
        },
    ).unwrap();

    // Here we draw the black background and triangle to the screen using the previously
    // initialised resources.
    //
    // In this case we use a closure for simplicity, however keep in mind that most serious
    // applications should probably use a function that takes the resources as an argument.
    let draw = move || {
        // building the uniforms
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color(1.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    };

    // Draw the triangle to the screen.
    draw();
    //--------------------
}