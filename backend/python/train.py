import argparse
import json
import pandas as pd
from sklearn.model_selection import train_test_split
from sklearn.ensemble import RandomForestClassifier
from sklearn.metrics import accuracy_score, classification_report
import joblib
import os

def train_model(dataset_path, output_path):
    """
    Train a simple ML model on the provided dataset
    
    Args:
        dataset_path (str): Path to the dataset file
        output_path (str): Path to save the trained model
    
    Returns:
        dict: Training metrics
    """
    # Load dataset
    try:
        df = pd.read_csv(dataset_path)
    except Exception as e:
        return {"error": f"Failed to load dataset: {str(e)}"}
    
    # Simple preprocessing
    # For simplicity, we'll assume the last column is the target
    X = df.iloc[:, :-1]
    y = df.iloc[:, -1]
    
    # Convert categorical data to numeric if needed
    if X.select_dtypes(include=['object']).shape[1] > 0:
        X = pd.get_dummies(X)
    
    # Split data
    X_train, X_test, y_train, y_test = train_test_split(
        X, y, test_size=0.2, random_state=42
    )
    
    # Train model
    model = RandomForestClassifier(n_estimators=100, random_state=42)
    model.fit(X_train, y_train)
    
    # Make predictions
    y_pred = model.predict(X_test)
    
    # Calculate metrics
    accuracy = accuracy_score(y_test, y_pred)
    report = classification_report(y_test, y_pred, output_dict=True)
    
    # Save model
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    joblib.dump(model, output_path)
    
    # Prepare metrics
    metrics = {
        "accuracy": accuracy,
        "precision": report["weighted avg"]["precision"],
        "recall": report["weighted avg"]["recall"],
        "f1_score": report["weighted avg"]["f1-score"],
        "model_size_bytes": os.path.getsize(output_path)
    }
    
    return metrics

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Train a machine learning model")
    parser.add_argument("--dataset", type=str, required=True, help="Path to the dataset file")
    parser.add_argument("--output", type=str, required=True, help="Path to save the trained model")
    
    args = parser.parse_args()
    
    try:
        metrics = train_model(args.dataset, args.output)
        print(json.dumps(metrics))
    except Exception as e:
        print(json.dumps({"error": str(e)}))
