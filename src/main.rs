use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use opencv::{
    prelude::*,
    core,
    imgcodecs,
    
    dnn, // Add dnn
};
use indicatif::{ProgressBar, ProgressStyle};
use walkdir::WalkDir;

use tokio;

// DNN 모델 파일 경로 및 설정
const DNN_MODEL_PROTOTXT: &str = "deploy.prototxt";
const DNN_MODEL_CAFFEMODEL: &str = "res10_300x300_ssd_iter_140000.caffemodel";
const DNN_CONFIDENCE_THRESHOLD: f32 = 0.5; // 얼굴 검출 최소 신뢰도

// 작업 결과를 나타내는 열거형
enum Classification {
    Pass,
    Fail(FailReason),
}

// 실패 원인을 나타내는 열거형
#[derive(Clone, Copy)]
enum FailReason {
    NotAnImage,
    MultiFace,
    NoFace,
    LowResolution,
}

impl FailReason {
    fn get_prefix(&self) -> &'static str {
        match self {
            FailReason::NotAnImage => "NotImg_",
            FailReason::MultiFace => "MultiFace_",
            FailReason::NoFace => "NoFace_",
            FailReason::LowResolution => "LowRes_",
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 명령줄 인자에서 경로를 가져옵니다。
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("사용법: {} <이미지 폴더 경로>", args[0]);
        return Ok(());
    }
    let image_dir = Path::new(&args[1]);

    // DNN 모델 파일 경로
    let project_root = env::current_dir()?; // Get the current working directory (project root)
    let models_dir = project_root.join("models");
    fs::create_dir_all(&models_dir)?; // Ensure models directory exists

    let prototxt_path = models_dir.join(DNN_MODEL_PROTOTXT);
    let caffemodel_path = models_dir.join(DNN_MODEL_CAFFEMODEL);

    // 모델 파일이 없으면 사용자에게 알립니다.
    if !prototxt_path.exists() || !caffemodel_path.exists() {
        println!("DNN 모델 파일이 없습니다. 'models' 디렉토리에 '{}'와 '{}' 파일을 넣어주세요.",
                 DNN_MODEL_PROTOTXT, DNN_MODEL_CAFFEMODEL);
        return Ok(());
    }

    // OpenCV 파라미터 설정 (DNN에 맞게 조정)
    let min_size_width = 300; // DNN 모델의 입력 크기에 맞춤
    let min_size_height = 300; // DNN 모델의 입력 크기에 맞춤
    let max_size_width = 1536; // 이미지 크기 제한은 유지
    let max_size_height = 1536; // 이미지 크기 제한은 유지

    // 이미지 자체의 최소 크기 제한
    let image_min_width = 512;
    let image_min_height = 512;

    // DNN 모델을 로드합니다.
    let mut net = dnn::read_net_from_caffe(
        prototxt_path.to_str().ok_or("Invalid prototxt path")?,
        caffemodel_path.to_str().ok_or("Invalid caffemodel path")?,
    )?;
    if net.empty()? {
        panic!("Error: DNN model not loaded. Check the paths: {} and {}",
                prototxt_path.display(), caffemodel_path.display());
    }
    

    // Set preferred backend and target
    net.set_preferable_backend(dnn::DNN_BACKEND_OPENCV)?;
    net.set_preferable_target(dnn::DNN_TARGET_CPU)?;

    let mut entries: Vec<PathBuf> = WalkDir::new(image_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            if !path.is_file() { return false; }
            if let Ok(relative_path) = path.strip_prefix(image_dir) {
                if relative_path.components().any(|comp| 
                    matches!(comp, std::path::Component::Normal(name) if name == "Pass" || name == "Fail")) {
                    return false;
                }
            }
            true
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    entries.sort();

    let bar = ProgressBar::new(entries.len() as u64);
    bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}/{eta_precise}] [{bar:40.cyan/blue}] {pos}/{len}\n{msg}")?);

    let mut pass_count = 0;
    let mut no_face_count = 0;
    let mut multi_face_count = 0;
    let mut low_res_count = 0;
    let mut not_image_count = 0;

    for path in &entries {
        let mut current_dir_display = "";
        if let Some(parent) = path.parent() {
            current_dir_display = parent.to_str().unwrap_or("");
        }

        bar.set_message(format!(
            "  Params (Min: {}x{}, Max: {}x{}, ImageMin: {}x{}, Conf: {})
  Scanning: {}
  Pass: {} | Fail (No Face: {}, Multi: {}, LowRes: {}, NotImg: {})", 
            min_size_width, min_size_height, max_size_width, max_size_height,
            image_min_width, image_min_height, DNN_CONFIDENCE_THRESHOLD,
            current_dir_display,
            pass_count, no_face_count, multi_face_count, low_res_count, not_image_count));

        if let Some(parent_dir) = path.parent() {
            let pass_dir = parent_dir.join("Pass");
            let fail_dir = parent_dir.join("Fail");

            fs::create_dir_all(&pass_dir)?;
            fs::create_dir_all(&fail_dir)?;

            let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
            
            let classification = match extension.as_str() {
                "jpg" | "jpeg" | "png" | "gif" | "webp" => {
                    
                    let img = imgcodecs::imread(path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
                    if img.empty() {
                        
                        Classification::Fail(FailReason::NotAnImage)
                    } else {
                        
                        let (width, height) = (img.cols(), img.rows());
                        let good_res = width >= image_min_width && height >= image_min_height;

                        // Convert image to blob
                        
                        let blob = dnn::blob_from_image(
                            &img,
                            1.0, // scale factor (will be applied in set_input)
                            core::Size::new(300, 300), // DNN input size
                            core::Scalar::default(), // No mean subtraction here
                            false, // swap RB
                            false, // crop
                            core::CV_32F, // depth
                        )?;

                        
                        net.set_input(&blob, "", 1.0, core::Scalar::new(104.0, 177.0, 123.0, 0.0))?;
                        // Get output layer names
                        let out_names = net.get_unconnected_out_layers_names()?;
                        

                        let output_layer_name = if out_names.is_empty() {
                            panic!("Error: No output layers found in the DNN model.");
                        } else {
                            out_names.get(0).unwrap()
                        };

                        // Perform forward pass (detections will be populated by get_blob later)
                        let mut output_blobs = core::Vector::<core::Mat>::new();
                        net.forward(&mut output_blobs, &core::Vector::<String>::from_iter(vec![output_layer_name.as_str()]))?;

                        // Get the detections Mat from the output_blobs vector
                        let detections = output_blobs.get(0)?;

                        
                        let mut faces_found = 0;
                        // detections is a 4D matrix: [1, 1, N, 7]
                        // where N is the number of detections, and 7 is [batchId, classId, confidence, left, top, right, bottom]
                        let data = detections.data_typed::<f32>()?;
                        for i in 0..detections.rows() {
                            let confidence = data[(i * 7 + 2) as usize];
                            if confidence > DNN_CONFIDENCE_THRESHOLD {
                                faces_found += 1;
                            }
                        }

                        if faces_found == 1 && good_res {
                            Classification::Pass
                        } else if faces_found == 0 {
                            Classification::Fail(FailReason::NoFace)
                        } else if faces_found > 1 {
                            Classification::Fail(FailReason::MultiFace)
                        } else { 
                            Classification::Fail(FailReason::LowResolution)
                        }
                    }
                }
                _ => Classification::Fail(FailReason::NotAnImage),
            };

            let new_path = match classification {
                Classification::Pass => {
                    pass_count += 1;
                    pass_dir.join(path.file_name().unwrap())
                }
                Classification::Fail(reason) => {
                    match reason {
                        FailReason::NoFace => no_face_count += 1,
                        FailReason::MultiFace => multi_face_count += 1,
                        FailReason::LowResolution => low_res_count += 1,
                        FailReason::NotAnImage => not_image_count += 1,
                    }
                    let prefix = reason.get_prefix();
                    let original_filename = path.file_name().unwrap().to_str().unwrap_or("");
                    let new_filename = format!("{}{}", prefix, original_filename);
                    fail_dir.join(new_filename)
                }
            };

            if !new_path.exists() {
                fs::rename(path, &new_path)?;
            }
        }
        bar.inc(1);
    }
    
    bar.finish_and_clear();
    let total_fails = no_face_count + multi_face_count + low_res_count + not_image_count;
    println!("\n작업이 완료되었습니다.");
    println!("---------------------------");
    println!("  Pass: {}", pass_count);
    println!("  Fail: {} (Total)", total_fails);
    println!("    - No Face:      {}", no_face_count);
    println!("    - Multi-Face:   {}", multi_face_count);
    println!("    - Low Resolution: {}", low_res_count);
    println!("    - Not an Image: {}", not_image_count);
    println!("---------------------------");

    Ok(())
}