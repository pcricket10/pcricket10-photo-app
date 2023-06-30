pub fn main() {
    let mut gradient_image = Image::new(500, 500);

    // Generate and save gradient image
    gradient_image.generate_gradient_image([0, 0, 0], [255, 255, 255]);
    let gradient_image_clone = gradient_image.clone().to_rgb_image();
    // gradient_image_clone.display();

    gradient_image_clone
        .save("images/original_gradient_image.png")
        .unwrap();

    // Clone and resize the image, then save
    let mut resized_image = gradient_image.clone();
    resized_image.resize(32, 32);
    let resized_image = resized_image.to_rgb_image();
    resized_image
        .save("images/resized_gradient_image.png")
        .unwrap();

    let mut cropped_image = gradient_image.clone();
    cropped_image.crop(100, 100, 200, 200);
    cropped_image
        .to_rgb_image()
        .save("images/cropped_image.png")
        .unwrap();
}
