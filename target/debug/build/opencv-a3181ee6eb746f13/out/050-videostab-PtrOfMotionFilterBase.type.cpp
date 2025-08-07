extern "C" {
	const cv::videostab::MotionFilterBase* cv_PtrLcv_videostab_MotionFilterBaseG_getInnerPtr_const(const cv::Ptr<cv::videostab::MotionFilterBase>* instance) {
			return instance->get();
	}
	
	cv::videostab::MotionFilterBase* cv_PtrLcv_videostab_MotionFilterBaseG_getInnerPtrMut(cv::Ptr<cv::videostab::MotionFilterBase>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_MotionFilterBaseG_delete(cv::Ptr<cv::videostab::MotionFilterBase>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::IMotionStabilizer>* cv_PtrLcv_videostab_MotionFilterBaseG_to_PtrOfIMotionStabilizer(cv::Ptr<cv::videostab::MotionFilterBase>* instance) {
			return new cv::Ptr<cv::videostab::IMotionStabilizer>(instance->dynamicCast<cv::videostab::IMotionStabilizer>());
	}
	
}

