extern "C" {
	const cv::videostab::VideoFileSource* cv_PtrLcv_videostab_VideoFileSourceG_getInnerPtr_const(const cv::Ptr<cv::videostab::VideoFileSource>* instance) {
			return instance->get();
	}
	
	cv::videostab::VideoFileSource* cv_PtrLcv_videostab_VideoFileSourceG_getInnerPtrMut(cv::Ptr<cv::videostab::VideoFileSource>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_VideoFileSourceG_delete(cv::Ptr<cv::videostab::VideoFileSource>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrLcv_videostab_VideoFileSourceG_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::VideoFileSource>* instance) {
			return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}
	
	cv::Ptr<cv::videostab::VideoFileSource>* cv_PtrLcv_videostab_VideoFileSourceG_new_const_VideoFileSource(cv::videostab::VideoFileSource* val) {
			return new cv::Ptr<cv::videostab::VideoFileSource>(val);
	}
	
}

