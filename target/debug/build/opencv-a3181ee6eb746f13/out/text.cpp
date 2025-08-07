#include "ocvrs_common.hpp"
#include <opencv2/text.hpp>
#include "text_types.hpp"

extern "C" {
	void cv_text_BaseOCR_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::BaseOCR* instance, cv::Mat* image, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_BaseOCR_run_MatR_stringR(cv::text::BaseOCR* instance, cv::Mat* image, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_BaseOCR_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::BaseOCR* instance, cv::Mat* image, cv::Mat* mask, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_BaseOCR_run_MatR_MatR_stringR(cv::text::BaseOCR* instance, cv::Mat* image, cv::Mat* mask, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::text::OCRBeamSearchDecoder* cv_text_BaseOCR_to_OCRBeamSearchDecoder(cv::text::BaseOCR* instance) {
			return dynamic_cast<cv::text::OCRBeamSearchDecoder*>(instance);
	}
	
	cv::text::OCRHMMDecoder* cv_text_BaseOCR_to_OCRHMMDecoder(cv::text::BaseOCR* instance) {
			return dynamic_cast<cv::text::OCRHMMDecoder*>(instance);
	}
	
	cv::text::OCRHolisticWordRecognizer* cv_text_BaseOCR_to_OCRHolisticWordRecognizer(cv::text::BaseOCR* instance) {
			return dynamic_cast<cv::text::OCRHolisticWordRecognizer*>(instance);
	}
	
	cv::text::OCRTesseract* cv_text_BaseOCR_to_OCRTesseract(cv::text::BaseOCR* instance) {
			return dynamic_cast<cv::text::OCRTesseract*>(instance);
	}
	
	void cv_text_BaseOCR_delete(cv::text::BaseOCR* instance) {
			delete instance;
	}
	
	void cv_text_ERFilter_run_const__InputArrayR_vectorLERStatGR(cv::text::ERFilter* instance, const cv::_InputArray* image, std::vector<cv::text::ERStat>* regions, ResultVoid* ocvrs_return) {
		try {
			instance->run(*image, *regions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_ERFilter_setCallback_const_PtrLCallbackGR(cv::text::ERFilter* instance, const cv::Ptr<cv::text::ERFilter::Callback>* cb, ResultVoid* ocvrs_return) {
		try {
			instance->setCallback(*cb);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_ERFilter_setThresholdDelta_int(cv::text::ERFilter* instance, int thresholdDelta, ResultVoid* ocvrs_return) {
		try {
			instance->setThresholdDelta(thresholdDelta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_ERFilter_setMinArea_float(cv::text::ERFilter* instance, float minArea, ResultVoid* ocvrs_return) {
		try {
			instance->setMinArea(minArea);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_ERFilter_setMaxArea_float(cv::text::ERFilter* instance, float maxArea, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxArea(maxArea);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_ERFilter_setMinProbability_float(cv::text::ERFilter* instance, float minProbability, ResultVoid* ocvrs_return) {
		try {
			instance->setMinProbability(minProbability);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_ERFilter_setMinProbabilityDiff_float(cv::text::ERFilter* instance, float minProbabilityDiff, ResultVoid* ocvrs_return) {
		try {
			instance->setMinProbabilityDiff(minProbabilityDiff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_ERFilter_setNonMaxSuppression_bool(cv::text::ERFilter* instance, bool nonMaxSuppression, ResultVoid* ocvrs_return) {
		try {
			instance->setNonMaxSuppression(nonMaxSuppression);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_ERFilter_getNumRejected_const(const cv::text::ERFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumRejected();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_text_ERFilter_to_Algorithm(cv::text::ERFilter* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_text_ERFilter_delete(cv::text::ERFilter* instance) {
			delete instance;
	}
	
	void cv_text_ERFilter_Callback_eval_const_ERStatR(cv::text::ERFilter::Callback* instance, const cv::text::ERStat* stat, Result<double>* ocvrs_return) {
		try {
			double ret = instance->eval(*stat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_ERFilter_Callback_delete(cv::text::ERFilter::Callback* instance) {
			delete instance;
	}
	
	void cv_text_ERStat_ERStat_int_int_int_int(int level, int pixel, int x, int y, Result<cv::text::ERStat*>* ocvrs_return) {
		try {
			cv::text::ERStat* ret = new cv::text::ERStat(level, pixel, x, y);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_ERStat_ERStat(Result<cv::text::ERStat*>* ocvrs_return) {
		try {
			cv::text::ERStat* ret = new cv::text::ERStat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	int cv_text_ERStat_propPixel_const(const cv::text::ERStat* instance) {
			int ret = instance->pixel;
			return ret;
	}
	
	void cv_text_ERStat_propPixel_const_int(cv::text::ERStat* instance, const int val) {
			instance->pixel = val;
	}
	
	int cv_text_ERStat_propLevel_const(const cv::text::ERStat* instance) {
			int ret = instance->level;
			return ret;
	}
	
	void cv_text_ERStat_propLevel_const_int(cv::text::ERStat* instance, const int val) {
			instance->level = val;
	}
	
	int cv_text_ERStat_propArea_const(const cv::text::ERStat* instance) {
			int ret = instance->area;
			return ret;
	}
	
	void cv_text_ERStat_propArea_const_int(cv::text::ERStat* instance, const int val) {
			instance->area = val;
	}
	
	int cv_text_ERStat_propPerimeter_const(const cv::text::ERStat* instance) {
			int ret = instance->perimeter;
			return ret;
	}
	
	void cv_text_ERStat_propPerimeter_const_int(cv::text::ERStat* instance, const int val) {
			instance->perimeter = val;
	}
	
	int cv_text_ERStat_propEuler_const(const cv::text::ERStat* instance) {
			int ret = instance->euler;
			return ret;
	}
	
	void cv_text_ERStat_propEuler_const_int(cv::text::ERStat* instance, const int val) {
			instance->euler = val;
	}
	
	void cv_text_ERStat_propRect_const(const cv::text::ERStat* instance, cv::Rect* ocvrs_return) {
			cv::Rect ret = instance->rect;
			*ocvrs_return = ret;
	}
	
	void cv_text_ERStat_propRect_const_Rect(cv::text::ERStat* instance, const cv::Rect* val) {
			instance->rect = *val;
	}
	
	const double** cv_text_ERStat_propRaw_moments_const(const cv::text::ERStat* instance) {
			const double(*ret)[2] = &instance->raw_moments;
			return (const double**)ret;
	}
	
	double** cv_text_ERStat_propRaw_moments(cv::text::ERStat* instance) {
			double(*ret)[2] = &instance->raw_moments;
			return (double**)ret;
	}
	
	const double** cv_text_ERStat_propCentral_moments_const(const cv::text::ERStat* instance) {
			const double(*ret)[3] = &instance->central_moments;
			return (const double**)ret;
	}
	
	double** cv_text_ERStat_propCentral_moments(cv::text::ERStat* instance) {
			double(*ret)[3] = &instance->central_moments;
			return (double**)ret;
	}
	
	float cv_text_ERStat_propMed_crossings_const(const cv::text::ERStat* instance) {
			float ret = instance->med_crossings;
			return ret;
	}
	
	void cv_text_ERStat_propMed_crossings_const_float(cv::text::ERStat* instance, const float val) {
			instance->med_crossings = val;
	}
	
	float cv_text_ERStat_propHole_area_ratio_const(const cv::text::ERStat* instance) {
			float ret = instance->hole_area_ratio;
			return ret;
	}
	
	void cv_text_ERStat_propHole_area_ratio_const_float(cv::text::ERStat* instance, const float val) {
			instance->hole_area_ratio = val;
	}
	
	float cv_text_ERStat_propConvex_hull_ratio_const(const cv::text::ERStat* instance) {
			float ret = instance->convex_hull_ratio;
			return ret;
	}
	
	void cv_text_ERStat_propConvex_hull_ratio_const_float(cv::text::ERStat* instance, const float val) {
			instance->convex_hull_ratio = val;
	}
	
	float cv_text_ERStat_propNum_inflexion_points_const(const cv::text::ERStat* instance) {
			float ret = instance->num_inflexion_points;
			return ret;
	}
	
	void cv_text_ERStat_propNum_inflexion_points_const_float(cv::text::ERStat* instance, const float val) {
			instance->num_inflexion_points = val;
	}
	
	double cv_text_ERStat_propProbability_const(const cv::text::ERStat* instance) {
			double ret = instance->probability;
			return ret;
	}
	
	void cv_text_ERStat_propProbability_const_double(cv::text::ERStat* instance, const double val) {
			instance->probability = val;
	}
	
	cv::text::ERStat** cv_text_ERStat_propParent(cv::text::ERStat* instance) {
			cv::text::ERStat* ret = instance->parent;
			return new cv::text::ERStat*(ret);
	}
	
	void cv_text_ERStat_propParent_ERStatX(cv::text::ERStat* instance, cv::text::ERStat* const val) {
			instance->parent = val;
	}
	
	cv::text::ERStat** cv_text_ERStat_propChild(cv::text::ERStat* instance) {
			cv::text::ERStat* ret = instance->child;
			return new cv::text::ERStat*(ret);
	}
	
	void cv_text_ERStat_propChild_ERStatX(cv::text::ERStat* instance, cv::text::ERStat* const val) {
			instance->child = val;
	}
	
	cv::text::ERStat** cv_text_ERStat_propNext(cv::text::ERStat* instance) {
			cv::text::ERStat* ret = instance->next;
			return new cv::text::ERStat*(ret);
	}
	
	void cv_text_ERStat_propNext_ERStatX(cv::text::ERStat* instance, cv::text::ERStat* const val) {
			instance->next = val;
	}
	
	cv::text::ERStat** cv_text_ERStat_propPrev(cv::text::ERStat* instance) {
			cv::text::ERStat* ret = instance->prev;
			return new cv::text::ERStat*(ret);
	}
	
	void cv_text_ERStat_propPrev_ERStatX(cv::text::ERStat* instance, cv::text::ERStat* const val) {
			instance->prev = val;
	}
	
	bool cv_text_ERStat_propLocal_maxima_const(const cv::text::ERStat* instance) {
			bool ret = instance->local_maxima;
			return ret;
	}
	
	void cv_text_ERStat_propLocal_maxima_const_bool(cv::text::ERStat* instance, const bool val) {
			instance->local_maxima = val;
	}
	
	cv::text::ERStat** cv_text_ERStat_propMax_probability_ancestor(cv::text::ERStat* instance) {
			cv::text::ERStat* ret = instance->max_probability_ancestor;
			return new cv::text::ERStat*(ret);
	}
	
	void cv_text_ERStat_propMax_probability_ancestor_ERStatX(cv::text::ERStat* instance, cv::text::ERStat* const val) {
			instance->max_probability_ancestor = val;
	}
	
	cv::text::ERStat** cv_text_ERStat_propMin_probability_ancestor(cv::text::ERStat* instance) {
			cv::text::ERStat* ret = instance->min_probability_ancestor;
			return new cv::text::ERStat*(ret);
	}
	
	void cv_text_ERStat_propMin_probability_ancestor_ERStatX(cv::text::ERStat* instance, cv::text::ERStat* const val) {
			instance->min_probability_ancestor = val;
	}
	
	void cv_text_ERStat_delete(cv::text::ERStat* instance) {
			delete instance;
	}
	
	void cv_text_OCRBeamSearchDecoder_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::OCRBeamSearchDecoder* instance, cv::Mat* image, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRBeamSearchDecoder_run_MatR_stringR(cv::text::OCRBeamSearchDecoder* instance, cv::Mat* image, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRBeamSearchDecoder_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::OCRBeamSearchDecoder* instance, cv::Mat* image, cv::Mat* mask, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRBeamSearchDecoder_run_MatR_MatR_stringR(cv::text::OCRBeamSearchDecoder* instance, cv::Mat* image, cv::Mat* mask, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_int_int(cv::text::OCRBeamSearchDecoder* instance, const cv::_InputArray* image, int min_confidence, int component_level, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, min_confidence, component_level);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_int(cv::text::OCRBeamSearchDecoder* instance, const cv::_InputArray* image, int min_confidence, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, min_confidence);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_const__InputArrayR_int_int(cv::text::OCRBeamSearchDecoder* instance, const cv::_InputArray* image, const cv::_InputArray* mask, int min_confidence, int component_level, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, *mask, min_confidence, component_level);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_const__InputArrayR_int(cv::text::OCRBeamSearchDecoder* instance, const cv::_InputArray* image, const cv::_InputArray* mask, int min_confidence, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, *mask, min_confidence);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRBeamSearchDecoder_create_const_PtrLClassifierCallbackG_const_stringR_const__InputArrayR_const__InputArrayR_decoder_mode_int(const cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>* classifier, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, cv::text::decoder_mode mode, int beam_size, Result<cv::Ptr<cv::text::OCRBeamSearchDecoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRBeamSearchDecoder> ret = cv::text::OCRBeamSearchDecoder::create(*classifier, std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table, mode, beam_size);
			Ok(new cv::Ptr<cv::text::OCRBeamSearchDecoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRBeamSearchDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR_decoder_mode_int(const char* filename, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, cv::text::decoder_mode mode, int beam_size, Result<cv::Ptr<cv::text::OCRBeamSearchDecoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRBeamSearchDecoder> ret = cv::text::OCRBeamSearchDecoder::create(std::string(filename), std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table, mode, beam_size);
			Ok(new cv::Ptr<cv::text::OCRBeamSearchDecoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRBeamSearchDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR(const char* filename, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, Result<cv::Ptr<cv::text::OCRBeamSearchDecoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRBeamSearchDecoder> ret = cv::text::OCRBeamSearchDecoder::create(std::string(filename), std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table);
			Ok(new cv::Ptr<cv::text::OCRBeamSearchDecoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::text::BaseOCR* cv_text_OCRBeamSearchDecoder_to_BaseOCR(cv::text::OCRBeamSearchDecoder* instance) {
			return dynamic_cast<cv::text::BaseOCR*>(instance);
	}
	
	void cv_text_OCRBeamSearchDecoder_delete(cv::text::OCRBeamSearchDecoder* instance) {
			delete instance;
	}
	
	void cv_text_OCRBeamSearchDecoder_ClassifierCallback_eval_const__InputArrayR_vectorLvectorLdoubleGGR_vectorLintGR(cv::text::OCRBeamSearchDecoder::ClassifierCallback* instance, const cv::_InputArray* image, std::vector<std::vector<double>>* recognition_probabilities, std::vector<int>* oversegmentation, ResultVoid* ocvrs_return) {
		try {
			instance->eval(*image, *recognition_probabilities, *oversegmentation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRBeamSearchDecoder_ClassifierCallback_getWindowSize(cv::text::OCRBeamSearchDecoder::ClassifierCallback* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRBeamSearchDecoder_ClassifierCallback_getStepSize(cv::text::OCRBeamSearchDecoder::ClassifierCallback* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getStepSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRBeamSearchDecoder_ClassifierCallback_delete(cv::text::OCRBeamSearchDecoder::ClassifierCallback* instance) {
			delete instance;
	}
	
	void cv_text_OCRHMMDecoder_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::OCRHMMDecoder* instance, cv::Mat* image, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRHMMDecoder_run_MatR_stringR(cv::text::OCRHMMDecoder* instance, cv::Mat* image, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRHMMDecoder_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::OCRHMMDecoder* instance, cv::Mat* image, cv::Mat* mask, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRHMMDecoder_run_MatR_MatR_stringR(cv::text::OCRHMMDecoder* instance, cv::Mat* image, cv::Mat* mask, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRHMMDecoder_run_const__InputArrayR_int_int(cv::text::OCRHMMDecoder* instance, const cv::_InputArray* image, int min_confidence, int component_level, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, min_confidence, component_level);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRHMMDecoder_run_const__InputArrayR_int(cv::text::OCRHMMDecoder* instance, const cv::_InputArray* image, int min_confidence, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, min_confidence);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRHMMDecoder_run_const__InputArrayR_const__InputArrayR_int_int(cv::text::OCRHMMDecoder* instance, const cv::_InputArray* image, const cv::_InputArray* mask, int min_confidence, int component_level, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, *mask, min_confidence, component_level);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRHMMDecoder_run_const__InputArrayR_const__InputArrayR_int(cv::text::OCRHMMDecoder* instance, const cv::_InputArray* image, const cv::_InputArray* mask, int min_confidence, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, *mask, min_confidence);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRHMMDecoder_create_const_PtrLClassifierCallbackG_const_StringR_const__InputArrayR_const__InputArrayR_int(const cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>* classifier, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, int mode, Result<cv::Ptr<cv::text::OCRHMMDecoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder> ret = cv::text::OCRHMMDecoder::create(*classifier, std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table, mode);
			Ok(new cv::Ptr<cv::text::OCRHMMDecoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRHMMDecoder_create_const_PtrLClassifierCallbackG_const_StringR_const__InputArrayR_const__InputArrayR(const cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>* classifier, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, Result<cv::Ptr<cv::text::OCRHMMDecoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder> ret = cv::text::OCRHMMDecoder::create(*classifier, std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table);
			Ok(new cv::Ptr<cv::text::OCRHMMDecoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRHMMDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR_int_int(const char* filename, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, int mode, int classifier, Result<cv::Ptr<cv::text::OCRHMMDecoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder> ret = cv::text::OCRHMMDecoder::create(std::string(filename), std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table, mode, classifier);
			Ok(new cv::Ptr<cv::text::OCRHMMDecoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRHMMDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR(const char* filename, const char* vocabulary, const cv::_InputArray* transition_probabilities_table, const cv::_InputArray* emission_probabilities_table, Result<cv::Ptr<cv::text::OCRHMMDecoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRHMMDecoder> ret = cv::text::OCRHMMDecoder::create(std::string(filename), std::string(vocabulary), *transition_probabilities_table, *emission_probabilities_table);
			Ok(new cv::Ptr<cv::text::OCRHMMDecoder>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::text::BaseOCR* cv_text_OCRHMMDecoder_to_BaseOCR(cv::text::OCRHMMDecoder* instance) {
			return dynamic_cast<cv::text::BaseOCR*>(instance);
	}
	
	void cv_text_OCRHMMDecoder_delete(cv::text::OCRHMMDecoder* instance) {
			delete instance;
	}
	
	void cv_text_OCRHMMDecoder_ClassifierCallback_eval_const__InputArrayR_vectorLintGR_vectorLdoubleGR(cv::text::OCRHMMDecoder::ClassifierCallback* instance, const cv::_InputArray* image, std::vector<int>* out_class, std::vector<double>* out_confidence, ResultVoid* ocvrs_return) {
		try {
			instance->eval(*image, *out_class, *out_confidence);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRHMMDecoder_ClassifierCallback_delete(cv::text::OCRHMMDecoder::ClassifierCallback* instance) {
			delete instance;
	}
	
	void cv_text_OCRHolisticWordRecognizer_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::OCRHolisticWordRecognizer* instance, cv::Mat* image, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRHolisticWordRecognizer_run_MatR_stringR(cv::text::OCRHolisticWordRecognizer* instance, cv::Mat* image, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRHolisticWordRecognizer_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::OCRHolisticWordRecognizer* instance, cv::Mat* image, cv::Mat* mask, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRHolisticWordRecognizer_run_MatR_MatR_stringR(cv::text::OCRHolisticWordRecognizer* instance, cv::Mat* image, cv::Mat* mask, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRHolisticWordRecognizer_create_const_stringR_const_stringR_const_stringR(const char* archFilename, const char* weightsFilename, const char* wordsFilename, Result<cv::Ptr<cv::text::OCRHolisticWordRecognizer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRHolisticWordRecognizer> ret = cv::text::OCRHolisticWordRecognizer::create(std::string(archFilename), std::string(weightsFilename), std::string(wordsFilename));
			Ok(new cv::Ptr<cv::text::OCRHolisticWordRecognizer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::text::BaseOCR* cv_text_OCRHolisticWordRecognizer_to_BaseOCR(cv::text::OCRHolisticWordRecognizer* instance) {
			return dynamic_cast<cv::text::BaseOCR*>(instance);
	}
	
	void cv_text_OCRHolisticWordRecognizer_delete(cv::text::OCRHolisticWordRecognizer* instance) {
			delete instance;
	}
	
	void cv_text_OCRTesseract_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::OCRTesseract* instance, cv::Mat* image, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRTesseract_run_MatR_stringR(cv::text::OCRTesseract* instance, cv::Mat* image, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRTesseract_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(cv::text::OCRTesseract* instance, cv::Mat* image, cv::Mat* mask, void** output_text, std::vector<cv::Rect>* component_rects, std::vector<std::string>* component_texts, std::vector<float>* component_confidences, int component_level, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out, component_rects, component_texts, component_confidences, component_level);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRTesseract_run_MatR_MatR_stringR(cv::text::OCRTesseract* instance, cv::Mat* image, cv::Mat* mask, void** output_text, ResultVoid* ocvrs_return) {
		try {
			std::string output_text_out;
			instance->run(*image, *mask, output_text_out);
			*output_text = ocvrs_create_string(output_text_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRTesseract_run_const__InputArrayR_int_int(cv::text::OCRTesseract* instance, const cv::_InputArray* image, int min_confidence, int component_level, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, min_confidence, component_level);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRTesseract_run_const__InputArrayR_int(cv::text::OCRTesseract* instance, const cv::_InputArray* image, int min_confidence, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, min_confidence);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRTesseract_run_const__InputArrayR_const__InputArrayR_int_int(cv::text::OCRTesseract* instance, const cv::_InputArray* image, const cv::_InputArray* mask, int min_confidence, int component_level, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, *mask, min_confidence, component_level);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRTesseract_run_const__InputArrayR_const__InputArrayR_int(cv::text::OCRTesseract* instance, const cv::_InputArray* image, const cv::_InputArray* mask, int min_confidence, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->run(*image, *mask, min_confidence);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRTesseract_setWhiteList_const_StringR(cv::text::OCRTesseract* instance, const char* char_whitelist, ResultVoid* ocvrs_return) {
		try {
			instance->setWhiteList(std::string(char_whitelist));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRTesseract_create_const_charX_const_charX_const_charX_int_int(const char* datapath, const char* language, const char* char_whitelist, int oem, int psmode, Result<cv::Ptr<cv::text::OCRTesseract>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRTesseract> ret = cv::text::OCRTesseract::create(datapath, language, char_whitelist, oem, psmode);
			Ok(new cv::Ptr<cv::text::OCRTesseract>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_OCRTesseract_create(Result<cv::Ptr<cv::text::OCRTesseract>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::OCRTesseract> ret = cv::text::OCRTesseract::create();
			Ok(new cv::Ptr<cv::text::OCRTesseract>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::text::BaseOCR* cv_text_OCRTesseract_to_BaseOCR(cv::text::OCRTesseract* instance) {
			return dynamic_cast<cv::text::BaseOCR*>(instance);
	}
	
	void cv_text_OCRTesseract_delete(cv::text::OCRTesseract* instance) {
			delete instance;
	}
	
	void cv_text_TextDetector_detect_const__InputArrayR_vectorLRectGR_vectorLfloatGR(cv::text::TextDetector* instance, const cv::_InputArray* inputImage, std::vector<cv::Rect>* Bbox, std::vector<float>* confidence, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*inputImage, *Bbox, *confidence);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::text::TextDetectorCNN* cv_text_TextDetector_to_TextDetectorCNN(cv::text::TextDetector* instance) {
			return dynamic_cast<cv::text::TextDetectorCNN*>(instance);
	}
	
	void cv_text_TextDetector_delete(cv::text::TextDetector* instance) {
			delete instance;
	}
	
	void cv_text_TextDetectorCNN_detect_const__InputArrayR_vectorLRectGR_vectorLfloatGR(cv::text::TextDetectorCNN* instance, const cv::_InputArray* inputImage, std::vector<cv::Rect>* Bbox, std::vector<float>* confidence, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*inputImage, *Bbox, *confidence);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_TextDetectorCNN_create_const_StringR_const_StringR_vectorLSizeG(const char* modelArchFilename, const char* modelWeightsFilename, std::vector<cv::Size>* detectionSizes, Result<cv::Ptr<cv::text::TextDetectorCNN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::TextDetectorCNN> ret = cv::text::TextDetectorCNN::create(std::string(modelArchFilename), std::string(modelWeightsFilename), *detectionSizes);
			Ok(new cv::Ptr<cv::text::TextDetectorCNN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_text_TextDetectorCNN_create_const_StringR_const_StringR(const char* modelArchFilename, const char* modelWeightsFilename, Result<cv::Ptr<cv::text::TextDetectorCNN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::text::TextDetectorCNN> ret = cv::text::TextDetectorCNN::create(std::string(modelArchFilename), std::string(modelWeightsFilename));
			Ok(new cv::Ptr<cv::text::TextDetectorCNN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::text::TextDetector* cv_text_TextDetectorCNN_to_TextDetector(cv::text::TextDetectorCNN* instance) {
			return dynamic_cast<cv::text::TextDetector*>(instance);
	}
	
	void cv_text_TextDetectorCNN_delete(cv::text::TextDetectorCNN* instance) {
			delete instance;
	}
	
}
