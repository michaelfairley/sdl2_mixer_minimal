extern crate sdl2;
use sdl2::mixer;

fn main() {
  let _sdl_context = sdl2::init().expect("Couldn't get sdl context");
  let _mixer = mixer::init(mixer::INIT_OGG);
  mixer::open_audio(22050, mixer::AUDIO_S16LSB, 2, 1024).expect("Couldn't open audio");

  let music = mixer::Music::from_file("music.ogg").unwrap();

  music.play(-1).unwrap();
  std::thread::sleep_ms(1000);
}
