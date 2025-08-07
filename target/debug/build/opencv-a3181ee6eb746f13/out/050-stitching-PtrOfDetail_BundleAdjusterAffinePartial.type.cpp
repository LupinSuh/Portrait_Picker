extern "C" {
	const cv::detail::BundleAdjusterAffinePartial* cv_PtrLcv_detail_BundleAdjusterAffinePartialG_getInnerPtr_const(const cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
			return instance->get();
	}
	
	cv::detail::BundleAdjusterAffinePartial* cv_PtrLcv_detail_BundleAdjusterAffinePartialG_getInnerPtrMut(cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_BundleAdjusterAffinePartialG_delete(cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrLcv_detail_BundleAdjusterAffinePartialG_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
			return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}
	
	cv::Ptr<cv::detail::Estimator>* cv_PtrLcv_detail_BundleAdjusterAffinePartialG_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
			return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}
	
	cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* cv_PtrLcv_detail_BundleAdjusterAffinePartialG_new_const_BundleAdjusterAffinePartial(cv::detail::BundleAdjusterAffinePartial* val) {
			return new cv::Ptr<cv::detail::BundleAdjusterAffinePartial>(val);
	}
	
}

