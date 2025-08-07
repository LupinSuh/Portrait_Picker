extern "C" {
	const cv::videostab::KeypointBasedMotionEstimator* cv_PtrLcv_videostab_KeypointBasedMotionEstimatorG_getInnerPtr_const(const cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* instance) {
			return instance->get();
	}
	
	cv::videostab::KeypointBasedMotionEstimator* cv_PtrLcv_videostab_KeypointBasedMotionEstimatorG_getInnerPtrMut(cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_KeypointBasedMotionEstimatorG_delete(cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* cv_PtrLcv_videostab_KeypointBasedMotionEstimatorG_to_PtrOfImageMotionEstimatorBase(cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* instance) {
			return new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(instance->dynamicCast<cv::videostab::ImageMotionEstimatorBase>());
	}
	
	cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* cv_PtrLcv_videostab_KeypointBasedMotionEstimatorG_new_const_KeypointBasedMotionEstimator(cv::videostab::KeypointBasedMotionEstimator* val) {
			return new cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>(val);
	}
	
}

