extern "C" {
	const cv::kinfu::KinFu* cv_PtrLcv_kinfu_KinFuG_getInnerPtr_const(const cv::Ptr<cv::kinfu::KinFu>* instance) {
			return instance->get();
	}
	
	cv::kinfu::KinFu* cv_PtrLcv_kinfu_KinFuG_getInnerPtrMut(cv::Ptr<cv::kinfu::KinFu>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_kinfu_KinFuG_delete(cv::Ptr<cv::kinfu::KinFu>* instance) {
			delete instance;
	}
	
}

