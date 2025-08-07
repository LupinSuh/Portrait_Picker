extern "C" {
	const cv::DescriptorMatcher* cv_PtrLcv_DescriptorMatcherG_getInnerPtr_const(const cv::Ptr<cv::DescriptorMatcher>* instance) {
			return instance->get();
	}
	
	cv::DescriptorMatcher* cv_PtrLcv_DescriptorMatcherG_getInnerPtrMut(cv::Ptr<cv::DescriptorMatcher>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_DescriptorMatcherG_delete(cv::Ptr<cv::DescriptorMatcher>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_DescriptorMatcherG_to_PtrOfAlgorithm(cv::Ptr<cv::DescriptorMatcher>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

