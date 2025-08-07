extern "C" {
	const cv::mcc::DetectorParameters* cv_PtrLcv_mcc_DetectorParametersG_getInnerPtr_const(const cv::Ptr<cv::mcc::DetectorParameters>* instance) {
			return instance->get();
	}
	
	cv::mcc::DetectorParameters* cv_PtrLcv_mcc_DetectorParametersG_getInnerPtrMut(cv::Ptr<cv::mcc::DetectorParameters>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_mcc_DetectorParametersG_delete(cv::Ptr<cv::mcc::DetectorParameters>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::mcc::DetectorParameters>* cv_PtrLcv_mcc_DetectorParametersG_new_const_DetectorParameters(cv::mcc::DetectorParameters* val) {
			return new cv::Ptr<cv::mcc::DetectorParameters>(val);
	}
	
}

