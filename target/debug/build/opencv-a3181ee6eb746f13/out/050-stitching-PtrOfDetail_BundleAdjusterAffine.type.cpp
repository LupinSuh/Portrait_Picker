extern "C" {
	const cv::detail::BundleAdjusterAffine* cv_PtrLcv_detail_BundleAdjusterAffineG_getInnerPtr_const(const cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
			return instance->get();
	}
	
	cv::detail::BundleAdjusterAffine* cv_PtrLcv_detail_BundleAdjusterAffineG_getInnerPtrMut(cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_BundleAdjusterAffineG_delete(cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrLcv_detail_BundleAdjusterAffineG_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
			return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}
	
	cv::Ptr<cv::detail::Estimator>* cv_PtrLcv_detail_BundleAdjusterAffineG_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
			return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}
	
	cv::Ptr<cv::detail::BundleAdjusterAffine>* cv_PtrLcv_detail_BundleAdjusterAffineG_new_const_BundleAdjusterAffine(cv::detail::BundleAdjusterAffine* val) {
			return new cv::Ptr<cv::detail::BundleAdjusterAffine>(val);
	}
	
}

