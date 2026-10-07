use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::Path,
    time::Duration,
};

use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return Err("请指定要查看的PPM图片的位置！".into());
    }

    let mut reader = read_lines(&args[1])?;
    let mut line = String::new();

    reader.read_line(&mut line)?;
    let first_line = line.trim().to_string();
    line.clear();

    if first_line != "P6" && first_line != "P3" {
        return Err("非PPM文件！".into());
    }

    reader.read_line(&mut line)?;
    if line.is_empty() {
        return Err("非完整文件！".into());
    }
    line.clear();

    reader.read_line(&mut line)?;
    let wh = line.trim().split(' ').collect::<Vec<&str>>();

    let width = wh[0]
        .parse::<u32>()
        .expect("将width从&str类型转换成u32类型时失败！");
    let height = wh[1]
        .parse::<u32>()
        .expect("将height从&str类型转换成u32类型时失败！");

    line.clear();

    // 跳过第四行
    reader.read_line(&mut line)?;
    if line.is_empty() {
        return Err("非完整文件！".into());
    }
    line.clear();

    let pixels;

    if first_line == "P6" {
        let mut pixels_p6 = vec![0u8; width as usize * height as usize * 3];
        reader.read_exact(&mut pixels_p6)?;
        pixels = pixels_p6;
    } else {
        let mut data = String::new();
        reader.read_to_string(&mut data)?;

        let mut pixels_p3 = Vec::with_capacity(width as usize * height as usize * 3);

        for value in data.split_whitespace() {
            pixels_p3.push(value.parse::<u8>()?);
        }

        pixels = pixels_p3;
    }

    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("ivrr", width, height)
        .position_centered()
        .build()
        .expect("创建窗口失败！");

    let mut canvas = window.into_canvas().build()?;

    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump()?;

    let mut texture =
        texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, width, height)?;
    texture.update(None, &pixels, width as usize * 3)?;

    canvas.copy(&texture, None, None)?;
    canvas.present();

    'runing: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'runing,

                _ => {}
            }
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn read_lines<P>(path: P) -> io::Result<BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;

    Ok(BufReader::new(file))
}
