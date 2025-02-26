use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Jwk {
    kid: String,  // Identificador de la clave
    kty: String,  // Tipo de clave (e.g., RSA)
    alg: Option<String>,  // Algoritmo (e.g., RS256)
    n: String,    // Módulo RSA
    e: String,    // Exponente RSA
    #[serde(default)]
    use_: Option<String>,    // Uso de la clave
}

#[derive(Debug, Serialize, Deserialize)]
struct Jwks {
    keys: Vec<Jwk>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,         // Identificador único del usuario
    name: Option<String>, // Nombre del usuario (opcional)
    email: Option<String>, // Correo del usuario (opcional)
    iss: String,         // Emisor del token
    aud: String,         // Audiencia (client_id de tu app)
    exp: i64,            // Tiempo de expiración (timestamp)
}

async fn get_jwks() -> Result<Jwks, Box<dyn Error>> {
    // Instead of making an HTTP request, use the embedded JWKs
    let jwks_json = r#"{
      "keys": [
        {
          "kty": "RSA",
          "use": "sig",
          "kid": "5d12ab782cb6096285f69e48aea99079bb59cb86",
          "n": "uac7NRcojCutcceWq1nrpLGJjQ7ywvgWsUcb1DWMKJ3KNNHiRzh9jshoi9tmq1zlarJ_h7GQg8iU1qD7SgpVYJmjlKG1MNVRAtuNrNMC0UAnNfG7mBBNorHFndfp-9cLTiMjXSXRzhNqiMvTVKeolRdMB2lH9RzJnwlpXOlPlMaOy1zxUnHn0uszU5mPRQk79i03BNrAdhwrAUB-ZuMnqpjaUcb9VU3KIwuZNPtsVenLN12sRYpaZ6WBw8Q9q7fAoaJUovM0Go8deC9pJYyxJuHdVo9HP0osyzg3g_rOYi14wmvMBuiDf3F4pTnudAfFyl3d0Mn_i4ZQ",
          "e": "AQAB",
          "alg": "RS256"
        },
        {
          "use": "sig",
          "kid": "763f7c4cd26a1eb2b1b39a88f4434d1f4d9a368b",
          "n": "y8TPCPz2Fp0OhBxsxu6d_7erT9f9XJ7mx7ZJPkkeZRxhdnKtg327D4IGYsC4fLAfpkC8qN58sZGkwRTNs-i7yaoD5_8nupq1tPYvnt38ddVghG9vws-2MvxfPQ9m2uxBEdRHmels8prEYGCH6oFKcuWVsNOt4l_OPoJRl4uiuiwd6trZik2GqDD_M6bn21_w6AD_jmbzN4mh8Od4vkA1Z9lKb3Qesksxdog-LWHsljN8ieiz1NhbG7M-GsIlzu-typJfud3tSJ1QHb-E_dEfoZ1iYK7pMcojb5ylMkaCj5QySRdJESq9ngqVRDjF4nX8DK5RQUS7AkrpHiwqyW0Csw",
          "alg": "RS256",
          "e": "AQAB",
          "kty": "RSA"
        },
        {
          "n": "0qTcwnqUqJqsyu57JAC4IOAgTuMrccabAKKj5T93F68NoCk4kAax0oJhDArisYpiLrQ__YJJ9HFm3TKkuiPZeb1xqSSXAnIZVo8UigTLQDQLCTq3O-aD5EyQTOhOHWxJBZcpyLO-dZVuOIbv8fNMcXpNCioHVHO04gI_mvaw8ZzbU_j8ZeHSPk4wTBNfmH4l0mYRDhoQHLkZxxvc2V71ppBPYbnX-4t6h7XcuTkLJKBxfrR43G5nNzDuFsIbBnS2fjVLEv_1LYj9G5Q5XwiCFS0BON-oqQNzRWF53nkf91bMm2TaROg21KKJbZqfEjUhCVlMDFmBW-MNv69-C19PZQ",
          "alg": "RS256",
          "use": "sig",
          "kty": "RSA",
          "kid": "25f8211713788b6145474b5029b0141bd5b3de9c",
          "e": "AQAB"
        }
      ]
    }"#;
    
    let jwks: Jwks = serde_json::from_str(jwks_json)?;
    Ok(jwks)
}

fn validate_id_token(token: &str, jwks: &Jwks, client_id: &str) -> Result<Claims, Box<dyn Error>> {
    // Obtener el encabezado del token para encontrar el 'kid'
    let header = jsonwebtoken::decode_header(token)?;
    let kid = header.kid.ok_or("Falta el 'kid' en el encabezado del token")?;

    // Buscar la clave JWK correspondiente al 'kid'
    let jwk = jwks.keys.iter().find(|k| k.kid == kid).ok_or("No se encontró una JWK coincidente")?;

    // Crear la clave de decodificación a partir de los componentes RSA
    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?;

    // Configurar las reglas de validación
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[client_id]); // Verificar que la audiencia coincide con tu client_id
    validation.set_issuer(&["https://accounts.google.com", "accounts.google.com"]); // Emisores válidos

    // Decodificar y validar el token
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;

    Ok(token_data.claims)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let id_token = "eyJhbGciOiJSUzI1NiIsImtpZCI6IjVBRkIzNzEzRTEwODJCODc2OTlERDIzRUVDRDY0MjExIiwidHlwIjoiSldUIn0.eyJpc3MiOiJodHRwczovL2FwcC5tZWxkcnguY29tIiwibmJmIjoxNzQwNTMzNzQ5LCJpYXQiOjE3NDA1MzM3NDksImV4cCI6MTc0MDUzNDA0OSwiYXVkIjoiNjNmZTJiY2NmZWJlNDE1YWIwZjAwOWFmZjg1ZDc5YmIiLCJhbXIiOlsicHdkIl0sImF0X2hhc2giOiJnNk5kbkRLLWpLWGxVdVhvdkpiV0pRIiwic2lkIjoiMjRFNTFEMjM2QTcxOUZFM0VGNkQzMTRERDJDQUZDNUMiLCJzdWIiOiJlMzFlYWUyOC1lNjgwLTRlYjAtYTM4Mi0xNTJhODg0MTgyZGMiLCJhdXRoX3RpbWUiOjE3NDA0ODk2NTYsImlkcCI6ImxvY2FsIn0.QvI4rJ0grXgpHXU0g2YWHW--F6OibaWwFpgIAziElQLGvMMzZiAxvACSYF5q9dBJukx2pKoSFmS6Tri7HJPeF_OK5gQFHzSzQtZntIgl7FdXcVqtKTnRTQtUtNVbn6KIKXz-xT7nFXMILvmG2eKO_u33qyeehtBbjbnx3cAaXuedZ-jNvSeWqydieeYs6XlitIueSCO4J7ZyUmll3WANH0QfNHH3vv4l6eG30KvVQ_UJEIzWXtEDV8PoW237fOJoyiNejey7D8ciS5tSCxA24wlYI8gH92GcHljMf7giV5dcdlxw_eVTtbIoT8RCfafcraYecQpH0J-VXz3P2UltFA";
    let client_id = "63fe2bccfebe415ab0f009aff85d79bb"; // Reemplaza con tu client_id

    // Obtener las claves JWKS usando los valores proporcionados
    let jwks = get_jwks().await?;

    // Validar el token y extraer los claims
    match validate_id_token(id_token, &jwks, client_id) {
        Ok(claims) => {
            println!("Usuario validado:");
            println!("ID del usuario (sub): {}", claims.sub);
            if let Some(name) = claims.name {
                println!("Nombre: {}", name);
            }
            if let Some(email) = claims.email {
                println!("Email: {}", email);
            }
        }
        Err(e) => println!("Error al validar el token: {}", e),
    }

    Ok(())
}