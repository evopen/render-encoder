use std::fs::File;
use std::path::Path;

use rav1e::prelude::*;

pub struct Encoder {
    ctx: Context<u8>,
    width: usize,
    file: File,
}

impl Encoder {
    pub fn new<T: num_traits::PrimInt, P: AsRef<Path>>(width: T, height: T, output: P) -> Self {
        let width = width.to_usize().unwrap();
        let height = height.to_usize().unwrap();
        let encoder_config = EncoderConfig {
            width: width,
            height: height,
            chroma_sampling: ChromaSampling::Cs444,
            speed_settings: SpeedSettings::from_preset(10),
            time_base: Rational { num: 1, den: 30 },
            enable_timing_info: true,
            rdo_lookahead_frames: 1,
            low_latency: true,
            ..Default::default()
        };
        let cfg = Config::new().with_encoder_config(encoder_config);
        let ctx: Context<u8> = cfg.new_context().unwrap();
        let mut file = std::fs::File::create(output).unwrap();
        ivf::write_ivf_header(&mut file, width, height, 30, 1);

        Self { ctx, width, file }
    }

    pub fn new_frame(&mut self, y_plane: &[u8], u_plane: &[u8], v_plane: &[u8]) {
        let mut frame = self.ctx.new_frame();
        frame.planes[0].copy_from_raw_u8(y_plane, self.width, 1);
        frame.planes[1].copy_from_raw_u8(u_plane, self.width, 1);
        frame.planes[2].copy_from_raw_u8(v_plane, self.width, 1);
        self.ctx.send_frame(frame).unwrap();
        loop {
            match self.ctx.receive_packet() {
                Ok(pkt) => {
                    ivf::write_ivf_frame(&mut self.file, pkt.input_frameno, &pkt.data);
                    println!("muxing");
                }
                Err(e) => match e {
                    EncoderStatus::NeedMoreData => {
                        break;
                    }
                    _ => {
                        dbg!(&e);
                    }
                },
            }
        }
    }

    pub fn flush(&mut self) {
        self.ctx.flush();
        loop {
            match self.ctx.receive_packet() {
                Ok(pkt) => {
                    ivf::write_ivf_frame(&mut self.file, pkt.input_frameno, &pkt.data);
                    println!("muxing");
                }
                Err(e) => match e {
                    EncoderStatus::NeedMoreData | EncoderStatus::LimitReached => {
                        break;
                    }
                    _ => {
                        dbg!(&e);
                    }
                },
            }
        }
    }
}
