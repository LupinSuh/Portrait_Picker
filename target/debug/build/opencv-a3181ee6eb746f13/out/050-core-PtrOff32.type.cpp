extern "C" {
	const float* cv_PtrLfloatG_getInnerPtr_const(const cv::Ptr<float>* instance) {
			return instance->get();
	}
	
	float* cv_PtrLfloatG_getInnerPtrMut(cv::Ptr<float>* instance) {
			return instance->get();
	}
	
	void cv_PtrLfloatG_delete(cv::Ptr<float>* instance) {
			delete instance;
	}
	
	cv::Ptr<float>* cv_PtrLfloatG_new_const_float(float val) {
			return new cv::Ptr<float>(new float(val));
	}
	
}

