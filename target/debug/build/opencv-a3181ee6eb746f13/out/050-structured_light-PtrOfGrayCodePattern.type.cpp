extern "C" {
	const cv::structured_light::GrayCodePattern* cv_PtrLcv_structured_light_GrayCodePatternG_getInnerPtr_const(const cv::Ptr<cv::structured_light::GrayCodePattern>* instance) {
			return instance->get();
	}
	
	cv::structured_light::GrayCodePattern* cv_PtrLcv_structured_light_GrayCodePatternG_getInnerPtrMut(cv::Ptr<cv::structured_light::GrayCodePattern>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_structured_light_GrayCodePatternG_delete(cv::Ptr<cv::structured_light::GrayCodePattern>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_structured_light_GrayCodePatternG_to_PtrOfAlgorithm(cv::Ptr<cv::structured_light::GrayCodePattern>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::structured_light::StructuredLightPattern>* cv_PtrLcv_structured_light_GrayCodePatternG_to_PtrOfStructuredLightPattern(cv::Ptr<cv::structured_light::GrayCodePattern>* instance) {
			return new cv::Ptr<cv::structured_light::StructuredLightPattern>(instance->dynamicCast<cv::structured_light::StructuredLightPattern>());
	}
	
}

