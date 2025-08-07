extern "C" {
	const cv::HausdorffDistanceExtractor* cv_PtrLcv_HausdorffDistanceExtractorG_getInnerPtr_const(const cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
			return instance->get();
	}
	
	cv::HausdorffDistanceExtractor* cv_PtrLcv_HausdorffDistanceExtractorG_getInnerPtrMut(cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_HausdorffDistanceExtractorG_delete(cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_HausdorffDistanceExtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ShapeDistanceExtractor>* cv_PtrLcv_HausdorffDistanceExtractorG_to_PtrOfShapeDistanceExtractor(cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
			return new cv::Ptr<cv::ShapeDistanceExtractor>(instance->dynamicCast<cv::ShapeDistanceExtractor>());
	}
	
}

