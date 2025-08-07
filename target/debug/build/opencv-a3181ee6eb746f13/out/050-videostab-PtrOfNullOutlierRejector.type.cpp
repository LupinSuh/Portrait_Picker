extern "C" {
	const cv::videostab::NullOutlierRejector* cv_PtrLcv_videostab_NullOutlierRejectorG_getInnerPtr_const(const cv::Ptr<cv::videostab::NullOutlierRejector>* instance) {
			return instance->get();
	}
	
	cv::videostab::NullOutlierRejector* cv_PtrLcv_videostab_NullOutlierRejectorG_getInnerPtrMut(cv::Ptr<cv::videostab::NullOutlierRejector>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_NullOutlierRejectorG_delete(cv::Ptr<cv::videostab::NullOutlierRejector>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::IOutlierRejector>* cv_PtrLcv_videostab_NullOutlierRejectorG_to_PtrOfIOutlierRejector(cv::Ptr<cv::videostab::NullOutlierRejector>* instance) {
			return new cv::Ptr<cv::videostab::IOutlierRejector>(instance->dynamicCast<cv::videostab::IOutlierRejector>());
	}
	
	cv::Ptr<cv::videostab::NullOutlierRejector>* cv_PtrLcv_videostab_NullOutlierRejectorG_new_const_NullOutlierRejector(cv::videostab::NullOutlierRejector* val) {
			return new cv::Ptr<cv::videostab::NullOutlierRejector>(val);
	}
	
}

