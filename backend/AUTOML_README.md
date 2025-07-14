# AutoML and Experiment Tracking

This document provides an overview of the AutoML and Experiment Tracking features added to the OpenMind IDE.

## Features

### 1. Experiment Tracker
- Tracks machine learning experiments with detailed metrics and parameters
- Stores experiment metadata, status, and results in SQLite
- Logs experiment output and metrics over time
- Supports querying and filtering experiments

### 2. AutoML Wizard
- Automated machine learning pipeline for classification and regression tasks
- Handles data preprocessing, feature engineering, and model selection
- Supports popular ML algorithms (Random Forest, XGBoost, etc.)
- Automatic hyperparameter tuning
- Model evaluation with comprehensive metrics

### 3. Terminal Access Panel
- Execute shell commands from the IDE
- Monitor long-running processes
- View command output in real-time

## API Endpoints

### Experiments
- `POST /api/experiments/` - Create a new experiment
- `GET /api/experiments/{id}` - Get experiment details
- `PATCH /api/experiments/{id}` - Update experiment metrics
- `PUT /api/experiments/{id}` - Update experiment status
- `POST /api/experiments/{id}/logs` - Add experiment log
- `GET /api/experiments/{id}/logs` - Get experiment logs

### AutoML
- `POST /api/automl/` - Start a new AutoML experiment
- `GET /api/automl/{experiment_id}` - Get AutoML experiment status

## Python Dependencies

The AutoML service requires Python 3.8+ with the following packages:
- numpy
- pandas
- scikit-learn
- xgboost
- lightgbm
- joblib
- openpyxl

Install them using:
```bash
pip install -r backend/python/requirements.txt
```

## Getting Started

1. Start the backend server:
   ```bash
   cd backend
   cargo run
   ```

2. Use the API to create and manage experiments:
   ```bash
   # Create a new experiment
   curl -X POST http://localhost:3001/api/experiments/ \
     -H "Content-Type: application/json" \
     -d '{"name": "My First Experiment", "description": "Testing the experiment tracker"}'

   # Start an AutoML experiment
   curl -X POST http://localhost:3001/api/automl/ \
     -H "Content-Type: application/json" \
     -d '{"dataset_id": "your-dataset-id", "target_column": "target"}'
   ```

## Directory Structure

```
backend/
├── python/
│   └── automl/
│       ├── service.py    # AutoML service implementation
│       └── requirements.txt  # Python dependencies
├── src/
│   ├── api/
│   │   ├── automl.rs     # AutoML API endpoints
│   │   └── experiments.rs # Experiment tracking endpoints
│   ├── automl.rs         # AutoML service (Rust)
│   └── models.rs         # Database models and operations
└── migrations/           # Database migrations
    └── ...
```

## Next Steps

1. Add support for more ML algorithms and preprocessing techniques
2. Implement model versioning and deployment
3. Add visualization for experiment metrics
4. Support for distributed training
5. Integrate with popular ML platforms (MLflow, Weights & Biases, etc.)
