use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use tokio::io;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinSet;
use tokio::time::{self, sleep, timeout};

// Custom error type
#[derive(Debug)]
enum AppError {
    IoError(io::Error),
    Timeout,
    ProcessingFailed(String),
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::IoError(err)
    }
}

type Result<T> = std::result::Result<T, AppError>;

// Simulate a data processing function with delay and file I/O
async fn process_data(id: usize, delay_ms: u64) -> Result<String> {
    // Simulate work with sleep
    sleep(Duration::from_millis(delay_ms)).await;
    
    // Simulate occasional failures
    if id % 7 == 0 {
        return Err(AppError::ProcessingFailed(format!("Task {id} failed randomly")));
    }
    
    // Demonstrate file I/O
    let content = format!("Result of task {id}");
    fs::write(format!("output_{id}.txt"), &content).await?;
    
    Ok(content)
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut set = JoinSet::new();
    
    // Shared results container protected by a mutex
    let results = Arc::new(Mutex::new(Vec::new()));
    
    // Channel for communication between tasks
    let (tx, mut rx) = mpsc::channel::<usize>(10);
    
    // Task 1: Producer - sends work items to channel
    set.spawn(async move {
        for i in 0..10 {
            if tx.send(i).await.is_err() {
                println!("Channel closed, stopping producer");
                break;
            }
            sleep(Duration::from_millis(50)).await;
        }
    });
    
    // Task 2: Process tasks with timeout protection
    let results_clone = results.clone();
    set.spawn(async move {
        while let Some(id) = rx.recv().await {
            let delay = 100 + (id * 50) as u64;
            let results = results_clone.clone();
            
            // Spawn a separate task for each work item
            tokio::spawn(async move {
                println!("Processing task {id}");
                
                // Execute with timeout protection
                let process_result = timeout(
                    Duration::from_millis(delay + 100),
                    process_data(id, delay)
                ).await;
                
                // Store the result
                let mut results_guard = results.lock().await;
                match process_result {
                    Ok(Ok(output)) => {
                        results_guard.push((id, format!("Success: {output}")));
                    },
                    Ok(Err(e)) => {
                        results_guard.push((id, format!("Error: {e:?}")));
                    },
                    Err(_) => {
                        results_guard.push((id, "Timeout".to_string()));
                    }
                }
            });
        }
    });
    
    // Task 3: Periodic status reporter using intervals
    let results_for_status = results.clone();
    set.spawn(async move {
        let mut interval = time::interval(Duration::from_millis(200));
        
        for _ in 0..15 {
            interval.tick().await;
            
            let results_guard = results_for_status.lock().await;
            println!("Status: {} tasks completed", results_guard.len());
            
            // Stop when all tasks are done
            if results_guard.len() >= 10 {
                break;
            }
        }
    });
    
    // Wait for tasks to complete with an overall timeout
    let timeout_duration = Duration::from_secs(5);
    match timeout(timeout_duration, async {
        while let Some(res) = set.join_next().await {
            if let Err(e) = res {
                eprintln!("Task join error: {e}");
            }
        }
    }).await {
        Ok(_) => println!("All main tasks completed successfully"),
        Err(_) => {
            println!("Not all tasks completed within timeout period");
            set.abort_all(); // Demonstrate task cancellation
        }
    }
    
    // Print final results
    let final_results = results.lock().await;
    println!("\nFinal results:");
    
    let mut sorted_results: Vec<_> = final_results.iter().collect();
    sorted_results.sort_by_key(|&(id, _)| *id);
    
    for (id, result) in sorted_results {
        println!("Task {id}: {result}");
    }
    
    Ok(())
}