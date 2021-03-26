use std::path::PathBuf;

use dcv_color_primitives as dcp;

use render_encoder::Encoder;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    dcp::initialize();

    let result_dir = PathBuf::from("/home/dhh/Documents/Frames");

    let mut encoder = Encoder::new(WIDTH, HEIGHT, "out.ivf");

    let dst_buffers: &mut [&mut [u8]] = &mut [
        &mut [0u8; (WIDTH as usize) * (HEIGHT as usize)],
        &mut [0u8; (WIDTH as usize) * (HEIGHT as usize)],
        &mut [0u8; (WIDTH as usize) * (HEIGHT as usize)],
    ];
    dbg!(&dst_buffers.len());
    dbg!(&dst_buffers[0].len());
    for img_path in result_dir.read_dir().unwrap().map(|p| p.unwrap().path()) {
        println!(
            "processing {}",
            &img_path.file_name().unwrap().to_str().unwrap()
        );
        let img = image::open(img_path).unwrap();
        let img = img.as_rgba8().unwrap();

        dcp::convert_image(
            img.width(),
            img.height(),
            &dcp::ImageFormat {
                pixel_format: dcp::PixelFormat::Bgra,
                color_space: dcp::ColorSpace::Lrgb,
                num_planes: 1,
            },
            None,
            &[img.as_raw()],
            &dcp::ImageFormat {
                pixel_format: dcp::PixelFormat::I444,
                color_space: dcp::ColorSpace::Bt601,
                num_planes: 3,
            },
            None,
            dst_buffers,
        )
        .unwrap();

        encoder.new_frame(
            dst_buffers[0].as_ref(),
            dst_buffers[1].as_ref(),
            dst_buffers[2].as_ref(),
        );
    }

    encoder.flush();
}
