extern "C" {
	const cv::videostab::NullDeblurer* cv_PtrLcv_videostab_NullDeblurerG_getInnerPtr_const(const cv::Ptr<cv::videostab::NullDeblurer>* instance) {
			return instance->get();
	}
	
	cv::videostab::NullDeblurer* cv_PtrLcv_videostab_NullDeblurerG_getInnerPtrMut(cv::Ptr<cv::videostab::NullDeblurer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_NullDeblurerG_delete(cv::Ptr<cv::videostab::NullDeblurer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::DeblurerBase>* cv_PtrLcv_videostab_NullDeblurerG_to_PtrOfDeblurerBase(cv::Ptr<cv::videostab::NullDeblurer>* instance) {
			return new cv::Ptr<cv::videostab::DeblurerBase>(instance->dynamicCast<cv::videostab::DeblurerBase>());
	}
	
	cv::Ptr<cv::videostab::NullDeblurer>* cv_PtrLcv_videostab_NullDeblurerG_new_const_NullDeblurer(cv::videostab::NullDeblurer* val) {
			return new cv::Ptr<cv::videostab::NullDeblurer>(val);
	}
	
}

