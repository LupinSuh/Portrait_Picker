extern "C" {
	const cv::videostab::NullInpainter* cv_PtrLcv_videostab_NullInpainterG_getInnerPtr_const(const cv::Ptr<cv::videostab::NullInpainter>* instance) {
			return instance->get();
	}
	
	cv::videostab::NullInpainter* cv_PtrLcv_videostab_NullInpainterG_getInnerPtrMut(cv::Ptr<cv::videostab::NullInpainter>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_NullInpainterG_delete(cv::Ptr<cv::videostab::NullInpainter>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrLcv_videostab_NullInpainterG_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::NullInpainter>* instance) {
			return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}
	
	cv::Ptr<cv::videostab::NullInpainter>* cv_PtrLcv_videostab_NullInpainterG_new_const_NullInpainter(cv::videostab::NullInpainter* val) {
			return new cv::Ptr<cv::videostab::NullInpainter>(val);
	}
	
}

