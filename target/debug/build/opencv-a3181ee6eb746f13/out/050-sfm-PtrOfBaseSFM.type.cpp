extern "C" {
	const cv::sfm::BaseSFM* cv_PtrLcv_sfm_BaseSFMG_getInnerPtr_const(const cv::Ptr<cv::sfm::BaseSFM>* instance) {
			return instance->get();
	}
	
	cv::sfm::BaseSFM* cv_PtrLcv_sfm_BaseSFMG_getInnerPtrMut(cv::Ptr<cv::sfm::BaseSFM>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_sfm_BaseSFMG_delete(cv::Ptr<cv::sfm::BaseSFM>* instance) {
			delete instance;
	}
	
}

