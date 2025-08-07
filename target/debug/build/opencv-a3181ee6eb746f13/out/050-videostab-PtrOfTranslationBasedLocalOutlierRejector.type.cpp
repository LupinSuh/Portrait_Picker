extern "C" {
	const cv::videostab::TranslationBasedLocalOutlierRejector* cv_PtrLcv_videostab_TranslationBasedLocalOutlierRejectorG_getInnerPtr_const(const cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* instance) {
			return instance->get();
	}
	
	cv::videostab::TranslationBasedLocalOutlierRejector* cv_PtrLcv_videostab_TranslationBasedLocalOutlierRejectorG_getInnerPtrMut(cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_TranslationBasedLocalOutlierRejectorG_delete(cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::IOutlierRejector>* cv_PtrLcv_videostab_TranslationBasedLocalOutlierRejectorG_to_PtrOfIOutlierRejector(cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* instance) {
			return new cv::Ptr<cv::videostab::IOutlierRejector>(instance->dynamicCast<cv::videostab::IOutlierRejector>());
	}
	
	cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* cv_PtrLcv_videostab_TranslationBasedLocalOutlierRejectorG_new_const_TranslationBasedLocalOutlierRejector(cv::videostab::TranslationBasedLocalOutlierRejector* val) {
			return new cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>(val);
	}
	
}

