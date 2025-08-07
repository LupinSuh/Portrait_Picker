extern "C" {
	const cv::bioinspired::TransientAreasSegmentationModule* cv_PtrLcv_bioinspired_TransientAreasSegmentationModuleG_getInnerPtr_const(const cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>* instance) {
			return instance->get();
	}
	
	cv::bioinspired::TransientAreasSegmentationModule* cv_PtrLcv_bioinspired_TransientAreasSegmentationModuleG_getInnerPtrMut(cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_bioinspired_TransientAreasSegmentationModuleG_delete(cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_bioinspired_TransientAreasSegmentationModuleG_to_PtrOfAlgorithm(cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

