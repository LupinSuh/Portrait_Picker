extern "C" {
	const cv::detail::BlocksCompensator* cv_PtrLcv_detail_BlocksCompensatorG_getInnerPtr_const(const cv::Ptr<cv::detail::BlocksCompensator>* instance) {
			return instance->get();
	}
	
	cv::detail::BlocksCompensator* cv_PtrLcv_detail_BlocksCompensatorG_getInnerPtrMut(cv::Ptr<cv::detail::BlocksCompensator>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_BlocksCompensatorG_delete(cv::Ptr<cv::detail::BlocksCompensator>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrLcv_detail_BlocksCompensatorG_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::BlocksCompensator>* instance) {
			return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}
	
}

