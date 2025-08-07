extern "C" {
	const cv::CalibrateDebevec* cv_PtrLcv_CalibrateDebevecG_getInnerPtr_const(const cv::Ptr<cv::CalibrateDebevec>* instance) {
			return instance->get();
	}
	
	cv::CalibrateDebevec* cv_PtrLcv_CalibrateDebevecG_getInnerPtrMut(cv::Ptr<cv::CalibrateDebevec>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_CalibrateDebevecG_delete(cv::Ptr<cv::CalibrateDebevec>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_CalibrateDebevecG_to_PtrOfAlgorithm(cv::Ptr<cv::CalibrateDebevec>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::CalibrateCRF>* cv_PtrLcv_CalibrateDebevecG_to_PtrOfCalibrateCRF(cv::Ptr<cv::CalibrateDebevec>* instance) {
			return new cv::Ptr<cv::CalibrateCRF>(instance->dynamicCast<cv::CalibrateCRF>());
	}
	
}

