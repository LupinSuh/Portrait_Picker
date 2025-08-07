extern "C" {
	const cv::detail::NoExposureCompensator* cv_PtrLcv_detail_NoExposureCompensatorG_getInnerPtr_const(const cv::Ptr<cv::detail::NoExposureCompensator>* instance) {
			return instance->get();
	}
	
	cv::detail::NoExposureCompensator* cv_PtrLcv_detail_NoExposureCompensatorG_getInnerPtrMut(cv::Ptr<cv::detail::NoExposureCompensator>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_NoExposureCompensatorG_delete(cv::Ptr<cv::detail::NoExposureCompensator>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrLcv_detail_NoExposureCompensatorG_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::NoExposureCompensator>* instance) {
			return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}
	
	cv::Ptr<cv::detail::NoExposureCompensator>* cv_PtrLcv_detail_NoExposureCompensatorG_new_const_NoExposureCompensator(cv::detail::NoExposureCompensator* val) {
			return new cv::Ptr<cv::detail::NoExposureCompensator>(val);
	}
	
}

