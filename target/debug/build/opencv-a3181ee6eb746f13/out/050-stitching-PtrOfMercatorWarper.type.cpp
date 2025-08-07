extern "C" {
	const cv::MercatorWarper* cv_PtrLcv_MercatorWarperG_getInnerPtr_const(const cv::Ptr<cv::MercatorWarper>* instance) {
			return instance->get();
	}
	
	cv::MercatorWarper* cv_PtrLcv_MercatorWarperG_getInnerPtrMut(cv::Ptr<cv::MercatorWarper>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_MercatorWarperG_delete(cv::Ptr<cv::MercatorWarper>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_MercatorWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::MercatorWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
	
	cv::Ptr<cv::MercatorWarper>* cv_PtrLcv_MercatorWarperG_new_const_MercatorWarper(cv::MercatorWarper* val) {
			return new cv::Ptr<cv::MercatorWarper>(val);
	}
	
}

