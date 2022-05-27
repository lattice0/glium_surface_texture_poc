#![cfg(feature = "android_surface_texture")]
/*!

Backend implementation for an Android Surface

# Features

Only available if the 'android_surface_texture' feature is enabled.

*/
use crate::Frame;
//use std::borrow::Borrow;
use crate::backend;
use crate::backend::Context;
use crate::context;
use crate::debug;
use std::cell::{Cell};
use std::rc::Rc;
use core::ops::Deref;
use crate::backend::Backend;
use crate::{SwapBuffersError};
pub use ndk::surface_texture::SurfaceTexture;
//use takeable_option::Takeable;
use std::os::raw::c_void;
use std::sync::{Arc, Mutex};
use crate::{IncompatibleOpenGl};

lazy_static! {
    static ref CURRENT_SURFACE_TEXTURE: Arc<Mutex<Option<SurfaceTexture>>> = Arc::new(Mutex::new(None));
}

/// Contains the context, implementing additional functionality like
/// keeping track if this SurfaceTexture is the current one being
/// rendered, or dimensions
#[derive(Clone)]
pub struct SurfaceTextureContext {
    surface_texture: Arc<Mutex<Option<SurfaceTexture>>>,
    dimensions: (u32, u32),
    is_current: bool,
    tex_name: i32
}

unsafe impl Sync for SurfaceTextureContext{}

impl SurfaceTextureContext {
    /// Crates a new  SurfaceTextureContext based on parameters
    pub fn new(surface_texture: Arc<Mutex<Option<SurfaceTexture>>>,dimensions: (u32, u32), is_current: bool, tex_name: i32) -> SurfaceTextureContext {
        SurfaceTextureContext {
            surface_texture,
            dimensions,
            is_current,
            tex_name
        }
    }
    /// Rebuild the SurfaceTexture for new requirements
    /// TODO: add new requirements in arguments
    pub fn rebuild(&self) -> SurfaceTextureContext {
        println!("rebuild called");
        todo!()
    }

    /// Ataches to the GL context
    pub fn attach(&self) {
        self.surface_texture.lock().unwrap().as_ref().unwrap().attach_to_gl_context(self.tex_name as u32).unwrap();
    }

    ///Detaches from the GL context
    pub fn detach(&self) {
        self.surface_texture.lock().unwrap().as_ref().unwrap().detach_from_gl_context().unwrap();
    }
}

extern "C" {
    fn eglGetProcAddress(name: *const libc::c_char) -> *const c_void;
}

/// The display
pub struct Display {
    context: Rc<context::Context>,
    // The glutin Window alongside its associated GL Context.
    //gl_window: Rc<RefCell<Takeable<SurfaceTextureContext>>>,
    // Used to check whether the framebuffer dimensions have changed between frames. If they have,
    // the glutin context must be resized accordingly.
    last_framebuffer_dimensions: Cell<(u32, u32)>,
}

impl Display {
    /// Creates a new display
    pub fn new(
        surface_texture_context: SurfaceTextureContext,
        debug: debug::DebugCallbackBehavior,
        checked: bool,
    ) -> Result<Self, IncompatibleOpenGl> {
        //let gl_window = Rc::new(RefCell::new(Takeable::new(surface_texture_context)));
        let glutin_backend = GlutinBackend(surface_texture_context);
        let framebuffer_dimensions = glutin_backend.get_framebuffer_dimensions();
        let context = unsafe { context::Context::new(glutin_backend, checked, debug) }?;
        Ok(Display {
            //gl_window,
            context,
            last_framebuffer_dimensions: Cell::new(framebuffer_dimensions),
        })
    }
    /// Draws
    #[inline]
    pub fn draw(&self) -> Frame {
        let (w, h) = self.get_framebuffer_dimensions();

        // If the size of the framebuffer has changed, resize the context.
        if self.last_framebuffer_dimensions.get() != (w, h) {
            self.last_framebuffer_dimensions.set((w, h));
            //self.gl_window.borrow().resize((w, h).into());
            println!("todo: resize!")
        }

        Frame::new(self.context.clone(), (w, h))
    }
}

impl Deref for Display {
    type Target = Context;
    #[inline]
    fn deref(&self) -> &Context {
        &self.context
    }
}

impl backend::Facade for Display {
    #[inline]
    fn get_context(&self) -> &Rc<Context> {
        &self.context
    }
}

unsafe impl Backend for SurfaceTextureContext {
    #[inline]
    fn swap_buffers(&self) -> Result<(), SwapBuffersError> {
        println!("swap_buffers!");
        Ok(())
    }

    #[inline]
    unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
        eglGetProcAddress(symbol.as_ptr() as *const libc::c_char)
    }

    #[inline]
    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        self.dimensions.clone()
    }

    #[inline]
    fn is_current(&self) -> bool {
        self.is_current
    }

    #[inline]
    unsafe fn make_current(&self) {
        //self.attach();
    }
}


/// The backend
#[derive(Clone)]
pub struct GlutinBackend(SurfaceTextureContext);

unsafe impl Backend for GlutinBackend {
    #[inline]
    fn swap_buffers(&self) -> Result<(), SwapBuffersError> {
        match self.0.swap_buffers() {
            Ok(()) => Ok(()),
            Err(e) => {
                panic!("Error while swapping buffers: {:?}", e)
            }
        }
    }

    #[inline]
    unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
        self.0.get_proc_address(symbol) as *const _
    }

    #[inline]
    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        self.0.get_framebuffer_dimensions()
    }

    #[inline]
    fn is_current(&self) -> bool {
        self.0.is_current()
    }

    #[inline]
    unsafe fn make_current(&self) {
        self.0.make_current()
    }
}


/*
/// An implementation of the `Backend` trait for glutin.
#[derive(Clone)]
pub struct SurfaceBackend(Rc<RefCell<Takeable<SurfaceTextureContext>>>);

impl Deref for SurfaceBackend {
    type Target = Rc<RefCell<Takeable<SurfaceTextureContext>>>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl Backend for SurfaceBackend {
    #[inline]
    fn swap_buffers(&self) -> Result<(), SwapBuffersError> {
        match self.borrow().swap_buffers() {
            Ok(()) => Ok(()),
            Err(e) => {
                //TODO: do not panic, treat more errors
                panic!("Error while swapping buffers: {:?}", e)
            }
        }
    }

    #[inline]
    unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
        self.borrow().get_proc_address(symbol) as *const _
    }

    #[inline]
    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        self.borrow().get_framebuffer_dimensions()
    }

    #[inline]
    fn is_current(&self) -> bool {
        self.borrow().is_current()
    }

    #[inline]
    unsafe fn make_current(&self) {
        self.borrow().make_current()
    }
}

extern "C" {
    fn eglGetProcAddress(name: *const libc::c_char) -> *const c_void;
}

unsafe impl Backend for SurfaceTextureContext {
    #[inline]
    fn swap_buffers(&self) -> Result<(), SwapBuffersError> {
        println!("swap_buffers!");
        Ok(())
    }

    #[inline]
    unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
        eglGetProcAddress(symbol.as_ptr() as *const libc::c_char)
    }

    #[inline]
    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        self.dimensions.clone()
    }

    #[inline]
    fn is_current(&self) -> bool {
        self.is_current
    }

    #[inline]
    unsafe fn make_current(&self) {
        self.attach();
    }
}
*/