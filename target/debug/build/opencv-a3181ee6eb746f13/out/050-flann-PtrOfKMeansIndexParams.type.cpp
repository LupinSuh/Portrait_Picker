extern "C" {
	const cv::flann::KMeansIndexParams* cv_PtrLcv_flann_KMeansIndexParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::KMeansIndexParams>* instance) {
			return instance->get();
	}
	
	cv::flann::KMeansIndexParams* cv_PtrLcv_flann_KMeansIndexParamsG_getInnerPtrMut(cv::Ptr<cv::flann::KMeansIndexParams>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_flann_KMeansIndexParamsG_delete(cv::Ptr<cv::flann::KMeansIndexParams>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_KMeansIndexParamsG_to_PtrOfIndexParams(cv::Ptr<cv::flann::KMeansIndexParams>* instance) {
			return new cv::Ptr<cv::flann::IndexParams>(instance->dynamicCast<cv::flann::IndexParams>());
	}
	
	cv::Ptr<cv::flann::KMeansIndexParams>* cv_PtrLcv_flann_KMeansIndexParamsG_new_const_KMeansIndexParams(cv::flann::KMeansIndexParams* val) {
			return new cv::Ptr<cv::flann::KMeansIndexParams>(val);
	}
	
}

