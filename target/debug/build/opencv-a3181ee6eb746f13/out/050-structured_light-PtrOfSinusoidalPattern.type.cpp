extern "C" {
	const cv::structured_light::SinusoidalPattern* cv_PtrLcv_structured_light_SinusoidalPatternG_getInnerPtr_const(const cv::Ptr<cv::structured_light::SinusoidalPattern>* instance) {
			return instance->get();
	}
	
	cv::structured_light::SinusoidalPattern* cv_PtrLcv_structured_light_SinusoidalPatternG_getInnerPtrMut(cv::Ptr<cv::structured_light::SinusoidalPattern>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_structured_light_SinusoidalPatternG_delete(cv::Ptr<cv::structured_light::SinusoidalPattern>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_structured_light_SinusoidalPatternG_to_PtrOfAlgorithm(cv::Ptr<cv::structured_light::SinusoidalPattern>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::structured_light::StructuredLightPattern>* cv_PtrLcv_structured_light_SinusoidalPatternG_to_PtrOfStructuredLightPattern(cv::Ptr<cv::structured_light::SinusoidalPattern>* instance) {
			return new cv::Ptr<cv::structured_light::StructuredLightPattern>(instance->dynamicCast<cv::structured_light::StructuredLightPattern>());
	}
	
}

