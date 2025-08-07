pub fn cv_wechat_qrcode_WeChatQRCode_WeChatQRCode_const_stringR_const_stringR_const_stringR_const_stringR(detector_prototxt_path: *const c_char, detector_caffe_model_path: *const c_char, super_resolution_prototxt_path: *const c_char, super_resolution_caffe_model_path: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
pub fn cv_wechat_qrcode_WeChatQRCode_WeChatQRCode(ocvrs_return: *mut Result<*mut c_void>);
pub fn cv_wechat_qrcode_WeChatQRCode_detectAndDecode_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, img: *const c_void, points: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
pub fn cv_wechat_qrcode_WeChatQRCode_detectAndDecode_const__InputArrayR(instance: *mut c_void, img: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
pub fn cv_wechat_qrcode_WeChatQRCode_setScaleFactor_float(instance: *mut c_void, _scaling_factor: f32, ocvrs_return: *mut ResultVoid);
pub fn cv_wechat_qrcode_WeChatQRCode_getScaleFactor(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
pub fn cv_wechat_qrcode_WeChatQRCode_delete(instance: *mut c_void);
