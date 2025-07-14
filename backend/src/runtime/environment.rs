use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::str;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RuntimeType {
    LocalCpu,
    LocalGpu,
    CloudAws,
    CloudGcp,
    CloudAzure,
    Kubernetes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeEnvironment {
    pub id: String,
    pub name: String,
    pub runtime_type: RuntimeType,
    pub description: String,
    pub active: bool,
    pub gpu_available: bool,
    pub cpu_count: Option<usize>,
    pub memory_gb: Option<f64>,
    pub gpu_info: Option<GpuInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub memory_gb: f64,
    pub cuda_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    pub default_environment: String,
    pub available_environments: Vec<RuntimeEnvironment>,
}

pub struct EnvironmentManager {
    config: EnvironmentConfig,
}

impl EnvironmentManager {
    pub fn new() -> Result<Self> {
        // Detect available environments
        let local_cpu = Self::detect_local_cpu_environment()?;
        let local_gpu = Self::detect_local_gpu_environment()?;
        
        let available_environments = vec![
            local_cpu,
            local_gpu,
            // Cloud environments would be added based on configuration
        ];
        
        let default_env = available_environments
            .iter()
            .find(|e| e.active)
            .map(|e| e.id.clone())
            .unwrap_or_else(|| "local_cpu".to_string());
            
        Ok(Self {
            config: EnvironmentConfig {
                default_environment: default_env,
                available_environments,
            },
        })
    }
    
    pub fn list_environments(&self) -> &[RuntimeEnvironment] {
        &self.config.available_environments
    }
    
    pub fn get_environment(&self, id: &str) -> Option<&RuntimeEnvironment> {
        self.config.available_environments.iter().find(|e| e.id == id)
    }
    
    pub fn get_default_environment(&self) -> Option<&RuntimeEnvironment> {
        self.get_environment(&self.config.default_environment)
    }
    
    pub fn set_default_environment(&mut self, id: &str) -> Result<()> {
        if self.get_environment(id).is_some() {
            self.config.default_environment = id.to_string();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Environment not found: {}", id))
        }
    }
    
    pub fn get_environment_config(&self) -> &EnvironmentConfig {
        &self.config
    }
    
    fn detect_local_cpu_environment() -> Result<RuntimeEnvironment> {
        // Get CPU info (cross-platform)
        let cpu_count = num_cpus::get();
        
        // Get total memory (cross-platform)
        let total_memory_bytes = sys_info::mem_info()
            .map(|mem| mem.total * 1024) // Convert from KB to bytes
            .unwrap_or(0);
        
        let memory_gb = (total_memory_bytes as f64) / (1024.0 * 1024.0 * 1024.0);
        
        Ok(RuntimeEnvironment {
            id: "local_cpu".to_string(),
            name: "Local CPU".to_string(),
            runtime_type: RuntimeType::LocalCpu,
            description: "Local machine CPU execution".to_string(),
            active: true,
            gpu_available: false,
            cpu_count: Some(cpu_count),
            memory_gb: Some(memory_gb),
            gpu_info: None,
        })
    }
    
    fn detect_local_gpu_environment() -> Result<RuntimeEnvironment> {
        // Try to detect NVIDIA GPU using nvidia-smi
        let gpu_info = match Command::new("nvidia-smi")
            .arg("--query-gpu=name,memory.total,driver_version")
            .arg("--format=csv,noheader,nounits")
            .output()
        {
            Ok(output) if output.status.success() => {
                let output_str = str::from_utf8(&output.stdout)?.trim();
                if let Some(first_line) = output_str.lines().next() {
                    let parts: Vec<&str> = first_line.split(',').map(|s| s.trim()).collect();
                    if parts.len() >= 2 {
                        let name = parts[0].to_string();
                        let memory_mb: f64 = parts[1].parse().unwrap_or(0.0);
                        let cuda_version = if parts.len() > 2 {
                            Some(parts[2].to_string())
                        } else {
                            None
                        };
                        
                        Some(GpuInfo {
                            name,
                            memory_gb: memory_mb / 1024.0,
                            cuda_version,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        };
        
        let has_gpu = gpu_info.is_some();
        
        let mut env = Self::detect_local_cpu_environment()?;
        env.id = "local_gpu".to_string();
        env.name = "Local GPU".to_string();
        env.runtime_type = RuntimeType::LocalGpu;
        env.description = "Local machine GPU execution".to_string();
        env.gpu_available = has_gpu;
        env.gpu_info = gpu_info;
        
        Ok(env)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_environment_detection() {
        let manager = EnvironmentManager::new().unwrap();
        let envs = manager.list_environments();
        
        assert!(!envs.is_empty());
        assert!(envs.iter().any(|e| e.id == "local_cpu"));
        
        // The local GPU environment should be detected if a GPU is available
        let has_gpu = envs.iter().any(|e| e.gpu_available);
        assert_eq!(
            envs.iter().any(|e| e.id == "local_gpu"),
            has_gpu
        );
        
        // Default environment should be set
        assert!(manager.get_default_environment().is_some());
    }
    
    #[test]
    fn test_environment_switching() {
        let mut manager = EnvironmentManager::new().unwrap();
        
        // Should be able to switch to any available environment
        for env in manager.list_environments() {
            manager.set_default_environment(&env.id).unwrap();
            assert_eq!(
                manager.get_default_environment().unwrap().id,
                env.id
            );
        }
        
        // Switching to non-existent environment should fail
        assert!(manager.set_default_environment("nonexistent").is_err());
    }
}
