extern "C" {
	const cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFillG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFillG_getInnerPtrMut(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFillG_delete(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFillG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFillG_to_PtrOfSelectiveSearchSegmentationStrategy(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>* instance) {
			return new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>(instance->dynamicCast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>());
	}
	
}

