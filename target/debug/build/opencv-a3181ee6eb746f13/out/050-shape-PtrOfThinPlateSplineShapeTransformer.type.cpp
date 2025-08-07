extern "C" {
	const cv::ThinPlateSplineShapeTransformer* cv_PtrLcv_ThinPlateSplineShapeTransformerG_getInnerPtr_const(const cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
			return instance->get();
	}
	
	cv::ThinPlateSplineShapeTransformer* cv_PtrLcv_ThinPlateSplineShapeTransformerG_getInnerPtrMut(cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ThinPlateSplineShapeTransformerG_delete(cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ThinPlateSplineShapeTransformerG_to_PtrOfAlgorithm(cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ShapeTransformer>* cv_PtrLcv_ThinPlateSplineShapeTransformerG_to_PtrOfShapeTransformer(cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
			return new cv::Ptr<cv::ShapeTransformer>(instance->dynamicCast<cv::ShapeTransformer>());
	}
	
}

