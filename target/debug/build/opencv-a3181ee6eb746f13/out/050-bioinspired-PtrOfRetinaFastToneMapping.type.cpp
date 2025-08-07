extern "C" {
	const cv::bioinspired::RetinaFastToneMapping* cv_PtrLcv_bioinspired_RetinaFastToneMappingG_getInnerPtr_const(const cv::Ptr<cv::bioinspired::RetinaFastToneMapping>* instance) {
			return instance->get();
	}
	
	cv::bioinspired::RetinaFastToneMapping* cv_PtrLcv_bioinspired_RetinaFastToneMappingG_getInnerPtrMut(cv::Ptr<cv::bioinspired::RetinaFastToneMapping>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_bioinspired_RetinaFastToneMappingG_delete(cv::Ptr<cv::bioinspired::RetinaFastToneMapping>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_bioinspired_RetinaFastToneMappingG_to_PtrOfAlgorithm(cv::Ptr<cv::bioinspired::RetinaFastToneMapping>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

