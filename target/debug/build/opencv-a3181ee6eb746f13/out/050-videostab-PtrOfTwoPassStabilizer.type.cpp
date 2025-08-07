extern "C" {
	const cv::videostab::TwoPassStabilizer* cv_PtrLcv_videostab_TwoPassStabilizerG_getInnerPtr_const(const cv::Ptr<cv::videostab::TwoPassStabilizer>* instance) {
			return instance->get();
	}
	
	cv::videostab::TwoPassStabilizer* cv_PtrLcv_videostab_TwoPassStabilizerG_getInnerPtrMut(cv::Ptr<cv::videostab::TwoPassStabilizer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_TwoPassStabilizerG_delete(cv::Ptr<cv::videostab::TwoPassStabilizer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrLcv_videostab_TwoPassStabilizerG_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::TwoPassStabilizer>* instance) {
			return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}
	
	cv::Ptr<cv::videostab::StabilizerBase>* cv_PtrLcv_videostab_TwoPassStabilizerG_to_PtrOfStabilizerBase(cv::Ptr<cv::videostab::TwoPassStabilizer>* instance) {
			return new cv::Ptr<cv::videostab::StabilizerBase>(instance->dynamicCast<cv::videostab::StabilizerBase>());
	}
	
	cv::Ptr<cv::videostab::TwoPassStabilizer>* cv_PtrLcv_videostab_TwoPassStabilizerG_new_const_TwoPassStabilizer(cv::videostab::TwoPassStabilizer* val) {
			return new cv::Ptr<cv::videostab::TwoPassStabilizer>(val);
	}
	
}

