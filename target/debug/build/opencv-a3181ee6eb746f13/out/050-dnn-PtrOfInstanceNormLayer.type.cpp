extern "C" {
	const cv::dnn::InstanceNormLayer* cv_PtrLcv_dnn_InstanceNormLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::InstanceNormLayer>* instance) {
			return instance->get();
	}
	
	cv::dnn::InstanceNormLayer* cv_PtrLcv_dnn_InstanceNormLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::InstanceNormLayer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_dnn_InstanceNormLayerG_delete(cv::Ptr<cv::dnn::InstanceNormLayer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_InstanceNormLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::InstanceNormLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_InstanceNormLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::InstanceNormLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}
	
	cv::Ptr<cv::dnn::InstanceNormLayer>* cv_PtrLcv_dnn_InstanceNormLayerG_new_const_InstanceNormLayer(cv::dnn::InstanceNormLayer* val) {
			return new cv::Ptr<cv::dnn::InstanceNormLayer>(val);
	}
	
}

