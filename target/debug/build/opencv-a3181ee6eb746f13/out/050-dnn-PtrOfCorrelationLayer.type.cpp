extern "C" {
	const cv::dnn::CorrelationLayer* cv_PtrLcv_dnn_CorrelationLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::CorrelationLayer>* instance) {
			return instance->get();
	}
	
	cv::dnn::CorrelationLayer* cv_PtrLcv_dnn_CorrelationLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::CorrelationLayer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_dnn_CorrelationLayerG_delete(cv::Ptr<cv::dnn::CorrelationLayer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_CorrelationLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::CorrelationLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_CorrelationLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::CorrelationLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}
	
	cv::Ptr<cv::dnn::CorrelationLayer>* cv_PtrLcv_dnn_CorrelationLayerG_new_const_CorrelationLayer(cv::dnn::CorrelationLayer* val) {
			return new cv::Ptr<cv::dnn::CorrelationLayer>(val);
	}
	
}

