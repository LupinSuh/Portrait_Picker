extern "C" {
	const cv::detail::BundleAdjusterRay* cv_PtrLcv_detail_BundleAdjusterRayG_getInnerPtr_const(const cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
			return instance->get();
	}
	
	cv::detail::BundleAdjusterRay* cv_PtrLcv_detail_BundleAdjusterRayG_getInnerPtrMut(cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_BundleAdjusterRayG_delete(cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrLcv_detail_BundleAdjusterRayG_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
			return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}
	
	cv::Ptr<cv::detail::Estimator>* cv_PtrLcv_detail_BundleAdjusterRayG_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
			return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}
	
	cv::Ptr<cv::detail::BundleAdjusterRay>* cv_PtrLcv_detail_BundleAdjusterRayG_new_const_BundleAdjusterRay(cv::detail::BundleAdjusterRay* val) {
			return new cv::Ptr<cv::detail::BundleAdjusterRay>(val);
	}
	
}

