use crate::audio_modifiers::low_pass_filter;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, StreamConfig};

use tracing::info;
use std::sync::{Arc, Mutex};

pub(crate) fn playback(audio_static: Arc<Mutex<Vec<f32>>>) {
    let host = cpal::default_host();
    let input_device: Device = host
        .default_input_device()
        .expect("Failed to get default input device");
    let output_device: Device = host
        .default_output_device()
        .expect("Failed to get default output device");

    info!("{:?}", &input_device.name().unwrap());
    info!("{:?}", &output_device.name().unwrap());

    let input_config: StreamConfig = input_device
        .default_input_config()
        .expect("Failed to get default input format")
        .config();
    let output_config: StreamConfig = output_device
        .default_output_config()
        .expect("Failed to get default output format")
        .config();

    info!("{:?}", &input_config);
    info!("{:?}", &output_config);

    let shared_data = Arc::new(Mutex::new(Vec::new()));

    let input_data = Arc::clone(&shared_data);
    let input_stream = input_device
        .build_input_stream(
            &input_config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut input_data = input_data.lock().unwrap();
                input_data.clear();
                let mut temp = data.to_vec();
                temp = low_pass_filter(&temp, 0.7);

                let mut audio_data = audio_static.lock().unwrap();
                *audio_data = temp.clone();

                input_data.extend_from_slice(&*temp);
            },
            move |err| {
                eprintln!("Error occurred on input stream: {}", err);
            },
        )
        .expect("Failed to build input stream");

    let volume = 0.7f32;
    let output_data = Arc::clone(&shared_data);
    let output_stream = output_device
        .build_output_stream(
            &output_config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let input_data = output_data.lock().expect("Unable to open input lock");

                if input_data.len() < 1 {
                    eprintln!("Empty audio frame");
                    return;
                }

                for (i, out_sample) in data.iter_mut().enumerate() {
                    let in_sample = input_data.get(i).cloned().unwrap_or(0.0);
                    *out_sample = in_sample * volume;
                }
            },
            move |err| {
                eprintln!("Error occurred on output stream: {}", err);
            },
        )
        .expect("Failed to build output stream");

    input_stream.play().expect("Failed to play input stream");
    output_stream.play().expect("Failed to play output stream");

    loop {}
}
