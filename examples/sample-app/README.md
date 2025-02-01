# Sample App - Análisis de Datos de Vuelos

Este proyecto demuestra el procesamiento paralelo de datos de vuelos usando Rust y Rayon.

## Descripción

El programa `main.rs` realiza las siguientes funciones:

- Procesa datos de vuelos desde un archivo CSV 
- Utiliza procesamiento paralelo para mejorar el rendimiento
- Calcula estadísticas como:
  - Total de pasajeros
  - Vuelos por origen/destino 
  - Temperaturas promedio
  - Retrasos de vuelos
  - Consumo de combustible
  - Análisis por fecha

## Requisitos

- Rust (última versión estable)
- Archivo `datos_aeropuerto.csv` con los datos de vuelos
- Python 3.x para el script generador de datos

## Ejecución

>>> python3 samplecsv/prog.py
>>> cargo run
