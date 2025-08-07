extern "C" {
	const cv::FastFeatureDetector* cv_PtrLcv_FastFeatureDetectorG_getInnerPtr_const(const cv::Ptr<cv::FastFeatureDetector>* instance) {
			return instance->get();
	}
	
	cv::FastFeatureDetector* cv_PtrLcv_FastFeatureDetectorG_getInnerPtrMut(cv::Ptr<cv::FastFeatureDetector>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_FastFeatureDetectorG_delete(cv::Ptr<cv::FastFeatureDetector>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_FastFeatureDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::FastFeatureDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_FastFeatureDetectorG_to_PtrOfFeature2D(cv::Ptr<cv::FastFeatureDetector>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
	
}

