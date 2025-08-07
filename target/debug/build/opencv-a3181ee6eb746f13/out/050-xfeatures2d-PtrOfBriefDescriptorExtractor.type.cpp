extern "C" {
	const cv::xfeatures2d::BriefDescriptorExtractor* cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* instance) {
			return instance->get();
	}
	
	cv::xfeatures2d::BriefDescriptorExtractor* cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_delete(cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
	
}

