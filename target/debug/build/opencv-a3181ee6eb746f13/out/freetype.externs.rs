pub fn cv_freetype_FreeType2_loadFontData_String_int(instance: *mut c_void, font_file_name: *const c_char, idx: i32, ocvrs_return: *mut ResultVoid);
pub fn cv_freetype_FreeType2_loadFontData_charX_size_t_int(instance: *mut c_void, p_buf: *mut c_char, buf_size: size_t, idx: i32, ocvrs_return: *mut ResultVoid);
pub fn cv_freetype_FreeType2_setSplitNumber_int(instance: *mut c_void, num: i32, ocvrs_return: *mut ResultVoid);
pub fn cv_freetype_FreeType2_putText_const__InputOutputArrayR_const_StringR_Point_int_Scalar_int_int_bool(instance: *mut c_void, img: *const c_void, text: *const c_char, org: *const core::Point, font_height: i32, color: *const core::Scalar, thickness: i32, line_type: i32, bottom_left_origin: bool, ocvrs_return: *mut ResultVoid);
pub fn cv_freetype_FreeType2_getTextSize_const_StringR_int_int_intX(instance: *mut c_void, text: *const c_char, font_height: i32, thickness: i32, base_line: *mut i32, ocvrs_return: *mut Result<core::Size>);
pub fn cv_freetype_FreeType2_to_Algorithm(instance: *mut c_void) -> *mut c_void;
pub fn cv_freetype_FreeType2_delete(instance: *mut c_void);
