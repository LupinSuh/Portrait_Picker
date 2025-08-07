extern "C" {
	const cv::kinfu::Params* cv_PtrLcv_kinfu_ParamsG_getInnerPtr_const(const cv::Ptr<cv::kinfu::Params>* instance) {
			return instance->get();
	}
	
	cv::kinfu::Params* cv_PtrLcv_kinfu_ParamsG_getInnerPtrMut(cv::Ptr<cv::kinfu::Params>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_kinfu_ParamsG_delete(cv::Ptr<cv::kinfu::Params>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::kinfu::Params>* cv_PtrLcv_kinfu_ParamsG_new_const_Params(cv::kinfu::Params* val) {
			return new cv::Ptr<cv::kinfu::Params>(val);
	}
	
}

