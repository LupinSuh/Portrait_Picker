extern "C" {
	const cv::videostab::MoreAccurateMotionWobbleSuppressorBase* cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorBaseG_getInnerPtr_const(const cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* instance) {
			return instance->get();
	}
	
	cv::videostab::MoreAccurateMotionWobbleSuppressorBase* cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorBaseG_getInnerPtrMut(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorBaseG_delete(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::WobbleSuppressorBase>* cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorBaseG_to_PtrOfWobbleSuppressorBase(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* instance) {
			return new cv::Ptr<cv::videostab::WobbleSuppressorBase>(instance->dynamicCast<cv::videostab::WobbleSuppressorBase>());
	}
	
}

