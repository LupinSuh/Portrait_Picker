extern "C" {
	const cv::line_descriptor::BinaryDescriptorMatcher* cv_PtrLcv_line_descriptor_BinaryDescriptorMatcherG_getInnerPtr_const(const cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>* instance) {
			return instance->get();
	}
	
	cv::line_descriptor::BinaryDescriptorMatcher* cv_PtrLcv_line_descriptor_BinaryDescriptorMatcherG_getInnerPtrMut(cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_line_descriptor_BinaryDescriptorMatcherG_delete(cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_line_descriptor_BinaryDescriptorMatcherG_to_PtrOfAlgorithm(cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>* cv_PtrLcv_line_descriptor_BinaryDescriptorMatcherG_new_const_BinaryDescriptorMatcher(cv::line_descriptor::BinaryDescriptorMatcher* val) {
			return new cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>(val);
	}
	
}

