// docs: https://github.com/oblitum/Interception/blob/master/library/interception.h

use std::ffi::c_void;
use std::mem::MaybeUninit;

pub type InterceptionContext = *mut c_void;
pub type InterceptionDevice = i32;
pub type InterceptionFilter = u16;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct InterceptionMouseStroke {
    pub state: u16,
    pub flags: u16,
    pub rolling: i16,
    pub x: i32,
    pub y: i32,
    pub information: u32,
}

const STROKE_SIZE: usize = std::mem::size_of::<InterceptionMouseStroke>();
type InterceptionStroke = [u8; STROKE_SIZE];

type InterceptionPredicate = unsafe extern "C" fn(device: InterceptionDevice) -> i32;

#[link(name = "interception")]
unsafe extern "C" {
    // InterceptionContext INTERCEPTION_API interception_create_context(void);
    fn interception_create_context() -> InterceptionContext;

    // void INTERCEPTION_API interception_destroy_context(InterceptionContext context);
    fn interception_destroy_context(context: InterceptionContext);

    // InterceptionDevice INTERCEPTION_API interception_wait(InterceptionContext context);
    fn interception_wait(context: InterceptionContext) -> InterceptionDevice;

    // int INTERCEPTION_API interception_receive(InterceptionContext context, InterceptionDevice device, InterceptionStroke *stroke, unsigned int nstroke);
    fn interception_receive(
        context: InterceptionContext,
        device: InterceptionDevice,
        stroke: *mut InterceptionStroke,
        nstroke: u32,
    ) -> i32;

    // int INTERCEPTION_API interception_send(InterceptionContext context, InterceptionDevice device, const InterceptionStroke *stroke, unsigned int nstroke);
    fn interception_send(
        conntext: InterceptionContext,
        device: InterceptionDevice,
        stroke: *const InterceptionStroke,
        nstroke: u32,
    ) -> i32;

    // int INTERCEPTION_API interception_is_mouse(InterceptionDevice device);
    fn interception_is_mouse(device: InterceptionDevice) -> i32;

    // void INTERCEPTION_API interception_set_filter(InterceptionContext context, InterceptionPredicate predicate, InterceptionFilter filter);
    fn interception_set_filter(
        context: InterceptionContext,
        predicate: InterceptionPredicate,
        filter: InterceptionFilter,
    );
}

pub struct Interceptor {
    context: InterceptionContext,
}

impl Drop for Interceptor {
    fn drop(&mut self) {
        unsafe {
            interception_destroy_context(self.context);
        }
    }
}

impl Interceptor {
    pub fn new() -> Result<Self, String> {
        let context = unsafe { interception_create_context() };

        if context.is_null() {
            return Err("error creating interceptor context".to_string());
        }

        unsafe {
            // INTERCEPTION_FILTER_MOUSE_MOVE               = 0x1000
            interception_set_filter(context, interception_is_mouse, 0x1000);
        }

        Ok(Self { context })
    }

    pub fn run(&self, params: &crate::accel::AccelParams) {
        let mut device;
        let mut stroke = MaybeUninit::<InterceptionStroke>::zeroed();
        while unsafe {
            device = interception_wait(self.context);
            interception_receive(self.context, device, stroke.as_mut_ptr(), 1)
        } > 0
        {
            // InterceptionMouseStroke &mstroke = *(InterceptionMouseStroke *) &stroke;
            let mstroke: &mut InterceptionMouseStroke =
                unsafe { &mut *(stroke.as_mut_ptr() as *mut InterceptionMouseStroke) };

            let (x, y) = crate::accel::apply(mstroke.x, mstroke.y, params);
            mstroke.x = x;
            mstroke.y = y;
            unsafe {
                interception_send(self.context, device, stroke.as_ptr(), 1);
            }
        }
    }
}
