use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NodeType {
    InputLayer,
    DenseLayer { units: u32, activation: String },
    DropoutLayer { rate: f32 },
    Conv2DLayer { filters: u32, kernel_size: (u32, u32), activation: String },
    MaxPooling2DLayer { pool_size: (u32, u32) },
    FlattenLayer,
    OutputLayer { units: u32, activation: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: String,
    pub node_type: NodeType,
    pub position: (f32, f32),
    pub data: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub target: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedModel {
    pub code: String,
    pub requirements: Vec<String>,
    pub model_summary: String,
}

pub struct ModelBuilder;

impl ModelBuilder {
    pub fn new() -> Self {
        ModelBuilder
    }

    pub fn generate_model(&self, graph: &ModelGraph) -> Result<GeneratedModel> {
        // Validate the graph
        self.validate_graph(graph)?;

        // Generate model code based on the graph
        self.generate_keras_model(graph)
    }

    fn validate_graph(&self, graph: &ModelGraph) -> Result<()> {
        // Check for empty graph
        if graph.nodes.is_empty() {
            return Err(anyhow::anyhow!("Model graph cannot be empty"));
        }

        // Check for input and output nodes
        let has_input = graph.nodes.iter().any(|n| matches!(n.node_type, NodeType::InputLayer));
        let has_output = graph.nodes.iter().any(|n| matches!(n.node_type, NodeType::OutputLayer { .. }));

        if !has_input {
            return Err(anyhow::anyhow!("Model graph must have at least one input layer"));
        }

        if !has_output {
            return Err(anyhow::anyhow!("Model graph must have at least one output layer"));
        }

        // Check for disconnected nodes
        let connected_nodes: std::collections::HashSet<_> = graph.edges
            .iter()
            .flat_map(|e| vec![&e.source, &e.target])
            .collect();

        let all_nodes: std::collections::HashSet<_> = graph.nodes.iter().map(|n| &n.id).collect();

        let disconnected_nodes: Vec<_> = all_nodes.difference(&connected_nodes).collect();
        if !disconnected_nodes.is_empty() {
            return Err(anyhow::anyhow!("Model graph has disconnected nodes: {:?}", disconnected_nodes));
        }

        Ok(())
    }

    fn generate_keras_model(&self, graph: &ModelGraph) -> Result<GeneratedModel> {
        let mut imports = vec![
            "import tensorflow as tf".to_string(),
            "from tensorflow.keras import layers, Model".to_string(),
        ];

        let mut code = vec!["def create_model(input_shape):".to_string()];
        code.push("    # Input layer".to_string());
        
        // Track the last layer's name for connections
        let mut last_layer = "input".to_string();
        
        // Process nodes in order (simplified - in a real implementation, you'd need to handle the graph structure properly)
        for (i, node) in graph.nodes.iter().enumerate() {
            match &node.node_type {
                NodeType::InputLayer => {
                    code.push(format!("    {} = layers.Input(shape=input_shape, name='input')", last_layer));
                }
                NodeType::DenseLayer { units, activation } => {
                    let layer_name = format!("dense_{}", i);
                    code.push(format!("    {} = layers.Dense({}, activation='{}', name='{}')({})", 
                        layer_name, units, activation, layer_name, last_layer));
                    last_layer = layer_name;
                }
                NodeType::DropoutLayer { rate } => {
                    let layer_name = format!("dropout_{}", i);
                    code.push(format!("    {} = layers.Dropout({}, name='{}')({})", 
                        layer_name, rate, layer_name, last_layer));
                    last_layer = layer_name;
                }
                NodeType::Conv2DLayer { filters, kernel_size, activation } => {
                    let layer_name = format!("conv2d_{}", i);
                    code.push(format!("    {} = layers.Conv2D({}, kernel_size={}, activation='{}', padding='same', name='{}')({})", 
                        layer_name, filters, format!("({}, {})", kernel_size.0, kernel_size.1), 
                        activation, layer_name, last_layer));
                    last_layer = layer_name;
                }
                NodeType::MaxPooling2DLayer { pool_size } => {
                    let layer_name = format!("maxpool_{}", i);
                    code.push(format!("    {} = layers.MaxPooling2D(pool_size={}, name='{}')({})", 
                        layer_name, format!("({}, {})", pool_size.0, pool_size.1), 
                        layer_name, last_layer));
                    last_layer = layer_name;
                }
                NodeType::FlattenLayer => {
                    let layer_name = format!("flatten_{}", i);
                    code.push(format!("    {} = layers.Flatten(name='{}')({})", 
                        layer_name, layer_name, last_layer));
                    last_layer = layer_name;
                }
                NodeType::OutputLayer { units, activation } => {
                    let layer_name = "output".to_string();
                    code.push(format!("    {} = layers.Dense({}, activation='{}', name='{}')({})", 
                        layer_name, units, activation, layer_name, last_layer));
                }
            }
        }

        // Create the model
        code.push("    model = Model(inputs=input, outputs=output, name='generated_model')".to_string());
        code.push("    return model".to_string());
        code.push("".to_string());
        
        // Add model compilation
        code.push("def compile_model(model, optimizer='adam', loss='sparse_categorical_crossentropy', metrics=['accuracy']):".to_string());
        code.push("    model.compile(optimizer=optimizer, loss=loss, metrics=metrics)".to_string());
        code.push("    return model".to_string());

        // Add model training
        code.push("
def train_model(model, X_train, y_train, X_val=None, y_val=None, epochs=10, batch_size=32, callbacks=None):".to_string());
        code.push("    history = model.fit(".to_string());
        code.push("        X_train, y_train,".to_string());
        code.push("        validation_data=(X_val, y_val) if X_val is not None and y_val is not None else None,".to_string());
        code.push("        epochs=epochs,".to_string());
        code.push("        batch_size=batch_size,".to_string());
        code.push("        callbacks=callbacks,".to_string());
        code.push("        verbose=1".to_string());
        code.push("    )".to_string());
        code.push("    return history".to_string());

        let model_code = code.join("\n");
        
        Ok(GeneratedModel {
            code: model_code,
            requirements: vec![
                "tensorflow>=2.0.0".to_string(),
                "numpy>=1.19.0".to_string(),
            ],
            model_summary: format!("Generated model with {} layers", graph.nodes.len()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_sequential_model() {
        let builder = ModelBuilder::new();
        
        let graph = ModelGraph {
            nodes: vec![
                Node {
                    id: "1".to_string(),
                    node_type: NodeType::InputLayer,
                    position: (0.0, 0.0),
                    data: HashMap::new(),
                },
                Node {
                    id: "2".to_string(),
                    node_type: NodeType::DenseLayer {
                        units: 64,
                        activation: "relu".to_string(),
                    },
                    position: (100.0, 0.0),
                    data: HashMap::new(),
                },
                Node {
                    id: "3".to_string(),
                    node_type: NodeType::OutputLayer {
                        units: 10,
                        activation: "softmax".to_string(),
                    },
                    position: (200.0, 0.0),
                    data: HashMap::new(),
                },
            ],
            edges: vec![
                Edge {
                    id: "e1-2".to_string(),
                    source: "1".to_string(),
                    target: "2".to_string(),
                },
                Edge {
                    id: "e2-3".to_string(),
                    source: "2".to_string(),
                    target: "3".to_string(),
                },
            ],
        };

        let result = builder.generate_model(&graph);
        assert!(result.is_ok());
        
        let model = result.unwrap();
        assert!(model.code.contains("Dense(64, activation='relu')"));
        assert!(model.code.contains("Dense(10, activation='softmax')"));
        assert!(model.requirements.contains(&"tensorflow>=2.0.0".to_string()));
    }
}
