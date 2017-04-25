use std::ptr;

use get_error;
use surface::SurfaceRef;
use video;
use EventPump;

use sys::mouse as ll;

mod relative;
pub use self::relative::RelativeMouseState;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(u32)]
pub enum SystemCursor {
    Arrow = ll::SDL_SYSTEM_CURSOR_ARROW as u32,
    IBeam = ll::SDL_SYSTEM_CURSOR_IBEAM as u32,
    Wait = ll::SDL_SYSTEM_CURSOR_WAIT as u32,
    Crosshair = ll::SDL_SYSTEM_CURSOR_CROSSHAIR as u32,
    WaitArrow = ll::SDL_SYSTEM_CURSOR_WAITARROW as u32,
    SizeNWSE = ll::SDL_SYSTEM_CURSOR_SIZENWSE as u32,
    SizeNESW = ll::SDL_SYSTEM_CURSOR_SIZENESW as u32,
    SizeWE = ll::SDL_SYSTEM_CURSOR_SIZEWE as u32,
    SizeNS = ll::SDL_SYSTEM_CURSOR_SIZENS as u32,
    SizeAll = ll::SDL_SYSTEM_CURSOR_SIZEALL as u32,
    No = ll::SDL_SYSTEM_CURSOR_NO as u32,
    Hand = ll::SDL_SYSTEM_CURSOR_HAND as u32,
}

pub struct Cursor {
    raw: *mut ll::SDL_Cursor
}

impl Drop for Cursor {
    #[inline]
    fn drop(&mut self) {
        unsafe { ll::SDL_FreeCursor(self.raw) };
    }
}

impl Cursor {
    pub fn new(data: &[u8], mask: &[u8], width: i32, height: i32, hot_x: i32, hot_y: i32) -> Result<Cursor, String> {
        unsafe {
            let raw = ll::SDL_CreateCursor(data.as_ptr(),
                                           mask.as_ptr(),
                                           width as i32, height as i32,
                                           hot_x as i32, hot_y as i32);

            if raw == ptr::null_mut() {
                Err(get_error())
            } else {
                Ok(Cursor{ raw: raw })
            }
        }
    }

    // TODO: figure out how to pass Surface in here correctly
    pub fn from_surface<S: AsRef<SurfaceRef>>(surface: S, hot_x: i32, hot_y: i32) -> Result<Cursor, String> {
        unsafe {
            let raw = ll::SDL_CreateColorCursor(surface.as_ref().raw(), hot_x, hot_y);

            if raw == ptr::null_mut() {
                Err(get_error())
            } else {
                Ok(Cursor{ raw: raw })
            }
        }
    }

    pub fn from_system(cursor: SystemCursor) -> Result<Cursor, String> {
        unsafe {
            let raw = ll::SDL_CreateSystemCursor(cursor as u32);

            if raw == ptr::null_mut() {
                Err(get_error())
            } else {
                Ok(Cursor{ raw: raw })
            }
        }
    }

    pub fn set(&self) {
        unsafe { ll::SDL_SetCursor(self.raw); }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MouseWheelDirection {
    Normal,
    Flipped,
    Unknown(u32),
}

// 0 and 1 are not fixed values in the SDL source code.  This value is defined as an enum which is then cast to a Uint32.
// The enum in C is defined as such:

/**
 * \brief Scroll direction types for the Scroll event
 */
//typedef enum
//{
//    SDL_MOUSEWHEEL_NORMAL,    /**< The scroll direction is normal */
//    SDL_MOUSEWHEEL_FLIPPED    /**< The scroll direction is flipped / natural */
//} SDL_MouseWheelDirection;

// Since no value is given in the enum definition these values are auto assigned by the C compiler starting at 0.
// Normally I would prefer to use the enum rather than hard code what it is implied to represent however
// the mouse wheel direction value could be described equally as well by a bool, so I don't think changes
// to this enum in the C source code are going to be a problem.

impl MouseWheelDirection {
    #[inline]
    pub fn from_ll(direction: u32) -> MouseWheelDirection {
        match direction {
            0 => MouseWheelDirection::Normal,
            1 => MouseWheelDirection::Flipped,
            _ => MouseWheelDirection::Unknown(direction),
        }
    }
    #[inline]
    pub fn to_ll(&self) -> u32 {
        match *self {
            MouseWheelDirection::Normal => 0,
            MouseWheelDirection::Flipped => 1,
            MouseWheelDirection::Unknown(direction) => direction,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MouseButton {
    Unknown = 0,
    Left   = ll::SDL_BUTTON_LEFT as u8,
    Middle = ll::SDL_BUTTON_MIDDLE as u8,
    Right  = ll::SDL_BUTTON_RIGHT as u8,
    X1     = ll::SDL_BUTTON_X1 as u8,
    X2     = ll::SDL_BUTTON_X2 as u8,
}

impl MouseButton {
    #[inline]
    pub fn from_ll(button: u8) -> MouseButton {
        match button {
            ll::SDL_BUTTON_LEFT   => MouseButton::Left,
            ll::SDL_BUTTON_MIDDLE => MouseButton::Middle,
            ll::SDL_BUTTON_RIGHT  => MouseButton::Right,
            ll::SDL_BUTTON_X1     => MouseButton::X1,
            ll::SDL_BUTTON_X2     => MouseButton::X2,
            _                     => MouseButton::Unknown,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MouseState {
    mouse_state: u32,
    x: i32,
    y: i32
}

impl MouseState {
    pub fn new(_e: &EventPump) -> MouseState {
        let mut x = 0;
        let mut y = 0;
        let mouse_state: u32 = unsafe {
            ll::SDL_GetMouseState(&mut x, &mut y)
        };

        MouseState {
            mouse_state: mouse_state,
            x: x as i32,
            y: y as i32
        }
    }

    pub fn from_sdl_state(state: u32) -> MouseState {
        MouseState { mouse_state : state, x: 0, y: 0 }
    }
    pub fn to_sdl_state(&self) -> u32 {
        self.mouse_state
    }

    /// Returns true if the left mouse button is pressed.
    ///
    /// # Example
    /// ```no_run
    /// use sdl2::mouse::MouseButton;
    ///
    /// fn is_a_pressed(e: &sdl2::EventPump) -> bool {
    ///     e.mouse_state().left()
    /// }
    /// ```
    pub fn left(&self) -> bool { (self.mouse_state & ll::SDL_BUTTON_LMASK) != 0 }

    /// Tests if the middle mouse button was pressed.
    pub fn middle(&self) -> bool { (self.mouse_state & ll::SDL_BUTTON_MMASK) != 0 }

    /// Tests if the right mouse button was pressed.
    pub fn right(&self) -> bool { (self.mouse_state & ll::SDL_BUTTON_RMASK) != 0 }

    /// Tests if the X1 mouse button was pressed.
    pub fn x1(&self) -> bool { (self.mouse_state & ll::SDL_BUTTON_X1MASK) != 0 }

    /// Tests if the X2 mouse button was pressed.
    pub fn x2(&self) -> bool { (self.mouse_state & ll::SDL_BUTTON_X2MASK) != 0 }

    /// Returns the x coordinate of the state
    pub fn x(&self) -> i32 { self.x }

    /// Returns the y coordinate of the state
    pub fn y(&self) -> i32 { self.y }

    /// Returns true if the mouse button is pressed.
    ///
    /// # Example
    /// ```no_run
    /// use sdl2::mouse::MouseButton;
    ///
    /// fn is_left_pressed(e: &sdl2::EventPump) -> bool {
    ///     e.mouse_state().is_mouse_button_pressed(MouseButton::Left)
    /// }
    /// ```
    pub fn is_mouse_button_pressed(&self, mouse_button: MouseButton) -> bool {
        let mask = 1 << ((mouse_button as u32)-1);
        self.mouse_state & mask != 0
    }

    /// Returns an iterator all mouse buttons with a boolean indicating if the scancode is pressed.
    ///
    /// # Example
    /// ```no_run
    /// use sdl2::mouse::MouseButton;
    /// use std::collections::HashMap;
    ///
    /// fn mouse_button_set(e: &sdl2::EventPump) -> HashMap<MouseButton, bool> {
    ///     e.mouse_state().mouse_buttons().collect()
    /// }
    ///
    /// fn find_first_pressed(e: &sdl2::EventPump) -> bool {
    ///     for (key,value) in mouse_button_set(e) {
    ///         return value != false
    ///     }
    ///     false
    /// }
    ///
    /// ```
    pub fn mouse_buttons(&self) -> MouseButtonIterator {
        MouseButtonIterator {
            cur_button: 1,
            mouse_state: &self.mouse_state
        }
    }

    /// Returns an iterator of pressed mouse buttons.
    ///
    /// # Example
    /// ```no_run
    /// use sdl2::mouse::MouseButton;
    /// use std::collections::HashSet;
    ///
    /// fn pressed_mouse_button_set(e: &sdl2::EventPump) -> HashSet<MouseButton> {
    ///     e.mouse_state().pressed_mouse_buttons().collect()
    /// }
    ///
    /// fn newly_pressed(old: &HashSet<MouseButton>, new: &HashSet<MouseButton>) -> HashSet<MouseButton> {
    ///     new - old
    ///     // sugar for: new.difference(old).collect()
    /// }
    /// ```
    pub fn pressed_mouse_buttons(&self) -> PressedMouseButtonIterator {
        PressedMouseButtonIterator {
            iter: self.mouse_buttons()
        }
    }
}

pub struct MouseButtonIterator<'a> {
    cur_button: u8,
    mouse_state: &'a u32
}

impl<'a> Iterator for MouseButtonIterator<'a> {
    type Item = (MouseButton, bool);

    fn next(&mut self) -> Option<(MouseButton, bool)> {
        if self.cur_button < MouseButton::X2 as u8 + 1 {
            let mouse_button = self.cur_button;
            let mask = 1 << ((self.cur_button as u32)-1);
            let pressed = self.mouse_state & mask != 0;
            self.cur_button += 1;
            Some((MouseButton::from_ll(mouse_button), pressed))
        } else {
            None
        }
    }
}

pub struct PressedMouseButtonIterator<'a> {
    iter: MouseButtonIterator<'a>
}

impl<'a> Iterator for PressedMouseButtonIterator<'a> {
    type Item = MouseButton;

    fn next(&mut self) -> Option<MouseButton> {
        while let Some((mouse_button, pressed)) = self.iter.next() {
            if pressed { return Some(mouse_button) }
        }
        None
    }
}

impl ::Sdl {
    #[inline]
    pub fn mouse(&self) -> MouseUtil {
        MouseUtil {
            _sdldrop: self.sdldrop()
        }
    }
}

/// Mouse utility functions. Access with `Sdl::mouse()`.
///
/// ```no_run
/// let sdl_context = sdl2::init().unwrap();
///
/// // Hide the cursor
/// sdl_context.mouse().show_cursor(false);
/// ```
pub struct MouseUtil {
    _sdldrop: ::std::rc::Rc<::SdlDrop>
}

impl MouseUtil {
    /// Gets the id of the window which currently has mouse focus.
    pub fn focused_window_id(&self) -> Option<u32> {
        let raw = unsafe { ll::SDL_GetMouseFocus() };
        if raw == ptr::null_mut() {
            None
        } else {
            let id = unsafe { ::sys::video::SDL_GetWindowID(raw) };
            Some(id)
        }
    }

    pub fn warp_mouse_in_window(&self, window: &video::Window, x: i32, y: i32) {
        unsafe { ll::SDL_WarpMouseInWindow(window.raw(), x, y); }
    }

    pub fn set_relative_mouse_mode(&self, on: bool) {
        unsafe { ll::SDL_SetRelativeMouseMode(on as i32); }
    }

    pub fn relative_mouse_mode(&self) -> bool {
        unsafe { ll::SDL_GetRelativeMouseMode() == 1 }
    }

    pub fn is_cursor_showing(&self) -> bool {
        unsafe { ll::SDL_ShowCursor(::sys::SDL_QUERY) == 1 }
    }

    pub fn show_cursor(&self, show: bool) {
        unsafe { ll::SDL_ShowCursor(show as i32); }
    }
}
