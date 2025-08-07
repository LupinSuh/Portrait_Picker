extern "C" {
	const cv::CalibrateRobertson* cv_PtrLcv_CalibrateRobertsonG_getInnerPtr_const(const cv::Ptr<cv::CalibrateRobertson>* instance) {
			return instance->get();
	}
	
	cv::CalibrateRobertson* cv_PtrLcv_CalibrateRobertsonG_getInnerPtrMut(cv::Ptr<cv::CalibrateRobertson>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_CalibrateRobertsonG_delete(cv::Ptr<cv::CalibrateRobertson>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_CalibrateRobertsonG_to_PtrOfAlgorithm(cv::Ptr<cv::CalibrateRobertson>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::CalibrateCRF>* cv_PtrLcv_CalibrateRobertsonG_to_PtrOfCalibrateCRF(cv::Ptr<cv::CalibrateRobertson>* instance) {
			return new cv::Ptr<cv::CalibrateCRF>(instance->dynamicCast<cv::CalibrateCRF>());
	}
	
}

