extern "C" {
	const cv::structured_light::StructuredLightPattern* cv_PtrLcv_structured_light_StructuredLightPatternG_getInnerPtr_const(const cv::Ptr<cv::structured_light::StructuredLightPattern>* instance) {
			return instance->get();
	}
	
	cv::structured_light::StructuredLightPattern* cv_PtrLcv_structured_light_StructuredLightPatternG_getInnerPtrMut(cv::Ptr<cv::structured_light::StructuredLightPattern>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_structured_light_StructuredLightPatternG_delete(cv::Ptr<cv::structured_light::StructuredLightPattern>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_structured_light_StructuredLightPatternG_to_PtrOfAlgorithm(cv::Ptr<cv::structured_light::StructuredLightPattern>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

