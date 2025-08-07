extern "C" {
	const cv::detail::BundleAdjusterReproj* cv_PtrLcv_detail_BundleAdjusterReprojG_getInnerPtr_const(const cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
			return instance->get();
	}
	
	cv::detail::BundleAdjusterReproj* cv_PtrLcv_detail_BundleAdjusterReprojG_getInnerPtrMut(cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_BundleAdjusterReprojG_delete(cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrLcv_detail_BundleAdjusterReprojG_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
			return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}
	
	cv::Ptr<cv::detail::Estimator>* cv_PtrLcv_detail_BundleAdjusterReprojG_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
			return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}
	
	cv::Ptr<cv::detail::BundleAdjusterReproj>* cv_PtrLcv_detail_BundleAdjusterReprojG_new_const_BundleAdjusterReproj(cv::detail::BundleAdjusterReproj* val) {
			return new cv::Ptr<cv::detail::BundleAdjusterReproj>(val);
	}
	
}

