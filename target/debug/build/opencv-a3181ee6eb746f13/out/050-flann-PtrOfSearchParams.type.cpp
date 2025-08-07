extern "C" {
	const cv::flann::SearchParams* cv_PtrLcv_flann_SearchParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::SearchParams>* instance) {
			return instance->get();
	}
	
	cv::flann::SearchParams* cv_PtrLcv_flann_SearchParamsG_getInnerPtrMut(cv::Ptr<cv::flann::SearchParams>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_flann_SearchParamsG_delete(cv::Ptr<cv::flann::SearchParams>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_SearchParamsG_to_PtrOfIndexParams(cv::Ptr<cv::flann::SearchParams>* instance) {
			return new cv::Ptr<cv::flann::IndexParams>(instance->dynamicCast<cv::flann::IndexParams>());
	}
	
	cv::Ptr<cv::flann::SearchParams>* cv_PtrLcv_flann_SearchParamsG_new_const_SearchParams(cv::flann::SearchParams* val) {
			return new cv::Ptr<cv::flann::SearchParams>(val);
	}
	
}

