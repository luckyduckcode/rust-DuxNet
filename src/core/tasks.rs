use crate::core::data_structures::*;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

pub struct TaskEngine {
    pub pending_tasks: Arc<RwLock<HashMap<TaskId, Task>>>,
    pub completed_tasks: Arc<RwLock<HashMap<TaskId, TaskResult>>>,
    pub processing_tasks: Arc<RwLock<HashMap<TaskId, String>>>, // task_id -> processor_did
}

impl TaskEngine {
    pub fn new() -> Self {
        TaskEngine {
            pending_tasks: Arc::new(RwLock::new(HashMap::new())),
            completed_tasks: Arc::new(RwLock::new(HashMap::new())),
            processing_tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn submit_task(&self, task: Task) -> Result<()> {
        let mut pending = self.pending_tasks.write().await;
        pending.insert(task.id.clone(), task.clone());
        info!("Submitted task: {}", task.id.0);
        Ok(())
    }

    pub async fn accept_task(&self, task_id: &TaskId, processor_did: String) -> Option<Task> {
        let mut pending = self.pending_tasks.write().await;
        if let Some(task) = pending.remove(task_id) {
            let mut processing = self.processing_tasks.write().await;
            processing.insert(task_id.clone(), processor_did.clone());
            info!("Task {} accepted by {}", task_id.0, processor_did);
            Some(task)
        } else {
            None
        }
    }

    pub async fn complete_task(&self, result: TaskResult) -> Result<()> {
        let mut completed = self.completed_tasks.write().await;
        let mut processing = self.processing_tasks.write().await;
        
        completed.insert(result.task_id.clone(), result.clone());
        processing.remove(&result.task_id);
        
        info!("Task {} completed by {}", result.task_id.0, result.processor_did);
        Ok(())
    }

    pub async fn process_task(&self, task: Task, processor_did: String) -> Result<TaskResult> {
        // Simulate task processing
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Generate mock result and proof
        let result_data = format!("Processed task {} by {}", task.id.0, processor_did);
        let proof = self.generate_proof(&task, &result_data);
        
        Ok(TaskResult {
            task_id: task.id,
            processor_did,
            result: result_data.into_bytes(),
            proof,
            completed_at: get_current_timestamp(),
        })
    }

    fn generate_proof(&self, task: &Task, result: &str) -> Vec<u8> {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(&task.payload);
        hasher.update(result.as_bytes());
        hasher.finalize().to_vec()
    }

    pub async fn get_pending_tasks(&self) -> Vec<Task> {
        let pending = self.pending_tasks.read().await;
        pending.values().cloned().collect()
    }

    pub async fn get_completed_tasks(&self) -> Vec<TaskResult> {
        let completed = self.completed_tasks.read().await;
        completed.values().cloned().collect()
    }

    pub async fn get_processing_tasks(&self) -> Vec<(TaskId, String)> {
        let processing = self.processing_tasks.read().await;
        processing.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    pub async fn get_task_status(&self, task_id: &TaskId) -> TaskStatus {
        let pending = self.pending_tasks.read().await;
        let completed = self.completed_tasks.read().await;
        let processing = self.processing_tasks.read().await;
        
        if pending.contains_key(task_id) {
            TaskStatus::Pending
        } else if processing.contains_key(task_id) {
            TaskStatus::Processing
        } else if completed.contains_key(task_id) {
            TaskStatus::Completed
        } else {
            TaskStatus::NotFound
        }
    }

    pub async fn process_pending_tasks(&self) -> Result<()> {
        let pending_tasks = self.get_pending_tasks().await;
        
        for task in pending_tasks {
            // In a real implementation, this would be distributed to available processors
            // For now, we'll just simulate processing
            let processor_did = "did:duxnet:processor".to_string();
            
            if let Some(task) = self.accept_task(&task.id, processor_did.clone()).await {
                let result = self.process_task(task, processor_did).await?;
                self.complete_task(result).await?;
            }
        }
        
        Ok(())
    }

    pub async fn get_stats(&self) -> TaskStats {
        let pending = self.pending_tasks.read().await;
        let completed = self.completed_tasks.read().await;
        let processing = self.processing_tasks.read().await;
        
        TaskStats {
            pending_count: pending.len(),
            processing_count: processing.len(),
            completed_count: completed.len(),
            total_tasks: pending.len() + processing.len() + completed.len(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Processing,
    Completed,
    NotFound,
}

#[derive(Debug, Clone)]
pub struct TaskStats {
    pub pending_count: usize,
    pub processing_count: usize,
    pub completed_count: usize,
    pub total_tasks: usize,
} 