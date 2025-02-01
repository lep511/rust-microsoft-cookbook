use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use serde::Deserialize;
use std::error::Error;
use std::time::Instant;
use std::sync::Arc;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct VueloInfo {
    fecha: String,
    vuelo: String,
    origen: String,
    destino: String,
    pasajeros: u32,
    retraso_minutos: u32,
    combustible_litros: u32,
    temperatura: i32,
    // Datos adicionales
    tripulacion: u32,
    equipaje_kg: f32,
    carga_kg: f32,
    velocidad_crucero: u32,
    altitud_crucero: u32,
    distancia_km: u32,
    duracion_prevista: u32,
    duracion_real: u32,
    escala_tecnica: bool,
    puerta_embarque: String,
    terminal: String,
    tipo_avion: String,
    capacidad_maxima: u32,
    asientos_business: u32,
    asientos_turista: u32,
    ocupacion_percent: f32,
    precio_medio: f32,
    ingresos: f32,
    costes_operacion: f32,
    satisfaccion_media: f32,
    incidencias: u32,
    clima_origen: String,
    clima_destino: String,
    visibilidad_origen: u32,
    visibilidad_destino: u32,
    viento_origen: f32,
    viento_destino: f32,
    presion_origen: f32,
    presion_destino: f32,
    humedad_origen: u32,
    humedad_destino: u32,
    conexiones: u32,
    equipaje_perdido: u32,
    comidas_servidas: u32,
    bebidas_servidas: u32,
    peliculas_disponibles: u32,
    wifi_disponible: bool,
    asientos_preferentes: u32,
    mascotas_abordo: u32,
    asistencias_especiales: u32,
    edad_media_pasajeros: f32,
    satisfaccion_comida: f32,
    satisfaccion_vuelo: f32,
    satisfaccion_tripulacion: f32,
    consumo_entretenimiento: f32,
    uso_wifi_percent: f32,
    compras_abordo: f32,
    nivel_combustible_llegada: u32,
    tiempo_taxi_despegue: u32,
    tiempo_taxi_aterrizaje: u32,
}

// Modificamos la estructura para usar Vec en lugar de array fijo
struct DatosGrandes {
    datos: Box<Vec<u64>>,  // Cambiamos a Vec para manejo dinámico
    descripcion: String,
}

// Estructura para compartir entre hilos
#[derive(Clone)]
struct EstadisticasCompartidas {
    total_procesado: u64,
    promedio: f64,
}

fn procesar_datos(numeros: &[u64]) {
    // Reducimos a un tamaño más razonable (10 millones = ~80MB)
    let datos_grandes = DatosGrandes {
        datos: Box::new(vec![0; 10_000_000]),  // ~80MB en lugar de ~800GB
        descripcion: "Datos de prueba".to_string(),
    };
    println!("Tamaño de descripción: {}", datos_grandes.descripcion.len());

    // Reducimos el tamaño de los datos compartidos
    let estadisticas = Arc::new(EstadisticasCompartidas {
        total_procesado: 0,
        promedio: 0.0,
    });
    
    // Clonar Arc para usar en parallel iterator
    let _stats_clone = Arc::clone(&estadisticas);

    println!("Recursos disponibles:");
    println!("CPU Cores: {}", num_cpus::get());
    println!("CPU Cores físicos: {}", num_cpus::get_physical());
    
    // Procesamiento secuencial
    let inicio = Instant::now();
    let suma_normal: u64 = numeros.iter()
        .map(|&x| x * x)
        .sum();
    let tiempo_normal = inicio.elapsed();
    
    // Procesamiento paralelo con Rayon
    let inicio = Instant::now();
    let suma_paralela: u64 = numeros.par_iter()
        .map(|&x| x * x)
        .sum();
    let tiempo_paralelo = inicio.elapsed();
    
    println!("Suma normal: {} (tiempo: {:?})", suma_normal, tiempo_normal);
    println!("Suma paralela: {} (tiempo: {:?})", suma_paralela, tiempo_paralelo);
    
    // Usar Arc en procesamiento paralelo
    let inicio = Instant::now();
    let pares: Vec<u64> = numeros.par_iter()
        .inspect(|&&x| {
            if x % 1_000_000 == 0 {
                println!("Procesando número grande: {}", x);
            }
        })
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x)
        .collect();
    let tiempo_filtrado = inicio.elapsed();
    
    println!("Cantidad de números pares: {} (tiempo: {:?})", pares.len(), tiempo_filtrado);
    
    // Demostrar que Arc permite acceso seguro desde múltiples hilos
    println!("Estadísticas compartidas - Total: {}", estadisticas.total_procesado);
}

fn procesar_datos_aeropuerto() -> Result<(), Box<dyn Error>> {
    let inicio_total = Instant::now();
    
    // Lectura optimizada del CSV
    let inicio_lectura = Instant::now();
    let file = File::open("datos_aeropuerto.csv")?;
    let buf_reader = BufReader::with_capacity(128 * 1024, file);
    let mut rdr = csv::ReaderBuilder::new()
        .buffer_capacity(128 * 1024)
        .from_reader(buf_reader);

    // Convertir los registros a un vector para procesamiento paralelo
    let records: Result<Vec<csv::StringRecord>, _> = rdr.records().collect();
    let records = records?;
    
    // Procesar registros en paralelo
    let vuelos: Vec<VueloInfo> = records.par_iter()
        .filter_map(|record| {
            match record.deserialize(None) {
                Ok(vuelo) => Some(vuelo),
                Err(e) => {
                    eprintln!("Error al deserializar registro: {}", e);
                    None
                }
            }
        })
        .collect();

    println!("Tiempo de lectura y deserialización: {:?}", inicio_lectura.elapsed());
    println!("Cantidad de vuelos procesados: {}", vuelos.len());
    
    // Estadísticas básicas
    let total_pasajeros: u32 = vuelos.par_iter()
        .map(|v| v.pasajeros)
        .sum();
        
    // Analizar vuelos por origen
    let vuelos_madrid: Vec<&VueloInfo> = vuelos.par_iter()
        .filter(|v| v.origen == "MAD")
        .collect();
        
    // Temperatura promedio por destino
    let temp_bcn: f64 = vuelos.par_iter()
        .filter(|v| v.destino == "BCN")
        .map(|v| v.temperatura as f64)
        .sum::<f64>() / vuelos.par_iter().filter(|v| v.destino == "BCN").count() as f64;
        
    // Analizar retrasos por aerolínea
    let retrasos_iberia: Vec<&VueloInfo> = vuelos.par_iter()
        .filter(|v| v.vuelo.starts_with("IB") && v.retraso_minutos > 0)
        .collect();
        
    // Calcular promedio de combustible
    let promedio_combustible: f64 = vuelos.par_iter()
        .map(|v| v.combustible_litros as f64)
        .sum::<f64>() / vuelos.len() as f64;
    
    // Análisis por fecha con tipos explícitos
    let vuelos_por_fecha: std::collections::HashMap<String, Vec<&VueloInfo>> = 
        vuelos.par_iter()
            .fold(
                || std::collections::HashMap::<String, Vec<&VueloInfo>>::new(),
                |mut acc: std::collections::HashMap<String, Vec<&VueloInfo>>, vuelo| {
                    acc.entry(vuelo.fecha.clone())
                       .or_insert_with(|| Vec::new())
                       .push(vuelo);
                    acc
                }
            )
            .reduce(
                || std::collections::HashMap::<String, Vec<&VueloInfo>>::new(),
                |mut map1, map2| {
                    for (fecha, vuelos) in map2 {
                        map1.entry(fecha)
                           .or_insert_with(|| Vec::new())
                           .extend(vuelos);
                    }
                    map1
                }
            );

    println!("\nAnálisis por fecha:");
    println!("----------------------------------------");
    let mut promedio_temperatura_dia: f64 = 0.0;
    for (fecha, vuelos) in vuelos_por_fecha.iter() {
        let total_pasajeros_dia: u32 = vuelos.iter().map(|v| v.pasajeros).sum();
        promedio_temperatura_dia = vuelos.iter().map(|v| v.temperatura as f64).sum::<f64>() / vuelos.len() as f64;
    }

    println!("\nEstadísticas detalladas del aeropuerto:");
    println!("----------------------------------------");
    println!("Total de vuelos: {}", vuelos.len());
    println!("Total de pasajeros: {}", total_pasajeros);
    println!("Vuelos desde Madrid: {}", vuelos_madrid.len());
    println!("Temperatura promedio en Barcelona: {:.1}°C", temp_bcn);
    println!("Vuelos Iberia con retraso: {}", retrasos_iberia.len());
    println!("Promedio de combustible: {:.2} litros", promedio_combustible);
    println!("Total de retrasos: {}", vuelos.par_iter().map(|v| v.retraso_minutos).sum::<u32>());
    println!("Promedio de temperatura por día: {:.1}°C", promedio_temperatura_dia);
    
    // Mostrar algunos detalles de vuelos
    println!("\nDetalle de algunos vuelos:");
    println!("----------------------------------------");
    for vuelo in vuelos.iter().take(3) {
        println!("Vuelo {} ({} → {}): {} pasajeros, {}°C",
            vuelo.vuelo, vuelo.origen, vuelo.destino, 
            vuelo.pasajeros, vuelo.temperatura);
    }
    
    println!("\nTiempo total de procesamiento: {:?}", inicio_total.elapsed());
    
    Ok(())
}

fn main() {
    // En Lambda, esto típicamente mostrará 2 vCPUs
    println!("vCPUs disponibles: {}", rayon::current_num_threads());
    
    // Reducimos también el vector de entrada a un tamaño más manejable
    let numeros: Vec<u64> = (0..1_000_000).collect();  // ~8MB
    
    // Forzamos 2 hilos ya que es el límite típico en Lambda
    match ThreadPoolBuilder::new()
        .num_threads(2)  // Lambda típicamente tiene 2 vCPUs
        .build()
    {
        Ok(pool) => {
            println!("Pool creado exitosamente con {} hilos", pool.current_num_threads());
            pool.install(|| procesar_datos(&numeros));
        },
        Err(e) => {
            eprintln!("Error al crear el pool de hilos: {}", e);
            std::process::exit(1);
        }
    }

    if let Err(e) = procesar_datos_aeropuerto() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
