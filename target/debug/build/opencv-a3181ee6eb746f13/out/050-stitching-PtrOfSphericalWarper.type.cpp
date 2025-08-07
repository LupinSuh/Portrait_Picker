extern "C" {
	const cv::SphericalWarper* cv_PtrLcv_SphericalWarperG_getInnerPtr_const(const cv::Ptr<cv::SphericalWarper>* instance) {
			return instance->get();
	}
	
	cv::SphericalWarper* cv_PtrLcv_SphericalWarperG_getInnerPtrMut(cv::Ptr<cv::SphericalWarper>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_SphericalWarperG_delete(cv::Ptr<cv::SphericalWarper>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_SphericalWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::SphericalWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
	
	cv::Ptr<cv::SphericalWarper>* cv_PtrLcv_SphericalWarperG_new_const_SphericalWarper(cv::SphericalWarper* val) {
			return new cv::Ptr<cv::SphericalWarper>(val);
	}
	
}

