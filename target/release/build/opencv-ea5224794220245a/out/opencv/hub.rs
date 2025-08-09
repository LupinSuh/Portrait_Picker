include!(concat!(env!("OUT_DIR"), "/opencv/core.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/dnn.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/highgui.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/imgcodecs.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/imgproc.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/objdetect.rs"));
pub mod types {
	include!(concat!(env!("OUT_DIR"), "/opencv/types.rs"));
}
#[doc(hidden)]
pub mod sys {
	include!(concat!(env!("OUT_DIR"), "/opencv/sys.rs"));
}
pub mod hub_prelude {
	pub use super::core::prelude::*;
	pub use super::dnn::prelude::*;
	pub use super::highgui::prelude::*;
	pub use super::imgcodecs::prelude::*;
	pub use super::imgproc::prelude::*;
	pub use super::objdetect::prelude::*;
}

mod ffi_exports {
	use crate::mod_prelude_sys::*;
	#[no_mangle] unsafe extern "C" fn ocvrs_create_string_0_95_1(s: *const c_char) -> *mut String { unsafe { crate::templ::ocvrs_create_string(s) } }
	#[no_mangle] unsafe extern "C" fn ocvrs_create_byte_string_0_95_1(v: *const u8, len: size_t) -> *mut Vec<u8> { unsafe { crate::templ::ocvrs_create_byte_string(v, len) } }
}
