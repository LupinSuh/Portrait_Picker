extern "C" {
	const cv::StereographicWarper* cv_PtrLcv_StereographicWarperG_getInnerPtr_const(const cv::Ptr<cv::StereographicWarper>* instance) {
			return instance->get();
	}
	
	cv::StereographicWarper* cv_PtrLcv_StereographicWarperG_getInnerPtrMut(cv::Ptr<cv::StereographicWarper>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_StereographicWarperG_delete(cv::Ptr<cv::StereographicWarper>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_StereographicWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::StereographicWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
	
	cv::Ptr<cv::StereographicWarper>* cv_PtrLcv_StereographicWarperG_new_const_StereographicWarper(cv::StereographicWarper* val) {
			return new cv::Ptr<cv::StereographicWarper>(val);
	}
	
}

