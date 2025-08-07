extern "C" {
	const cv::bioinspired::Retina* cv_PtrLcv_bioinspired_RetinaG_getInnerPtr_const(const cv::Ptr<cv::bioinspired::Retina>* instance) {
			return instance->get();
	}
	
	cv::bioinspired::Retina* cv_PtrLcv_bioinspired_RetinaG_getInnerPtrMut(cv::Ptr<cv::bioinspired::Retina>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_bioinspired_RetinaG_delete(cv::Ptr<cv::bioinspired::Retina>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_bioinspired_RetinaG_to_PtrOfAlgorithm(cv::Ptr<cv::bioinspired::Retina>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

