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