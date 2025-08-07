extern "C" {
	const cv::ml::SVM::Kernel* cv_PtrLcv_ml_SVM_KernelG_getInnerPtr_const(const cv::Ptr<cv::ml::SVM::Kernel>* instance) {
			return instance->get();
	}
	
	cv::ml::SVM::Kernel* cv_PtrLcv_ml_SVM_KernelG_getInnerPtrMut(cv::Ptr<cv::ml::SVM::Kernel>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ml_SVM_KernelG_delete(cv::Ptr<cv::ml::SVM::Kernel>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_SVM_KernelG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::SVM::Kernel>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

