extern crate image;
extern crate rand;
use rand::Rng;

// create a function that takes a string and converts each character to its numeric value
pub fn convert_to_numeric_value(string: &str) -> Vec<u32> {
    let mut numeric_values: Vec<u32> = Vec::new();
    for character in string.chars() {
        numeric_values.push(character as u32);
    }
    numeric_values
}

// create a function that takes in a vector of integers and randomly samples with replacement from the vector for n times and returns a vector of the sampled values
pub fn sample_with_replacement(numeric_values: &Vec<u32>, n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut sampled_values: Vec<u32> = Vec::new();
    for _ in 0..n {
        let index = rng.gen_range(0..numeric_values.len());
        sampled_values.push(numeric_values[index]);
    }
    sampled_values
}

// write a function that takes in a u32 and returns the rgb values of the u32 in an array
pub fn get_rgb_values(value: u32, first_num:u32, second_num:u32) -> [u8; 3] {
    let mut rng = rand::thread_rng();
    let order = rng.gen_range(0..3);

    let first_num = rng.gen_range(0..(256 - value - first_num)) + first_num;
    let second_num = rng.gen_range(0..(value - second_num)) + second_num;

    let mut rgb_values: [u8; 3] = [0; 3];
    rgb_values[order] = value as u8;
    
    if order == 0 {
        rgb_values[1] = (value + first_num) as u8;
        rgb_values[2] = (value - second_num) as u8;
    }
    else if order == 1{
        rgb_values[0] = (value + first_num) as u8;
        rgb_values[2] = (value - second_num) as u8;
    }
    else {
        rgb_values[0] = (value + first_num) as u8;
        rgb_values[1] = (value - second_num) as u8;
    }
    rgb_values
}

// create a function that takes in a vector of 2*n integers and makes a nxn pixel image with the values of the vectors as the pixel values and saves the image
pub fn make_image(sampled_values: &[u32], n: &f32, first_num:u32, second_num:u32) {
    let n_sqrt = n.sqrt();
    
    let mut imgbuf = image::RgbImage::new(n_sqrt as u32, n_sqrt as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let index = (x + y) as usize;
        let value = sampled_values[index];
        let rgb = get_rgb_values(value, first_num, second_num);
        *pixel = image::Rgb(rgb);
    }
    imgbuf.save("image.png").unwrap();
}
