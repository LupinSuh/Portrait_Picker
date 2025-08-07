extern "C" {
	const cv::CylindricalWarper* cv_PtrLcv_CylindricalWarperG_getInnerPtr_const(const cv::Ptr<cv::CylindricalWarper>* instance) {
			return instance->get();
	}
	
	cv::CylindricalWarper* cv_PtrLcv_CylindricalWarperG_getInnerPtrMut(cv::Ptr<cv::CylindricalWarper>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_CylindricalWarperG_delete(cv::Ptr<cv::CylindricalWarper>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_CylindricalWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::CylindricalWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
	
	cv::Ptr<cv::CylindricalWarper>* cv_PtrLcv_CylindricalWarperG_new_const_CylindricalWarper(cv::CylindricalWarper* val) {
			return new cv::Ptr<cv::CylindricalWarper>(val);
	}
	
}

