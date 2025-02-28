use serde_json::Value;

pub fn get_main_page(
    json_data: &Value,
    iss: &str,
    state: &str,
) -> String {
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

            /* Controls container */
            .controls {
                width: 600px;
                margin: 0 auto 20px auto;
                display: flex;
                gap: 10px;
            }

            /* Search input styling */
            #searchInput {
                flex: 1;
                padding: 8px;
                border: 1px solid #ddd;
                border-radius: 4px;
            }

            /* Dropdown styling */
            #nameSelector {
                padding: 8px;
                border: 1px solid #ddd;
                border-radius: 4px;
                min-width: 200px;
            }

            /* No results message */
            #noResults {
                text-align: center;
                color: #666;
                padding: 10px;
            }

            /* Pagination container */
            .pagination {
                width: 600px;
                margin: 20px auto 0 auto;
                display: flex;
                justify-content: center;
                gap: 5px;
            }

            /* Pagination button styling */
            .pagination button {
                width: 30px;
                height: 30px;
                border: 1px solid #ddd;
                background-color: #fff;
                cursor: pointer;
                border-radius: 4px;
                display: flex;
                align-items: center;
                justify-content: center;
            }

            /* Active page button */
            .pagination button.active {
                background-color: #4682B4;
                color: white;
                border-color: #4682B4;
            }

            /* Hover effect for pagination buttons */
            .pagination button:hover:not(.active) {
                background-color: #f5f5f5;
            }

            /* Select button styling */
            .select-btn {
                background-color: #4682B4;
                color: white;
                border: none;
                border-radius: 4px;
                padding: 4px 8px;
                cursor: pointer;
                font-size: 0.8em;
            }

            .select-btn:hover {
                background-color: #3b6e96;
            }
        </style>
    </head>
    <body>
        <!-- Search and filter controls -->
        <div class="controls">
            <input type="text" id="searchInput" placeholder="Search by name...">
            <select id="nameSelector">
                <option value="">All Patients</option>
                <!-- Options will be populated by JavaScript -->
            </select>
        </div>

        <table>
            <caption>Patient Information</caption>
            <thead>
                <tr>
                    <th>Family Name</th>
                    <th>Given Names</th>
                    <th>Last Modified</th>
                    <th>Action</th>
                </tr>
            </thead>
            <tbody id="tableBody">
                <tr>
                    <td colspan="4" id="loading">Loading data...</td>
                </tr>
            </tbody>
        </table>

        <!-- Pagination controls -->
        <div class="pagination" id="pagination">
            <!-- Pagination buttons will be populated by JavaScript -->
        </div>

        <!-- Embedded JSON data -->
        <script type="application/json" id="patientData">
            <json_data_placeholder>
        </script>

        <script>
            // Store all patient data for filtering
            let allPatients = [];
            let filteredPatients = [];
            let currentPage = 1;
            const patientsPerPage = 5;

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

            // Function to handle patient selection
            function selectPatient(patientId, familyName, givenNames) {
                console.log(`Selected patient: ${familyName}, ${givenNames} (ID: ${patientId})`);
                window.location.href = `/tasks?patient_id=${patientId}&iss=<iss_value>&state=<state_value>`;
            }

            // Function to populate dropdown with patient names
            function populateDropdown(patients) {
                const dropdown = document.getElementById('nameSelector');
                
                // Create a set to store unique family names
                const uniqueFamilyNames = new Set();
                
                // Add family names to the set
                patients.forEach(entry => {
                    const patient = entry.resource;
                    const nameInfo = getNameInfo(patient);
                    if (nameInfo.family !== "No family name") {
                        uniqueFamilyNames.add(nameInfo.family);
                    }
                });
                
                // Convert set to array and sort alphabetically
                const sortedNames = Array.from(uniqueFamilyNames).sort();
                
                // Add options to dropdown
                sortedNames.forEach(familyName => {
                    const option = document.createElement('option');
                    option.value = familyName;
                    option.textContent = familyName;
                    dropdown.appendChild(option);
                });
            }

            // Function to create pagination buttons
            function createPagination() {
                const pagination = document.getElementById('pagination');
                pagination.innerHTML = '';
                
                // Calculate total number of pages
                const totalPages = Math.ceil(filteredPatients.length / patientsPerPage);
                
                // Don't show pagination if there's only one page or no results
                if (totalPages <= 1) {
                    return;
                }
                
                // Create page buttons
                for (let i = 1; i <= totalPages; i++) {
                    const button = document.createElement('button');
                    button.textContent = i;
                    button.dataset.page = i;
                    
                    if (i === currentPage) {
                        button.classList.add('active');
                    }
                    
                    button.addEventListener('click', () => {
                        currentPage = parseInt(button.dataset.page);
                        displayPatients();
                        updatePaginationActiveState();
                    });
                    
                    pagination.appendChild(button);
                }
            }

            // Function to update which pagination button is active
            function updatePaginationActiveState() {
                const buttons = document.querySelectorAll('#pagination button');
                buttons.forEach(button => {
                    if (parseInt(button.dataset.page) === currentPage) {
                        button.classList.add('active');
                    } else {
                        button.classList.remove('active');
                    }
                });
            }

            // Function to display patients for the current page
            function displayPatients() {
                const tableBody = document.getElementById('tableBody');
                tableBody.innerHTML = '';
                
                if (filteredPatients.length === 0) {
                    tableBody.innerHTML = `
                        <tr>
                            <td colspan="4" id="noResults">No matching patients found</td>
                        </tr>
                    `;
                    return;
                }
                
                // Calculate start and end index for current page
                const startIndex = (currentPage - 1) * patientsPerPage;
                const endIndex = Math.min(startIndex + patientsPerPage, filteredPatients.length);
                
                // Display only patients for current page
                for (let i = startIndex; i < endIndex; i++) {
                    const entry = filteredPatients[i];
                    const patient = entry.resource;
                    const patientId = entry.fullUrl.split('/').pop();
                    const lastModified = entry.response?.lastModified;
                    const nameInfo = getNameInfo(patient);
                    
                    const row = document.createElement('tr');
                    row.innerHTML = `
                        <td>${nameInfo.family}</td>
                        <td>${nameInfo.given}</td>
                        <td>${lastModified ? formatDate(lastModified) : 'N/A'}</td>
                        <td><button class="select-btn" data-id="${patientId}">Select</button></td>
                    `;
                    
                    // Add event listener to the select button
                    row.querySelector('.select-btn').addEventListener('click', function() {
                        selectPatient(patientId, nameInfo.family, nameInfo.given);
                    });
                    
                    tableBody.appendChild(row);
                }
            }

            // Function to filter patients based on search input and dropdown
            function filterPatients() {
                const searchTerm = document.getElementById('searchInput').value.toLowerCase();
                const selectedFamily = document.getElementById('nameSelector').value;
                
                // Reset to first page when filter changes
                currentPage = 1;
                
                // Filter patients based on criteria
                filteredPatients = allPatients.filter(entry => {
                    const patient = entry.resource;
                    const nameInfo = getNameInfo(patient);
                    
                    const matchesSearch = searchTerm === '' || 
                        nameInfo.family.toLowerCase().includes(searchTerm) || 
                        nameInfo.given.toLowerCase().includes(searchTerm);
                    
                    const matchesDropdown = selectedFamily === '' || 
                        nameInfo.family === selectedFamily;
                    
                    return matchesSearch && matchesDropdown;
                });
                
                // Update the display
                displayPatients();
                createPagination();
            }

            // Function to load data and set up event listeners
            function loadData() {
                try {
                    // Get the JSON data from the embedded script tag
                    const jsonElement = document.getElementById('patientData');
                    const jsonData = JSON.parse(jsonElement.textContent);
                    
                    // Make sure we have entries to process
                    if (!jsonData.entry || jsonData.entry.length === 0) {
                        const tableBody = document.getElementById('tableBody');
                        tableBody.innerHTML = '<tr><td colspan="4">No patient data available</td></tr>';
                        return;
                    }
                    
                    // Store all patients for filtering
                    allPatients = jsonData.entry;
                    filteredPatients = [...allPatients];
                    
                    // Populate dropdown with patient names
                    populateDropdown(allPatients);
                    
                    // Initial display of patients
                    displayPatients();
                    createPagination();
                    
                    // Set up event listeners for search and dropdown
                    document.getElementById('searchInput').addEventListener('input', filterPatients);
                    document.getElementById('nameSelector').addEventListener('change', filterPatients);
                    
                } catch (error) {
                    // Handle errors by displaying a message in the table
                    const tableBody = document.getElementById('tableBody');
                    tableBody.innerHTML = `
                        <tr>
                            <td colspan="4">Error loading data: ${error.message}</td>
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
    let response_fmt = response_fmt.replace("<iss_value>", iss);
    let response_fmt = response_fmt.replace("<state_value>", state);
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
                <p>&copy; Hackathon - Predictive AI In Healthcare with FHIR.<br>
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
            :root {
                --primary-color: #4a6cf7;
            }
            
            body {
                font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                text-align: center;
                margin: 0;
                padding: 0;
                height: 100vh;
                display: flex;
                align-items: center;
                justify-content: center;
                background-color: #f8f9fa;
            }
            
            .container {
                width: 90%;
                max-width: 500px;
                margin: 0 auto;
                padding: 2rem;
                background-color: white;
                border-radius: 12px;
                box-shadow: 0 10px 25px rgba(0,0,0,0.05);
            }
            
            h1 {
                color: #1a1a1a;
                font-size: 1.8rem;
                font-weight: 600;
                margin-bottom: 0.5rem;
            }
            
            p {
                color: #666;
                line-height: 1.6;
                margin: 0.75rem 0;
            }
            
            .redirect-link {
                color: var(--primary-color);
                text-decoration: none;
                font-weight: 500;
                transition: all 0.2s ease;
            }
            
            .redirect-link:hover {
                text-decoration: underline;
            }
            
            /* Loading animation */
            .loading-container {
                display: flex;
                justify-content: center;
                margin: 1.5rem 0;
            }
            
            .spinner {
                width: 40px;
                height: 40px;
                position: relative;
            }
            
            .spinner-circle {
                box-sizing: border-box;
                width: 100%;
                height: 100%;
                border: 4px solid #eee;
                border-top-color: var(--primary-color);
                border-radius: 50%;
                animation: spinner 0.8s linear infinite;
            }
            
            .progress-bar {
                height: 4px;
                width: 100%;
                background-color: #eee;
                border-radius: 4px;
                overflow: hidden;
                margin: 1rem 0;
                position: relative;
            }
            
            .progress-fill {
                height: 100%;
                width: 0%;
                background-color: var(--primary-color);
                border-radius: 4px;
                transition: width 0.1s linear;
            }
            
            .countdown {
                font-size: 0.875rem;
                color: #888;
                margin-top: 0.5rem;
            }
            
            @keyframes spinner {
                to {
                    transform: rotate(360deg);
                }
            }
            
            @media (max-width: 480px) {
                .container {
                    padding: 1.5rem;
                }
                
                h1 {
                    font-size: 1.5rem;
                }
            }
        </style>
    </head>
    <body>
        <div class="container">
            <h1>Redirecting...</h1>
            <p>You are being redirected to APP.</p>
            
            <div class="loading-container">
                <div class="spinner">
                    <div class="spinner-circle"></div>
                </div>
            </div>
            
            <div class="progress-bar">
                <div class="progress-fill" id="progress-fill"></div>
            </div>
            
            <div class="countdown" id="countdown"></div>
            
            <p>If you are not redirected automatically, 
                <a href="<url_to_redirect>" class="redirect-link">click here</a>.
            </p>
        </div>

        <script>
            document.addEventListener('DOMContentLoaded', () => {
                // Target URL
                const targetUrl = "<url_to_redirect>";
                
                // Redirect timeout in milliseconds
                const redirectTime = 3000;
                
                // Progress animation
                const progressFill = document.getElementById('progress-fill');
                const countdown = document.getElementById('countdown');
                
                // Start time
                const startTime = Date.now();
                
                // Update progress bar and countdown
                function updateProgress() {
                    const elapsedTime = Date.now() - startTime;
                    const remainingTime = Math.max(0, redirectTime - elapsedTime);
                    const progress = (elapsedTime / redirectTime) * 100;
                    
                    // Update progress bar
                    progressFill.style.width = `${Math.min(progress, 100)}%`;
                    
                    // Update countdown text
                    countdown.textContent = `Redirecting in ${Math.ceil(remainingTime/1000)} second${Math.ceil(remainingTime/1000) !== 1 ? 's' : ''}`;
                    
                    // Continue updating if there's time left
                    if (remainingTime > 0) {
                        requestAnimationFrame(updateProgress);
                    }
                }
                
                // Start progress animation
                updateProgress();
                
                // Set redirect timeout
                setTimeout(() => {
                    window.location.href = targetUrl;
                }, redirectTime);
                
                // Make sure the redirect link works
                document.querySelector('.redirect-link').addEventListener('click', (e) => {
                    e.preventDefault();
                    window.location.href = targetUrl;
                });
            });
        </script>
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
            <p>&copy; Hackathon - Predictive AI In Healthcare with FHIR.<br>
            Need help? <a href="/support">Contact Support</a> | <a href="/privacy">Privacy Policy</a> | <a href="/terms">Terms of Service</a></p>
        </footer>
        </body>
        </html>
    "#;
    
    let response_fmt = response.replace("<error_code>", error_code);
    response_fmt
}

pub fn get_server_error(error_code: &str) -> String {
    let response = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Error 500 - Internal Server Error</title>
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
            <h1>Error 500: Internal Server Error</h1>
            <p>We apologize, but something went wrong on our server while processing your request. Please try again later or contact support if the problem persists.</p>
            <div class="security-badge">
            Error code: <error_code>
            </div>
        </div>
        <footer class="footer">
            <p>&copy; Hackathon - Predictive AI In Healthcare with FHIR.<br>
            Need help? <a href="/support">Contact Support</a> | <a href="/privacy">Privacy Policy</a> | <a href="/terms">Terms of Service</a></p>
        </footer>
        </body>
        </html>
    "#;
    
    let response_fmt = response.replace("<error_code>", error_code);
    response_fmt
}


