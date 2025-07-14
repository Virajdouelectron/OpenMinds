use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum OTError {
    #[error("Invalid operation")]
    InvalidOperation,
    
    #[error("Version mismatch")]
    VersionMismatch,
    
    #[error("Operation out of bounds")]
    OutOfBounds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OTOperation {
    Insert { position: usize, text: String },
    Delete { position: usize, length: usize },
    Retain { length: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OTPair {
    pub operation: OTOperation,
    pub version: u64,
    pub client_id: Uuid,
    pub timestamp: u64,
}

#[derive(Debug, Default)]
pub struct OTEngine {
    document: String,
    version: u64,
    operations: Vec<OTPair>,
    client_versions: HashMap<Uuid, u64>,
}

impl OTEngine {
    pub fn new(initial_content: &str) -> Self {
        Self {
            document: initial_content.to_string(),
            version: 0,
            operations: Vec::new(),
            client_versions: HashMap::new(),
        }
    }

    pub fn get_document(&self) -> &str {
        &self.document
    }

    pub fn get_version(&self) -> u64 {
        self.version
    }

    pub fn apply_operation(
        &mut self,
        operation: OTOperation,
        client_id: Uuid,
    ) -> Result<u64, OTError> {
        let client_version = self.client_versions.get(&client_id).copied().unwrap_or(0);
        
        if client_version > self.version {
            return Err(OTError::VersionMismatch);
        }

        // Apply the operation to the document
        match &operation {
            OTOperation::Insert { position, text } => {
                if *position > self.document.len() {
                    return Err(OTError::OutOfBounds);
                }
                self.document.insert_str(*position, text);
            }
            OTOperation::Delete { position, length } => {
                if position + length > self.document.len() {
                    return Err(OTError::OutOfBounds);
                }
                self.document.replace_range(*position..(position + length), "");
            }
            OTOperation::Retain { .. } => {}
        }

        // Update version and store the operation
        self.version += 1;
        self.operations.push(OTPair {
            operation,
            version: self.version,
            client_id,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        });

        self.client_versions.insert(client_id, self.version);

        Ok(self.version)
    }

    pub fn transform_operation(
        &self,
        operation: &OTOperation,
        since_version: u64,
    ) -> Result<OTOperation, OTError> {
        if since_version > self.version {
            return Err(OTError::VersionMismatch);
        }

        let mut transformed_op = operation.clone();

        // Apply all operations that happened after since_version
        for op_pair in self.operations.iter().filter(|op| op.version > since_version) {
            transformed_op = self.transform(&transformed_op, &op_pair.operation)?;
        }

        Ok(transformed_op)
    }

    fn transform(
        &self,
        op1: &OTOperation,
        op2: &OTOperation,
    ) -> Result<OTOperation, OTError> {
        match (op1, op2) {
            // Insert-Insert
            (
                OTOperation::Insert { position: pos1, text: text1 },
                OTOperation::Insert { position: pos2, text: text2 },
            ) => {
                if pos1 < pos2 {
                    Ok(OTOperation::Insert {
                        position: *pos1,
                        text: text1.clone(),
                    })
                } else {
                    Ok(OTOperation::Insert {
                        position: pos1 + text2.len(),
                        text: text1.clone(),
                    })
                }
            }
            
            // Insert-Delete
            (
                OTOperation::Insert { position: pos1, text: text1 },
                OTOperation::Delete { position: pos2, length },
            ) => {
                if pos1 < pos2 {
                    Ok(OTOperation::Insert {
                        position: *pos1,
                        text: text1.clone(),
                    })
                } else {
                    Ok(OTOperation::Insert {
                        position: pos1.saturating_sub(*length),
                        text: text1.clone(),
                    })
                }
            }
            
            // Delete-Insert
            (
                OTOperation::Delete { position: pos1, length: len1 },
                OTOperation::Insert { position: pos2, text: text2 },
            ) => {
                if pos1 + len1 <= pos2 {
                    Ok(OTOperation::Delete {
                        position: *pos1,
                        length: *len1,
                    })
                } else if *pos1 > *pos2 + text2.len() {
                    Ok(OTOperation::Delete {
                        position: pos1 - text2.len(),
                        length: *len1,
                    })
                } else {
                    // Handle overlapping operations
                    let mut new_pos = *pos1;
                    let mut new_len = *len1;
                    
                    if *pos1 > *pos2 {
                        new_pos = pos2 + text2.len();
                        new_len = new_len.saturating_sub((*pos1 - *pos2) as usize);
                    }
                    
                    let end = pos1 + len1;
                    let op2_end = pos2 + text2.len();
                    
                    if end > op2_end {
                        new_len = new_len.saturating_sub((end - op2_end) as usize);
                    }
                    
                    if new_len > 0 {
                        Ok(OTOperation::Delete {
                            position: new_pos,
                            length: new_len,
                        })
                    } else {
                        Err(OTError::InvalidOperation)
                    }
                }
            }
            
            // Delete-Delete
            (
                OTOperation::Delete { position: pos1, length: len1 },
                OTOperation::Delete { position: pos2, length: len2 },
            ) => {
                let end1 = pos1 + len1;
                let end2 = pos2 + len2;
                
                if end1 <= *pos2 {
                    // No overlap, operation 1 is completely before operation 2
                    Ok(OTOperation::Delete {
                        position: *pos1,
                        length: *len1,
                    })
                } else if *pos1 >= end2 {
                    // No overlap, operation 1 is completely after operation 2
                    Ok(OTOperation::Delete {
                        position: pos1 - len2,
                        length: *len1,
                    })
                } else {
                    // Overlapping deletes
                    let new_pos = std::cmp::min(*pos1, *pos2);
                    let new_end = std::cmp::max(end1, end2);
                    let new_len = new_end - new_pos;
                    
                    Ok(OTOperation::Delete {
                        position: new_pos,
                        length: new_len,
                    })
                }
            }
            
            // Retain operations are no-ops in transformation
            (OTOperation::Retain { .. }, _) => Ok(op1.clone()),
            (_, OTOperation::Retain { .. }) => Ok(op1.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_insert_insert() {
        let mut engine = OTEngine::new("");
        let client_id = Uuid::new_v4();
        
        // First insert
        engine.apply_operation(
            OTOperation::Insert {
                position: 0,
                text: "hello".to_string(),
            },
            client_id,
        ).unwrap();
        
        // Concurrent insert at the same position
        let op = OTOperation::Insert {
            position: 0,
            text: "world ".to_string(),
        };
        
        let transformed = engine.transform_operation(&op, 0).unwrap();
        assert_eq!(transformed.position, 5); // Should be moved after "hello"
    }
    
    #[test]
    fn test_insert_delete() {
        let mut engine = OTEngine::new("hello world");
        let client_id = Uuid::new_v4();
        
        // Delete "world"
        engine.apply_operation(
            OTOperation::Delete {
                position: 6,
                length: 5,
            },
            client_id,
        ).unwrap();
        
        // Concurrent insert at position 6
        let op = OTOperation::Insert {
            position: 6,
            text: "everyone".to_string(),
        };
        
        let transformed = engine.transform_operation(&op, 0).unwrap();
        assert_eq!(transformed.position, 6); // Should stay at 6 since delete was before
    }
}
