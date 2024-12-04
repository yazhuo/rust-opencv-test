use opencv::core;
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn resize_image(buffer: &[u8], width: i32, height: i32) -> Result<Vec<u8>, String> {

    let img = imgcodecs::imdecode(&opencv::core::Vector::from(buffer), imgcodecs::IMREAD_COLOR)
        .map_err(|e| e.to_string())?;
    
    if img.empty() {
        return Err("Failed to decode the image. Ensure the buffer contains valid image data.".to_string());
    }
    let mut resized_img = Mat::default();

    let new_size = core::Size::new(width, height);

    // Resize the image
    imgproc::resize(
        &img,
        &mut resized_img,
        new_size,
        0.0,
        0.0,
        imgproc::INTER_LINEAR,
    ).map_err(|e| e.to_string())?;

    let mut output_buffer = opencv::core::Vector::new();
    imgcodecs::imencode(".jpg", &resized_img, &mut output_buffer, &core::Vector::new())
        .map_err(|e| e.to_string())?;

    Ok(output_buffer.to_vec())
}



// fn main() -> opencv::Result<()> {
//     // Load an image from file (ensure the file exists in the current directory)
//     let img = imgcodecs::imread("/home/yazzhang/workspace/opencv-test/src/test.jpg", imgcodecs::IMREAD_COLOR)?;
//     if img.empty() {
//         return Err(opencv::Error::new(
//             core::StsError,
//             "Failed to load the image. Ensure the file path is correct.".to_string(),
//         ));
//     }

//     // Create an empty Mat for the resized image
//     let mut resized_img = Mat::default();

//     // Define the new size (e.g., width = 300, height = 300)
//     let new_size = core::Size::new(300, 300);

//     // Resize the image
//     imgproc::resize(
//         &img,
//         &mut resized_img,
//         new_size,
//         0.0,
//         0.0,
//         imgproc::INTER_LINEAR,
//     )?;

//     // Save the resized image to a new file
//     imgcodecs::imwrite("resized_example.jpg", &resized_img, &core::Vector::new());
//     println!("Resized image saved to 'resized_example.jpg'.");

//     Ok(())
// }


