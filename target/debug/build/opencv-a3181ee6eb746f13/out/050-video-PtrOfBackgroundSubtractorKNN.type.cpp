extern "C" {
	const cv::BackgroundSubtractorKNN* cv_PtrLcv_BackgroundSubtractorKNNG_getInnerPtr_const(const cv::Ptr<cv::BackgroundSubtractorKNN>* instance) {
			return instance->get();
	}
	
	cv::BackgroundSubtractorKNN* cv_PtrLcv_BackgroundSubtractorKNNG_getInnerPtrMut(cv::Ptr<cv::BackgroundSubtractorKNN>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_BackgroundSubtractorKNNG_delete(cv::Ptr<cv::BackgroundSubtractorKNN>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_BackgroundSubtractorKNNG_to_PtrOfAlgorithm(cv::Ptr<cv::BackgroundSubtractorKNN>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::BackgroundSubtractor>* cv_PtrLcv_BackgroundSubtractorKNNG_to_PtrOfBackgroundSubtractor(cv::Ptr<cv::BackgroundSubtractorKNN>* instance) {
			return new cv::Ptr<cv::BackgroundSubtractor>(instance->dynamicCast<cv::BackgroundSubtractor>());
	}
	
}

