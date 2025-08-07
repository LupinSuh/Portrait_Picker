//! # High-level GUI
//! 
//! While OpenCV was designed for use in full-scale applications and can be used within functionally
//! rich UI frameworks (such as Qt\*, WinForms\*, or Cocoa\*) or without any UI at all, sometimes there
//! it is required to try functionality quickly and visualize the results. This is what the HighGUI
//! module has been designed for.
//! 
//! It provides easy interface to:
//! 
//! *   Create and manipulate windows that can display images and "remember" their content (no need to
//!    handle repaint events from OS).
//! *   Add trackbars to the windows, handle simple mouse events as well as keyboard commands.
//!    # Flags related creating and manipulating HighGUI windows and mouse events
//!    # OpenGL support
//!    # Qt New Functions
//! 
//!    ![image](https://docs.opencv.org/4.12.0/qtgui.png)
//! 
//!    This figure explains new functionality implemented with Qt\* GUI. The new GUI provides a statusbar,
//!    a toolbar, and a control panel. The control panel can have trackbars and buttonbars attached to it.
//!    If you cannot see the control panel, press Ctrl+P or right-click any Qt window and select **Display
//!    properties window**.
//! 
//!    *   To attach a trackbar, the window name parameter must be NULL.
//! 
//!    *   To attach a buttonbar, a button must be created. If the last bar attached to the control panel
//!        is a buttonbar, the new button is added to the right of the last button. If the last bar
//!        attached to the control panel is a trackbar, or the control panel is empty, a new buttonbar is
//!        created. Then, a new button is attached to it.
//! 
//!    See below the example used to generate the figure:
//! 
//!    @include highgui_qt.cpp
//! 
//!    # WinRT support
//! 
//!    This figure explains new functionality implemented with WinRT GUI. The new GUI provides an Image control,
//!    and a slider panel. Slider panel holds trackbars attached to it.
//! 
//!    Sliders are attached below the image control. Every new slider is added below the previous one.
//! 
//!    See below the example used to generate the figure:
//!    ```C++
//!    void sample_app::MainPage::ShowWindow()
//!    {
//!        static cv::String windowName("sample");
//!        cv::winrt_initContainer(this->cvContainer);
//!        cv::namedWindow(windowName); // not required
//! 
//!        cv::Mat image = cv::imread("Assets/sample.jpg");
//!        cv::Mat converted = cv::Mat(image.rows, image.cols, CV_8UC4);
//!        cv::cvtColor(image, converted, COLOR_BGR2BGRA);
//!        cv::imshow(windowName, converted); // this will create window if it hasn't been created before
//! 
//!        int state = 42;
//!        cv::TrackbarCallback callback = [](int pos, void* userdata)
//!        {
//!            if (pos == 0) {
//!                cv::destroyWindow(windowName);
//!            }
//!        };
//!        cv::TrackbarCallback callbackTwin = [](int pos, void* userdata)
//!        {
//!            if (pos >= 70) {
//!                cv::destroyAllWindows();
//!            }
//!        };
//!        cv::createTrackbar("Sample trackbar", windowName, &state, 100, callback);
//!        cv::createTrackbar("Twin brother", windowName, &state, 100, callbackTwin);
//!    }
//!    ```
//! 
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::QtFontTraitConst, super::QtFontTrait };
}

/// indicates that ALT Key is pressed.
pub const EVENT_FLAG_ALTKEY: i32 = 32;
/// indicates that CTRL Key is pressed.
pub const EVENT_FLAG_CTRLKEY: i32 = 8;
/// indicates that the left mouse button is down.
pub const EVENT_FLAG_LBUTTON: i32 = 1;
/// indicates that the middle mouse button is down.
pub const EVENT_FLAG_MBUTTON: i32 = 4;
/// indicates that the right mouse button is down.
pub const EVENT_FLAG_RBUTTON: i32 = 2;
/// indicates that SHIFT Key is pressed.
pub const EVENT_FLAG_SHIFTKEY: i32 = 16;
/// indicates that left mouse button is double clicked.
pub const EVENT_LBUTTONDBLCLK: i32 = 7;
/// indicates that the left mouse button is pressed.
pub const EVENT_LBUTTONDOWN: i32 = 1;
/// indicates that left mouse button is released.
pub const EVENT_LBUTTONUP: i32 = 4;
/// indicates that middle mouse button is double clicked.
pub const EVENT_MBUTTONDBLCLK: i32 = 9;
/// indicates that the middle mouse button is pressed.
pub const EVENT_MBUTTONDOWN: i32 = 3;
/// indicates that middle mouse button is released.
pub const EVENT_MBUTTONUP: i32 = 6;
/// positive and negative values mean right and left scrolling, respectively.
pub const EVENT_MOUSEHWHEEL: i32 = 11;
/// indicates that the mouse pointer has moved over the window.
pub const EVENT_MOUSEMOVE: i32 = 0;
/// positive and negative values mean forward and backward scrolling, respectively.
pub const EVENT_MOUSEWHEEL: i32 = 10;
/// indicates that right mouse button is double clicked.
pub const EVENT_RBUTTONDBLCLK: i32 = 8;
/// indicates that the right mouse button is pressed.
pub const EVENT_RBUTTONDOWN: i32 = 2;
/// indicates that right mouse button is released.
pub const EVENT_RBUTTONUP: i32 = 5;
/// Checkbox button.
pub const QT_CHECKBOX: i32 = 1;
/// Weight of 87
pub const QT_FONT_BLACK: i32 = 87;
/// Weight of 75
pub const QT_FONT_BOLD: i32 = 75;
/// Weight of 63
pub const QT_FONT_DEMIBOLD: i32 = 63;
/// Weight of 25
pub const QT_FONT_LIGHT: i32 = 25;
/// Weight of 50
pub const QT_FONT_NORMAL: i32 = 50;
/// Button should create a new buttonbar
pub const QT_NEW_BUTTONBAR: i32 = 1024;
/// Push button.
pub const QT_PUSH_BUTTON: i32 = 0;
/// Radiobox button.
pub const QT_RADIOBOX: i32 = 2;
/// Italic font.
pub const QT_STYLE_ITALIC: i32 = 1;
/// Normal font.
pub const QT_STYLE_NORMAL: i32 = 0;
/// Oblique font.
pub const QT_STYLE_OBLIQUE: i32 = 2;
/// the user cannot resize the window, the size is constrainted by the image displayed.
pub const WINDOW_AUTOSIZE: i32 = 1;
/// the image expends as much as it can (no ratio constraint).
pub const WINDOW_FREERATIO: i32 = 256;
/// change the window to fullscreen.
pub const WINDOW_FULLSCREEN: i32 = 1;
/// status bar and tool bar
pub const WINDOW_GUI_EXPANDED: i32 = 0;
/// old fashious way
pub const WINDOW_GUI_NORMAL: i32 = 16;
/// the ratio of the image is respected.
pub const WINDOW_KEEPRATIO: i32 = 0;
/// the user can resize the window (no constraint) / also use to switch a fullscreen window to a normal size.
pub const WINDOW_NORMAL: i32 = 0;
/// window with opengl support.
pub const WINDOW_OPENGL: i32 = 4096;
/// window's aspect ration (can be set to WINDOW_FREERATIO or WINDOW_KEEPRATIO).
pub const WND_PROP_ASPECT_RATIO: i32 = 2;
/// autosize property      (can be WINDOW_NORMAL or WINDOW_AUTOSIZE).
pub const WND_PROP_AUTOSIZE: i32 = 1;
/// fullscreen property    (can be WINDOW_NORMAL or WINDOW_FULLSCREEN).
pub const WND_PROP_FULLSCREEN: i32 = 0;
/// opengl support.
pub const WND_PROP_OPENGL: i32 = 3;
/// property to toggle normal window being topmost or not
pub const WND_PROP_TOPMOST: i32 = 5;
/// checks whether the window exists and is visible
pub const WND_PROP_VISIBLE: i32 = 4;
/// enable or disable VSYNC (in OpenGL mode)
pub const WND_PROP_VSYNC: i32 = 6;
/// Mouse Event Flags see cv::MouseCallback
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MouseEventFlags {
	/// indicates that the left mouse button is down.
	EVENT_FLAG_LBUTTON = 1,
	/// indicates that the right mouse button is down.
	EVENT_FLAG_RBUTTON = 2,
	/// indicates that the middle mouse button is down.
	EVENT_FLAG_MBUTTON = 4,
	/// indicates that CTRL Key is pressed.
	EVENT_FLAG_CTRLKEY = 8,
	/// indicates that SHIFT Key is pressed.
	EVENT_FLAG_SHIFTKEY = 16,
	/// indicates that ALT Key is pressed.
	EVENT_FLAG_ALTKEY = 32,
}

opencv_type_enum! { crate::highgui::MouseEventFlags }

/// Mouse Events see cv::MouseCallback
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MouseEventTypes {
	/// indicates that the mouse pointer has moved over the window.
	EVENT_MOUSEMOVE = 0,
	/// indicates that the left mouse button is pressed.
	EVENT_LBUTTONDOWN = 1,
	/// indicates that the right mouse button is pressed.
	EVENT_RBUTTONDOWN = 2,
	/// indicates that the middle mouse button is pressed.
	EVENT_MBUTTONDOWN = 3,
	/// indicates that left mouse button is released.
	EVENT_LBUTTONUP = 4,
	/// indicates that right mouse button is released.
	EVENT_RBUTTONUP = 5,
	/// indicates that middle mouse button is released.
	EVENT_MBUTTONUP = 6,
	/// indicates that left mouse button is double clicked.
	EVENT_LBUTTONDBLCLK = 7,
	/// indicates that right mouse button is double clicked.
	EVENT_RBUTTONDBLCLK = 8,
	/// indicates that middle mouse button is double clicked.
	EVENT_MBUTTONDBLCLK = 9,
	/// positive and negative values mean forward and backward scrolling, respectively.
	EVENT_MOUSEWHEEL = 10,
	/// positive and negative values mean right and left scrolling, respectively.
	EVENT_MOUSEHWHEEL = 11,
}

opencv_type_enum! { crate::highgui::MouseEventTypes }

/// Qt "button" type
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum QtButtonTypes {
	/// Push button.
	QT_PUSH_BUTTON = 0,
	/// Checkbox button.
	QT_CHECKBOX = 1,
	/// Radiobox button.
	QT_RADIOBOX = 2,
	/// Button should create a new buttonbar
	QT_NEW_BUTTONBAR = 1024,
}

opencv_type_enum! { crate::highgui::QtButtonTypes }

/// Qt font style
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum QtFontStyles {
	/// Normal font.
	QT_STYLE_NORMAL = 0,
	/// Italic font.
	QT_STYLE_ITALIC = 1,
	/// Oblique font.
	QT_STYLE_OBLIQUE = 2,
}

opencv_type_enum! { crate::highgui::QtFontStyles }

/// Qt font weight
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum QtFontWeights {
	/// Weight of 25
	QT_FONT_LIGHT = 25,
	/// Weight of 50
	QT_FONT_NORMAL = 50,
	/// Weight of 63
	QT_FONT_DEMIBOLD = 63,
	/// Weight of 75
	QT_FONT_BOLD = 75,
	/// Weight of 87
	QT_FONT_BLACK = 87,
}

opencv_type_enum! { crate::highgui::QtFontWeights }

/// Flags for cv::namedWindow
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WindowFlags {
	/// the user can resize the window (no constraint) / also use to switch a fullscreen window to a normal size.
	WINDOW_NORMAL = 0,
	/// the user cannot resize the window, the size is constrainted by the image displayed.
	WINDOW_AUTOSIZE = 1,
	/// window with opengl support.
	WINDOW_OPENGL = 4096,
	// change the window to fullscreen.
	// Duplicate, use WINDOW_AUTOSIZE instead
	// WINDOW_FULLSCREEN = 1,
	/// the image expends as much as it can (no ratio constraint).
	WINDOW_FREERATIO = 256,
	// the ratio of the image is respected.
	// Duplicate, use WINDOW_NORMAL instead
	// WINDOW_KEEPRATIO = 0,
	// status bar and tool bar
	// Duplicate, use WINDOW_KEEPRATIO instead
	// WINDOW_GUI_EXPANDED = 0,
	/// old fashious way
	WINDOW_GUI_NORMAL = 16,
}

opencv_type_enum! { crate::highgui::WindowFlags }

/// Flags for cv::setWindowProperty / cv::getWindowProperty
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WindowPropertyFlags {
	/// fullscreen property    (can be WINDOW_NORMAL or WINDOW_FULLSCREEN).
	WND_PROP_FULLSCREEN = 0,
	/// autosize property      (can be WINDOW_NORMAL or WINDOW_AUTOSIZE).
	WND_PROP_AUTOSIZE = 1,
	/// window's aspect ration (can be set to WINDOW_FREERATIO or WINDOW_KEEPRATIO).
	WND_PROP_ASPECT_RATIO = 2,
	/// opengl support.
	WND_PROP_OPENGL = 3,
	/// checks whether the window exists and is visible
	WND_PROP_VISIBLE = 4,
	/// property to toggle normal window being topmost or not
	WND_PROP_TOPMOST = 5,
	/// enable or disable VSYNC (in OpenGL mode)
	WND_PROP_VSYNC = 6,
}

opencv_type_enum! { crate::highgui::WindowPropertyFlags }

/// Callback function for a button created by cv::createButton
/// ## Parameters
/// * state: current state of the button. It could be -1 for a push button, 0 or 1 for a check/radio box button.
/// * userdata: The optional parameter.
pub type ButtonCallback = Option<Box<dyn FnMut(i32) -> () + Send + Sync + 'static>>;
/// Callback function for mouse events. see cv::setMouseCallback
/// ## Parameters
/// * event: one of the cv::MouseEventTypes constants.
/// * x: The x-coordinate of the mouse event.
/// * y: The y-coordinate of the mouse event.
/// * flags: one of the cv::MouseEventFlags constants.
/// * userdata: The optional parameter.
pub type MouseCallback = Option<Box<dyn FnMut(i32, i32, i32, i32) -> () + Send + Sync + 'static>>;
/// Callback function defined to be called every frame. See cv::setOpenGlDrawCallback
/// ## Parameters
/// * userdata: The optional parameter.
pub type OpenGlDrawCallback = Option<Box<dyn FnMut() -> () + Send + Sync + 'static>>;
/// Callback function for Trackbar see cv::createTrackbar
/// ## Parameters
/// * pos: current position of the specified trackbar.
/// * userdata: The optional parameter.
pub type TrackbarCallback = Option<Box<dyn FnMut(i32) -> () + Send + Sync + 'static>>;
/// Constant methods for [crate::highgui::QtFont]
pub trait QtFontTraitConst {
	fn as_raw_QtFont(&self) -> *const c_void;

	/// Name of the font
	#[inline]
	fn name_font(&self) -> String {
		let ret = unsafe { sys::cv_QtFont_propNameFont_const(self.as_raw_QtFont()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	/// Color of the font. Scalar(blue_component, green_component, red_component[, alpha_component])
	#[inline]
	fn color(&self) -> core::Scalar {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_QtFont_propColor_const(self.as_raw_QtFont(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	/// See cv::QtFontStyles
	#[inline]
	fn font_face(&self) -> i32 {
		let ret = unsafe { sys::cv_QtFont_propFont_face_const(self.as_raw_QtFont()) };
		ret
	}
	
	/// font data and metrics
	#[inline]
	fn ascii(&self) -> *const i32 {
		let ret = unsafe { sys::cv_QtFont_propAscii_const(self.as_raw_QtFont()) };
		ret
	}
	
	#[inline]
	fn greek(&self) -> *const i32 {
		let ret = unsafe { sys::cv_QtFont_propGreek_const(self.as_raw_QtFont()) };
		ret
	}
	
	#[inline]
	fn cyrillic(&self) -> *const i32 {
		let ret = unsafe { sys::cv_QtFont_propCyrillic_const(self.as_raw_QtFont()) };
		ret
	}
	
	#[inline]
	fn hscale(&self) -> f32 {
		let ret = unsafe { sys::cv_QtFont_propHscale_const(self.as_raw_QtFont()) };
		ret
	}
	
	#[inline]
	fn vscale(&self) -> f32 {
		let ret = unsafe { sys::cv_QtFont_propVscale_const(self.as_raw_QtFont()) };
		ret
	}
	
	/// slope coefficient: 0 - normal, >0 - italic
	#[inline]
	fn shear(&self) -> f32 {
		let ret = unsafe { sys::cv_QtFont_propShear_const(self.as_raw_QtFont()) };
		ret
	}
	
	/// See cv::QtFontWeights
	#[inline]
	fn thickness(&self) -> i32 {
		let ret = unsafe { sys::cv_QtFont_propThickness_const(self.as_raw_QtFont()) };
		ret
	}
	
	/// horizontal interval between letters
	#[inline]
	fn dx(&self) -> f32 {
		let ret = unsafe { sys::cv_QtFont_propDx_const(self.as_raw_QtFont()) };
		ret
	}
	
	/// PointSize
	#[inline]
	fn line_type(&self) -> i32 {
		let ret = unsafe { sys::cv_QtFont_propLine_type_const(self.as_raw_QtFont()) };
		ret
	}
	
}

/// Mutable methods for [crate::highgui::QtFont]
pub trait QtFontTrait: crate::highgui::QtFontTraitConst {
	fn as_raw_mut_QtFont(&mut self) -> *mut c_void;

	/// Color of the font. Scalar(blue_component, green_component, red_component[, alpha_component])
	#[inline]
	fn set_color(&mut self, val: core::Scalar) {
		let ret = unsafe { sys::cv_QtFont_propColor_const_Scalar(self.as_raw_mut_QtFont(), val.opencv_as_extern()) };
		ret
	}
	
	/// See cv::QtFontStyles
	#[inline]
	fn set_font_face(&mut self, val: i32) {
		let ret = unsafe { sys::cv_QtFont_propFont_face_const_int(self.as_raw_mut_QtFont(), val) };
		ret
	}
	
	#[inline]
	fn set_hscale(&mut self, val: f32) {
		let ret = unsafe { sys::cv_QtFont_propHscale_const_float(self.as_raw_mut_QtFont(), val) };
		ret
	}
	
	#[inline]
	fn set_vscale(&mut self, val: f32) {
		let ret = unsafe { sys::cv_QtFont_propVscale_const_float(self.as_raw_mut_QtFont(), val) };
		ret
	}
	
	/// slope coefficient: 0 - normal, >0 - italic
	#[inline]
	fn set_shear(&mut self, val: f32) {
		let ret = unsafe { sys::cv_QtFont_propShear_const_float(self.as_raw_mut_QtFont(), val) };
		ret
	}
	
	/// See cv::QtFontWeights
	#[inline]
	fn set_thickness(&mut self, val: i32) {
		let ret = unsafe { sys::cv_QtFont_propThickness_const_int(self.as_raw_mut_QtFont(), val) };
		ret
	}
	
	/// horizontal interval between letters
	#[inline]
	fn set_dx(&mut self, val: f32) {
		let ret = unsafe { sys::cv_QtFont_propDx_const_float(self.as_raw_mut_QtFont(), val) };
		ret
	}
	
	/// PointSize
	#[inline]
	fn set_line_type(&mut self, val: i32) {
		let ret = unsafe { sys::cv_QtFont_propLine_type_const_int(self.as_raw_mut_QtFont(), val) };
		ret
	}
	
}

/// QtFont available only for Qt. See cv::fontQt
pub struct QtFont {
	ptr: *mut c_void
}

opencv_type_boxed! { QtFont }

impl Drop for QtFont {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_QtFont_delete(self.as_raw_mut_QtFont()) };
	}
}

unsafe impl Send for QtFont {}

impl crate::highgui::QtFontTraitConst for QtFont {
	#[inline] fn as_raw_QtFont(&self) -> *const c_void { self.as_raw() }
}

impl crate::highgui::QtFontTrait for QtFont {
	#[inline] fn as_raw_mut_QtFont(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl QtFont {
}

impl std::fmt::Debug for QtFont {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("QtFont")
			.field("name_font", &crate::highgui::QtFontTraitConst::name_font(self))
			.field("color", &crate::highgui::QtFontTraitConst::color(self))
			.field("font_face", &crate::highgui::QtFontTraitConst::font_face(self))
			.field("ascii", &crate::highgui::QtFontTraitConst::ascii(self))
			.field("greek", &crate::highgui::QtFontTraitConst::greek(self))
			.field("cyrillic", &crate::highgui::QtFontTraitConst::cyrillic(self))
			.field("hscale", &crate::highgui::QtFontTraitConst::hscale(self))
			.field("vscale", &crate::highgui::QtFontTraitConst::vscale(self))
			.field("shear", &crate::highgui::QtFontTraitConst::shear(self))
			.field("thickness", &crate::highgui::QtFontTraitConst::thickness(self))
			.field("dx", &crate::highgui::QtFontTraitConst::dx(self))
			.field("line_type", &crate::highgui::QtFontTraitConst::line_type(self))
			.finish()
	}
}
