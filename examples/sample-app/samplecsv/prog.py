import csv
import random
import datetime

def generate_flight_data(num_rows):
    """Generates fake flight data and writes it to a CSV file."""

    with open('../datos_aeropuerto.csv', 'w', newline='') as csvfile:
        fieldnames = [
            'fecha', 'vuelo', 'origen', 'destino', 'pasajeros', 'retraso_minutos',
            'combustible_litros', 'temperatura', 'tripulacion', 'equipaje_kg',
            'carga_kg', 'velocidad_crucero', 'altitud_crucero', 'distancia_km',
            'duracion_prevista', 'duracion_real', 'escala_tecnica', 'puerta_embarque',
            'terminal', 'tipo_avion', 'capacidad_maxima', 'asientos_business',
            'asientos_turista', 'ocupacion_percent', 'precio_medio', 'ingresos',
            'costes_operacion', 'satisfaccion_media', 'incidencias', 'clima_origen',
            'clima_destino', 'visibilidad_origen', 'visibilidad_destino',
            'viento_origen', 'viento_destino', 'presion_origen', 'presion_destino',
            'humedad_origen', 'humedad_destino', 'conexiones', 'equipaje_perdido',
            'comidas_servidas', 'bebidas_servidas', 'peliculas_disponibles',
            'wifi_disponible', 'asientos_preferentes', 'mascotas_abordo',
            'asistencias_especiales', 'edad_media_pasajeros', 'satisfaccion_comida',
            'satisfaccion_vuelo', 'satisfaccion_tripulacion', 'consumo_entretenimiento',
            'uso_wifi_percent', 'compras_abordo', 'nivel_combustible_llegada',
            'tiempo_taxi_despegue', 'tiempo_taxi_aterrizaje'
        ]
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
        writer.writeheader()

        for i in range(num_rows):
            fecha = datetime.date(2023, 1, 1) + datetime.timedelta(days=i)
            vuelo = random.choice(['IB', 'VY']) + str(random.randint(1000, 9999))
            origen = random.choice(['MAD', 'BCN'])
            destino = random.choice(['LHR', 'ORY', 'CDG', 'FCO', 'MXP', 'AMS', 'LIS', 'PMI', 'AGP', 'BIO', 'TFN', 'LPA', 'OVD', 'SCQ', 'SVQ', 'VLC', 'MAH', 'IBZ', 'XRY', 'ACE', 'FAO', 'FUE', 'GRO', 'GRX', 'HOR', 'LCG', 'LEI', 'SPC', 'TFS', 'BJZ', 'VGO'])
            
            if origen == 'MAD':
              terminal = 'T4'
            else:
              terminal = 'T1'

            pasajeros = random.randint(140, 280)
            retraso_minutos = random.randint(0, 30)
            combustible_litros = random.randint(4000, 7500)
            temperatura = random.randint(7, 22)
            tripulacion = random.randint(3, 8)
            equipaje_kg = round(random.uniform(1700, 3800), 1)
            carga_kg = round(random.uniform(800, 2500), 1)
            velocidad_crucero = random.randint(800, 890)
            altitud_crucero = random.randint(9000, 12200)
            distancia_km = random.randint(400, 1300)
            duracion_prevista = random.randint(50, 150)
            duracion_real = duracion_prevista + random.randint(0, (retraso_minutos + 10))
            escala_tecnica = random.choice(['true', 'false'])
            puerta_embarque = random.choice(['A', 'B', 'C']) + str(random.randint(1, 25))
            
            tipo_avion = random.choice(['A319', 'A320', 'A321', 'A330', 'A350'])

            if tipo_avion == 'A319':
                capacidad_maxima = 180
                asientos_business = 18
                asientos_turista = 162
            elif tipo_avion == 'A320':
                capacidad_maxima = 200
                asientos_business = 20
                asientos_turista = 180
            elif tipo_avion == 'A321':
                capacidad_maxima = 220
                asientos_business = 22
                asientos_turista = 198
            elif tipo_avion == 'A330':
                capacidad_maxima = 250
                asientos_business = 25
                asientos_turista = 225
            elif tipo_avion == 'A350':
                capacidad_maxima = 300
                asientos_business = 30
                asientos_turista = 270

            
            ocupacion_percent = round((pasajeros / capacidad_maxima) * 100, 1)
            precio_medio = round(random.uniform(100, 300), 1)
            ingresos = round(pasajeros * precio_medio, 1)
            costes_operacion = round(random.uniform(14000, 50000), 1)
            satisfaccion_media = round(random.uniform(3.7, 4.9), 1)
            incidencias = random.randint(0, 3)
            clima_origen = random.choice(['Soleado', 'Nublado', 'Lluvia'])
            clima_destino = random.choice(['Soleado', 'Nublado', 'Lluvia'])
            visibilidad_origen = random.randint(2, 10) if clima_origen == 'Lluvia' else 10
            visibilidad_destino = random.randint(2, 10) if clima_destino == 'Lluvia' else 10
            viento_origen = round(random.uniform(12, 22), 1)
            viento_destino = round(random.uniform(12, 22), 1)
            presion_origen = round(random.uniform(1008, 1020), 1)
            presion_destino = round(random.uniform(1008, 1020), 1)
            humedad_origen = random.randint(60, 90)
            humedad_destino = random.randint(60, 90)
            conexiones = random.randint(1, 5)
            equipaje_perdido = random.randint(0, 2)
            comidas_servidas = int(pasajeros * 0.9)
            bebidas_servidas = pasajeros * 2
            peliculas_disponibles = random.randint(40, 65)
            wifi_disponible = random.choice(['true', 'false'])
            asientos_preferentes = int(capacidad_maxima * 0.1)
            mascotas_abordo = random.randint(0, 8)
            asistencias_especiales = random.randint(1, 10)
            edad_media_pasajeros = round(random.uniform(33, 50), 1)
            satisfaccion_comida = round(random.uniform(3.7, 4.9), 1)
            satisfaccion_vuelo = round(random.uniform(3.7, 4.9), 1)
            satisfaccion_tripulacion = round(random.uniform(3.7, 4.9), 1)
            consumo_entretenimiento = round(random.uniform(50, 80), 1)
            uso_wifi_percent = round(random.uniform(60, 90), 1)
            compras_abordo = round(random.uniform(700, 2500), 1)
            nivel_combustible_llegada = random.randint(1300, 2400)
            tiempo_taxi_despegue = random.randint(6, 15)
            tiempo_taxi_aterrizaje = random.randint(5, 12)

            writer.writerow({
                'fecha': fecha,
                'vuelo': vuelo,
                'origen': origen,
                'destino': destino,
                'pasajeros': pasajeros,
                'retraso_minutos': retraso_minutos,
                'combustible_litros': combustible_litros,
                'temperatura': temperatura,
                'tripulacion': tripulacion,
                'equipaje_kg': equipaje_kg,
                'carga_kg': carga_kg,
                'velocidad_crucero': velocidad_crucero,
                'altitud_crucero': altitud_crucero,
                'distancia_km': distancia_km,
                'duracion_prevista': duracion_prevista,
                'duracion_real': duracion_real,
                'escala_tecnica': escala_tecnica,
                'puerta_embarque': puerta_embarque,
                'terminal': terminal,
                'tipo_avion': tipo_avion,
                'capacidad_maxima': capacidad_maxima,
                'asientos_business': asientos_business,
                'asientos_turista': asientos_turista,
                'ocupacion_percent': ocupacion_percent,
                'precio_medio': precio_medio,
                'ingresos': ingresos,
                'costes_operacion': costes_operacion,
                'satisfaccion_media': satisfaccion_media,
                'incidencias': incidencias,
                'clima_origen': clima_origen,
                'clima_destino': clima_destino,
                'visibilidad_origen': visibilidad_origen,
                'visibilidad_destino': visibilidad_destino,
                'viento_origen': viento_origen,
                'viento_destino': viento_destino,
                'presion_origen': presion_origen,
                'presion_destino': presion_destino,
                'humedad_origen': humedad_origen,
                'humedad_destino': humedad_destino,
                'conexiones': conexiones,
                'equipaje_perdido': equipaje_perdido,
                'comidas_servidas': comidas_servidas,
                'bebidas_servidas': bebidas_servidas,
                'peliculas_disponibles': peliculas_disponibles,
                'wifi_disponible': wifi_disponible,
                'asientos_preferentes': asientos_preferentes,
                'mascotas_abordo': mascotas_abordo,
                'asistencias_especiales': asistencias_especiales,
                'edad_media_pasajeros': edad_media_pasajeros,
                'satisfaccion_comida': satisfaccion_comida,
                'satisfaccion_vuelo': satisfaccion_vuelo,
                'satisfaccion_tripulacion': satisfaccion_tripulacion,
                'consumo_entretenimiento': consumo_entretenimiento,
                'uso_wifi_percent': uso_wifi_percent,
                'compras_abordo': compras_abordo,
                'nivel_combustible_llegada': nivel_combustible_llegada,
                'tiempo_taxi_despegue': tiempo_taxi_despegue,
                'tiempo_taxi_aterrizaje': tiempo_taxi_aterrizaje
            })

# Generate data for rows
generate_flight_data(1500)
