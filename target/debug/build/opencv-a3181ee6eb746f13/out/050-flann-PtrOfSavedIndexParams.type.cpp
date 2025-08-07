extern "C" {
	const cv::flann::SavedIndexParams* cv_PtrLcv_flann_SavedIndexParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::SavedIndexParams>* instance) {
			return instance->get();
	}
	
	cv::flann::SavedIndexParams* cv_PtrLcv_flann_SavedIndexParamsG_getInnerPtrMut(cv::Ptr<cv::flann::SavedIndexParams>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_flann_SavedIndexParamsG_delete(cv::Ptr<cv::flann::SavedIndexParams>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_SavedIndexParamsG_to_PtrOfIndexParams(cv::Ptr<cv::flann::SavedIndexParams>* instance) {
			return new cv::Ptr<cv::flann::IndexParams>(instance->dynamicCast<cv::flann::IndexParams>());
	}
	
	cv::Ptr<cv::flann::SavedIndexParams>* cv_PtrLcv_flann_SavedIndexParamsG_new_const_SavedIndexParams(cv::flann::SavedIndexParams* val) {
			return new cv::Ptr<cv::flann::SavedIndexParams>(val);
	}
	
}

