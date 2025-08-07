extern "C" {
	const cv::AgastFeatureDetector* cv_PtrLcv_AgastFeatureDetectorG_getInnerPtr_const(const cv::Ptr<cv::AgastFeatureDetector>* instance) {
			return instance->get();
	}
	
	cv::AgastFeatureDetector* cv_PtrLcv_AgastFeatureDetectorG_getInnerPtrMut(cv::Ptr<cv::AgastFeatureDetector>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_AgastFeatureDetectorG_delete(cv::Ptr<cv::AgastFeatureDetector>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_AgastFeatureDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::AgastFeatureDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_AgastFeatureDetectorG_to_PtrOfFeature2D(cv::Ptr<cv::AgastFeatureDetector>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
	
}

