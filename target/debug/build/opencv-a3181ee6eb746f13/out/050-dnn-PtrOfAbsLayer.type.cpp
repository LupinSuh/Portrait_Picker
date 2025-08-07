extern "C" {
	const cv::dnn::AbsLayer* cv_PtrLcv_dnn_AbsLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::AbsLayer>* instance) {
			return instance->get();
	}
	
	cv::dnn::AbsLayer* cv_PtrLcv_dnn_AbsLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::AbsLayer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_dnn_AbsLayerG_delete(cv::Ptr<cv::dnn::AbsLayer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_AbsLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::AbsLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_AbsLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::AbsLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}
	
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_AbsLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::AbsLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}
	
	cv::Ptr<cv::dnn::AbsLayer>* cv_PtrLcv_dnn_AbsLayerG_new_const_AbsLayer(cv::dnn::AbsLayer* val) {
			return new cv::Ptr<cv::dnn::AbsLayer>(val);
	}
	
}

