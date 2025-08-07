extern "C" {
	const cv::AffineWarper* cv_PtrLcv_AffineWarperG_getInnerPtr_const(const cv::Ptr<cv::AffineWarper>* instance) {
			return instance->get();
	}
	
	cv::AffineWarper* cv_PtrLcv_AffineWarperG_getInnerPtrMut(cv::Ptr<cv::AffineWarper>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_AffineWarperG_delete(cv::Ptr<cv::AffineWarper>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_AffineWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::AffineWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
	
	cv::Ptr<cv::AffineWarper>* cv_PtrLcv_AffineWarperG_new_const_AffineWarper(cv::AffineWarper* val) {
			return new cv::Ptr<cv::AffineWarper>(val);
	}
	
}

