extern "C" {
	const cv::videostab::WeightingDeblurer* cv_PtrLcv_videostab_WeightingDeblurerG_getInnerPtr_const(const cv::Ptr<cv::videostab::WeightingDeblurer>* instance) {
			return instance->get();
	}
	
	cv::videostab::WeightingDeblurer* cv_PtrLcv_videostab_WeightingDeblurerG_getInnerPtrMut(cv::Ptr<cv::videostab::WeightingDeblurer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_WeightingDeblurerG_delete(cv::Ptr<cv::videostab::WeightingDeblurer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::DeblurerBase>* cv_PtrLcv_videostab_WeightingDeblurerG_to_PtrOfDeblurerBase(cv::Ptr<cv::videostab::WeightingDeblurer>* instance) {
			return new cv::Ptr<cv::videostab::DeblurerBase>(instance->dynamicCast<cv::videostab::DeblurerBase>());
	}
	
	cv::Ptr<cv::videostab::WeightingDeblurer>* cv_PtrLcv_videostab_WeightingDeblurerG_new_const_WeightingDeblurer(cv::videostab::WeightingDeblurer* val) {
			return new cv::Ptr<cv::videostab::WeightingDeblurer>(val);
	}
	
}

