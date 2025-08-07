extern "C" {
	const cv::TonemapDrago* cv_PtrLcv_TonemapDragoG_getInnerPtr_const(const cv::Ptr<cv::TonemapDrago>* instance) {
			return instance->get();
	}
	
	cv::TonemapDrago* cv_PtrLcv_TonemapDragoG_getInnerPtrMut(cv::Ptr<cv::TonemapDrago>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_TonemapDragoG_delete(cv::Ptr<cv::TonemapDrago>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_TonemapDragoG_to_PtrOfAlgorithm(cv::Ptr<cv::TonemapDrago>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Tonemap>* cv_PtrLcv_TonemapDragoG_to_PtrOfTonemap(cv::Ptr<cv::TonemapDrago>* instance) {
			return new cv::Ptr<cv::Tonemap>(instance->dynamicCast<cv::Tonemap>());
	}
	
}

