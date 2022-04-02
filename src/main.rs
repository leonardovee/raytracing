fn main() {
    let image_width = 256;
    let image_height = 256;
    let mut image_out = "".to_owned();
    image_out.push_str("P3\n");
    image_out.push_str("256");
    image_out.push_str(" ");
    image_out.push_str("256");
    image_out.push_str("\n255\n");
    for j in (0..image_width).rev() {
        for _i in 1..image_height {
            let r = j / (image_width - 1);
            let g = j / (image_height - 1);
            let b = 0.25;
            let ir = 255 * r;
            let ig = 255 * g;
            let ib = 255 * b;
            image_out.push_str(ir.to_string());
            image_out.push_str(" ");
            image_out.push_str(&ig.to_string());
            image_out.push_str(" ");
            image_out.push_str(&ib.to_string());
            image_out.push_str("\n");
        }
    }
    println!("{}", image_out);
}
