use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;
use rand::Rng;
use hound::{WavSpec, WavWriter};
use rodio::{Decoder, OutputStream, OutputStreamHandle};

fn main() {
    // Lee el archivo de entrada
    let file = File::open("input.mp3").expect("No se pudo abrir el archivo");
    let source = Decoder::new(BufReader::new(file)).unwrap();
    // Genera un número aleatorio para la interferencia destructiva
    let mut rng = rand::thread_rng();

    // Crea el archivo de salida
    let spec = WavSpec {
        channels: 2,
        sample_rate: source.sample_rate(),
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::create("output.wav", spec).expect("No se pudo crear el archivo de salida");

    // Crea el stream de salida
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    // Reproduce el archivo de entrada
    for sample in source {
        let sample_left = sample.unwrap();
        let sample_right = rng.gen::<i16>();

        // Escribe la muestra en el archivo de salida
        writer.write_sample(sample_left).expect("No se pudo escribir la muestra en el archivo");

        // Reproduce la muestra
        let handle = stream_handle.clone();
        thread::spawn(move || {
            let data = [sample_left, sample_right];
            handle.play_raw(data.iter().cloned()).unwrap();
            thread::sleep(Duration::from_millis(4)); // Espera 4 milisegundos para evitar el clipping
        });
    }

   // Espera 5 segundos para que termine de reproducirse el archivo
    thread::sleep(Duration::from_secs(5));
}
