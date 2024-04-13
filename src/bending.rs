use std::ops::Range;

use image::io::Reader;
use rand::Rng;


fn clampf64 (min:f64, max:f64, a:f64) -> f64 {
    if a < min {return min};
    if a > max {return max};
    return a;
}

#[derive(Clone)]
pub struct Stream{
    pub buffer: Vec<f64>,
    pub width: usize,
    pub height: usize,
}

impl Stream{
    pub fn new(path: &str) -> Stream{
        let image = Reader::open(path)
            .unwrap()
            .decode()
            .unwrap();

        //convert [u8] to [f64]
        let image_f64 = image
            .as_bytes().iter()
            .map(|x| *x as f64 / 255.0)
            .collect::<Vec<f64>>();

        Stream{
            buffer: image_f64,
            width: image.width() as usize,
            height: image.height() as usize,
        }
    }

    pub fn save(&self, path: &str) -> Stream{
        //convert [f64] to [u8]
        let image_u8 = self.buffer
            .iter()
            .map(|x| (x*255.0 as f64) as u8)
            .collect::<Vec<u8>>();
        let _ =image::save_buffer_with_format(
            path,
            &image_u8,
            self.width as u32,
            self.height as u32,
            image::ExtendedColorType::Rgb8,
            image::ImageFormat::Tiff
        );

        self.clone()
    }

    pub fn resize(&self, n:usize) -> Stream{
        let mut buffer = self.buffer.clone();
        buffer.resize(n, 0.0);
        Stream{
            buffer,
            width: self.width,
            height: self.height,
        }
    }

    pub fn buffer(&self) -> Vec<f64>{
        self.buffer.clone()
    }

    pub fn len(&self) -> usize{
        self.buffer.len()
    }

    pub fn clamp(self, min: f64, max: f64) -> Stream{
        let mut buffer = self.buffer;
        for x in buffer.iter_mut(){
            *x = clampf64(min, max, *x);
        }
        Stream{
            buffer,
            width: self.width,
            height: self.height,
        }
    }
    //create a buffer with the same length as the orignail
    //but only contains 0.0 except for the selection
    pub fn substream(&self, start: usize, end: usize) -> Stream{
        let mut buffer = vec![0.0; self.buffer.len()];
        for i in start..end{
            buffer[i] = self.buffer[i];
        }
        Stream{
            buffer,
            width: self.width,
            height: self.height,
        }

    }

    pub fn normalize(self) -> Stream{
        let mut buffer = self.buffer;
        let max = buffer.iter().fold(0.0, |acc, x| if *x > acc { *x } else { acc });
        for x in buffer.iter_mut(){
            *x /= max;
        }
        Stream{
            buffer,
            width: self.width,
            height: self.height,
        }
    }

    pub fn absolute(self) -> Stream{
        let mut buffer = self.buffer;
        for x in buffer.iter_mut(){
            *x = x.abs();
        }
        Stream{
            buffer,
            width: self.width,
            height: self.height,
        }
    }
}

pub fn stack_streams(streams: Vec<&Stream>) -> Stream{
    let mut buffer = streams[0].buffer.clone();
    for stream in streams.iter().skip(1){
        for (i, x) in buffer.iter_mut().enumerate(){
            if stream.buffer[i] > 0.0 {
                *x = stream.buffer[i];
            }
        }
    }
    Stream{
        buffer,
        width: streams[0].width,
        height: streams[0].height,
    }
}

pub fn add_streams(streams: Vec<&Stream>) -> Stream{
    let mut buffer = streams[0].buffer.clone();
    for stream in streams.iter().skip(1){
        for (i, x) in buffer.iter_mut().enumerate(){
            *x += stream.buffer[i];
        }
    }
    Stream{
        buffer,
        width: streams[0].width,
        height: streams[0].height,
    }
}

pub fn subtract_streams(streams: Vec<&Stream>) -> Stream{
    let mut buffer = streams[0].buffer.clone();
    for stream in streams.iter().skip(1){
        for (i, x) in buffer.iter_mut().enumerate(){
            *x -= stream.buffer[i];
        }
    }
    Stream{
        buffer,
        width: streams[0].width,
        height: streams[0].height,
    }
}

pub fn multiply_streams(streams: Vec<&Stream>) -> Stream{
    let mut buffer = streams[0].buffer.clone();
    for stream in streams.iter().skip(1){
        for (i, x) in buffer.iter_mut().enumerate(){
            *x *= stream.buffer[i];
        }
    }
    Stream{
        buffer,
        width: streams[0].width,
        height: streams[0].height,
    }
}

pub fn divide_streams(streams: Vec<&Stream>) -> Stream{
    let mut buffer = streams[0].buffer.clone();
    for stream in streams.iter().skip(1){
        for (i, x) in buffer.iter_mut().enumerate(){
            if stream.buffer[i] == 0.0 {continue};
            *x /= stream.buffer[i];
        }
    }
    Stream{
        buffer,
        width: streams[0].width,
        height: streams[0].height,
    }
}

pub fn mod_streams(streams: Vec<&Stream>) -> Stream{
    let mut buffer = streams[0].buffer.clone();
    for stream in streams.iter().skip(1){
        for (i, x) in buffer.iter_mut().enumerate(){
            if stream.buffer[i] == 0.0 {continue};
            *x %= stream.buffer[i];
        }
    }
    Stream{
        buffer,
        width: streams[0].width,
        height: streams[0].height,
    }
}

pub fn average_streams(streams: Vec<&Stream>) -> Stream{
    let mut buffer = streams[0].buffer.clone();
    for stream in streams.iter().skip(1){
        for (i, x) in buffer.iter_mut().enumerate(){
            *x += stream.buffer[i];
        }
    }
    for x in buffer.iter_mut(){
        *x /= streams.len() as f64;
    }
    Stream{
        buffer,
        width: streams[0].width,
        height: streams[0].height,
    }
}

pub fn delay(stream: &Stream, offset: usize, decay: f64) -> Stream{
    let mut buffer = stream.buffer.clone();
    for (i, s) in buffer.iter_mut().enumerate() {
        if i < stream.buffer.len() - offset - 1 {
            let a = i+offset;
            *s = stream.buffer[a] + stream.buffer[i] * decay ;
        }
    }
    Stream{
        buffer,
        width: stream.width,
        height: stream.height,
    }
}

pub fn reverb(stream: &Stream, offset: usize, decay: f64, iterations: usize) -> Stream{
    let mut buffer = stream.buffer.clone();
    for i in 0..iterations{
        buffer = delay(&Stream{
            buffer, 
            width: stream.width, 
            height: stream.height
        }, offset*i, decay*(i as f64)).buffer;
    }
    Stream{
        buffer,
        width: stream.width,
        height: stream.height,
    }
}

pub fn rotate_array(stream: &Stream, offset: usize) -> Stream{
    let mut buffer = stream.buffer.clone();
    for i in 0..stream.buffer.len(){
        buffer[i] = stream.buffer[(i+offset)%stream.buffer.len()];
    }
    Stream{
        buffer,
        width: stream.width,
        height: stream.height,
    }
}

//retutn a stream where 0 represents no edge and 1 represents an edge
pub fn detect_edges(stream: &Stream, threshold: f64) -> Stream{
    let mut buffer = stream.buffer.clone();
    for i in 0..stream.buffer.len()-1{
        //if the distance between two points is greater than the threshold
        //then this point is an edge
        if (stream.buffer[i+1]-stream.buffer[i]).abs() > threshold{
            buffer[i] = 1.0;
        }
        else{
            buffer[i] = 0.0;
        }
    }
    Stream{
        buffer,
        width: stream.width,
        height: stream.height,
    }
}

pub fn rand_in_range(stream: &Stream, range:Range<i32>) -> Stream {
    let mut rng = rand::thread_rng();
    let mut buffer = stream.buffer.clone();
    //move the current pixel to a random position inside the provided range
    //so long at the random position is not outside the buffer length
    for i in 0..stream.buffer.len(){
        let random = rng.gen_range(range.clone());
        if random > 0 && (i as i32 + random - 1) < stream.buffer.len() as i32 - 1 {
            buffer[i] = stream.buffer[(i as i32 + random) as usize];
        };
    }
    Stream{
        buffer,
        width: stream.width,
        height: stream.height,
    }

}