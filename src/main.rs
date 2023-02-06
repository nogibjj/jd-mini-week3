use jd_mini_week3::convert_to_numeric_value;
use jd_mini_week3::make_image;
use jd_mini_week3::sample_with_replacement;

fn main() {
    let string = "jackiedu";
    let numeric_values = convert_to_numeric_value(string);
    let sampled = sample_with_replacement(&numeric_values, 65536);
    make_image(&sampled, &65536., 250, 100);
}
