use std::{io::{BufReader, BufWriter, Cursor}, path::Path, sync::Arc};

use image::{codecs::tiff::{TiffDecoder, TiffEncoder}, io::Reader, save_buffer_with_format, ColorType, DynamicImage, ExtendedColorType, ImageBuffer, ImageDecoder, ImageEncoder};

use fundsp::{hacker::{
    hammond_hz, multipass, reverb_stereo, sine, sine_hz, soft_saw_hz, square_hz, wave64, Wave64,
}};

use fundsp::prelude::*;

mod bending;
use bending::*;

fn main() {
    println!("Hello, world!");
    //println!("{}", path.canonicalize().unwrap().display());

    let image1 = Stream::new("./assets/cloud.tif");
    let image2 = Stream::new("./assets/walk.tif");
    let image2 = image2.resize(image1.buffer.len());
    let mask = Stream::new("./assets/something_somewhere_text.tif");
    let mask = mask.resize(image1.buffer.len());

    let delta = 32;
    let delay1 = bending::delay(&image1, delta, 0.5);
    let delay2 = bending::delay(&image1, delta*2, 0.5);
    let delay3 = bending::delay(&image1, delta*3, 0.5);
    let delay4 = bending::delay(&image1, delta*4, 0.5);
    //bending::add_streams(vec![&delay1, &delay2, &delay3, &delay4]).normalize().save("./output/add_delay.tif");
    //bending::multiply_streams(vec![&delay1, &delay2, &delay3, &delay4]).normalize().save("./output/multiply_delay.tif");
    //bending::average_streams(vec![&delay1, &delay2, &delay3, &delay4]).normalize().save("./output/average_delay.tif");
    //let modulus = bending::mod_streams(vec![&delay1, &delay2, &delay3, &delay4]).normalize().save("./output/mod_delay.tif");
    //keep this
    let subtract = bending::subtract_streams(vec![&delay1, &delay2, &delay3, &delay4]).absolute().normalize().save("./output/subtract_delay.tif");
    let divide = bending::divide_streams(vec![&delay1, &delay2, &delay3, &delay4]).normalize().save("./output/divide_delay.tif");
    let average = bending::average_streams(vec![&divide, &subtract, &image1]).absolute().normalize().save("./output/divide_subtract_image.tif");
    
    let delta2 = 16;
    let mask_delay1 = bending::delay(&mask, delta2, 0.5);
    let mask_delay2 = bending::delay(&mask, delta2*2, 0.5);
    let mask_delay3 = bending::delay(&mask, delta2*3, 0.5);
    let mask_delay4 = bending::delay(&mask, delta2*4, 0.5);
    let subtract_mask = bending::subtract_streams(vec![&mask_delay1, &mask_delay2, &mask_delay3, &mask_delay4]).absolute().clamp(0.0, 1.0).save("./output/subtract_mask.tif");

    let something_mask = bending::subtract_streams(vec![&image2, &subtract_mask]).clamp(0.0, 1.0).save("./output/multiply_mask.tif");

    bending::stack_streams(vec![&average, &something_mask]).save("./output/stack_mask.tif");
    bending::rand_in_range(&image1, 0..4).save("./output/rand_in_range.tif");
    bending::detect_edges(&image1, 0.1).save("./output/detect_edges.tif");

    /* let k_edge = bending::detect_edges(&image2, 0.1).save("./output/detect_edges.tif");
    let sub1 = k_edge.substream(image1.width*100*3, image1.width*300*3);
    let sub2 = k_edge.substream(image1.width*350*3, image1.width*600*3);
    let sub3 = k_edge.substream(image1.width*800*3, image1.width*1000*3);
    let glitch = bending::stack_streams(vec![&sub1, &sub2, &sub3]).save("./output/glitch.tif");

    bending::add_streams(vec![&glitch, &image1]).clamp(0.0,1.0).normalize().save("./output/glitch_image.tif");

    let reverb = bending::reverb(&image1, 16, 0.5, 4)
        //.save("./output/reverb_raw.tif")
        .normalize()
        .save("./output/reverb_normalized.tif"); */
}

