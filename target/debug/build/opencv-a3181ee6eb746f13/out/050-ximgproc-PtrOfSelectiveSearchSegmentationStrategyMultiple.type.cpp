extern "C" {
	const cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultipleG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultipleG_getInnerPtrMut(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultipleG_delete(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultipleG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultipleG_to_PtrOfSelectiveSearchSegmentationStrategy(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>* instance) {
			return new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>(instance->dynamicCast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>());
	}
	
}

