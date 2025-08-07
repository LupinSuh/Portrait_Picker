extern "C" {
	const cv::videostab::GaussianMotionFilter* cv_PtrLcv_videostab_GaussianMotionFilterG_getInnerPtr_const(const cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
			return instance->get();
	}
	
	cv::videostab::GaussianMotionFilter* cv_PtrLcv_videostab_GaussianMotionFilterG_getInnerPtrMut(cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_GaussianMotionFilterG_delete(cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::IMotionStabilizer>* cv_PtrLcv_videostab_GaussianMotionFilterG_to_PtrOfIMotionStabilizer(cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
			return new cv::Ptr<cv::videostab::IMotionStabilizer>(instance->dynamicCast<cv::videostab::IMotionStabilizer>());
	}
	
	cv::Ptr<cv::videostab::MotionFilterBase>* cv_PtrLcv_videostab_GaussianMotionFilterG_to_PtrOfMotionFilterBase(cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
			return new cv::Ptr<cv::videostab::MotionFilterBase>(instance->dynamicCast<cv::videostab::MotionFilterBase>());
	}
	
	cv::Ptr<cv::videostab::GaussianMotionFilter>* cv_PtrLcv_videostab_GaussianMotionFilterG_new_const_GaussianMotionFilter(cv::videostab::GaussianMotionFilter* val) {
			return new cv::Ptr<cv::videostab::GaussianMotionFilter>(val);
	}
	
}

