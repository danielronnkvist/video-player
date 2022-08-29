use ffmpeg_next::{decoder::Video, format::context::Input, software::scaling::Context};

pub struct VideoStream {
    stream_index: usize,
    scaler: Context,
    decoder: Video,
    ictx: Input,
}

impl VideoStream {
    pub fn new(path: &str) -> Self {
        let ictx = ffmpeg_next::format::input(&path.to_string()).expect("to read the input");

        let input = ictx
            .streams()
            .best(ffmpeg_next::media::Type::Video)
            .expect("video stream");
        let stream_index = input.index();

        let context_decoder =
            ffmpeg_next::codec::context::Context::from_parameters(input.parameters())
                .expect("context decoder");
        let decoder = context_decoder.decoder().video().expect("video decoder");

        let scaler = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            ffmpeg_next::format::Pixel::RGBA,
            decoder.width(),
            decoder.height(),
            ffmpeg_next::software::scaling::flag::Flags::BILINEAR,
        )
        .expect("scaler");

        Self {
            stream_index,
            scaler,
            decoder,
            ictx,
        }
    }

    pub fn get_next_frame(&mut self) -> Option<ffmpeg_next::util::frame::Video> {
        for (stream, packet) in self.ictx.packets() {
            if stream.index() == self.stream_index {
                self.decoder.send_packet(&packet).expect("send packet");
                let mut decoded = ffmpeg_next::util::frame::Video::empty();

                if self.decoder.receive_frame(&mut decoded).is_ok() {
                    let mut rgb_frame = ffmpeg_next::util::frame::Video::empty();
                    self.scaler
                        .run(&decoded, &mut rgb_frame)
                        .expect("scale frame");
                    return Some(rgb_frame);
                }
            }
        }
        None
    }

    pub fn frame_time(&self) -> u128 {
        let frame_rate = self.decoder.frame_rate().unwrap();
        (frame_rate.numerator() as f32 / frame_rate.denominator() as f32) as u128
    }
}
