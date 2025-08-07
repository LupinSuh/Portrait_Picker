extern "C" {
	const cv::videostab::MotionEstimatorRansacL2* cv_PtrLcv_videostab_MotionEstimatorRansacL2G_getInnerPtr_const(const cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* instance) {
			return instance->get();
	}
	
	cv::videostab::MotionEstimatorRansacL2* cv_PtrLcv_videostab_MotionEstimatorRansacL2G_getInnerPtrMut(cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_MotionEstimatorRansacL2G_delete(cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::MotionEstimatorBase>* cv_PtrLcv_videostab_MotionEstimatorRansacL2G_to_PtrOfMotionEstimatorBase(cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* instance) {
			return new cv::Ptr<cv::videostab::MotionEstimatorBase>(instance->dynamicCast<cv::videostab::MotionEstimatorBase>());
	}
	
	cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* cv_PtrLcv_videostab_MotionEstimatorRansacL2G_new_const_MotionEstimatorRansacL2(cv::videostab::MotionEstimatorRansacL2* val) {
			return new cv::Ptr<cv::videostab::MotionEstimatorRansacL2>(val);
	}
	
}

