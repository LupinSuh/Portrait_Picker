extern "C" {
	const cv::videostab::MotionEstimatorL1* cv_PtrLcv_videostab_MotionEstimatorL1G_getInnerPtr_const(const cv::Ptr<cv::videostab::MotionEstimatorL1>* instance) {
			return instance->get();
	}
	
	cv::videostab::MotionEstimatorL1* cv_PtrLcv_videostab_MotionEstimatorL1G_getInnerPtrMut(cv::Ptr<cv::videostab::MotionEstimatorL1>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_MotionEstimatorL1G_delete(cv::Ptr<cv::videostab::MotionEstimatorL1>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::MotionEstimatorBase>* cv_PtrLcv_videostab_MotionEstimatorL1G_to_PtrOfMotionEstimatorBase(cv::Ptr<cv::videostab::MotionEstimatorL1>* instance) {
			return new cv::Ptr<cv::videostab::MotionEstimatorBase>(instance->dynamicCast<cv::videostab::MotionEstimatorBase>());
	}
	
	cv::Ptr<cv::videostab::MotionEstimatorL1>* cv_PtrLcv_videostab_MotionEstimatorL1G_new_const_MotionEstimatorL1(cv::videostab::MotionEstimatorL1* val) {
			return new cv::Ptr<cv::videostab::MotionEstimatorL1>(val);
	}
	
}

