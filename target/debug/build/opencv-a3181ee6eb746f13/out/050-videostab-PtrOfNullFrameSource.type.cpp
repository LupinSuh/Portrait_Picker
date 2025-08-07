extern "C" {
	const cv::videostab::NullFrameSource* cv_PtrLcv_videostab_NullFrameSourceG_getInnerPtr_const(const cv::Ptr<cv::videostab::NullFrameSource>* instance) {
			return instance->get();
	}
	
	cv::videostab::NullFrameSource* cv_PtrLcv_videostab_NullFrameSourceG_getInnerPtrMut(cv::Ptr<cv::videostab::NullFrameSource>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_NullFrameSourceG_delete(cv::Ptr<cv::videostab::NullFrameSource>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrLcv_videostab_NullFrameSourceG_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::NullFrameSource>* instance) {
			return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}
	
	cv::Ptr<cv::videostab::NullFrameSource>* cv_PtrLcv_videostab_NullFrameSourceG_new_const_NullFrameSource(cv::videostab::NullFrameSource* val) {
			return new cv::Ptr<cv::videostab::NullFrameSource>(val);
	}
	
}

