extern "C" {
	const cv::StereoMatcher* cv_PtrLcv_StereoMatcherG_getInnerPtr_const(const cv::Ptr<cv::StereoMatcher>* instance) {
			return instance->get();
	}
	
	cv::StereoMatcher* cv_PtrLcv_StereoMatcherG_getInnerPtrMut(cv::Ptr<cv::StereoMatcher>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_StereoMatcherG_delete(cv::Ptr<cv::StereoMatcher>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_StereoMatcherG_to_PtrOfAlgorithm(cv::Ptr<cv::StereoMatcher>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

