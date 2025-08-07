extern "C" {
	const cv::videostab::MoreAccurateMotionWobbleSuppressor* cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_getInnerPtr_const(const cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>* instance) {
			return instance->get();
	}
	
	cv::videostab::MoreAccurateMotionWobbleSuppressor* cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_getInnerPtrMut(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_delete(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_to_PtrOfMoreAccurateMotionWobbleSuppressorBase(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>* instance) {
			return new cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>(instance->dynamicCast<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>());
	}
	
	cv::Ptr<cv::videostab::WobbleSuppressorBase>* cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_to_PtrOfWobbleSuppressorBase(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>* instance) {
			return new cv::Ptr<cv::videostab::WobbleSuppressorBase>(instance->dynamicCast<cv::videostab::WobbleSuppressorBase>());
	}
	
	cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>* cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_new_const_MoreAccurateMotionWobbleSuppressor(cv::videostab::MoreAccurateMotionWobbleSuppressor* val) {
			return new cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>(val);
	}
	
}

