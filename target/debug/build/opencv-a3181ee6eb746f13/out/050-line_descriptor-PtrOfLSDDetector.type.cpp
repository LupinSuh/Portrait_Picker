extern "C" {
	const cv::line_descriptor::LSDDetector* cv_PtrLcv_line_descriptor_LSDDetectorG_getInnerPtr_const(const cv::Ptr<cv::line_descriptor::LSDDetector>* instance) {
			return instance->get();
	}
	
	cv::line_descriptor::LSDDetector* cv_PtrLcv_line_descriptor_LSDDetectorG_getInnerPtrMut(cv::Ptr<cv::line_descriptor::LSDDetector>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_line_descriptor_LSDDetectorG_delete(cv::Ptr<cv::line_descriptor::LSDDetector>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_line_descriptor_LSDDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::line_descriptor::LSDDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::line_descriptor::LSDDetector>* cv_PtrLcv_line_descriptor_LSDDetectorG_new_const_LSDDetector(cv::line_descriptor::LSDDetector* val) {
			return new cv::Ptr<cv::line_descriptor::LSDDetector>(val);
	}
	
}

