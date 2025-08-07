extern "C" {
	const cv::videostab::OnePassStabilizer* cv_PtrLcv_videostab_OnePassStabilizerG_getInnerPtr_const(const cv::Ptr<cv::videostab::OnePassStabilizer>* instance) {
			return instance->get();
	}
	
	cv::videostab::OnePassStabilizer* cv_PtrLcv_videostab_OnePassStabilizerG_getInnerPtrMut(cv::Ptr<cv::videostab::OnePassStabilizer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_OnePassStabilizerG_delete(cv::Ptr<cv::videostab::OnePassStabilizer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrLcv_videostab_OnePassStabilizerG_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::OnePassStabilizer>* instance) {
			return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}
	
	cv::Ptr<cv::videostab::StabilizerBase>* cv_PtrLcv_videostab_OnePassStabilizerG_to_PtrOfStabilizerBase(cv::Ptr<cv::videostab::OnePassStabilizer>* instance) {
			return new cv::Ptr<cv::videostab::StabilizerBase>(instance->dynamicCast<cv::videostab::StabilizerBase>());
	}
	
	cv::Ptr<cv::videostab::OnePassStabilizer>* cv_PtrLcv_videostab_OnePassStabilizerG_new_const_OnePassStabilizer(cv::videostab::OnePassStabilizer* val) {
			return new cv::Ptr<cv::videostab::OnePassStabilizer>(val);
	}
	
}

