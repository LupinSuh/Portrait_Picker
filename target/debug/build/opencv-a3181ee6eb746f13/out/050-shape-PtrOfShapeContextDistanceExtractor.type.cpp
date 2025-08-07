extern "C" {
	const cv::ShapeContextDistanceExtractor* cv_PtrLcv_ShapeContextDistanceExtractorG_getInnerPtr_const(const cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
			return instance->get();
	}
	
	cv::ShapeContextDistanceExtractor* cv_PtrLcv_ShapeContextDistanceExtractorG_getInnerPtrMut(cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ShapeContextDistanceExtractorG_delete(cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ShapeContextDistanceExtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ShapeDistanceExtractor>* cv_PtrLcv_ShapeContextDistanceExtractorG_to_PtrOfShapeDistanceExtractor(cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
			return new cv::Ptr<cv::ShapeDistanceExtractor>(instance->dynamicCast<cv::ShapeDistanceExtractor>());
	}
	
}

