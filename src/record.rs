use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, Host, Device, SupportedStreamConfig, BuildStreamError};
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};

pub fn main(){
    let host:Host = cpal::default_host();
    let device:Device = host.default_input_device().expect("failed to find input device");
    println!("Input device: {}", device.name().unwrap());

    let config:SupportedStreamConfig = device.default_input_config().expect("Failed to get default input config");
    println!("Default input config: {:?}", config);

    // The WAV file we're recording to.
    const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/recorded.wav");
    println!("Path to recording: {:?}", PATH);
    let spec = wav_spec_from_config(&config);
    let writer = hound::WavWriter::create(PATH, spec).unwrap();

    /*  
    - Wrapping inside a Mutex, which is used for synchronization. 
    It allows multiple threads to safely access the Option
    by locking and unlocking the Mutex when needed.

    - Then creating a Arc instance. 
    It allows multiple parts of your code to access and modify the Mutex safely 
    by keeping track of the number of references to it. 
    */
    let writer = Arc::new(Mutex::new(Some(writer)));

    // A flag to indicate that recording is in progress.
    println!("Begin recording...");

    // Run the input stream on a separate thread.
    // This is shared reference to Mutex we created above.
    let writer_2 = writer.clone();

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };

    let stream:cpal::Stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i8, i8>(data, &writer_2),
            err_fn,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
            err_fn,
            None,
        ),
        cpal::SampleFormat::I32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i32, i32>(data, &writer_2),
            err_fn,
            None,
        ),
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<f32, f32>(data, &writer_2),
            err_fn,
            None,
        ),
        _sample_format => Err(BuildStreamError::DeviceNotAvailable),
    }.unwrap();

    stream.play().unwrap();

    // Prompt the user for input
    println!("Press any key to stop : ");
    
    // Capture user input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    drop(stream);

    // close the writer
    writer.lock().unwrap().take().unwrap().finalize().unwrap();
    println!("Recording {} complete!", PATH);

}

fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
    if format.is_float() {
        hound::SampleFormat::Float
    } else {
        hound::SampleFormat::Int
    }
}

fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> hound::WavSpec {
    hound::WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
        sample_format: sample_format(config.sample_format()),
    }
}

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = U::from_sample(sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}
