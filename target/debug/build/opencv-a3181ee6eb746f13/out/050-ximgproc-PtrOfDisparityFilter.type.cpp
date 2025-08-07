extern "C" {
	const cv::ximgproc::DisparityFilter* cv_PtrLcv_ximgproc_DisparityFilterG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::DisparityFilter>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::DisparityFilter* cv_PtrLcv_ximgproc_DisparityFilterG_getInnerPtrMut(cv::Ptr<cv::ximgproc::DisparityFilter>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_DisparityFilterG_delete(cv::Ptr<cv::ximgproc::DisparityFilter>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_DisparityFilterG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::DisparityFilter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

