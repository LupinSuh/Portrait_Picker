extern "C" {
	const cv::videostab::LpMotionStabilizer* cv_PtrLcv_videostab_LpMotionStabilizerG_getInnerPtr_const(const cv::Ptr<cv::videostab::LpMotionStabilizer>* instance) {
			return instance->get();
	}
	
	cv::videostab::LpMotionStabilizer* cv_PtrLcv_videostab_LpMotionStabilizerG_getInnerPtrMut(cv::Ptr<cv::videostab::LpMotionStabilizer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_LpMotionStabilizerG_delete(cv::Ptr<cv::videostab::LpMotionStabilizer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::IMotionStabilizer>* cv_PtrLcv_videostab_LpMotionStabilizerG_to_PtrOfIMotionStabilizer(cv::Ptr<cv::videostab::LpMotionStabilizer>* instance) {
			return new cv::Ptr<cv::videostab::IMotionStabilizer>(instance->dynamicCast<cv::videostab::IMotionStabilizer>());
	}
	
	cv::Ptr<cv::videostab::LpMotionStabilizer>* cv_PtrLcv_videostab_LpMotionStabilizerG_new_const_LpMotionStabilizer(cv::videostab::LpMotionStabilizer* val) {
			return new cv::Ptr<cv::videostab::LpMotionStabilizer>(val);
	}
	
}

