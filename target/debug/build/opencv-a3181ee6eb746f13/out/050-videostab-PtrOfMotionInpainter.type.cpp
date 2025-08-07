extern "C" {
	const cv::videostab::MotionInpainter* cv_PtrLcv_videostab_MotionInpainterG_getInnerPtr_const(const cv::Ptr<cv::videostab::MotionInpainter>* instance) {
			return instance->get();
	}
	
	cv::videostab::MotionInpainter* cv_PtrLcv_videostab_MotionInpainterG_getInnerPtrMut(cv::Ptr<cv::videostab::MotionInpainter>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_MotionInpainterG_delete(cv::Ptr<cv::videostab::MotionInpainter>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrLcv_videostab_MotionInpainterG_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::MotionInpainter>* instance) {
			return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}
	
	cv::Ptr<cv::videostab::MotionInpainter>* cv_PtrLcv_videostab_MotionInpainterG_new_const_MotionInpainter(cv::videostab::MotionInpainter* val) {
			return new cv::Ptr<cv::videostab::MotionInpainter>(val);
	}
	
}

