extern "C" {
	const cv::line_descriptor::BinaryDescriptor* cv_PtrLcv_line_descriptor_BinaryDescriptorG_getInnerPtr_const(const cv::Ptr<cv::line_descriptor::BinaryDescriptor>* instance) {
			return instance->get();
	}
	
	cv::line_descriptor::BinaryDescriptor* cv_PtrLcv_line_descriptor_BinaryDescriptorG_getInnerPtrMut(cv::Ptr<cv::line_descriptor::BinaryDescriptor>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_line_descriptor_BinaryDescriptorG_delete(cv::Ptr<cv::line_descriptor::BinaryDescriptor>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_line_descriptor_BinaryDescriptorG_to_PtrOfAlgorithm(cv::Ptr<cv::line_descriptor::BinaryDescriptor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::line_descriptor::BinaryDescriptor>* cv_PtrLcv_line_descriptor_BinaryDescriptorG_new_const_BinaryDescriptor(cv::line_descriptor::BinaryDescriptor* val) {
			return new cv::Ptr<cv::line_descriptor::BinaryDescriptor>(val);
	}
	
}

