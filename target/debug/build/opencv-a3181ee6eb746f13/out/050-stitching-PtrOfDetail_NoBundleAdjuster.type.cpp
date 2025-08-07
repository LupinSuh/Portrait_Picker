extern "C" {
	const cv::detail::NoBundleAdjuster* cv_PtrLcv_detail_NoBundleAdjusterG_getInnerPtr_const(const cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
			return instance->get();
	}
	
	cv::detail::NoBundleAdjuster* cv_PtrLcv_detail_NoBundleAdjusterG_getInnerPtrMut(cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_NoBundleAdjusterG_delete(cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrLcv_detail_NoBundleAdjusterG_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
			return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}
	
	cv::Ptr<cv::detail::Estimator>* cv_PtrLcv_detail_NoBundleAdjusterG_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
			return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}
	
	cv::Ptr<cv::detail::NoBundleAdjuster>* cv_PtrLcv_detail_NoBundleAdjusterG_new_const_NoBundleAdjuster(cv::detail::NoBundleAdjuster* val) {
			return new cv::Ptr<cv::detail::NoBundleAdjuster>(val);
	}
	
}

