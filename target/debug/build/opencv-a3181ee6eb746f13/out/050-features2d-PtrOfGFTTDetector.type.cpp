extern "C" {
	const cv::GFTTDetector* cv_PtrLcv_GFTTDetectorG_getInnerPtr_const(const cv::Ptr<cv::GFTTDetector>* instance) {
			return instance->get();
	}
	
	cv::GFTTDetector* cv_PtrLcv_GFTTDetectorG_getInnerPtrMut(cv::Ptr<cv::GFTTDetector>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_GFTTDetectorG_delete(cv::Ptr<cv::GFTTDetector>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_GFTTDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::GFTTDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_GFTTDetectorG_to_PtrOfFeature2D(cv::Ptr<cv::GFTTDetector>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
	
}

