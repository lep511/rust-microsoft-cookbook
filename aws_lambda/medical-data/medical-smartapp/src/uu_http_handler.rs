use lambda_http::{Body, Error, Request, RequestExt, Response};
use lambda_http::tracing::{error, info};
use anyhow::anyhow;
use openidconnect::{
    AccessTokenHash,
    AuthenticationFlow,
    AuthorizationCode,
    ClientId,
    ClientSecret,
    CsrfToken,
    IssuerUrl,
    Nonce,
    OAuth2TokenResponse,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
};
use openidconnect::core::{
  CoreAuthenticationFlow,
  CoreClient,
  CoreProviderMetadata,
  CoreResponseType,
  CoreUserInfoClaims,
};
use openidconnect::reqwest;
use url::Url;
use std::env;

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    info!("Event: {:?}", event);

    // Get Smart App callback
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let url_str = event.uri().to_string();
    
    let (resource, version) = match extract_resource_ver(&url_str) {
        Ok(resource) => resource,
        Err(e) => {
            error!("Error extracting resource and version: {}", e);
            return Ok(Response::builder()
                .status(404)
                .header("content-type", "text/html")
                .body("<!DOCTYPE html><html><head><title>Not Found</title></head><body><h1>Not Found.</h1></body></html>".into())
                .map_err(Box::new)?);
        }
    };

    if version == "v1" {
        match resource.as_str() {
            "launch" => {
                info!("Resource: {}", resource);
                let (iss, launch) = match extract_query_params(&url_str) {
                    Ok((iss, launch)) => (iss, launch),
                    Err(e) => {
                        error!("Error extracting query parameters: {}", e);
                        return Ok(Response::builder()
                            .status(404)
                            .header("content-type", "text/html")
                            .body("<!DOCTYPE html><html><head><title>Not Found</title></head><body><h1>Not Found.</h1></body></html>".into())
                            .map_err(Box::new)?);
                    }
                };
            
                info!("iss: {}", iss);
                info!("launch: {}", launch);

            }
            "callback" => {
                info!("Callback resource: {}", resource);
            }
            "patient" => {
                info!("Patient resource: {}", resource);
            }
            _ => {
                error!("Resource not found: {}", resource);
            }
        }
    }

    let message = get_http_page();

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

async fn oidc_connect(
    client_id: &str,
    redirect_uri: &str,
    iss: &str,
    launch: &str,
) -> Result<(), anyhow::Error> {
    let http_client = reqwest::blocking::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");
    
    // Use OpenID Connect Discovery to fetch the provider metadata.
    let provider_metadata = CoreProviderMetadata::discover(
        &IssuerUrl::new("https://accounts.example.com".to_string())?,
        &http_client,
    )?;
    

    Ok(())
}

fn extract_query_params(
    url_str: &str
) -> Result<(String, String), Box<dyn std::error::Error>> {
    // Parse the URL
    let url = Url::parse(url_str)?;

    // Get the query pairs as a HashMap-like structure
    let query_pairs: Vec<_> = url.query_pairs().collect();

    // Extract 'iss' and 'launch' values
    let iss = query_pairs
        .iter()
        .find(|(key, _)| key == "iss")
        .map(|(_, value)| value.to_string())
        .ok_or("Missing 'iss' parameter")?;

    let launch = query_pairs
        .iter()
        .find(|(key, _)| key == "launch")
        .map(|(_, value)| value.to_string())
        .ok_or("Missing 'launch' parameter")?;

    Ok((iss, launch))
}

fn extract_resource_ver(
    url_str: &str
) -> Result<(String, String), Box<dyn std::error::Error>> {
    // Parse the URL
    let url = Url::parse(url_str)?;

    // Get resource
    let resource = url
        .path_segments()
        .and_then(|mut segments| segments.nth(1))
        .ok_or("Invalid URL format")?
        .to_string();

    // Get version
    let version = url
        .path_segments()
        .and_then(|mut segments| segments.next())
        .ok_or("Invalid URL format")?
        .to_string();

    Ok((resource, version))
}


async fn x_openid_connect(
    client_id: &str,
    redirect_uri: &str,
    iss: &str,
    launch: &str,
) -> Result<(), anyhow::Error> {
    let issuer_url = IssuerUrl::new(iss.to_string()).expect("Invalid issuer URL");
    let client_id = ClientId::new(client_id.to_string());
    let client_secret = ClientSecret::new("secret".to_string());
    let redirect_url = RedirectUrl::new(redirect_uri.to_string()).expect("Invalid redirect URL");

    let provider_metadata = CoreProviderMetadata::discover(&issuer_url, reqwest::async_http_client)
        .await
        .map_err(|e| anyhow!("Failed to discover OpenID Provider: {}", e))?;

    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        client_id,
        Some(client_secret),
    );

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_state, nonce) = client
        .authorize_url(
            AuthenticationFlow::<CoreAuthenticationFlow>::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
            [Scope::OpenId, Scope::Profile].to_vec(),
            Some(pkce_challenge),
            Some(redirect_url.clone()),
        )
        .url();

    // Redirect the user to the authorization URL
    info!("Redirecting to: {}", auth_url);

    // Simulate user authentication and obtain the authorization code
    let authorization_code = AuthorizationCode::new("authorization_code".to_string());

    // Exchange the authorization code for an access token
    let token_response = client
        .exchange_code(authorization_code)
        .request_async(reqwest::async_http_client)
        .await
        .map_err(|e| anyhow!("Failed to exchange code for token: {}", e))?;

    // Use the access token to make authenticated requests to the resource server
    let user_info: CoreUserInfoClaims = client
        .user_info(token_response.access_token(), None)
        .map_err(|e| anyhow!("Failed to get user info: {}", e))?
        .request_async(reqwest::async_http_client)
        .await
        .map_err(|e| anyhow!("Failed to get user info: {}", e))?;

    info!("User Info: {:?}", user_info);

    Ok(())

}