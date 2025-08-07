extern "C" {
	const cv::dnn::PaddingLayer* cv_PtrLcv_dnn_PaddingLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::PaddingLayer>* instance) {
			return instance->get();
	}
	
	cv::dnn::PaddingLayer* cv_PtrLcv_dnn_PaddingLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::PaddingLayer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_dnn_PaddingLayerG_delete(cv::Ptr<cv::dnn::PaddingLayer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_PaddingLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::PaddingLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_PaddingLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::PaddingLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}
	
	cv::Ptr<cv::dnn::PaddingLayer>* cv_PtrLcv_dnn_PaddingLayerG_new_const_PaddingLayer(cv::dnn::PaddingLayer* val) {
			return new cv::Ptr<cv::dnn::PaddingLayer>(val);
	}
	
}

