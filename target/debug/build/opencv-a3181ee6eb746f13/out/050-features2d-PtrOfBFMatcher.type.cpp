extern "C" {
	const cv::BFMatcher* cv_PtrLcv_BFMatcherG_getInnerPtr_const(const cv::Ptr<cv::BFMatcher>* instance) {
			return instance->get();
	}
	
	cv::BFMatcher* cv_PtrLcv_BFMatcherG_getInnerPtrMut(cv::Ptr<cv::BFMatcher>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_BFMatcherG_delete(cv::Ptr<cv::BFMatcher>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_BFMatcherG_to_PtrOfAlgorithm(cv::Ptr<cv::BFMatcher>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::DescriptorMatcher>* cv_PtrLcv_BFMatcherG_to_PtrOfDescriptorMatcher(cv::Ptr<cv::BFMatcher>* instance) {
			return new cv::Ptr<cv::DescriptorMatcher>(instance->dynamicCast<cv::DescriptorMatcher>());
	}
	
	cv::Ptr<cv::BFMatcher>* cv_PtrLcv_BFMatcherG_new_const_BFMatcher(cv::BFMatcher* val) {
			return new cv::Ptr<cv::BFMatcher>(val);
	}
	
}

