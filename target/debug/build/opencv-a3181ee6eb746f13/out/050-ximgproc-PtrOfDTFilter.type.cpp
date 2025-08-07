extern "C" {
	const cv::ximgproc::DTFilter* cv_PtrLcv_ximgproc_DTFilterG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::DTFilter>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::DTFilter* cv_PtrLcv_ximgproc_DTFilterG_getInnerPtrMut(cv::Ptr<cv::ximgproc::DTFilter>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_DTFilterG_delete(cv::Ptr<cv::ximgproc::DTFilter>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_DTFilterG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::DTFilter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

