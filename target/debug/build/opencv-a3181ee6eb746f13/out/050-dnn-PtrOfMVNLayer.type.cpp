extern "C" {
	const cv::dnn::MVNLayer* cv_PtrLcv_dnn_MVNLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::MVNLayer>* instance) {
			return instance->get();
	}
	
	cv::dnn::MVNLayer* cv_PtrLcv_dnn_MVNLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::MVNLayer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_dnn_MVNLayerG_delete(cv::Ptr<cv::dnn::MVNLayer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_MVNLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::MVNLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_MVNLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::MVNLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}
	
	cv::Ptr<cv::dnn::MVNLayer>* cv_PtrLcv_dnn_MVNLayerG_new_const_MVNLayer(cv::dnn::MVNLayer* val) {
			return new cv::Ptr<cv::dnn::MVNLayer>(val);
	}
	
}

