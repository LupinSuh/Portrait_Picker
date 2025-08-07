extern "C" {
	const cv::detail::AffineBasedEstimator* cv_PtrLcv_detail_AffineBasedEstimatorG_getInnerPtr_const(const cv::Ptr<cv::detail::AffineBasedEstimator>* instance) {
			return instance->get();
	}
	
	cv::detail::AffineBasedEstimator* cv_PtrLcv_detail_AffineBasedEstimatorG_getInnerPtrMut(cv::Ptr<cv::detail::AffineBasedEstimator>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_AffineBasedEstimatorG_delete(cv::Ptr<cv::detail::AffineBasedEstimator>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::Estimator>* cv_PtrLcv_detail_AffineBasedEstimatorG_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::AffineBasedEstimator>* instance) {
			return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}
	
	cv::Ptr<cv::detail::AffineBasedEstimator>* cv_PtrLcv_detail_AffineBasedEstimatorG_new_const_AffineBasedEstimator(cv::detail::AffineBasedEstimator* val) {
			return new cv::Ptr<cv::detail::AffineBasedEstimator>(val);
	}
	
}

