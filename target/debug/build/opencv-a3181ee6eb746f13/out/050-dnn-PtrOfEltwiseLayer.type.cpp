extern "C" {
	const cv::dnn::EltwiseLayer* cv_PtrLcv_dnn_EltwiseLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::EltwiseLayer>* instance) {
			return instance->get();
	}
	
	cv::dnn::EltwiseLayer* cv_PtrLcv_dnn_EltwiseLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::EltwiseLayer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_dnn_EltwiseLayerG_delete(cv::Ptr<cv::dnn::EltwiseLayer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_EltwiseLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::EltwiseLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_EltwiseLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::EltwiseLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}
	
	cv::Ptr<cv::dnn::EltwiseLayer>* cv_PtrLcv_dnn_EltwiseLayerG_new_const_EltwiseLayer(cv::dnn::EltwiseLayer* val) {
			return new cv::Ptr<cv::dnn::EltwiseLayer>(val);
	}
	
}

