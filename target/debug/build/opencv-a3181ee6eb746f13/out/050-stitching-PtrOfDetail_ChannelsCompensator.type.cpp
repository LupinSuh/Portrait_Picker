extern "C" {
	const cv::detail::ChannelsCompensator* cv_PtrLcv_detail_ChannelsCompensatorG_getInnerPtr_const(const cv::Ptr<cv::detail::ChannelsCompensator>* instance) {
			return instance->get();
	}
	
	cv::detail::ChannelsCompensator* cv_PtrLcv_detail_ChannelsCompensatorG_getInnerPtrMut(cv::Ptr<cv::detail::ChannelsCompensator>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_ChannelsCompensatorG_delete(cv::Ptr<cv::detail::ChannelsCompensator>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrLcv_detail_ChannelsCompensatorG_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::ChannelsCompensator>* instance) {
			return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}
	
	cv::Ptr<cv::detail::ChannelsCompensator>* cv_PtrLcv_detail_ChannelsCompensatorG_new_const_ChannelsCompensator(cv::detail::ChannelsCompensator* val) {
			return new cv::Ptr<cv::detail::ChannelsCompensator>(val);
	}
	
}

