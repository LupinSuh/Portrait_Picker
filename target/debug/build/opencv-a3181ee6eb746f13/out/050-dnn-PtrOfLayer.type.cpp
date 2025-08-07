extern "C" {
	const cv::dnn::Layer* cv_PtrLcv_dnn_LayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::Layer>* instance) {
			return instance->get();
	}
	
	cv::dnn::Layer* cv_PtrLcv_dnn_LayerG_getInnerPtrMut(cv::Ptr<cv::dnn::Layer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_dnn_LayerG_delete(cv::Ptr<cv::dnn::Layer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_LayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::Layer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_LayerG_new_const_Layer(cv::dnn::Layer* val) {
			return new cv::Ptr<cv::dnn::Layer>(val);
	}
	
}

