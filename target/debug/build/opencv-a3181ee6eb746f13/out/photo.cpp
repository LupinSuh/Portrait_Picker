#include "photo.hpp"
#include "photo_types.hpp"

extern "C" {
	void cv_cuda_fastNlMeansDenoisingColored_const_GpuMatR_GpuMatR_float_float(const cv::cuda::GpuMat* src, cv::cuda::GpuMat* dst, float h_luminance, float photo_render, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::fastNlMeansDenoisingColored(*src, *dst, h_luminance, photo_render);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_cuda_fastNlMeansDenoisingColored_const_GpuMatR_GpuMatR_float_float_int_int_StreamR(const cv::cuda::GpuMat* src, cv::cuda::GpuMat* dst, float h_luminance, float photo_render, int search_window, int block_size, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::fastNlMeansDenoisingColored(*src, *dst, h_luminance, photo_render, search_window, block_size, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_cuda_fastNlMeansDenoising_const_GpuMatR_GpuMatR_float(const cv::cuda::GpuMat* src, cv::cuda::GpuMat* dst, float h, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::fastNlMeansDenoising(*src, *dst, h);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_cuda_fastNlMeansDenoising_const_GpuMatR_GpuMatR_float_int_int_StreamR(const cv::cuda::GpuMat* src, cv::cuda::GpuMat* dst, float h, int search_window, int block_size, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::fastNlMeansDenoising(*src, *dst, h, search_window, block_size, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_cuda_nonLocalMeans_const_GpuMatR_GpuMatR_float(const cv::cuda::GpuMat* src, cv::cuda::GpuMat* dst, float h, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::nonLocalMeans(*src, *dst, h);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_cuda_nonLocalMeans_const_GpuMatR_GpuMatR_float_int_int_int_StreamR(const cv::cuda::GpuMat* src, cv::cuda::GpuMat* dst, float h, int search_window, int block_size, int borderMode, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::nonLocalMeans(*src, *dst, h, search_window, block_size, borderMode, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_AlignExposures_process_const__InputArrayR_vectorLMatGR_const__InputArrayR_const__InputArrayR(cv::AlignExposures* instance, const cv::_InputArray* src, std::vector<cv::Mat>* dst, const cv::_InputArray* times, const cv::_InputArray* response, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst, *times, *response);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::AlignMTB* cv_AlignExposures_to_AlignMTB(cv::AlignExposures* instance) {
			return dynamic_cast<cv::AlignMTB*>(instance);
	}
	
	cv::Algorithm* cv_AlignExposures_to_Algorithm(cv::AlignExposures* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_AlignExposures_delete(cv::AlignExposures* instance) {
			delete instance;
	}
	
	void cv_AlignMTB_process_const__InputArrayR_vectorLMatGR_const__InputArrayR_const__InputArrayR(cv::AlignMTB* instance, const cv::_InputArray* src, std::vector<cv::Mat>* dst, const cv::_InputArray* times, const cv::_InputArray* response, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst, *times, *response);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_AlignMTB_process_const__InputArrayR_vectorLMatGR(cv::AlignMTB* instance, const cv::_InputArray* src, std::vector<cv::Mat>* dst, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_AlignMTB_calculateShift_const__InputArrayR_const__InputArrayR(cv::AlignMTB* instance, const cv::_InputArray* img0, const cv::_InputArray* img1, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->calculateShift(*img0, *img1);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_AlignMTB_shiftMat_const__InputArrayR_const__OutputArrayR_const_Point(cv::AlignMTB* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Point* shift, ResultVoid* ocvrs_return) {
		try {
			instance->shiftMat(*src, *dst, *shift);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_AlignMTB_computeBitmaps_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::AlignMTB* instance, const cv::_InputArray* img, const cv::_OutputArray* tb, const cv::_OutputArray* eb, ResultVoid* ocvrs_return) {
		try {
			instance->computeBitmaps(*img, *tb, *eb);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_AlignMTB_getMaxBits_const(const cv::AlignMTB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxBits();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_AlignMTB_setMaxBits_int(cv::AlignMTB* instance, int max_bits, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxBits(max_bits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_AlignMTB_getExcludeRange_const(const cv::AlignMTB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getExcludeRange();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_AlignMTB_setExcludeRange_int(cv::AlignMTB* instance, int exclude_range, ResultVoid* ocvrs_return) {
		try {
			instance->setExcludeRange(exclude_range);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_AlignMTB_getCut_const(const cv::AlignMTB* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getCut();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_AlignMTB_setCut_bool(cv::AlignMTB* instance, bool value, ResultVoid* ocvrs_return) {
		try {
			instance->setCut(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_AlignMTB_to_Algorithm(cv::AlignMTB* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::AlignExposures* cv_AlignMTB_to_AlignExposures(cv::AlignMTB* instance) {
			return dynamic_cast<cv::AlignExposures*>(instance);
	}
	
	void cv_AlignMTB_delete(cv::AlignMTB* instance) {
			delete instance;
	}
	
	void cv_CalibrateCRF_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR(cv::CalibrateCRF* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst, *times);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::CalibrateDebevec* cv_CalibrateCRF_to_CalibrateDebevec(cv::CalibrateCRF* instance) {
			return dynamic_cast<cv::CalibrateDebevec*>(instance);
	}
	
	cv::CalibrateRobertson* cv_CalibrateCRF_to_CalibrateRobertson(cv::CalibrateCRF* instance) {
			return dynamic_cast<cv::CalibrateRobertson*>(instance);
	}
	
	cv::Algorithm* cv_CalibrateCRF_to_Algorithm(cv::CalibrateCRF* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_CalibrateCRF_delete(cv::CalibrateCRF* instance) {
			delete instance;
	}
	
	void cv_CalibrateDebevec_getLambda_const(const cv::CalibrateDebevec* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_CalibrateDebevec_setLambda_float(cv::CalibrateDebevec* instance, float lambda, ResultVoid* ocvrs_return) {
		try {
			instance->setLambda(lambda);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_CalibrateDebevec_getSamples_const(const cv::CalibrateDebevec* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSamples();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_CalibrateDebevec_setSamples_int(cv::CalibrateDebevec* instance, int samples, ResultVoid* ocvrs_return) {
		try {
			instance->setSamples(samples);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_CalibrateDebevec_getRandom_const(const cv::CalibrateDebevec* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getRandom();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_CalibrateDebevec_setRandom_bool(cv::CalibrateDebevec* instance, bool random, ResultVoid* ocvrs_return) {
		try {
			instance->setRandom(random);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_CalibrateDebevec_to_Algorithm(cv::CalibrateDebevec* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::CalibrateCRF* cv_CalibrateDebevec_to_CalibrateCRF(cv::CalibrateDebevec* instance) {
			return dynamic_cast<cv::CalibrateCRF*>(instance);
	}
	
	void cv_CalibrateDebevec_delete(cv::CalibrateDebevec* instance) {
			delete instance;
	}
	
	void cv_CalibrateRobertson_getMaxIter_const(const cv::CalibrateRobertson* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxIter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_CalibrateRobertson_setMaxIter_int(cv::CalibrateRobertson* instance, int max_iter, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxIter(max_iter);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_CalibrateRobertson_getThreshold_const(const cv::CalibrateRobertson* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_CalibrateRobertson_setThreshold_float(cv::CalibrateRobertson* instance, float threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_CalibrateRobertson_getRadiance_const(const cv::CalibrateRobertson* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getRadiance();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_CalibrateRobertson_to_Algorithm(cv::CalibrateRobertson* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::CalibrateCRF* cv_CalibrateRobertson_to_CalibrateCRF(cv::CalibrateRobertson* instance) {
			return dynamic_cast<cv::CalibrateCRF*>(instance);
	}
	
	void cv_CalibrateRobertson_delete(cv::CalibrateRobertson* instance) {
			delete instance;
	}
	
	void cv_MergeDebevec_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(cv::MergeDebevec* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times, const cv::_InputArray* response, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst, *times, *response);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_MergeDebevec_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR(cv::MergeDebevec* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst, *times);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_MergeDebevec_to_Algorithm(cv::MergeDebevec* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::MergeExposures* cv_MergeDebevec_to_MergeExposures(cv::MergeDebevec* instance) {
			return dynamic_cast<cv::MergeExposures*>(instance);
	}
	
	void cv_MergeDebevec_delete(cv::MergeDebevec* instance) {
			delete instance;
	}
	
	void cv_MergeExposures_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(cv::MergeExposures* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times, const cv::_InputArray* response, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst, *times, *response);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::MergeDebevec* cv_MergeExposures_to_MergeDebevec(cv::MergeExposures* instance) {
			return dynamic_cast<cv::MergeDebevec*>(instance);
	}
	
	cv::MergeMertens* cv_MergeExposures_to_MergeMertens(cv::MergeExposures* instance) {
			return dynamic_cast<cv::MergeMertens*>(instance);
	}
	
	cv::MergeRobertson* cv_MergeExposures_to_MergeRobertson(cv::MergeExposures* instance) {
			return dynamic_cast<cv::MergeRobertson*>(instance);
	}
	
	cv::Algorithm* cv_MergeExposures_to_Algorithm(cv::MergeExposures* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_MergeExposures_delete(cv::MergeExposures* instance) {
			delete instance;
	}
	
	void cv_MergeMertens_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(cv::MergeMertens* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times, const cv::_InputArray* response, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst, *times, *response);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_MergeMertens_process_const__InputArrayR_const__OutputArrayR(cv::MergeMertens* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_MergeMertens_getContrastWeight_const(const cv::MergeMertens* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getContrastWeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_MergeMertens_setContrastWeight_float(cv::MergeMertens* instance, float contrast_weiht, ResultVoid* ocvrs_return) {
		try {
			instance->setContrastWeight(contrast_weiht);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_MergeMertens_getSaturationWeight_const(const cv::MergeMertens* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSaturationWeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_MergeMertens_setSaturationWeight_float(cv::MergeMertens* instance, float saturation_weight, ResultVoid* ocvrs_return) {
		try {
			instance->setSaturationWeight(saturation_weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_MergeMertens_getExposureWeight_const(const cv::MergeMertens* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getExposureWeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_MergeMertens_setExposureWeight_float(cv::MergeMertens* instance, float exposure_weight, ResultVoid* ocvrs_return) {
		try {
			instance->setExposureWeight(exposure_weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_MergeMertens_to_Algorithm(cv::MergeMertens* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::MergeExposures* cv_MergeMertens_to_MergeExposures(cv::MergeMertens* instance) {
			return dynamic_cast<cv::MergeExposures*>(instance);
	}
	
	void cv_MergeMertens_delete(cv::MergeMertens* instance) {
			delete instance;
	}
	
	void cv_MergeRobertson_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(cv::MergeRobertson* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times, const cv::_InputArray* response, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst, *times, *response);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_MergeRobertson_process_const__InputArrayR_const__OutputArrayR_const__InputArrayR(cv::MergeRobertson* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* times, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst, *times);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_MergeRobertson_to_Algorithm(cv::MergeRobertson* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::MergeExposures* cv_MergeRobertson_to_MergeExposures(cv::MergeRobertson* instance) {
			return dynamic_cast<cv::MergeExposures*>(instance);
	}
	
	void cv_MergeRobertson_delete(cv::MergeRobertson* instance) {
			delete instance;
	}
	
	void cv_Tonemap_process_const__InputArrayR_const__OutputArrayR(cv::Tonemap* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->process(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Tonemap_getGamma_const(const cv::Tonemap* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Tonemap_setGamma_float(cv::Tonemap* instance, float gamma, ResultVoid* ocvrs_return) {
		try {
			instance->setGamma(gamma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::TonemapDrago* cv_Tonemap_to_TonemapDrago(cv::Tonemap* instance) {
			return dynamic_cast<cv::TonemapDrago*>(instance);
	}
	
	cv::TonemapMantiuk* cv_Tonemap_to_TonemapMantiuk(cv::Tonemap* instance) {
			return dynamic_cast<cv::TonemapMantiuk*>(instance);
	}
	
	cv::TonemapReinhard* cv_Tonemap_to_TonemapReinhard(cv::Tonemap* instance) {
			return dynamic_cast<cv::TonemapReinhard*>(instance);
	}
	
	cv::Algorithm* cv_Tonemap_to_Algorithm(cv::Tonemap* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_Tonemap_delete(cv::Tonemap* instance) {
			delete instance;
	}
	
	void cv_TonemapDrago_getSaturation_const(const cv::TonemapDrago* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSaturation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_TonemapDrago_setSaturation_float(cv::TonemapDrago* instance, float saturation, ResultVoid* ocvrs_return) {
		try {
			instance->setSaturation(saturation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_TonemapDrago_getBias_const(const cv::TonemapDrago* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getBias();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_TonemapDrago_setBias_float(cv::TonemapDrago* instance, float bias, ResultVoid* ocvrs_return) {
		try {
			instance->setBias(bias);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_TonemapDrago_to_Algorithm(cv::TonemapDrago* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::Tonemap* cv_TonemapDrago_to_Tonemap(cv::TonemapDrago* instance) {
			return dynamic_cast<cv::Tonemap*>(instance);
	}
	
	void cv_TonemapDrago_delete(cv::TonemapDrago* instance) {
			delete instance;
	}
	
	void cv_TonemapMantiuk_getScale_const(const cv::TonemapMantiuk* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_TonemapMantiuk_setScale_float(cv::TonemapMantiuk* instance, float scale, ResultVoid* ocvrs_return) {
		try {
			instance->setScale(scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_TonemapMantiuk_getSaturation_const(const cv::TonemapMantiuk* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSaturation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_TonemapMantiuk_setSaturation_float(cv::TonemapMantiuk* instance, float saturation, ResultVoid* ocvrs_return) {
		try {
			instance->setSaturation(saturation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_TonemapMantiuk_to_Algorithm(cv::TonemapMantiuk* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::Tonemap* cv_TonemapMantiuk_to_Tonemap(cv::TonemapMantiuk* instance) {
			return dynamic_cast<cv::Tonemap*>(instance);
	}
	
	void cv_TonemapMantiuk_delete(cv::TonemapMantiuk* instance) {
			delete instance;
	}
	
	void cv_TonemapReinhard_getIntensity_const(const cv::TonemapReinhard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getIntensity();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_TonemapReinhard_setIntensity_float(cv::TonemapReinhard* instance, float intensity, ResultVoid* ocvrs_return) {
		try {
			instance->setIntensity(intensity);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_TonemapReinhard_getLightAdaptation_const(const cv::TonemapReinhard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getLightAdaptation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_TonemapReinhard_setLightAdaptation_float(cv::TonemapReinhard* instance, float light_adapt, ResultVoid* ocvrs_return) {
		try {
			instance->setLightAdaptation(light_adapt);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_TonemapReinhard_getColorAdaptation_const(const cv::TonemapReinhard* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getColorAdaptation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_TonemapReinhard_setColorAdaptation_float(cv::TonemapReinhard* instance, float color_adapt, ResultVoid* ocvrs_return) {
		try {
			instance->setColorAdaptation(color_adapt);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_TonemapReinhard_to_Algorithm(cv::TonemapReinhard* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::Tonemap* cv_TonemapReinhard_to_Tonemap(cv::TonemapReinhard* instance) {
			return dynamic_cast<cv::Tonemap*>(instance);
	}
	
	void cv_TonemapReinhard_delete(cv::TonemapReinhard* instance) {
			delete instance;
	}
	
}
