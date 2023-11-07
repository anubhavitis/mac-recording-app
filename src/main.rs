use recorder::{get_recorder, RecorderType};

mod recorder;

fn main() {
    let mut recorder = get_recorder(RecorderType::Audio);
    recorder.start_recording();

    println!("Recording audio... (Sing Smelly cat). Press any key when you're done!");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    recorder.stop_recording();
}
