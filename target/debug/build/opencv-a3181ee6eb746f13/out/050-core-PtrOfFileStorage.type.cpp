extern "C" {
	const cv::FileStorage* cv_PtrLcv_FileStorageG_getInnerPtr_const(const cv::Ptr<cv::FileStorage>* instance) {
			return instance->get();
	}
	
	cv::FileStorage* cv_PtrLcv_FileStorageG_getInnerPtrMut(cv::Ptr<cv::FileStorage>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_FileStorageG_delete(cv::Ptr<cv::FileStorage>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::FileStorage>* cv_PtrLcv_FileStorageG_new_const_FileStorage(cv::FileStorage* val) {
			return new cv::Ptr<cv::FileStorage>(val);
	}
	
}

