extern "C" {
	const cv::ShapeTransformer* cv_PtrLcv_ShapeTransformerG_getInnerPtr_const(const cv::Ptr<cv::ShapeTransformer>* instance) {
			return instance->get();
	}
	
	cv::ShapeTransformer* cv_PtrLcv_ShapeTransformerG_getInnerPtrMut(cv::Ptr<cv::ShapeTransformer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ShapeTransformerG_delete(cv::Ptr<cv::ShapeTransformer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ShapeTransformerG_to_PtrOfAlgorithm(cv::Ptr<cv::ShapeTransformer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

