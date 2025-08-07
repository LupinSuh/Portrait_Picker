extern "C" {
	const cv::AffineTransformer* cv_PtrLcv_AffineTransformerG_getInnerPtr_const(const cv::Ptr<cv::AffineTransformer>* instance) {
			return instance->get();
	}
	
	cv::AffineTransformer* cv_PtrLcv_AffineTransformerG_getInnerPtrMut(cv::Ptr<cv::AffineTransformer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_AffineTransformerG_delete(cv::Ptr<cv::AffineTransformer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_AffineTransformerG_to_PtrOfAlgorithm(cv::Ptr<cv::AffineTransformer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ShapeTransformer>* cv_PtrLcv_AffineTransformerG_to_PtrOfShapeTransformer(cv::Ptr<cv::AffineTransformer>* instance) {
			return new cv::Ptr<cv::ShapeTransformer>(instance->dynamicCast<cv::ShapeTransformer>());
	}
	
}

