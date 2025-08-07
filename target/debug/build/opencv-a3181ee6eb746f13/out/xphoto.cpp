#include "ocvrs_common.hpp"
#include <opencv2/xphoto.hpp>
#include "xphoto_types.hpp"

extern "C" {
	void cv_xphoto_GrayworldWB_getSaturationThreshold_const(const cv::xphoto::GrayworldWB* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSaturationThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_GrayworldWB_setSaturationThreshold_float(cv::xphoto::GrayworldWB* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setSaturationThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_xphoto_GrayworldWB_to_Algorithm(cv::xphoto::GrayworldWB* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::xphoto::WhiteBalancer* cv_xphoto_GrayworldWB_to_WhiteBalancer(cv::xphoto::GrayworldWB* instance) {
			return dynamic_cast<cv::xphoto::WhiteBalancer*>(instance);
	}
	
	void cv_xphoto_GrayworldWB_delete(cv::xphoto::GrayworldWB* instance) {
			delete instance;
	}
	
	void cv_xphoto_LearningBasedWB_extractSimpleFeatures_const__InputArrayR_const__OutputArrayR(cv::xphoto::LearningBasedWB* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->extractSimpleFeatures(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_LearningBasedWB_getRangeMaxVal_const(const cv::xphoto::LearningBasedWB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getRangeMaxVal();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_LearningBasedWB_setRangeMaxVal_int(cv::xphoto::LearningBasedWB* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setRangeMaxVal(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_LearningBasedWB_getSaturationThreshold_const(const cv::xphoto::LearningBasedWB* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSaturationThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_LearningBasedWB_setSaturationThreshold_float(cv::xphoto::LearningBasedWB* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setSaturationThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_LearningBasedWB_getHistBinNum_const(const cv::xphoto::LearningBasedWB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getHistBinNum();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_LearningBasedWB_setHistBinNum_int(cv::xphoto::LearningBasedWB* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setHistBinNum(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_xphoto_LearningBasedWB_to_Algorithm(cv::xphoto::LearningBasedWB* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::xphoto::WhiteBalancer* cv_xphoto_LearningBasedWB_to_WhiteBalancer(cv::xphoto::LearningBasedWB* instance) {
			return dynamic_cast<cv::xphoto::WhiteBalancer*>(instance);
	}
	
	void cv_xphoto_LearningBasedWB_delete(cv::xphoto::LearningBasedWB* instance) {
			delete instance;
	}
	
	void cv_xphoto_SimpleWB_getInputMin_const(const cv::xphoto::SimpleWB* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getInputMin();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_SimpleWB_setInputMin_float(cv::xphoto::SimpleWB* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setInputMin(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_SimpleWB_getInputMax_const(const cv::xphoto::SimpleWB* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getInputMax();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_SimpleWB_setInputMax_float(cv::xphoto::SimpleWB* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setInputMax(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_SimpleWB_getOutputMin_const(const cv::xphoto::SimpleWB* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getOutputMin();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_SimpleWB_setOutputMin_float(cv::xphoto::SimpleWB* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setOutputMin(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_SimpleWB_getOutputMax_const(const cv::xphoto::SimpleWB* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getOutputMax();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_SimpleWB_setOutputMax_float(cv::xphoto::SimpleWB* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setOutputMax(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_SimpleWB_getP_const(const cv::xphoto::SimpleWB* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getP();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_SimpleWB_setP_float(cv::xphoto::SimpleWB* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setP(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_xphoto_SimpleWB_to_Algorithm(cv::xphoto::SimpleWB* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::xphoto::WhiteBalancer* cv_xphoto_SimpleWB_to_WhiteBalancer(cv::xphoto::SimpleWB* instance) {
			return dynamic_cast<cv::xphoto::WhiteBalancer*>(instance);
	}
	
	void cv_xphoto_SimpleWB_delete(cv::xphoto::SimpleWB* instance) {
			delete instance;
	}
	
	void cv_xphoto_TonemapDurand_getSaturation_const(const cv::xphoto::TonemapDurand* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSaturation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_TonemapDurand_setSaturation_float(cv::xphoto::TonemapDurand* instance, float saturation, ResultVoid* ocvrs_return) {
		try {
			instance->setSaturation(saturation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_TonemapDurand_getContrast_const(const cv::xphoto::TonemapDurand* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getContrast();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_TonemapDurand_setContrast_float(cv::xphoto::TonemapDurand* instance, float contrast, ResultVoid* ocvrs_return) {
		try {
			instance->setContrast(contrast);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_TonemapDurand_getSigmaSpace_const(const cv::xphoto::TonemapDurand* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSigmaSpace();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_TonemapDurand_setSigmaSpace_float(cv::xphoto::TonemapDurand* instance, float sigma_space, ResultVoid* ocvrs_return) {
		try {
			instance->setSigmaSpace(sigma_space);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_TonemapDurand_getSigmaColor_const(const cv::xphoto::TonemapDurand* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSigmaColor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_xphoto_TonemapDurand_setSigmaColor_float(cv::xphoto::TonemapDurand* instance, float sigma_color, ResultVoid* ocvrs_return) {
		try {
			instance->setSigmaColor(sigma_color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_xphoto_TonemapDurand_to_Algorithm(cv::xphoto::TonemapDurand* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::Tonemap* cv_xphoto_TonemapDurand_to_Tonemap(cv::xphoto::TonemapDurand* instance) {
			return dynamic_cast<cv::Tonemap*>(instance);
	}
	
	void cv_xphoto_TonemapDurand_delete(cv::xphoto::TonemapDurand* instance) {
			delete instance;
	}
	
	void cv_xphoto_WhiteBalancer_balanceWhite_const__InputArrayR_const__OutputArrayR(cv::xphoto::WhiteBalancer* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->balanceWhite(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::xphoto::GrayworldWB* cv_xphoto_WhiteBalancer_to_GrayworldWB(cv::xphoto::WhiteBalancer* instance) {
			return dynamic_cast<cv::xphoto::GrayworldWB*>(instance);
	}
	
	cv::xphoto::LearningBasedWB* cv_xphoto_WhiteBalancer_to_LearningBasedWB(cv::xphoto::WhiteBalancer* instance) {
			return dynamic_cast<cv::xphoto::LearningBasedWB*>(instance);
	}
	
	cv::xphoto::SimpleWB* cv_xphoto_WhiteBalancer_to_SimpleWB(cv::xphoto::WhiteBalancer* instance) {
			return dynamic_cast<cv::xphoto::SimpleWB*>(instance);
	}
	
	cv::Algorithm* cv_xphoto_WhiteBalancer_to_Algorithm(cv::xphoto::WhiteBalancer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_xphoto_WhiteBalancer_delete(cv::xphoto::WhiteBalancer* instance) {
			delete instance;
	}
	
}
