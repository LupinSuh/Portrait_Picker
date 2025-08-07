extern "C" {
	const cv::detail::BundleAdjusterBase* cv_PtrLcv_detail_BundleAdjusterBaseG_getInnerPtr_const(const cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
			return instance->get();
	}
	
	cv::detail::BundleAdjusterBase* cv_PtrLcv_detail_BundleAdjusterBaseG_getInnerPtrMut(cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_BundleAdjusterBaseG_delete(cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::Estimator>* cv_PtrLcv_detail_BundleAdjusterBaseG_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
			return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}
	
}

