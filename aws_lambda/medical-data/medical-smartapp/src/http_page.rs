use serde_json::Value;

pub fn get_main_page(json_data: &Value) -> String {
    let response = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Patients Information Table</title>
        <style>
            /* Reset default margins and set a clean font */
            body {
                font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Fira Sans", "Droid Sans", "Helvetica Neue", sans-serif;
                font-size: 16px;
                line-height: 1.5;
                padding: 20px;
                margin: 0;
            }

            /* Table styling */
            table {
                width: 600px;
                margin: 0 auto;
                border-collapse: collapse;
            }

            /* Caption styling */
            caption {
                text-align: left;
                font-size: 1.2em;
                margin-bottom: 10px;
            }

            /* Shared cell styling */
            th, td {
                border-bottom: 1px solid #ddd;
                padding: 8px;
                text-align: left;
            }

            /* Header-specific styling */
            th {
                font-weight: bold;
            }

            /* Right-align the last column */
            td:last-child, th:last-child {
                text-align: right;
            }

            /* Subtle alternating row colors */
            tbody tr:nth-child(even) {
                background-color: #f9f9f9;
            }

            /* Hover effect for interactivity */
            tbody tr:hover {
                background-color: #f5f5f5;
            }

            /* Loading state */
            #loading {
                text-align: center;
                color: #666;
            }
        </style>
    </head>
    <body>
        <table>
            <caption>Patient Information</caption>
            <thead>
                <tr>
                    <th>Family Name</th>
                    <th>Given Names</th>
                    <th>Last Modified</th>
                </tr>
            </thead>
            <tbody id="tableBody">
                <tr>
                    <td colspan="3" id="loading">Loading data...</td>
                </tr>
            </tbody>
        </table>

        <!-- Embedded JSON data -->
        <script type="application/json" id="patientData">
        <json_data_placeholder>
        </script>

        <script>
            // Function to format date string for better readability
            function formatDate(dateString) {
                try {
                    const date = new Date(dateString);
                    return date.toLocaleString();
                } catch (e) {
                    return dateString;
                }
            }

            // Function to get name information safely
            function getNameInfo(patient) {
                // If patient has no name array or empty array
                if (!patient.name || patient.name.length === 0) {
                    return {
                        family: "No name provided",
                        given: "No name provided"
                    };
                }

                // Use the first name entry that has a family name
                const nameEntry = patient.name.find(n => n.family) || patient.name[0];
                
                return {
                    family: nameEntry.family || "No family name",
                    given: nameEntry.given ? nameEntry.given.join(" ") : "No given name"
                };
            }

            // Function to populate table with embedded JSON data
            function loadData() {
                try {
                    // Get the JSON data from the embedded script tag
                    const jsonElement = document.getElementById('patientData');
                    const jsonData = JSON.parse(jsonElement.textContent);

                    // Get the table body element
                    const tableBody = document.getElementById('tableBody');
                    
                    // Clear the loading message
                    tableBody.innerHTML = '';

                    // Make sure we have entries to process
                    if (!jsonData.entry || jsonData.entry.length === 0) {
                        tableBody.innerHTML = '<tr><td colspan="3">No patient data available</td></tr>';
                        return;
                    }

                    // Populate the table with patient data
                    jsonData.entry.forEach(entry => {
                        const patient = entry.resource;
                        const lastModified = entry.response?.lastModified;
                        
                        // Get name information
                        const nameInfo = getNameInfo(patient);
                        
                        const row = document.createElement('tr');
                        row.innerHTML = `
                            <td>${nameInfo.family}</td>
                            <td>${nameInfo.given}</td>
                            <td>${lastModified ? formatDate(lastModified) : 'N/A'}</td>
                        `;
                        tableBody.appendChild(row);
                    });
                } catch (error) {
                    // Handle errors by displaying a message in the table
                    const tableBody = document.getElementById('tableBody');
                    tableBody.innerHTML = `
                        <tr>
                            <td colspan="3">Error loading data: ${error.message}</td>
                        </tr>
                    `;
                    console.error("Error loading data:", error);
                }
            }

            // Call the function when the page loads
            window.onload = loadData;
        </script>
    </body>
    </html>
    "#.to_string();

    let json_data_str = match serde_json::to_string_pretty(json_data) {
        Ok(json_str) => json_str,
        Err(_) => String::from("{}"),
    };
    let response_fmt = response.replace("<json_data_placeholder>", &json_data_str);
    response_fmt
}

pub fn get_connect_page(auth_link: &str) -> String {
    
    let response = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Authorize App - MeldRx</title>
            <style>
                body {
                    font-family: 'Inter', -apple-system, BlinkMacSystemFont, Arial, sans-serif;
                    background-color: #f8fafc;
                    margin: 0;
                    padding: 0;
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    min-height: 100vh;
                    color: #1e293b;
                }
                .container {
                    max-width: 600px;
                    width: 90%;
                    padding: 48px;
                    background-color: white;
                    border-radius: 16px;
                    box-shadow: 0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -1px rgba(0,0,0,0.06);
                    text-align: center;
                    margin: 20px;
                }
                h1 {
                    font-size: 28px;
                    margin-bottom: 16px;
                    color: #0f172a;
                    font-weight: 600;
                }
                p {
                    font-size: 16px;
                    margin-bottom: 24px;
                    color: #475569;
                    line-height: 1.6;
                }
                .button {
                    display: inline-block;
                    padding: 14px 32px;
                    background-color: #3b82f6;
                    color: white;
                    text-decoration: none;
                    border-radius: 8px;
                    font-size: 16px;
                    font-weight: 500;
                    transition: all 0.2s ease;
                }
                .button:hover {
                    background-color: #2563eb;
                    transform: translateY(-1px);
                    box-shadow: 0 4px 6px -1px rgba(59, 130, 246, 0.2);
                }
                .footer {
                    margin-top: 24px;
                    padding: 16px;
                    text-align: center;
                    font-size: 14px;
                    color: #64748b;
                }
                .footer a {
                    color: #3b82f6;
                    text-decoration: none;
                }
                .footer a:hover {
                    text-decoration: underline;
                }
                .security-badge {
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    margin-top: 24px;
                    color: #64748b;
                    font-size: 14px;
                }
                .security-badge svg {
                    margin-right: 8px;
                }
                @media (max-width: 600px) {
                    .container {
                        padding: 32px 24px;
                        width: 85%;
                    }
                    h1 {
                        font-size: 24px;
                    }
                    p {
                        font-size: 15px;
                    }
                    .button {
                        font-size: 15px;
                        padding: 12px 24px;
                        width: 100%;
                        box-sizing: border-box;
                    }
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Authorize App for MeldRx</h1>
                <p>Authorize the application to access your healthcare data via MeldRx. Your data will be handled securely and in compliance with HIPAA regulations.</p>
                <a href="<authorize>" class="button">Authorize App</a>
                <div class="security-badge">
                    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M8 0L2 2V7.5C2 11.5 4.5 15 8 16C11.5 15 14 11.5 14 7.5V2L8 0Z" fill="\#64748b"/>
                    </svg>
                    Secure HIPAA-Compliant Authorization
                </div>
            </div>
            <footer class="footer">
                <p>&copy; 2025 MeldRx. All rights reserved.<br>
                Need help? <a href="/support">Contact Support</a> | <a href="/privacy">Privacy Policy</a> | <a href="/terms">Terms of Service</a></p>
            </footer>
        </body>
        </html>
    "#;

    let response_fmt = response.replace("<authorize>", auth_link);
    response_fmt
}

pub fn redirect_url(url: &str) -> String {
    let response = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Redirecting...</title>
        <style>
            body {
                font-family: Arial, sans-serif;
                text-align: center;
                margin-top: 100px;
                background-color: #f5f5f5;
            }
            .container {
                max-width: 600px;
                margin: 0 auto;
                padding: 20px;
                background-color: white;
                border-radius: 5px;
                box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            }
            h1 {
                color: #333;
            }
            p {
                color: #666;
            }
            .loading {
                margin: 20px 0;
                font-size: 14px;
            }
            .redirect-link {
                color: #0066cc;
                text-decoration: underline;
                cursor: pointer;
            }
        </style>
        <script>
            // Redirect to the target URL after page loads
            window.onload = function() {
                // Change the URL below to your target destination
                const targetUrl = "<url_to_redirect>";
                
                // Set a small timeout to allow users to see the redirect message
                setTimeout(function() {
                    window.location.href = targetUrl;
                }, 2000);
            };
        </script>
    </head>
    <body>
        <div class="container">
            <h1>Redirecting...</h1>
            <p>You are being redirected to APP.</p>
            <div class="loading">Please wait...</div>
            <p>If you are not redirected automatically, 
            <a href="<url_to_redirect>" class="redirect-link" 
                onclick="window.location.href='<url_to_redirect>'; return false;">click here</a>.
            </p>
        </div>
    </body>
    </html>
    "#;
    
    let response_fmt = response.replace("<url_to_redirect>", url);
    response_fmt
}

pub fn get_error_page(error_code: &str) -> String {
    let response = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Error 404 - Page Not Found</title>
        <style>
            body {
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, Arial, sans-serif;
            background-color: #f8fafc;
            margin: 0;
            padding: 0;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            color: #1e293b;
            }
            .container {
            max-width: 600px;
            width: 90%;
            padding: 48px;
            background-color: white;
            border-radius: 16px;
            box-shadow: 0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -1px rgba(0,0,0,0.06);
            text-align: center;
            margin: 20px;
            }
            h1 {
            font-size: 28px;
            margin-bottom: 16px;
            color: #0f172a;
            font-weight: 600;
            }
            p {
            font-size: 16px;
            margin-bottom: 24px;
            color: #475569;
            line-height: 1.6;
            }
            .button {
            display: inline-block;
            padding: 14px 32px;
            background-color: #3b82f6;
            color: white;
            text-decoration: none;
            border-radius: 8px;
            font-size: 16px;
            font-weight: 500;
            transition: all 0.2s ease;
            }
            .button:hover {
            background-color: #2563eb;
            transform: translateY(-1px);
            box-shadow: 0 4px 6px -1px rgba(59, 130, 246, 0.2);
            }
            .footer {
            margin-top: 24px;
            padding: 16px;
            text-align: center;
            font-size: 14px;
            color: #64748b;
            }
            .footer a {
            color: #3b82f6;
            text-decoration: none;
            }
            .footer a:hover {
            text-decoration: underline;
            }
            .security-badge {
            display: flex;
            align-items: center;
            justify-content: center;
            margin-top: 24px;
            color: #64748b;
            font-size: 12px;
            }
            .security-badge svg {
            margin-right: 8px;
            }
            @media (max-width: 600px) {
            .container {
                padding: 32px 24px;
                width: 85%;
            }
            h1 {
                font-size: 24px;
            }
            p {
                font-size: 15px;
            }
            .button {
                font-size: 15px;
                padding: 12px 24px;
                width: 100%;
                box-sizing: border-box;
            }
            }
        </style>
        </head>
        <body>
        <div class="container">
            <h1>Error 404: Page Not Found</h1>
            <p>We're sorry, the page you're looking for doesn't exist or has been moved. Please check the URL or return to the homepage.</p>
            <div class="security-badge">
            Error code: <error_code>
            </div>
        </div>
        <footer class="footer">
            <p>&copy; 2025 MeldRx. All rights reserved.<br>
            Need help? <a href="/support">Contact Support</a> | <a href="/privacy">Privacy Policy</a> | <a href="/terms">Terms of Service</a></p>
        </footer>
        </body>
        </html>
    "#;
    
    let response_fmt = response.replace("<error_code>", error_code);
    response_fmt
}


