use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use opencv::{
    prelude::*,
    core,
    imgcodecs,
    objdetect,
};
use indicatif::{ProgressBar, ProgressStyle};
use walkdir::WalkDir;
use reqwest;
use tokio;

const CASCADE_DOWNLOAD_URL: &str = "https://raw.githubusercontent.com/opencv/opencv/4.x/data/haarcascades/haarcascade_frontalface_default.xml";

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

    // 얼굴 검출을 위한 Haar Cascade 분류기 파일 경로
    let cascade_path = if let Some(path) = env::var_os("FACE_DETECTOR_CASCADE_PATH") {
        PathBuf::from(path)
    } else {
        let mut path = env::current_exe()?;
        path.pop(); // Remove executable name
        let models_dir = path.join("models");
        fs::create_dir_all(&models_dir)?; // Ensure models directory exists
        models_dir.join("haarcascade_frontalface_default.xml")
    };
    let cascade_path_str = cascade_path.to_str().ok_or("Invalid cascade path")?;

    // 모델 파일이 없으면 다운로드합니다.
    if !cascade_path.exists() {
        println!("모델 파일이 없습니다. 다운로드 중...");
        let response = reqwest::get(CASCADE_DOWNLOAD_URL).await?;
        let content = response.bytes().await?;
        fs::write(&cascade_path, &content)?;
        println!("모델 파일 다운로드 완료: {}", cascade_path_str);
    }

    // OpenCV 파라미터 설정
    let scale_factor = 1.1;
    let min_neighbors = 3; 
    let min_size_width = 256; 
    let min_size_height = 256; 
    let max_size_width = 1536; 
    let max_size_height = 1536; 

    // 이미지 자체의 최소 크기 제한
    let image_min_width = 512;
    let image_min_height = 512;

    // Haar Cascade 분류기를 로드합니다。
    let mut cascade = objdetect::CascadeClassifier::new(cascade_path_str)?;
    if cascade.empty()? {
        panic!("Error: Cascade Classifier not loaded. Check the path: {}", cascade_path_str);
    }

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
            "  Params (scaleFactor: {}, minNeighbors: {}, minSize: {}x{}, maxSize: {}x{}, ImageMinSize: {}x{})
  Scanning: {}\n  Pass: {} | Fail (No Face: {}, Multi: {}, LowRes: {}, NotImg: {})", 
            scale_factor, min_neighbors, min_size_width, min_size_height, max_size_width, max_size_height,
            image_min_width, image_min_height,
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
                        let mut faces = core::Vector::new();
                        let gray_img = imgcodecs::imread(path.to_str().unwrap(), imgcodecs::IMREAD_GRAYSCALE)?;
                        cascade.detect_multi_scale(&gray_img, &mut faces, scale_factor, min_neighbors, objdetect::CASCADE_SCALE_IMAGE, core::Size::new(min_size_width, min_size_height), core::Size::new(max_size_width, max_size_height))?;
                        
                        let faces_found = faces.len();
                        let (width, height) = (img.cols(), img.rows());
                        let good_res = width >= image_min_width && height >= image_min_height;

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