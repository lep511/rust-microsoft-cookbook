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
    // Link to another page
    window.location.href = `https://myappdata.com?id=${patientId}`;
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