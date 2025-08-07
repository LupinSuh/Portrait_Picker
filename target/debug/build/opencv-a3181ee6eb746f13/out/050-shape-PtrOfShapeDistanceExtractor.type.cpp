extern "C" {
	const cv::ShapeDistanceExtractor* cv_PtrLcv_ShapeDistanceExtractorG_getInnerPtr_const(const cv::Ptr<cv::ShapeDistanceExtractor>* instance) {
			return instance->get();
	}
	
	cv::ShapeDistanceExtractor* cv_PtrLcv_ShapeDistanceExtractorG_getInnerPtrMut(cv::Ptr<cv::ShapeDistanceExtractor>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ShapeDistanceExtractorG_delete(cv::Ptr<cv::ShapeDistanceExtractor>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ShapeDistanceExtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::ShapeDistanceExtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

