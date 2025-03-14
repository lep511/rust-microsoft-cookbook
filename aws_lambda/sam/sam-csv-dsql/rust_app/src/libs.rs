use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FlightData {
    pub year: i32,                      // Año del vuelo
    pub month: i32,                      // Mes del vuelo
    pub day: i32,                        // Día del mes
    pub day_of_week: i32,                // Día de la semana
    pub scheduled_departure: i32,       // Hora programada de salida (HHMM)
    pub actual_departure: i32,          // Hora real de salida (HHMM)
    pub scheduled_arrival: i32,         // Hora programada de llegada (HHMM)
    pub actual_arrival: i32,            // Hora real de llegada (HHMM)
    pub airline_code: String,           // Código de la aerolínea
    pub flight_number: i32,             // Número de vuelo
    pub aircraft_registration: String,  // Matrícula de la aeronave
    pub scheduled_flight_time: i32,     // Tiempo programado de vuelo en minutos
    pub actual_flight_time: i32,        // Tiempo real de vuelo en minutos
    pub air_time: i32,                  // Tiempo en el aire en minutos
    pub departure_delay: i32,           // Retraso en salida en minutos
    pub arrival_delay: i32,             // Retraso en llegada en minutos
    pub origin_airport: String,         // Aeropuerto de salida
    pub destination_airport: String,    // Aeropuerto de llegada
    pub distance: i32,                  // Distancia en millas
    pub taxi_out: i32,                   // Tiempo de taxi al despegar en minutos
    pub taxi_in: i32,                    // Tiempo de taxi al aterrizar en minutos
    pub carrier_delay: i32,              // Retraso por operador de la aerolínea (opcional)
    pub weather_delay: i32,              // Retraso por condiciones meteorológicas (opcional)
    pub security_delay: i32,             // Retraso por problemas de seguridad (opcional)
    pub nas_delay: i32,                  // Retraso por causas del sistema aéreo nacional (opcional)
    pub other_delay: i32,                // Otros tipos de retrasos (opcional)
}

impl FlightData {
    pub fn from_vec(vec: &Vec<&str>) -> Result<Self, String> {
        let year = vec.get(0)
            .ok_or("Missing year field")?
            .parse::<i32>()
            .map_err(|_| "Invalid year value".to_string())?;
            
        let month = vec.get(1)
            .ok_or("Missing month field")?
            .parse::<i32>()
            .map_err(|_| "Invalid month value".to_string())?;

        let day = vec.get(2)
            .ok_or("Missing day field")?
            .parse::<i32>()
            .map_err(|_| "Invalid day value".to_string())?;

        let day_of_week = vec.get(3)
            .ok_or("Missing day_of_week field")?
            .parse::<i32>()
            .map_err(|_| "Invalid day_of_week value".to_string())?;

        let scheduled_departure = vec.get(4)
            .ok_or("Missing scheduled_departure field")?
            .parse::<i32>()
            .unwrap_or(0); // Default to 0 if parsing fails

        let actual_departure = vec.get(5)
            .ok_or("Missing actual_departure field")?
            .parse::<i32>()
            .map_err(|_| "Invalid actual_departure value".to_string())?;

        let scheduled_arrival = vec.get(6)
            .ok_or("Missing scheduled_arrival field")?
            .parse::<i32>()
            .unwrap_or(0); // Default to 0 if parsing fails

        let actual_arrival = vec.get(7)
            .ok_or("Missing actual_arrival field")?
            .parse::<i32>()
            .unwrap_or(0); // Default to 0 if parsing fails

        let airline_code = vec.get(8)
            .ok_or("Missing airline_code field")?
            .to_string();

        let flight_number = vec.get(9)
            .ok_or("Missing flight_number field")?
            .parse::<i32>()
            .unwrap_or(0); // Default to 0 if parsing fails

        let aircraft_registration = vec.get(10)
            .ok_or("Missing aircraft_registration field")?
            .to_string();

        let scheduled_flight_time = vec.get(11)
            .ok_or("Missing scheduled_flight_time field")?
            .parse::<i32>()
            .unwrap_or(0); // Default to 0 if parsing fails

        let actual_flight_time = vec.get(12)
            .ok_or("Missing actual_flight_time field")?
            .parse::<i32>()
            .unwrap_or(0); // Default to 0 if parsing fails

        let air_time = vec.get(13)
            .ok_or("Missing air_time field")?
            .parse::<i32>()
            .unwrap_or(0); // Default to 0 if parsing fails

        let departure_delay = vec.get(14)
            .ok_or("Missing departure_delay field")?
            .parse::<i32>()
            .unwrap_or(0); // Default to 0 if parsing fails

        let arrival_delay = vec.get(15)
            .ok_or("Missing arrival_delay field")?
            .parse::<i32>()
            .unwrap_or(0); // Default to 0 if parsing fails

        let origin_airport = vec.get(16)
            .ok_or("Missing origin_airport field")?
            .to_string();

        let destination_airport = vec.get(17)
            .ok_or("Missing destination_airport field")?
            .to_string();

        let distance = vec.get(18)
            .ok_or("Missing distance field")?
            .parse::<i32>()
            .unwrap_or(0); // Default to 0 if parsing fails

        let taxi_out = vec.get(19)
            .ok_or("Missing taxi_out field")?
            .parse::<i32>()
            .unwrap_or(0); // Default to 0 if parsing fails

        let taxi_in = vec.get(20)
            .ok_or("Missing taxi_in field")?
            .parse::<i32>()
            .unwrap_or(0); // Default to 0 if parsing fails

        let carrier_delay = vec.get(21)
            .ok_or("Missing carrier_delay field")?
            .parse::<i32>()
            .unwrap_or(0); // Default to 0 if parsing fails

        let weather_delay = vec.get(22)
            .ok_or("Missing weather_delay field")?
            .parse::<i32>()
            .unwrap_or(0); // Default to 0 if parsing fails

        let security_delay = vec.get(23)
            .ok_or("Missing taxi_security_delayin field")?
            .parse::<i32>()
            .unwrap_or(0); // Default to 0 if parsing fails

        let nas_delay = vec.get(24)
            .ok_or("Missing nas_delay field")?
            .parse::<i32>()
            .unwrap_or(0); // Default to 0 if parsing fails

        let other_delay = vec.get(25)
            .ok_or("Missing other_delay field")?
            .parse::<i32>()
            .unwrap_or(0); // Default to 0 if parsing fails

        Ok(Self {
            year,
            month,
            day,
            day_of_week,
            scheduled_departure,
            actual_departure,
            scheduled_arrival,
            actual_arrival,
            airline_code,
            flight_number,
            aircraft_registration,
            scheduled_flight_time,
            actual_flight_time,
            air_time,
            departure_delay,
            arrival_delay,
            origin_airport,
            destination_airport,
            distance,
            taxi_out,
            taxi_in,
            carrier_delay,
            weather_delay,
            security_delay,
            nas_delay,
            other_delay,
        })
    }
}

