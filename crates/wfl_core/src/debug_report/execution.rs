use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Instant;
use once_cell::sync::Lazy;
use chrono::Local;

static EXECUTION_REPORT_ACTIVE: AtomicBool = AtomicBool::new(false);
static EXECUTION_START_TIME: Lazy<Mutex<Option<Instant>>> = Lazy::new(|| Mutex::new(None));
static EXECUTION_REPORT_PATH: Lazy<Mutex<Option<PathBuf>>> = Lazy::new(|| Mutex::new(None));

pub fn start_execution_report(title: &str) {
    let report_path = generate_execution_report_path();
    
    if let Ok(mut file) = File::create(&report_path) {
        let header = format!(
            "=== WFL Execution Report ===\n\
             {}\n\
             Started: {}\n\
             ===========================\n\n",
            title,
            Local::now().format("%Y-%m-%d %H:%M:%S")
        );
        
        if let Err(e) = file.write_all(header.as_bytes()) {
            eprintln!("Warning: Failed to write execution report header: {}", e);
            return;
        }
        
        *EXECUTION_REPORT_PATH.lock().unwrap() = Some(report_path);
        *EXECUTION_START_TIME.lock().unwrap() = Some(Instant::now());
        EXECUTION_REPORT_ACTIVE.store(true, Ordering::SeqCst);
    } else {
        eprintln!("Warning: Failed to create execution report file");
    }
}

pub fn end_execution_report() {
    if !EXECUTION_REPORT_ACTIVE.load(Ordering::SeqCst) {
        return;
    }
    
    let elapsed = if let Some(start_time) = *EXECUTION_START_TIME.lock().unwrap() {
        start_time.elapsed()
    } else {
        return;
    };
    
    let report_path = if let Some(path) = &*EXECUTION_REPORT_PATH.lock().unwrap() {
        path.clone()
    } else {
        return;
    };
    
    if let Ok(mut file) = std::fs::OpenOptions::new().append(true).open(&report_path) {
        let footer = format!(
            "\n=== Execution Completed ===\n\
             Ended: {}\n\
             Total execution time: {:.3} seconds\n\
             ===========================\n",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            elapsed.as_secs_f64()
        );
        
        if let Err(e) = file.write_all(footer.as_bytes()) {
            eprintln!("Warning: Failed to write execution report footer: {}", e);
        }
    }
    
    EXECUTION_REPORT_ACTIVE.store(false, Ordering::SeqCst);
    *EXECUTION_START_TIME.lock().unwrap() = None;
    *EXECUTION_REPORT_PATH.lock().unwrap() = None;
}

fn generate_execution_report_path() -> PathBuf {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let temp_dir = std::env::temp_dir();
    temp_dir.join(format!("wfl_execution_{}.log", timestamp))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn test_execution_report_lifecycle() {
        start_execution_report("Test Execution");
        
        assert!(EXECUTION_REPORT_ACTIVE.load(Ordering::SeqCst));
        
        let report_path = EXECUTION_REPORT_PATH.lock().unwrap().clone().unwrap();
        
        end_execution_report();
        
        assert!(!EXECUTION_REPORT_ACTIVE.load(Ordering::SeqCst));
        
        let content = fs::read_to_string(&report_path).unwrap();
        assert!(content.contains("=== WFL Execution Report ==="));
        assert!(content.contains("Test Execution"));
        assert!(content.contains("=== Execution Completed ==="));
        
        let _ = fs::remove_file(report_path);
    }
}
