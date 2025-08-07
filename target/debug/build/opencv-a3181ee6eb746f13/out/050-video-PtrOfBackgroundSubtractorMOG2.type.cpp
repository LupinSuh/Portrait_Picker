extern "C" {
	const cv::BackgroundSubtractorMOG2* cv_PtrLcv_BackgroundSubtractorMOG2G_getInnerPtr_const(const cv::Ptr<cv::BackgroundSubtractorMOG2>* instance) {
			return instance->get();
	}
	
	cv::BackgroundSubtractorMOG2* cv_PtrLcv_BackgroundSubtractorMOG2G_getInnerPtrMut(cv::Ptr<cv::BackgroundSubtractorMOG2>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_BackgroundSubtractorMOG2G_delete(cv::Ptr<cv::BackgroundSubtractorMOG2>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_BackgroundSubtractorMOG2G_to_PtrOfAlgorithm(cv::Ptr<cv::BackgroundSubtractorMOG2>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::BackgroundSubtractor>* cv_PtrLcv_BackgroundSubtractorMOG2G_to_PtrOfBackgroundSubtractor(cv::Ptr<cv::BackgroundSubtractorMOG2>* instance) {
			return new cv::Ptr<cv::BackgroundSubtractor>(instance->dynamicCast<cv::BackgroundSubtractor>());
	}
	
}

