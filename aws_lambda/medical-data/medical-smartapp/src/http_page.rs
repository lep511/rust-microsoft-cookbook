pub fn get_http_page() -> String {
    let response = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Minimalistic Table with GET Data</title>
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

                /* Right-align the Company column */
                td:nth-child(3), th:nth-child(3) {
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
                <caption>User Information</caption>
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Email</th>
                        <th>Company</th>
                    </tr>
                </thead>
                <tbody id="tableBody">
                    <tr>
                        <td colspan="3" id="loading">Loading data...</td>
                    </tr>
                </tbody>
            </table>

            <script>
                // Function to fetch and populate table with data
                async function fetchData() {
                    try {
                        // Fetch data from the API using GET request
                        const response = await fetch('https://jsonplaceholder.typicode.com/users');
                        
                        // Check if the response is successful
                        if (!response.ok) {
                            throw new Error('Network response was not ok');
                        }

                        // Parse the JSON data
                        const data = await response.json();

                        // Get the table body element
                        const tableBody = document.getElementById('tableBody');
                        
                        // Clear the loading message
                        tableBody.innerHTML = '';

                        // Populate the table with fetched data
                        data.forEach(user => {
                            const row = document.createElement('tr');
                            row.innerHTML = `
                                <td>${user.name}</td>
                                <td>${user.email}</td>
                                <td>${user.company.name}</td>
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
                    }
                }

                // Call the function when the page loads
                window.onload = fetchData;
            </script>
        </body>
        </html>
    "#.to_string();
    response
}

pub fn get_connect_page(auth_link: &str) -> String {
    let support_link = "XXXXXXXXXXXXXXX";
    let privacy_link = "XXXXXXXXXXXXXXX";
    let terms_link = "XXXXXXXXXXX";
    
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
                Need help? <a href="<support>">Contact Support</a> | <a href="<privacy>">Privacy Policy</a> | <a href="<terms>">Terms of Service</a></p>
            </footer>
        </body>
        </html>
    "#;

    let response_fmt = response.replace("<authorize>", auth_link)
        .replace("<support>", support_link)
        .replace("<privacy>", privacy_link)
        .replace("<terms>", terms_link);

    response_fmt
}
