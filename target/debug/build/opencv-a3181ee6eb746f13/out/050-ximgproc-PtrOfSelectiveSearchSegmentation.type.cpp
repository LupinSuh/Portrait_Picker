extern "C" {
	const cv::ximgproc::segmentation::SelectiveSearchSegmentation* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::segmentation::SelectiveSearchSegmentation* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationG_getInnerPtrMut(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationG_delete(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

