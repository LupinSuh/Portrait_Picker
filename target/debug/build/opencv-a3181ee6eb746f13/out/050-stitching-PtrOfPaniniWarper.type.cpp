extern "C" {
	const cv::PaniniWarper* cv_PtrLcv_PaniniWarperG_getInnerPtr_const(const cv::Ptr<cv::PaniniWarper>* instance) {
			return instance->get();
	}
	
	cv::PaniniWarper* cv_PtrLcv_PaniniWarperG_getInnerPtrMut(cv::Ptr<cv::PaniniWarper>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_PaniniWarperG_delete(cv::Ptr<cv::PaniniWarper>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_PaniniWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::PaniniWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
	
	cv::Ptr<cv::PaniniWarper>* cv_PtrLcv_PaniniWarperG_new_const_PaniniWarper(cv::PaniniWarper* val) {
			return new cv::Ptr<cv::PaniniWarper>(val);
	}
	
}

