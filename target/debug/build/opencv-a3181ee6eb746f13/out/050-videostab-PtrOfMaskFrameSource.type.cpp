extern "C" {
	const cv::videostab::MaskFrameSource* cv_PtrLcv_videostab_MaskFrameSourceG_getInnerPtr_const(const cv::Ptr<cv::videostab::MaskFrameSource>* instance) {
			return instance->get();
	}
	
	cv::videostab::MaskFrameSource* cv_PtrLcv_videostab_MaskFrameSourceG_getInnerPtrMut(cv::Ptr<cv::videostab::MaskFrameSource>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_MaskFrameSourceG_delete(cv::Ptr<cv::videostab::MaskFrameSource>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrLcv_videostab_MaskFrameSourceG_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::MaskFrameSource>* instance) {
			return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}
	
	cv::Ptr<cv::videostab::MaskFrameSource>* cv_PtrLcv_videostab_MaskFrameSourceG_new_const_MaskFrameSource(cv::videostab::MaskFrameSource* val) {
			return new cv::Ptr<cv::videostab::MaskFrameSource>(val);
	}
	
}

