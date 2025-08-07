extern "C" {
	const cv::ximgproc::FastGlobalSmootherFilter* cv_PtrLcv_ximgproc_FastGlobalSmootherFilterG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::FastGlobalSmootherFilter* cv_PtrLcv_ximgproc_FastGlobalSmootherFilterG_getInnerPtrMut(cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_FastGlobalSmootherFilterG_delete(cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_FastGlobalSmootherFilterG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

