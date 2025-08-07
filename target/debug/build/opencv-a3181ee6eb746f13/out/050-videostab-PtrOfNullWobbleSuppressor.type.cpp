extern "C" {
	const cv::videostab::NullWobbleSuppressor* cv_PtrLcv_videostab_NullWobbleSuppressorG_getInnerPtr_const(const cv::Ptr<cv::videostab::NullWobbleSuppressor>* instance) {
			return instance->get();
	}
	
	cv::videostab::NullWobbleSuppressor* cv_PtrLcv_videostab_NullWobbleSuppressorG_getInnerPtrMut(cv::Ptr<cv::videostab::NullWobbleSuppressor>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_NullWobbleSuppressorG_delete(cv::Ptr<cv::videostab::NullWobbleSuppressor>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::WobbleSuppressorBase>* cv_PtrLcv_videostab_NullWobbleSuppressorG_to_PtrOfWobbleSuppressorBase(cv::Ptr<cv::videostab::NullWobbleSuppressor>* instance) {
			return new cv::Ptr<cv::videostab::WobbleSuppressorBase>(instance->dynamicCast<cv::videostab::WobbleSuppressorBase>());
	}
	
	cv::Ptr<cv::videostab::NullWobbleSuppressor>* cv_PtrLcv_videostab_NullWobbleSuppressorG_new_const_NullWobbleSuppressor(cv::videostab::NullWobbleSuppressor* val) {
			return new cv::Ptr<cv::videostab::NullWobbleSuppressor>(val);
	}
	
}

