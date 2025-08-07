extern "C" {
	const cv::detail::BlocksGainCompensator* cv_PtrLcv_detail_BlocksGainCompensatorG_getInnerPtr_const(const cv::Ptr<cv::detail::BlocksGainCompensator>* instance) {
			return instance->get();
	}
	
	cv::detail::BlocksGainCompensator* cv_PtrLcv_detail_BlocksGainCompensatorG_getInnerPtrMut(cv::Ptr<cv::detail::BlocksGainCompensator>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_BlocksGainCompensatorG_delete(cv::Ptr<cv::detail::BlocksGainCompensator>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::BlocksCompensator>* cv_PtrLcv_detail_BlocksGainCompensatorG_to_PtrOfDetail_BlocksCompensator(cv::Ptr<cv::detail::BlocksGainCompensator>* instance) {
			return new cv::Ptr<cv::detail::BlocksCompensator>(instance->dynamicCast<cv::detail::BlocksCompensator>());
	}
	
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrLcv_detail_BlocksGainCompensatorG_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::BlocksGainCompensator>* instance) {
			return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}
	
	cv::Ptr<cv::detail::BlocksGainCompensator>* cv_PtrLcv_detail_BlocksGainCompensatorG_new_const_BlocksGainCompensator(cv::detail::BlocksGainCompensator* val) {
			return new cv::Ptr<cv::detail::BlocksGainCompensator>(val);
	}
	
}

