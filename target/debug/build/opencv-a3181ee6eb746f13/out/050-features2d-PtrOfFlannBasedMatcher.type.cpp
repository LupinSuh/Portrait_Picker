extern "C" {
	const cv::FlannBasedMatcher* cv_PtrLcv_FlannBasedMatcherG_getInnerPtr_const(const cv::Ptr<cv::FlannBasedMatcher>* instance) {
			return instance->get();
	}
	
	cv::FlannBasedMatcher* cv_PtrLcv_FlannBasedMatcherG_getInnerPtrMut(cv::Ptr<cv::FlannBasedMatcher>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_FlannBasedMatcherG_delete(cv::Ptr<cv::FlannBasedMatcher>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_FlannBasedMatcherG_to_PtrOfAlgorithm(cv::Ptr<cv::FlannBasedMatcher>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::DescriptorMatcher>* cv_PtrLcv_FlannBasedMatcherG_to_PtrOfDescriptorMatcher(cv::Ptr<cv::FlannBasedMatcher>* instance) {
			return new cv::Ptr<cv::DescriptorMatcher>(instance->dynamicCast<cv::DescriptorMatcher>());
	}
	
	cv::Ptr<cv::FlannBasedMatcher>* cv_PtrLcv_FlannBasedMatcherG_new_const_FlannBasedMatcher(cv::FlannBasedMatcher* val) {
			return new cv::Ptr<cv::FlannBasedMatcher>(val);
	}
	
}

