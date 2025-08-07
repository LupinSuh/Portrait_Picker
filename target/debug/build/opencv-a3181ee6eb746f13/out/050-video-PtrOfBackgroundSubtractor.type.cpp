extern "C" {
	const cv::BackgroundSubtractor* cv_PtrLcv_BackgroundSubtractorG_getInnerPtr_const(const cv::Ptr<cv::BackgroundSubtractor>* instance) {
			return instance->get();
	}
	
	cv::BackgroundSubtractor* cv_PtrLcv_BackgroundSubtractorG_getInnerPtrMut(cv::Ptr<cv::BackgroundSubtractor>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_BackgroundSubtractorG_delete(cv::Ptr<cv::BackgroundSubtractor>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_BackgroundSubtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::BackgroundSubtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

