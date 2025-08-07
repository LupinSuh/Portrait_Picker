extern "C" {
	const cv::videostab::ColorAverageInpainter* cv_PtrLcv_videostab_ColorAverageInpainterG_getInnerPtr_const(const cv::Ptr<cv::videostab::ColorAverageInpainter>* instance) {
			return instance->get();
	}
	
	cv::videostab::ColorAverageInpainter* cv_PtrLcv_videostab_ColorAverageInpainterG_getInnerPtrMut(cv::Ptr<cv::videostab::ColorAverageInpainter>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_ColorAverageInpainterG_delete(cv::Ptr<cv::videostab::ColorAverageInpainter>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrLcv_videostab_ColorAverageInpainterG_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::ColorAverageInpainter>* instance) {
			return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}
	
	cv::Ptr<cv::videostab::ColorAverageInpainter>* cv_PtrLcv_videostab_ColorAverageInpainterG_new_const_ColorAverageInpainter(cv::videostab::ColorAverageInpainter* val) {
			return new cv::Ptr<cv::videostab::ColorAverageInpainter>(val);
	}
	
}

