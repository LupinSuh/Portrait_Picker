extern "C" {
	const cv::ximgproc::RidgeDetectionFilter* cv_PtrLcv_ximgproc_RidgeDetectionFilterG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::RidgeDetectionFilter>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::RidgeDetectionFilter* cv_PtrLcv_ximgproc_RidgeDetectionFilterG_getInnerPtrMut(cv::Ptr<cv::ximgproc::RidgeDetectionFilter>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_RidgeDetectionFilterG_delete(cv::Ptr<cv::ximgproc::RidgeDetectionFilter>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_RidgeDetectionFilterG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::RidgeDetectionFilter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

