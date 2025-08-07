extern "C" {
	const cv::PaniniPortraitWarper* cv_PtrLcv_PaniniPortraitWarperG_getInnerPtr_const(const cv::Ptr<cv::PaniniPortraitWarper>* instance) {
			return instance->get();
	}
	
	cv::PaniniPortraitWarper* cv_PtrLcv_PaniniPortraitWarperG_getInnerPtrMut(cv::Ptr<cv::PaniniPortraitWarper>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_PaniniPortraitWarperG_delete(cv::Ptr<cv::PaniniPortraitWarper>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_PaniniPortraitWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::PaniniPortraitWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
	
	cv::Ptr<cv::PaniniPortraitWarper>* cv_PtrLcv_PaniniPortraitWarperG_new_const_PaniniPortraitWarper(cv::PaniniPortraitWarper* val) {
			return new cv::Ptr<cv::PaniniPortraitWarper>(val);
	}
	
}

