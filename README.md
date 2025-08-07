## Features
   * Detects faces in images within a specified directory and its subdirectories.
   * Moves files to Pass or Fail subfolders based on strict criteria (exactly one face, image size >=
     512x512).
   * Categorizes Fail reasons (NoFace, MultiFace, LowResolution, NotAnImage) and prefixes filenames
     accordingly.
   * Displays a detailed, multi-line progress bar showing:
       * Elapsed time, estimated time remaining, and file count.
       * OpenCV parameters (scaleFactor, minNeighbors, minSize, maxSize) and ImageMinSize.
       * The currently scanned folder.
       * Real-time counts for Pass and each Fail category.
   * Provides a comprehensive summary of results at the end.
   * Accepts the target directory as a command-line argument (supporting relative paths).
   * Skips files already processed within Pass/Fail subfolders.

## How to Run
after build
$ face_detector <target_directory>