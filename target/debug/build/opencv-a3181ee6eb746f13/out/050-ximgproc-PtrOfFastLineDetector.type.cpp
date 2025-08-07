extern "C" {
	const cv::ximgproc::FastLineDetector* cv_PtrLcv_ximgproc_FastLineDetectorG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::FastLineDetector>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::FastLineDetector* cv_PtrLcv_ximgproc_FastLineDetectorG_getInnerPtrMut(cv::Ptr<cv::ximgproc::FastLineDetector>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_FastLineDetectorG_delete(cv::Ptr<cv::ximgproc::FastLineDetector>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_FastLineDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::FastLineDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

