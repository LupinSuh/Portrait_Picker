pub fn cv_xobjdetect_WBDetector_read_const_FileNodeR(instance: *mut c_void, node: *const c_void, ocvrs_return: *mut ResultVoid);
pub fn cv_xobjdetect_WBDetector_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut ResultVoid);
pub fn cv_xobjdetect_WBDetector_train_const_stringR_const_stringR(instance: *mut c_void, pos_samples: *const c_char, neg_imgs: *const c_char, ocvrs_return: *mut ResultVoid);
pub fn cv_xobjdetect_WBDetector_detect_const_MatR_vectorLRectGR_vectorLdoubleGR(instance: *mut c_void, img: *const c_void, bboxes: *mut c_void, confidences: *mut c_void, ocvrs_return: *mut ResultVoid);
pub fn cv_xobjdetect_WBDetector_create(ocvrs_return: *mut Result<*mut c_void>);
pub fn cv_xobjdetect_WBDetector_delete(instance: *mut c_void);
