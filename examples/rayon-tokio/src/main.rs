use rayon::join;
use std::f64::consts::PI;
use tokio::{task, time};
use num_complex::Complex;
use std::time::Duration;

/// Computa la FFT de un conjunto de valores complejos.
fn fft(input: &mut [Complex<f64>]) {
    let n = input.len();
    if n <= 1 {
        return;
    }

    // Divide los elementos en pares e impares
    let mut even: Vec<Complex<f64>> = input.iter().step_by(2).cloned().collect();
    let mut odd: Vec<Complex<f64>> = input.iter().skip(1).step_by(2).cloned().collect();

    // Recursión en paralelo con Rayon
    join(|| fft(&mut even), || fft(&mut odd));

    for k in 0..n / 2 {
        let t = odd[k] * Complex::from_polar(1.0, -2.0 * PI * k as f64 / n as f64);
        input[k] = even[k] + t;
        input[k + n / 2] = even[k] - t;
    }
}

#[tokio::main]
async fn main() {
    let size = 20_000_000;
    let mut data: Vec<Complex<f64>> = (0..size)
        .map(|i| Complex::new(i as f64, 0.0))
        .collect();

    let start = std::time::Instant::now();

    // Ejecutar la FFT en un thread separado sin bloquear Tokio
    println!("Iniciando FFT...");
    let fft_task = task::spawn_blocking(move || {
        fft(&mut data);
        println!("FFT completada!");
        let duration = start.elapsed();
        println!("Tiempo total: {:?}", duration);
    });

    // Ejecutar otra tarea en paralelo (simulación de otra operación)
    let other_task = task::spawn(async {
        for i in 1..=3 {
            println!("Realizando otra tarea... paso {}", i);
            time::sleep(Duration::from_secs(10)).await;
        }
    });

    // Esperar ambas tareas en paralelo
    tokio::join!(fft_task, other_task);

}
