extern "C" {
	const cv::detail::BlocksChannelsCompensator* cv_PtrLcv_detail_BlocksChannelsCompensatorG_getInnerPtr_const(const cv::Ptr<cv::detail::BlocksChannelsCompensator>* instance) {
			return instance->get();
	}
	
	cv::detail::BlocksChannelsCompensator* cv_PtrLcv_detail_BlocksChannelsCompensatorG_getInnerPtrMut(cv::Ptr<cv::detail::BlocksChannelsCompensator>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_BlocksChannelsCompensatorG_delete(cv::Ptr<cv::detail::BlocksChannelsCompensator>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::BlocksCompensator>* cv_PtrLcv_detail_BlocksChannelsCompensatorG_to_PtrOfDetail_BlocksCompensator(cv::Ptr<cv::detail::BlocksChannelsCompensator>* instance) {
			return new cv::Ptr<cv::detail::BlocksCompensator>(instance->dynamicCast<cv::detail::BlocksCompensator>());
	}
	
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrLcv_detail_BlocksChannelsCompensatorG_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::BlocksChannelsCompensator>* instance) {
			return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}
	
	cv::Ptr<cv::detail::BlocksChannelsCompensator>* cv_PtrLcv_detail_BlocksChannelsCompensatorG_new_const_BlocksChannelsCompensator(cv::detail::BlocksChannelsCompensator* val) {
			return new cv::Ptr<cv::detail::BlocksChannelsCompensator>(val);
	}
	
}

