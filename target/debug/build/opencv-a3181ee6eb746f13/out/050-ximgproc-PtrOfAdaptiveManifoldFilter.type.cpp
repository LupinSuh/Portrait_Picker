extern "C" {
	const cv::ximgproc::AdaptiveManifoldFilter* cv_PtrLcv_ximgproc_AdaptiveManifoldFilterG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::AdaptiveManifoldFilter* cv_PtrLcv_ximgproc_AdaptiveManifoldFilterG_getInnerPtrMut(cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_AdaptiveManifoldFilterG_delete(cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_AdaptiveManifoldFilterG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

