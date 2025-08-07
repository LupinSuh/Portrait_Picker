extern "C" {
	const cv::CalibrateCRF* cv_PtrLcv_CalibrateCRFG_getInnerPtr_const(const cv::Ptr<cv::CalibrateCRF>* instance) {
			return instance->get();
	}
	
	cv::CalibrateCRF* cv_PtrLcv_CalibrateCRFG_getInnerPtrMut(cv::Ptr<cv::CalibrateCRF>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_CalibrateCRFG_delete(cv::Ptr<cv::CalibrateCRF>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_CalibrateCRFG_to_PtrOfAlgorithm(cv::Ptr<cv::CalibrateCRF>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

