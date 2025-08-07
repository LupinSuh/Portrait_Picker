#include "ocvrs_common.hpp"
#include <opencv2/highgui.hpp>
#include "highgui_types.hpp"

extern "C" {
	void* cv_QtFont_propNameFont_const(const cv::QtFont* instance) {
			const char* ret = instance->nameFont;
			return ocvrs_create_string(ret);
	}
	
	void cv_QtFont_propColor_const(const cv::QtFont* instance, cv::Scalar* ocvrs_return) {
			cv::Scalar ret = instance->color;
			*ocvrs_return = ret;
	}
	
	void cv_QtFont_propColor_const_Scalar(cv::QtFont* instance, const cv::Scalar* val) {
			instance->color = *val;
	}
	
	int cv_QtFont_propFont_face_const(const cv::QtFont* instance) {
			int ret = instance->font_face;
			return ret;
	}
	
	void cv_QtFont_propFont_face_const_int(cv::QtFont* instance, const int val) {
			instance->font_face = val;
	}
	
	const int* cv_QtFont_propAscii_const(const cv::QtFont* instance) {
			const int* ret = instance->ascii;
			return ret;
	}
	
	const int* cv_QtFont_propGreek_const(const cv::QtFont* instance) {
			const int* ret = instance->greek;
			return ret;
	}
	
	const int* cv_QtFont_propCyrillic_const(const cv::QtFont* instance) {
			const int* ret = instance->cyrillic;
			return ret;
	}
	
	float cv_QtFont_propHscale_const(const cv::QtFont* instance) {
			float ret = instance->hscale;
			return ret;
	}
	
	void cv_QtFont_propHscale_const_float(cv::QtFont* instance, const float val) {
			instance->hscale = val;
	}
	
	float cv_QtFont_propVscale_const(const cv::QtFont* instance) {
			float ret = instance->vscale;
			return ret;
	}
	
	void cv_QtFont_propVscale_const_float(cv::QtFont* instance, const float val) {
			instance->vscale = val;
	}
	
	float cv_QtFont_propShear_const(const cv::QtFont* instance) {
			float ret = instance->shear;
			return ret;
	}
	
	void cv_QtFont_propShear_const_float(cv::QtFont* instance, const float val) {
			instance->shear = val;
	}
	
	int cv_QtFont_propThickness_const(const cv::QtFont* instance) {
			int ret = instance->thickness;
			return ret;
	}
	
	void cv_QtFont_propThickness_const_int(cv::QtFont* instance, const int val) {
			instance->thickness = val;
	}
	
	float cv_QtFont_propDx_const(const cv::QtFont* instance) {
			float ret = instance->dx;
			return ret;
	}
	
	void cv_QtFont_propDx_const_float(cv::QtFont* instance, const float val) {
			instance->dx = val;
	}
	
	int cv_QtFont_propLine_type_const(const cv::QtFont* instance) {
			int ret = instance->line_type;
			return ret;
	}
	
	void cv_QtFont_propLine_type_const_int(cv::QtFont* instance, const int val) {
			instance->line_type = val;
	}
	
	void cv_QtFont_delete(cv::QtFont* instance) {
			delete instance;
	}
	
}
