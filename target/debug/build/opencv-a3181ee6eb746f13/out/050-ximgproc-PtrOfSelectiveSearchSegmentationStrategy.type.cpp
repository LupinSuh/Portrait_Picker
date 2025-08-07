extern "C" {
	const cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyG_getInnerPtrMut(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyG_delete(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

