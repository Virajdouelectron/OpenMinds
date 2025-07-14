import os
import json
import logging
import numpy as np
import pandas as pd
from typing import Dict, Any, Optional, List, Union
from pathlib import Path
from datetime import datetime
from enum import Enum

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

class TaskType(Enum):
    CLASSIFICATION = "classification"
    REGRESSION = "regression"
    CLUSTERING = "clustering"

class AutoMLService:
    def __init__(self, experiment_id: str, output_dir: str = "data/experiments"):
        """Initialize the AutoML service.
        
        Args:
            experiment_id: Unique identifier for the experiment
            output_dir: Directory to store experiment outputs
        """
        self.experiment_id = experiment_id
        self.output_dir = Path(output_dir) / experiment_id
        self.output_dir.mkdir(parents=True, exist_ok=True)
        
        # Initialize models and results
        self.models = {}
        self.metrics = {}
        self.feature_importances = {}
        
        logger.info(f"Initialized AutoML service for experiment {experiment_id}")
    
    def log_metric(self, name: str, value: float, step: Optional[int] = None):
        """Log a metric for the experiment."""
        if name not in self.metrics:
            self.metrics[name] = []
        self.metrics[name].append({"step": step or len(self.metrics[name]), "value": value})
        logger.info(f"Metric '{name}': {value} (step: {step or 'latest'})")
    
    def detect_task_type(self, target: pd.Series) -> TaskType:
        """Detect the type of ML task based on the target variable."""
        # Simple heuristic: if target has few unique values, it's likely classification
        n_unique = target.nunique()
        if n_unique <= 10:  # Arbitrary threshold
            return TaskType.CLASSIFICATION
        return TaskType.REGRESSION
    
    def preprocess_data(self, data: pd.DataFrame, target_col: str) -> tuple:
        """Preprocess the input data."""
        logger.info("Preprocessing data...")
        
        # Handle missing values
        data = data.dropna(how='all')
        
        # Separate features and target
        X = data.drop(columns=[target_col])
        y = data[target_col]
        
        # Detect and encode categorical variables
        categorical_cols = X.select_dtypes(include=['object', 'category']).columns
        if not categorical_cols.empty:
            X = pd.get_dummies(X, columns=categorical_cols, drop_first=True)
        
        return X, y
    
    def train_model(self, X: pd.DataFrame, y: pd.Series, task_type: TaskType, **kwargs):
        """Train a model based on the task type."""
        logger.info(f"Training {task_type.value} model...")
        
        # Import models here to avoid unnecessary imports
        if task_type == TaskType.CLASSIFICATION:
            from sklearn.ensemble import RandomForestClassifier
            model = RandomForestClassifier(**kwargs)
        elif task_type == TaskType.REGRESSION:
            from sklearn.ensemble import RandomForestRegressor
            model = RandomForestRegressor(**kwargs)
        else:
            raise ValueError(f"Unsupported task type: {task_type}")
        
        # Train the model
        model.fit(X, y)
        
        # Store feature importances if available
        if hasattr(model, 'feature_importances_'):
            self.feature_importances = dict(zip(X.columns, model.feature_importances_))
        
        self.models[task_type.value] = model
        return model
    
    def evaluate_model(self, model, X_test: pd.DataFrame, y_test: pd.Series, task_type: TaskType):
        """Evaluate the trained model."""
        from sklearn.metrics import (
            accuracy_score, precision_score, recall_score, f1_score,
            mean_squared_error, mean_absolute_error, r2_score
        )
        
        y_pred = model.predict(X_test)
        
        if task_type == TaskType.CLASSIFICATION:
            metrics = {
                "accuracy": accuracy_score(y_test, y_pred),
                "precision": precision_score(y_test, y_pred, average='weighted', zero_division=0),
                "recall": recall_score(y_test, y_pred, average='weighted', zero_division=0),
                "f1": f1_score(y_test, y_pred, average='weighted')
            }
        else:  # Regression
            metrics = {
                "mse": mean_squared_error(y_test, y_pred),
                "mae": mean_absolute_error(y_test, y_pred),
                "r2": r2_score(y_test, y_pred)
            }
        
        # Log metrics
        for name, value in metrics.items():
            self.log_metric(name, value)
        
        return metrics
    
    def save_model(self, model, filename: str = "model.pkl"):
        """Save the trained model to disk."""
        import joblib
        
        model_path = self.output_dir / filename
        joblib.dump(model, model_path)
        logger.info(f"Model saved to {model_path}")
        return str(model_path)
    
    def save_results(self, filename: str = "results.json"):
        """Save experiment results to a JSON file."""
        results = {
            "experiment_id": self.experiment_id,
            "timestamp": datetime.utcnow().isoformat(),
            "metrics": self.metrics,
            "feature_importances": self.feature_importances,
            "models": list(self.models.keys())
        }
        
        results_path = self.output_dir / filename
        with open(results_path, 'w') as f:
            json.dump(results, f, indent=2)
        
        logger.info(f"Results saved to {results_path}")
        return str(results_path)

def run_automl(
    data_path: str,
    target_col: str,
    experiment_id: str,
    task_type: Optional[str] = None,
    test_size: float = 0.2,
    **kwargs
) -> Dict[str, Any]:
    """Run an AutoML pipeline on the given dataset.
    
    Args:
        data_path: Path to the input data file (CSV, Excel, etc.)
        target_col: Name of the target column
        experiment_id: Unique identifier for the experiment
        task_type: Type of ML task ('classification', 'regression', or None for auto-detect)
        test_size: Proportion of data to use for testing
        **kwargs: Additional arguments to pass to the model
        
    Returns:
        Dictionary containing experiment results
    """
    from sklearn.model_selection import train_test_split
    
    # Initialize the AutoML service
    automl = AutoMLService(experiment_id)
    
    try:
        # Load data
        logger.info(f"Loading data from {data_path}")
        data = pd.read_csv(data_path)  # Add support for other formats if needed
        
        # Preprocess data
        X, y = automl.preprocess_data(data, target_col)
        
        # Split data
        X_train, X_test, y_train, y_test = train_test_split(
            X, y, test_size=test_size, random_state=42
        )
        
        # Determine task type if not specified
        if task_type is None:
            task_type_enum = automl.detect_task_type(y)
        else:
            task_type_enum = TaskType(task_type.lower())
        
        # Train model
        model = automl.train_model(X_train, y_train, task_type_enum, **kwargs)
        
        # Evaluate model
        metrics = automl.evaluate_model(model, X_test, y_test, task_type_enum)
        
        # Save artifacts
        model_path = automl.save_model()
        results_path = automl.save_results()
        
        return {
            "status": "success",
            "experiment_id": experiment_id,
            "task_type": task_type_enum.value,
            "metrics": metrics,
            "model_path": model_path,
            "results_path": results_path,
            "feature_importances": automl.feature_importances
        }
        
    except Exception as e:
        logger.error(f"Error in AutoML pipeline: {str(e)}", exc_info=True)
        return {
            "status": "error",
            "experiment_id": experiment_id,
            "error": str(e)
        }

if __name__ == "__main__":
    # Example usage
    import argparse
    
    parser = argparse.ArgumentParser(description="Run AutoML pipeline")
    parser.add_argument("--data", required=True, help="Path to input data file")
    parser.add_argument("--target", required=True, help="Name of target column")
    parser.add_argument("--experiment-id", required=True, help="Experiment ID")
    parser.add_argument("--task-type", choices=[t.value for t in TaskType], 
                       help="Type of ML task (default: auto-detect)")
    parser.add_argument("--test-size", type=float, default=0.2, 
                       help="Proportion of data to use for testing")
    
    args = parser.parse_args()
    
    result = run_automl(
        data_path=args.data,
        target_col=args.target,
        experiment_id=args.experiment_id,
        task_type=args.task_type,
        test_size=args.test_size
    )
    
    print("\nAutoML Results:")
    print(json.dumps(result, indent=2))
