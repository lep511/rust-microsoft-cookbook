pub fn get_main_page(
    json_data: &str,
) -> String {
    let response = r#"
    <!DOCTYPE html><html lang="en"><head><meta http-equiv="Content-Security-Policy" content="default-src 'self' 'unsafe-inline' 'unsafe-eval' data: blob: https://cdnjs.cloudflare.com https://cdn.jsdelivr.net https://code.jquery.com https://unpkg.com https://d3js.org https://threejs.org https://cdn.plot.ly https://stackpath.bootstrapcdn.com https://maps.googleapis.com https://cdn.tailwindcss.com https://ajax.googleapis.com https://kit.fontawesome.com https://cdn.datatables.net https://maxcdn.bootstrapcdn.com https://code.highcharts.com https://tako-static-assets-production.s3.amazonaws.com https://www.youtube.com https://fonts.googleapis.com https://fonts.gstatic.com https://pfst.cf2.poecdn.net https://puc.poecdn.net https://i.imgur.com https://wikimedia.org https://*.icons8.com https://*.giphy.com https://picsum.photos https://images.unsplash.com; frame-src 'self' https://www.youtube.com https://trytako.com; child-src 'self'; manifest-src 'self'; worker-src 'self'; upgrade-insecure-requests; block-all-mixed-content;">
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Patient Dashboard</title>
        <style>
            :root {
                --dark-bg: #121926;
                --card-bg: #1e2a3b;
                --text-color: #e6e9ed;
                --highlight-color: #3e86f5;
                --accent-color: #304259;
                --chart-color: #46be8a;
            }
            
            * {
                margin: 0;
                padding: 0;
                box-sizing: border-box;
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            }
            
            body {
                background-color: var(--dark-bg);
                color: var(--text-color);
                padding: 0;
                margin: 0;
            }
            
            .container {
                max-width: 1200px;
                margin: 0 auto;
                padding: 0 15px;
            }
            
            .header {
                display: flex;
                align-items: center;
                padding: 20px 0;
                border-bottom: 1px solid var(--accent-color);
            }
            
            .avatar {
                width: 50px;
                height: 50px;
                background-color: var(--accent-color);
                border-radius: 50%;
                display: flex;
                align-items: center;
                justify-content: center;
                margin-right: 15px;
                overflow: hidden;
            }
            
            .avatar svg {
                width: 30px;
                height: 30px;
                fill: var(--text-color);
            }
            
            .patient-info {
                flex-grow: 1;
            }
            
            .patient-name {
                font-size: 22px;
                font-weight: bold;
                margin-bottom: 5px;
            }
            
            .patient-details {
                display: flex;
                gap: 10px;
                color: #a0aec0;
                font-size: 14px;
            }
            
            .patient-details span::after {
                content: "•";
                margin-left: 10px;
            }
            
            .patient-details span:last-child::after {
                content: "";
            }
            
            .contact-info {
                margin-top: 15px;
                display: flex;
                gap: 20px;
                color: #a0aec0;
                font-size: 14px;
            }
            
            .emergency-contact {
                margin-top: 15px;
                font-size: 14px;
            }
            
            .emergency-label {
                font-weight: bold;
                display: block;
                margin-bottom: 5px;
                color: #a0aec0;
            }
            
            .section-title {
                color: #a0aec0;
                font-size: 14px;
                margin-bottom: 10px;
                font-weight: bold;
                text-transform: uppercase;
                letter-spacing: 1px;
            }
            
            .medical-info {
                display: grid;
                grid-template-columns: 1fr 1fr;
                gap: 20px;
                margin-top: 20px;
            }
            
            @media (max-width: 768px) {
                .medical-info {
                    grid-template-columns: 1fr;
                }
            }
            
            .tag-container {
                display: flex;
                flex-wrap: wrap;
                gap: 8px;
            }
            
            .tag {
                background-color: #35415a;
                color: var(--text-color);
                padding: 5px 12px;
                border-radius: 15px;
                font-size: 13px;
            }
            
            .tag.allergy-penicillin {
                background-color: #e53e3e;
            }
            
            .tag.allergy-peanuts {
                background-color: #dd6b20;
            }
            
            .medications {
                margin-top: 20px;
            }
            
            .medication-card {
                background-color: var(--card-bg);
                border-radius: 8px;
                padding: 15px;
                margin-bottom: 10px;
            }
            
            .medication-name {
                font-weight: bold;
                margin-bottom: 5px;
            }
            
            .medication-details {
                color: #a0aec0;
                font-size: 14px;
            }
            
            .tabs {
                display: flex;
                margin-top: 30px;
                border-bottom: 1px solid var(--accent-color);
            }
            
            .tab {
                padding: 10px 20px;
                cursor: pointer;
                opacity: 0.6;
            }
            
            .tab.active {
                border-bottom: 2px solid var(--highlight-color);
                opacity: 1;
            }
            
            .tab-content {
                display: none;
                padding: 20px 0;
            }
            
            .tab-content.active {
                display: block;
            }
            
            .vital-signs-grid {
                display: grid;
                grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
                gap: 15px;
                margin-bottom: 20px;
            }
            
            .vital-card {
                background-color: var(--card-bg);
                border-radius: 8px;
                padding: 15px;
            }
            
            .vital-title {
                color: #a0aec0;
                font-size: 14px;
                margin-bottom: 10px;
            }
            
            .vital-value {
                font-size: 24px;
                font-weight: bold;
            }
            
            .vital-unit {
                font-size: 14px;
                color: #a0aec0;
            }
            
            .chart-tabs {
                display: flex;
                gap: 10px;
                margin-bottom: 15px;
            }
            
            .chart-tab {
                background-color: var(--card-bg);
                padding: 8px 15px;
                border-radius: 5px;
                cursor: pointer;
                font-size: 14px;
            }
            
            .chart-tab.active {
                background-color: var(--accent-color);
            }
            
            .chart-container {
                background-color: var(--card-bg);
                border-radius: 8px;
                padding: 20px;
                height: 300px;
                position: relative;
            }
            
            .chart {
                width: 100%;
                height: 100%;
                position: relative;
            }
            
            .chart-line {
                stroke: var(--chart-color);
                stroke-width: 2;
                fill: none;
            }
            
            .chart-dot {
                fill: var(--chart-color);
            }
            
            .chart-grid-line {
                stroke: #304259;
                stroke-width: 1;
                stroke-dasharray: 3,3;
            }
            
            .chart-label {
                fill: #a0aec0;
                font-size: 12px;
            }
            
            .treatment-card, .appointment-card {
                background-color: var(--card-bg);
                border-radius: 8px;
                padding: 15px;
                margin-bottom: 15px;
            }
            
            .treatment-date, .appointment-date {
                color: #a0aec0;
                font-size: 14px;
                margin-bottom: 5px;
            }
            
            .treatment-type, .appointment-type {
                font-weight: bold;
                margin-bottom: 5px;
            }
            
            .treatment-provider, .appointment-provider {
                font-size: 14px;
                margin-bottom: 5px;
            }
            
            .treatment-notes, .appointment-location {
                color: #a0aec0;
                font-size: 14px;
            }
            
            .timeline {
                margin-top: 20px;
            }
            
            .timeline-item {
                display: flex;
                margin-bottom: 20px;
            }
            
            .timeline-year {
                min-width: 60px;
                font-weight: bold;
            }
            
            .timeline-content {
                margin-left: 20px;
                padding-left: 20px;
                border-left: 1px solid var(--accent-color);
                padding-bottom: 20px;
                position: relative;
            }
            
            .timeline-icon {
                position: absolute;
                left: -18px;
                top: 0;
                background: var(--card-bg);
                border-radius: 50%;
                padding: 5px;
                width: 36px;
                height: 36px;
                display: flex;
                align-items: center;
                justify-content: center;
            }
            
            .timeline-icon svg {
                color: var(--text-color);
            }
            
            .timeline-title {
                font-weight: bold;
                margin-bottom: 5px;
                padding-top: 8px;
            }
            
            .timeline-description {
                color: #a0aec0;
                font-size: 14px;
            }
            
            .timeline-highlight .timeline-title {
                color: var(--highlight-color);
            }
            
            .timeline-highlight .timeline-icon {
                border: 2px solid var(--highlight-color);
            }
        </style>
    </head>
    <body>
        <div class="container">
            <div class="header">
                <div class="avatar">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                        <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"></path>
                    </svg>
                </div>
                <div class="patient-info">
                    <h1 class="patient-name" id="patientName"></h1>
                    <div class="patient-details">
                        <span id="patientId"></span>
                        <span id="patientAge"></span>
                        <span id="patientGender"></span>
                        <span id="patientEthnicity"></span>
                    </div>
                    <div class="contact-info">
                        <span id="patientPhone"></span>
                        <span id="patientEmail"></span>
                    </div>
                    <div class="emergency-contact">
                        <span class="emergency-label">EMERGENCY CONTACT</span>
                        <span id="emergencyContact"></span>
                    </div>
                </div>
            </div>
            
            <div class="medical-info">
                <div>
                    <h2 class="section-title">ALLERGIES</h2>
                    <div class="tag-container" id="allergiesContainer"></div>
                </div>
                <div>
                    <h2 class="section-title">CHRONIC CONDITIONS</h2>
                    <div class="tag-container" id="conditionsContainer"></div>
                </div>
            </div>
            
            <div class="medications">
                <h2 class="section-title">CURRENT MEDICATIONS</h2>
                <div id="medicationsContainer"></div>
            </div>
            
            <div class="tabs">
                <div class="tab active" data-tab="vital-signs">Vital Signs</div>
                <div class="tab" data-tab="timeline">Timeline</div>
                <div class="tab" data-tab="treatment-history">Treatment History</div>
                <div class="tab" data-tab="appointments">Appointments</div>
            </div>
            
            <div class="tab-content active" id="vital-signs">
                <div class="vital-signs-grid" id="vitalSignsGrid"></div>
                
                <div class="chart-tabs">
                    <div class="chart-tab active" data-chart="heart-rate">Heart Rate</div>
                    <div class="chart-tab" data-chart="blood-pressure">Blood Pressure</div>
                    <div class="chart-tab" data-chart="temperature">Temperature</div>
                </div>
                
                <div class="chart-container">
                    <svg class="chart" id="vitalChart" preserveAspectRatio="none"></svg>
                </div>
            </div>
            
            <div class="tab-content" id="timeline">
                <div class="timeline" id="timelineContainer"></div>
            </div>
            
            <div class="tab-content" id="treatment-history">
                <div id="treatmentsContainer"></div>
            </div>
            
            <div class="tab-content" id="appointments">
                <div id="appointmentsContainer"></div>
            </div>
        </div>

        <script>
            // Patient data from JSON
            const patientData = <json_data_placeholder>;

            // Icons collection
            const icons = {
                HeartPulse: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-heart-pulse"><path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z"/><path d="M3.22 12H9.5l.5-1 2 4.5 2-7 1.5 3.5h5.27"/></svg>`,
                Activity: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-activity"><path d="M22 12h-2.48a2 2 0 0 0-1.93 1.46l-2.35 8.36a.25.25 0 0 1-.48 0L9.24 2.18a.25.25 0 0 0-.48 0l-2.35 8.36A2 2 0 0 1 4.49 12H2"/></svg>`,
                Stethoscope: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-stethoscope"><path d="M11 2v2M5 2v2m0-1H4a2 2 0 0 0-2 2v4a6 6 0 0 0 12 0V5a2 2 0 0 0-2-2h-1"/><path d="M8 15a6 6 0 0 0 12 0v-3"/><circle cx="20" cy="10" r="2"/></svg>`,
                Syringe: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-syringe"><path d="m18 2 4 4m-5 1 3-3m-1 5L8.7 19.3c-1 1-2.5 1-3.4 0l-.6-.6c-1-1-1-2.5 0-3.4L15 5m-6 6 4 4m-8 4-3 3M14 4l6 6"/></svg>`,
                Pill: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-pill"><path d="m10.5 20.5 10-10a4.95 4.95 0 1 0-7-7l-10 10a4.95 4.95 0 1 0 7 7m-2-12 7 7"/></svg>`,
                Hospital: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-hospital"><path d="M12 6v4m2 4h-4m4 4h-4m4-10h-4m8 4h2a2 2 0 0 1 2 2v6a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-9a2 2 0 0 1 2-2h2"/><path d="M18 22V4a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v18"/></svg>`
            };

            // Initialize patient information
            document.addEventListener('DOMContentLoaded', function() {
                // Basic patient info
                document.getElementById('patientName').textContent = patientData.name;
                document.getElementById('patientId').textContent = `ID: ${patientData.id}`;
                document.getElementById('patientAge').textContent = `${patientData.age} years`;
                document.getElementById('patientGender').textContent = patientData.gender;
                document.getElementById('patientEthnicity').textContent = `Ethnicity: ${patientData.ethnicity}`;
                document.getElementById('patientPhone').textContent = patientData.phone;
                document.getElementById('patientEmail').textContent = patientData.email;
                document.getElementById('emergencyContact').textContent = patientData.emergencyContact;
                
                // Allergies
                const allergiesContainer = document.getElementById('allergiesContainer');
                patientData.allergies.forEach(allergy => {
                    const allergyCleaned = allergy.toLowerCase().replace(/\s+/g, '-');
                    const tagElement = document.createElement('div');
                    tagElement.className = `tag allergy-${allergyCleaned}`;
                    tagElement.textContent = allergy;
                    allergiesContainer.appendChild(tagElement);
                });
                
                // Chronic conditions
                const conditionsContainer = document.getElementById('conditionsContainer');
                patientData.chronicConditions.forEach(condition => {
                    const tagElement = document.createElement('div');
                    tagElement.className = 'tag';
                    tagElement.textContent = condition;
                    conditionsContainer.appendChild(tagElement);
                });
                
                // Medications
                const medicationsContainer = document.getElementById('medicationsContainer');
                patientData.currentMedications.forEach(med => {
                    const medCard = document.createElement('div');
                    medCard.className = 'medication-card';
                    
                    const medName = document.createElement('div');
                    medName.className = 'medication-name';
                    medName.textContent = med.name;
                    
                    const medDetails = document.createElement('div');
                    medDetails.className = 'medication-details';
                    medDetails.textContent = `${med.dosage} • ${med.frequency}`;
                    
                    medCard.appendChild(medName);
                    medCard.appendChild(medDetails);
                    medicationsContainer.appendChild(medCard);
                });
                
                // Latest vital signs
                const latestVitals = patientData.vitalSigns[patientData.vitalSigns.length - 1];
                const vitalSignsGrid = document.getElementById('vitalSignsGrid');
                
                const vitalTypes = [
                    { name: 'Heart Rate', value: latestVitals.heartRate, unit: '\xa0 bpm' },
                    { name: 'Blood Pressure', value: latestVitals.bloodPressure, unit: '\xa0 mmHg' },
                    { name: 'Temperature', value: latestVitals.temperature, unit: '\xa0 °F' },
                    { name: 'Respiratory Rate', value: latestVitals.respiratoryRate, unit: '\xa0 bpm' },
                    { name: 'Oxygen Saturation', value: latestVitals.oxygenSaturation, unit: '\xa0 %' }
                ];
                
                vitalTypes.forEach(vital => {
                    const vitalCard = document.createElement('div');
                    vitalCard.className = 'vital-card';
                    
                    const vitalTitle = document.createElement('div');
                    vitalTitle.className = 'vital-title';
                    vitalTitle.textContent = vital.name;
                    
                    const vitalValue = document.createElement('div');
                    vitalValue.className = 'vital-value';
                    vitalValue.textContent = vital.value;
                    
                    const vitalUnit = document.createElement('span');
                    vitalUnit.className = 'vital-unit';
                    vitalUnit.textContent = vital.unit;
                    vitalValue.appendChild(vitalUnit);
                    
                    vitalCard.appendChild(vitalTitle);
                    vitalCard.appendChild(vitalValue);
                    vitalSignsGrid.appendChild(vitalCard);
                });
                
                // Treatments
                const treatmentsContainer = document.getElementById('treatmentsContainer');
                patientData.treatments.forEach(treatment => {
                    const treatmentCard = document.createElement('div');
                    treatmentCard.className = 'treatment-card';
                    
                    const treatmentDate = document.createElement('div');
                    treatmentDate.className = 'treatment-date';
                    const dateObj = new Date(treatment.date);
                    treatmentDate.textContent = dateObj.toLocaleDateString('en-US', { 
                        year: 'numeric', 
                        month: 'long', 
                        day: 'numeric' 
                    });
                    
                    const treatmentType = document.createElement('div');
                    treatmentType.className = 'treatment-type';
                    treatmentType.textContent = treatment.type;
                    
                    const treatmentProvider = document.createElement('div');
                    treatmentProvider.className = 'treatment-provider';
                    treatmentProvider.textContent = treatment.provider;
                    
                    const treatmentNotes = document.createElement('div');
                    treatmentNotes.className = 'treatment-notes';
                    treatmentNotes.textContent = treatment.notes;
                    
                    treatmentCard.appendChild(treatmentDate);
                    treatmentCard.appendChild(treatmentType);
                    treatmentCard.appendChild(treatmentProvider);
                    treatmentCard.appendChild(treatmentNotes);
                    treatmentsContainer.appendChild(treatmentCard);
                });
                
                // Appointments
                const appointmentsContainer = document.getElementById('appointmentsContainer');
                patientData.appointments.forEach(appointment => {
                    const appointmentCard = document.createElement('div');
                    appointmentCard.className = 'appointment-card';
                    
                    const appointmentDate = document.createElement('div');
                    appointmentDate.className = 'appointment-date';
                    const dateObj = new Date(appointment.date);
                    appointmentDate.textContent = `${dateObj.toLocaleDateString('en-US', { 
                        year: 'numeric', 
                        month: 'long', 
                        day: 'numeric' 
                    })} - ${appointment.time}`;
                    
                    const appointmentType = document.createElement('div');
                    appointmentType.className = 'appointment-type';
                    appointmentType.textContent = appointment.type;
                    
                    const appointmentProvider = document.createElement('div');
                    appointmentProvider.className = 'appointment-provider';
                    appointmentProvider.textContent = appointment.provider;
                    
                    const appointmentLocation = document.createElement('div');
                    appointmentLocation.className = 'appointment-location';
                    appointmentLocation.textContent = appointment.location;
                    
                    appointmentCard.appendChild(appointmentDate);
                    appointmentCard.appendChild(appointmentType);
                    appointmentCard.appendChild(appointmentProvider);
                    appointmentCard.appendChild(appointmentLocation);
                    appointmentsContainer.appendChild(appointmentCard);
                });
                
                // Timeline with icons
                const timelineContainer = document.getElementById('timelineContainer');
                patientData.timeline.forEach(item => {
                    const timelineItem = document.createElement('div');
                    timelineItem.className = 'timeline-item';
                    if (item.highlight) {
                        timelineItem.classList.add('timeline-highlight');
                    }
                    
                    const timelineYear = document.createElement('div');
                    timelineYear.className = 'timeline-year';
                    timelineYear.textContent = item.year;
                    
                    const timelineContent = document.createElement('div');
                    timelineContent.className = 'timeline-content';
                    
                    // Add icon
                    if (item.icon && icons[item.icon]) {
                        const iconContainer = document.createElement('div');
                        iconContainer.className = 'timeline-icon';
                        iconContainer.innerHTML = icons[item.icon];
                        timelineContent.appendChild(iconContainer);
                    }
                    
                    const timelineTitle = document.createElement('div');
                    timelineTitle.className = 'timeline-title';
                    timelineTitle.textContent = `→${item.title}`;
                    
                    const timelineDescription = document.createElement('div');
                    timelineDescription.className = 'timeline-description';
                    timelineDescription.textContent = item.description;
                    
                    timelineContent.appendChild(timelineTitle);
                    timelineContent.appendChild(timelineDescription);
                    
                    timelineItem.appendChild(timelineYear);
                    timelineItem.appendChild(timelineContent);
                    timelineContainer.appendChild(timelineItem);
                });
                
                // Draw the vital signs chart
                drawHeartRateChart();
                
                // Add event listeners for tabs
                const tabs = document.querySelectorAll('.tab');
                tabs.forEach(tab => {
                    tab.addEventListener('click', function() {
                        // Remove active class from all tabs
                        tabs.forEach(t => t.classList.remove('active'));
                        // Add active class to clicked tab
                        this.classList.add('active');
                        
                        // Hide all tab contents
                        const tabContents = document.querySelectorAll('.tab-content');
                        tabContents.forEach(content => content.classList.remove('active'));
                        
                        // Show the selected tab content
                        const tabId = this.getAttribute('data-tab');
                        document.getElementById(tabId).classList.add('active');
                    });
                });
                
                // Add event listeners for chart tabs
                const chartTabs = document.querySelectorAll('.chart-tab');
                chartTabs.forEach(tab => {
                    tab.addEventListener('click', function() {
                        // Remove active class from all chart tabs
                        chartTabs.forEach(t => t.classList.remove('active'));
                        // Add active class to clicked chart tab
                        this.classList.add('active');
                        
                        // Draw the appropriate chart
                        const chartType = this.getAttribute('data-chart');
                        if (chartType === 'heart-rate') {
                            drawHeartRateChart();
                        } else if (chartType === 'blood-pressure') {
                            drawBloodPressureChart();
                        } else if (chartType === 'temperature') {
                            drawTemperatureChart();
                        }
                    });
                });
            });
            
            function drawHeartRateChart() {
                const svg = document.getElementById('vitalChart');
                svg.innerHTML = '';
                
                const width = svg.clientWidth;
                const height = svg.clientHeight;
                const padding = { top: 20, right: 30, bottom: 40, left: 40 };
                
                // Extract heart rate data
                const heartRateData = patientData.vitalSigns.map(vital => ({
                    date: new Date(vital.date),
                    value: vital.heartRate
                }));
                
                // Find min and max values
                const minValue = Math.min(...heartRateData.map(d => d.value)) - 5;
                const maxValue = Math.max(...heartRateData.map(d => d.value)) + 5;
                
                // Scale functions
                const xScale = (index) => padding.left + (index / (heartRateData.length - 1)) * (width - padding.left - padding.right);
                const yScale = (value) => height - padding.bottom - ((value - minValue) / (maxValue - minValue)) * (height - padding.top - padding.bottom);
                
                // Draw grid lines
                for (let i = Math.ceil(minValue / 4) * 4; i <= maxValue; i += 4) {
                    const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
                    line.setAttribute('x1', padding.left);
                    line.setAttribute('y1', yScale(i));
                    line.setAttribute('x2', width - padding.right);
                    line.setAttribute('y2', yScale(i));
                    line.setAttribute('class', 'chart-grid-line');
                    svg.appendChild(line);
                    
                    // Add y-axis labels
                    const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
                    text.setAttribute('x', padding.left - 10);
                    text.setAttribute('y', yScale(i) + 4);
                    text.setAttribute('class', 'chart-label');
                    text.setAttribute('text-anchor', 'end');
                    text.textContent = i;
                    svg.appendChild(text);
                }
                
                // Draw line chart
                let pathData = '';
                heartRateData.forEach((d, i) => {
                    if (i === 0) {
                        pathData += `M ${xScale(i)} ${yScale(d.value)}`;
                    } else {
                        pathData += ` C ${xScale(i-0.5)} ${yScale(heartRateData[i-1].value)}, ${xScale(i-0.5)} ${yScale(d.value)}, ${xScale(i)} ${yScale(d.value)}`;
                    }
                });
                
                const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
                path.setAttribute('d', pathData);
                path.setAttribute('class', 'chart-line');
                svg.appendChild(path);
                
                // Add data points
                heartRateData.forEach((d, i) => {
                    const circle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
                    circle.setAttribute('cx', xScale(i));
                    circle.setAttribute('cy', yScale(d.value));
                    circle.setAttribute('r', 4);
                    circle.setAttribute('class', 'chart-dot');
                    svg.appendChild(circle);
                });
                
                // Add x-axis labels
                heartRateData.forEach((d, i) => {
                    const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
                    const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
                    text.setAttribute('x', xScale(i));
                    text.setAttribute('y', height - padding.bottom + 20);
                    text.setAttribute('class', 'chart-label');
                    text.setAttribute('text-anchor', 'middle');
                    text.textContent = `${months[d.date.getMonth()]} ${d.date.getDate()}`;
                    svg.appendChild(text);
                });
            }
            
            function drawBloodPressureChart() {
                const svg = document.getElementById('vitalChart');
                svg.innerHTML = '';
                
                const width = svg.clientWidth;
                const height = svg.clientHeight;
                const padding = { top: 20, right: 30, bottom: 40, left: 40 };
                
                // Extract blood pressure data
                const bpData = patientData.vitalSigns.map(vital => {
                    const [systolic, diastolic] = vital.bloodPressure.split('/').map(Number);
                    return {
                        date: new Date(vital.date),
                        systolic,
                        diastolic
                    };
                });
                
                // Find min and max values
                const minSystolic = Math.min(...bpData.map(d => d.systolic)) - 5;
                const maxSystolic = Math.max(...bpData.map(d => d.systolic)) + 5;
                const minDiastolic = Math.min(...bpData.map(d => d.diastolic)) - 5;
                const maxDiastolic = Math.max(...bpData.map(d => d.diastolic)) + 5;
                
                const minValue = Math.min(minSystolic, minDiastolic);
                const maxValue = Math.max(maxSystolic, maxDiastolic);
                
                // Scale functions
                const xScale = (index) => padding.left + (index / (bpData.length - 1)) * (width - padding.left - padding.right);
                const yScale = (value) => height - padding.bottom - ((value - minValue) / (maxValue - minValue)) * (height - padding.top - padding.bottom);
                
                // Draw grid lines
                for (let i = Math.ceil(minValue / 10) * 10; i <= maxValue; i += 10) {
                    const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
                    line.setAttribute('x1', padding.left);
                    line.setAttribute('y1', yScale(i));
                    line.setAttribute('x2', width - padding.right);
                    line.setAttribute('y2', yScale(i));
                    line.setAttribute('class', 'chart-grid-line');
                    svg.appendChild(line);
                    
                    // Add y-axis labels
                    const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
                    text.setAttribute('x', padding.left - 10);
                    text.setAttribute('y', yScale(i) + 4);
                    text.setAttribute('class', 'chart-label');
                    text.setAttribute('text-anchor', 'end');
                    text.textContent = i;
                    svg.appendChild(text);
                }
                
                // Draw systolic line chart
                let pathDataSystolic = '';
                bpData.forEach((d, i) => {
                    if (i === 0) {
                        pathDataSystolic += `M ${xScale(i)} ${yScale(d.systolic)}`;
                    } else {
                        pathDataSystolic += ` C ${xScale(i-0.5)} ${yScale(bpData[i-1].systolic)}, ${xScale(i-0.5)} ${yScale(d.systolic)}, ${xScale(i)} ${yScale(d.systolic)}`;
                    }
                });
                
                const pathSystolic = document.createElementNS('http://www.w3.org/2000/svg', 'path');
                pathSystolic.setAttribute('d', pathDataSystolic);
                pathSystolic.setAttribute('class', 'chart-line');
                pathSystolic.setAttribute('stroke', '#46be8a');
                svg.appendChild(pathSystolic);
                
                // Draw diastolic line chart
                let pathDataDiastolic = '';
                bpData.forEach((d, i) => {
                    if (i === 0) {
                        pathDataDiastolic += `M ${xScale(i)} ${yScale(d.diastolic)}`;
                    } else {
                        pathDataDiastolic += ` C ${xScale(i-0.5)} ${yScale(bpData[i-1].diastolic)}, ${xScale(i-0.5)} ${yScale(d.diastolic)}, ${xScale(i)} ${yScale(d.diastolic)}`;
                    }
                });
                
                const pathDiastolic = document.createElementNS('http://www.w3.org/2000/svg', 'path');
                pathDiastolic.setAttribute('d', pathDataDiastolic);
                pathDiastolic.setAttribute('class', 'chart-line');
                pathDiastolic.setAttribute('stroke', '#3e86f5');
                svg.appendChild(pathDiastolic);
                
                // Add data points (systolic)
                bpData.forEach((d, i) => {
                    const circle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
                    circle.setAttribute('cx', xScale(i));
                    circle.setAttribute('cy', yScale(d.systolic));
                    circle.setAttribute('r', 4);
                    circle.setAttribute('class', 'chart-dot');
                    circle.setAttribute('fill', '#46be8a');
                    svg.appendChild(circle);
                });
                
                // Add data points (diastolic)
                bpData.forEach((d, i) => {
                    const circle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
                    circle.setAttribute('cx', xScale(i));
                    circle.setAttribute('cy', yScale(d.diastolic));
                    circle.setAttribute('r', 4);
                    circle.setAttribute('class', 'chart-dot');
                    circle.setAttribute('fill', '#3e86f5');
                    svg.appendChild(circle);
                });
                
                // Add x-axis labels
                bpData.forEach((d, i) => {
                    const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
                    const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
                    text.setAttribute('x', xScale(i));
                    text.setAttribute('y', height - padding.bottom + 20);
                    text.setAttribute('class', 'chart-label');
                    text.setAttribute('text-anchor', 'middle');
                    text.textContent = `${months[d.date.getMonth()]} ${d.date.getDate()}`;
                    svg.appendChild(text);
                });
            }
            
            function drawTemperatureChart() {
                const svg = document.getElementById('vitalChart');
                svg.innerHTML = '';
                
                const width = svg.clientWidth;
                const height = svg.clientHeight;
                const padding = { top: 20, right: 30, bottom: 40, left: 40 };
                
                // Extract temperature data
                const tempData = patientData.vitalSigns.map(vital => ({
                    date: new Date(vital.date),
                    value: vital.temperature
                }));
                
                // Find min and max values
                const minValue = Math.min(...tempData.map(d => d.value)) - 0.5;
                const maxValue = Math.max(...tempData.map(d => d.value)) + 0.5;
                
                // Scale functions
                const xScale = (index) => padding.left + (index / (tempData.length - 1)) * (width - padding.left - padding.right);
                const yScale = (value) => height - padding.bottom - ((value - minValue) / (maxValue - minValue)) * (height - padding.top - padding.bottom);
                
                // Draw grid lines
                for (let i = Math.floor(minValue * 10) / 10; i <= maxValue; i += 0.2) {
                    i = Math.round(i * 10) / 10; // Fix floating point issues
                    const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
                    line.setAttribute('x1', padding.left);
                    line.setAttribute('y1', yScale(i));
                    line.setAttribute('x2', width - padding.right);
                    line.setAttribute('y2', yScale(i));
                    line.setAttribute('class', 'chart-grid-line');
                    svg.appendChild(line);
                    
                    // Add y-axis labels
                    if (i % 0.4 < 0.001) { // Only show labels every 0.4 degrees
                        const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
                        text.setAttribute('x', padding.left - 10);
                        text.setAttribute('y', yScale(i) + 4);
                        text.setAttribute('class', 'chart-label');
                        text.setAttribute('text-anchor', 'end');
                        text.textContent = i.toFixed(1);
                        svg.appendChild(text);
                    }
                }
                
                // Draw line chart
                let pathData = '';
                tempData.forEach((d, i) => {
                    if (i === 0) {
                        pathData += `M ${xScale(i)} ${yScale(d.value)}`;
                    } else {
                        pathData += ` C ${xScale(i-0.5)} ${yScale(tempData[i-1].value)}, ${xScale(i-0.5)} ${yScale(d.value)}, ${xScale(i)} ${yScale(d.value)}`;
                    }
                });
                
                const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
                path.setAttribute('d', pathData);
                path.setAttribute('class', 'chart-line');
                path.setAttribute('stroke', '#f59846');
                svg.appendChild(path);
                
                // Add data points
                tempData.forEach((d, i) => {
                    const circle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
                    circle.setAttribute('cx', xScale(i));
                    circle.setAttribute('cy', yScale(d.value));
                    circle.setAttribute('r', 4);
                    circle.setAttribute('class', 'chart-dot');
                    circle.setAttribute('fill', '#f59846');
                    svg.appendChild(circle);
                });
                
                // Add x-axis labels
                tempData.forEach((d, i) => {
                    const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
                    const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
                    text.setAttribute('x', xScale(i));
                    text.setAttribute('y', height - padding.bottom + 20);
                    text.setAttribute('class', 'chart-label');
                    text.setAttribute('text-anchor', 'middle');
                    text.textContent = `${months[d.date.getMonth()]} ${d.date.getDate()}`;
                    svg.appendChild(text);
                });
            }
        </script>


    </body></html>
    "#.to_string();

    let response_fmt = response.replace("<json_data_placeholder>", json_data);
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

pub fn session_out(error_code: &str) -> String {
    let response = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Error 440 - Session Expired</title>
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
            <h1>Error 440: Session Expired</h1>
            <p>Your session has timed out due to inactivity. For your security, please log in again to continue.</p>
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


