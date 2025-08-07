extern "C" {
	const cv::detail::HomographyBasedEstimator* cv_PtrLcv_detail_HomographyBasedEstimatorG_getInnerPtr_const(const cv::Ptr<cv::detail::HomographyBasedEstimator>* instance) {
			return instance->get();
	}
	
	cv::detail::HomographyBasedEstimator* cv_PtrLcv_detail_HomographyBasedEstimatorG_getInnerPtrMut(cv::Ptr<cv::detail::HomographyBasedEstimator>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_HomographyBasedEstimatorG_delete(cv::Ptr<cv::detail::HomographyBasedEstimator>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::Estimator>* cv_PtrLcv_detail_HomographyBasedEstimatorG_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::HomographyBasedEstimator>* instance) {
			return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}
	
	cv::Ptr<cv::detail::HomographyBasedEstimator>* cv_PtrLcv_detail_HomographyBasedEstimatorG_new_const_HomographyBasedEstimator(cv::detail::HomographyBasedEstimator* val) {
			return new cv::Ptr<cv::detail::HomographyBasedEstimator>(val);
	}
	
}

