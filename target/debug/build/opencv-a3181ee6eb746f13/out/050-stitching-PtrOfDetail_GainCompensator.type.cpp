extern "C" {
	const cv::detail::GainCompensator* cv_PtrLcv_detail_GainCompensatorG_getInnerPtr_const(const cv::Ptr<cv::detail::GainCompensator>* instance) {
			return instance->get();
	}
	
	cv::detail::GainCompensator* cv_PtrLcv_detail_GainCompensatorG_getInnerPtrMut(cv::Ptr<cv::detail::GainCompensator>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_GainCompensatorG_delete(cv::Ptr<cv::detail::GainCompensator>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrLcv_detail_GainCompensatorG_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::GainCompensator>* instance) {
			return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}
	
	cv::Ptr<cv::detail::GainCompensator>* cv_PtrLcv_detail_GainCompensatorG_new_const_GainCompensator(cv::detail::GainCompensator* val) {
			return new cv::Ptr<cv::detail::GainCompensator>(val);
	}
	
}

