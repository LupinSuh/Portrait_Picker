extern "C" {
	const cv::videostab::ConsistentMosaicInpainter* cv_PtrLcv_videostab_ConsistentMosaicInpainterG_getInnerPtr_const(const cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* instance) {
			return instance->get();
	}
	
	cv::videostab::ConsistentMosaicInpainter* cv_PtrLcv_videostab_ConsistentMosaicInpainterG_getInnerPtrMut(cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_ConsistentMosaicInpainterG_delete(cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrLcv_videostab_ConsistentMosaicInpainterG_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* instance) {
			return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}
	
	cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* cv_PtrLcv_videostab_ConsistentMosaicInpainterG_new_const_ConsistentMosaicInpainter(cv::videostab::ConsistentMosaicInpainter* val) {
			return new cv::Ptr<cv::videostab::ConsistentMosaicInpainter>(val);
	}
	
}

