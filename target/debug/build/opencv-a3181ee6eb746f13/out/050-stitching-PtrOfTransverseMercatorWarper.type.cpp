extern "C" {
	const cv::TransverseMercatorWarper* cv_PtrLcv_TransverseMercatorWarperG_getInnerPtr_const(const cv::Ptr<cv::TransverseMercatorWarper>* instance) {
			return instance->get();
	}
	
	cv::TransverseMercatorWarper* cv_PtrLcv_TransverseMercatorWarperG_getInnerPtrMut(cv::Ptr<cv::TransverseMercatorWarper>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_TransverseMercatorWarperG_delete(cv::Ptr<cv::TransverseMercatorWarper>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_TransverseMercatorWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::TransverseMercatorWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
	
	cv::Ptr<cv::TransverseMercatorWarper>* cv_PtrLcv_TransverseMercatorWarperG_new_const_TransverseMercatorWarper(cv::TransverseMercatorWarper* val) {
			return new cv::Ptr<cv::TransverseMercatorWarper>(val);
	}
	
}

