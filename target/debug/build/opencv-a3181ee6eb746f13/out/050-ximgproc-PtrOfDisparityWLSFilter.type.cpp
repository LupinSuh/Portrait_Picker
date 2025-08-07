extern "C" {
	const cv::ximgproc::DisparityWLSFilter* cv_PtrLcv_ximgproc_DisparityWLSFilterG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::DisparityWLSFilter>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::DisparityWLSFilter* cv_PtrLcv_ximgproc_DisparityWLSFilterG_getInnerPtrMut(cv::Ptr<cv::ximgproc::DisparityWLSFilter>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_DisparityWLSFilterG_delete(cv::Ptr<cv::ximgproc::DisparityWLSFilter>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_DisparityWLSFilterG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::DisparityWLSFilter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ximgproc::DisparityFilter>* cv_PtrLcv_ximgproc_DisparityWLSFilterG_to_PtrOfDisparityFilter(cv::Ptr<cv::ximgproc::DisparityWLSFilter>* instance) {
			return new cv::Ptr<cv::ximgproc::DisparityFilter>(instance->dynamicCast<cv::ximgproc::DisparityFilter>());
	}
	
}

