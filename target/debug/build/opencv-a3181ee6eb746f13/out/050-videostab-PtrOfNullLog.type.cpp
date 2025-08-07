extern "C" {
	const cv::videostab::NullLog* cv_PtrLcv_videostab_NullLogG_getInnerPtr_const(const cv::Ptr<cv::videostab::NullLog>* instance) {
			return instance->get();
	}
	
	cv::videostab::NullLog* cv_PtrLcv_videostab_NullLogG_getInnerPtrMut(cv::Ptr<cv::videostab::NullLog>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_NullLogG_delete(cv::Ptr<cv::videostab::NullLog>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::ILog>* cv_PtrLcv_videostab_NullLogG_to_PtrOfILog(cv::Ptr<cv::videostab::NullLog>* instance) {
			return new cv::Ptr<cv::videostab::ILog>(instance->dynamicCast<cv::videostab::ILog>());
	}
	
	cv::Ptr<cv::videostab::NullLog>* cv_PtrLcv_videostab_NullLogG_new_const_NullLog(cv::videostab::NullLog* val) {
			return new cv::Ptr<cv::videostab::NullLog>(val);
	}
	
}

