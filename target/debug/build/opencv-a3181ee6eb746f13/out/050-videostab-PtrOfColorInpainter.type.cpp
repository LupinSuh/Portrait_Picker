extern "C" {
	const cv::videostab::ColorInpainter* cv_PtrLcv_videostab_ColorInpainterG_getInnerPtr_const(const cv::Ptr<cv::videostab::ColorInpainter>* instance) {
			return instance->get();
	}
	
	cv::videostab::ColorInpainter* cv_PtrLcv_videostab_ColorInpainterG_getInnerPtrMut(cv::Ptr<cv::videostab::ColorInpainter>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_ColorInpainterG_delete(cv::Ptr<cv::videostab::ColorInpainter>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrLcv_videostab_ColorInpainterG_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::ColorInpainter>* instance) {
			return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}
	
	cv::Ptr<cv::videostab::ColorInpainter>* cv_PtrLcv_videostab_ColorInpainterG_new_const_ColorInpainter(cv::videostab::ColorInpainter* val) {
			return new cv::Ptr<cv::videostab::ColorInpainter>(val);
	}
	
}

