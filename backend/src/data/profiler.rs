use anyhow::{Context, Result};
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnStats {
    pub name: String,
    pub dtype: String,
    pub null_count: usize,
    pub unique_count: Option<usize>,
    pub stats: ColumnStatsValues,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColumnStatsValues {
    Numeric {
        min: Option<f64>,
        max: Option<f64>,
        mean: Option<f64>,
        median: Option<f64>,
        std_dev: Option<f64>,
        q1: Option<f64>,
        q3: Option<f64>,
        skewness: Option<f64>,
        kurtosis: Option<f64>,
    },
    Categorical {
        top_values: Vec<(String, usize)>,
        num_categories: usize,
    },
    Boolean {
        true_count: usize,
        false_count: usize,
    },
    DateTime {
        min: Option<String>,
        max: Option<String>,
    },
    Text {
        avg_length: f64,
        min_length: Option<u32>,
        max_length: Option<u32>,
    },
    Unsupported {},
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataProfile {
    pub file_path: String,
    pub num_rows: usize,
    pub num_columns: usize,
    pub total_size_bytes: u64,
    pub columns: Vec<ColumnStats>,
    pub missing_values: bool,
    pub potential_issues: Vec<String>,
    pub suggested_actions: Vec<String>,
}

pub struct DataProfiler;

impl DataProfiler {
    pub fn new() -> Self {
        DataProfiler
    }

    pub async fn profile_file<P: AsRef<Path>>(&self, file_path: P) -> Result<DataProfile> {
        let file_path = file_path.as_ref();
        let file_size = std::fs::metadata(file_path)?.len();

        // Read the file with Polars
        let df = self.read_file(file_path).await?;
        
        // Get basic info
        let num_rows = df.height();
        let num_columns = df.width();
        
        // Profile each column
        let mut columns = Vec::with_capacity(num_columns);
        let mut has_missing_values = false;
        
        for name in df.get_column_names() {
            let series = df.column(name)?;
            let stats = self.profile_column(series)?;
            
            if stats.null_count > 0 {
                has_missing_values = true;
            }
            
            columns.push(stats);
        }
        
        // Detect potential issues
        let potential_issues = self.detect_issues(&df).await?;
        
        // Generate suggestions
        let suggested_actions = self.generate_suggestions(&df, &potential_issues).await?;

        Ok(DataProfile {
            file_path: file_path.to_string_lossy().to_string(),
            num_rows,
            num_columns,
            total_size_bytes: file_size,
            columns,
            missing_values: has_missing_values,
            potential_issues,
            suggested_actions,
        })
    }
    
    async fn read_file<P: AsRef<Path>>(&self, file_path: P) -> Result<DataFrame> {
        let file_path = file_path.as_ref();
        let ext = file_path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        match ext.as_str() {
            "csv" => CsvReader::from_path(file_path)?.finish(),
            "parquet" | "pq" => {
                let file = std::fs::File::open(file_path)?;
                ParquetReader::new(file).finish()
            },
            "json" => JsonReader::new(std::fs::File::open(file_path)?).finish(),
            "xlsx" | "xls" => {
                let xls: Xlsx<_> = Xlsx::new(file_path)?;
                xls.finish()
            },
            _ => Err(PolarsError::ComputeError(
                format!("Unsupported file format: {}", ext).into()
            )),
        }
        .map_err(Into::into)
    }
    
    fn profile_column(&self, series: &Series) -> Result<ColumnStats> {
        let name = series.name().to_string();
        let dtype = series.dtype().to_string();
        let null_count = series.null_count();
        let unique_count = series.unique()?.len();
        
        let stats = match series.dtype() {
            DataType::Int8 | DataType::Int16 | DataType::Int32 | DataType::Int64 |
            DataType::UInt8 | DataType::UInt16 | DataType::UInt32 | DataType::UInt64 |
            DataType::Float32 | DataType::Float64 => {
                let s = series.cast(&DataType::Float64)?.f64()?;
                ColumnStatsValues::Numeric {
                    min: s.min(),
                    max: s.max(),
                    mean: s.mean(),
                    median: s.median(),
                    std_dev: s.std(1.0),
                    q1: s.quantile(0.25).unwrap_or(None),
                    q3: s.quantile(0.75).unwrap_or(None),
                    skewness: s.skew(1.0).ok(),
                    kurtosis: s.kurtosis(1.0).ok(),
                }
            },
            DataType::Boolean => {
                let s = series.bool()?;
                let true_count = s.sum().unwrap_or(0) as usize;
                ColumnStatsValues::Boolean {
                    true_count,
                    false_count: s.len() - true_count,
                }
            },
            DataType::Utf8 => {
                let s = series.utf8()?;
                let lengths = s.into_iter().map(|opt| opt.map(|s| s.len() as u32));
                let lengths: Vec<u32> = lengths.filter_map(|x| x).collect();
                
                if lengths.is_empty() {
                    ColumnStatsValues::Text {
                        avg_length: 0.0,
                        min_length: None,
                        max_length: None,
                    }
                } else {
                    let avg_length = lengths.iter().sum::<u32>() as f64 / lengths.len() as f64;
                    ColumnStatsValues::Text {
                        avg_length,
                        min_length: lengths.iter().min().copied(),
                        max_length: lengths.iter().max().copied(),
                    }
                }
            },
            DataType::Date | DataType::Datetime(_, _) | DataType::Duration(_) | DataType::Time => {
                let s = series.utf8()?;
                ColumnStatsValues::DateTime {
                    min: s.min().map(|s| s.to_string()),
                    max: s.max().map(|s| s.to_string()),
                }
            },
            DataType::Categorical(_) => {
                let s = series.categorical()?;
                let counts = s.value_counts()?;
                let top_values = counts
                    .iter()
                    .map(|(val, count)| (val.to_string(), count as usize))
                    .collect();
                    
                ColumnStatsValues::Categorical {
                    top_values,
                    num_categories: s.get_categories().len(),
                }
            },
            _ => ColumnStatsValues::Unsupported {},
        };
        
        Ok(ColumnStats {
            name,
            dtype,
            null_count,
            unique_count: Some(unique_count),
            stats,
        })
    }
    
    async fn detect_issues(&self, df: &DataFrame) -> Result<Vec<String>> {
        let mut issues = Vec::new();
        
        // Check for high percentage of missing values
        for name in df.get_column_names() {
            let series = df.column(name)?;
            let null_count = series.null_count();
            let total = series.len();
            let null_percentage = (null_count as f64 / total as f64) * 100.0;
            
            if null_percentage > 30.0 {
                issues.push(format!(
                    "Column '{}' has {:.1}% missing values",
                    name, null_percentage
                ));
            }
        }
        
        // Check for constant columns
        for name in df.get_column_names() {
            let series = df.column(name)?;
            if series.n_unique()? == 1 {
                issues.push(format!("Column '{}' is constant", name));
            }
        }
        
        // Check for duplicate rows
        let duplicate_count = df.height() - df.unique(None)?.height();
        if duplicate_count > 0 {
            issues.push(format!("Found {} duplicate rows", duplicate_count));
        }
        
        // Check for high cardinality categorical variables
        for name in df.get_column_names() {
            let series = df.column(name)?;
            if let DataType::Categorical(_) = series.dtype() {
                let cardinality = series.n_unique()?;
                let cardinality_ratio = cardinality as f64 / series.len() as f64;
                
                if cardinality_ratio > 0.5 && series.len() > 1000 {
                    issues.push(format!(
                        "Column '{}' has high cardinality ({} unique values, {:.1}% of rows)",
                        name, cardinality, cardinality_ratio * 100.0
                    ));
                }
            }
        }
        
        Ok(issues)
    }
    
    async fn generate_suggestions(&self, df: &DataFrame, issues: &[String]) -> Result<Vec<String>> {
        let mut suggestions = Vec::new();
        
        // Add general suggestions
        if df.height() < 1000 {
            suggestions.push("Dataset is relatively small. Consider collecting more data or using data augmentation techniques.".to_string());
        }
        
        // Add issue-specific suggestions
        for issue in issues {
            if issue.contains("missing values") {
                suggestions.push("Consider imputing missing values or removing rows/columns with high missing rates.".to_string());
            } else if issue.contains("duplicate rows") {
                suggestions.push("Consider removing duplicate rows if they don't provide additional information.".to_string());
            } else if issue.contains("high cardinality") {
                suggestions.push("Consider using target encoding, hashing, or other techniques for high cardinality categorical variables.".to_string());
            }
        }
        
        // Add type-specific suggestions
        for name in df.get_column_names() {
            let series = df.column(name)?;
            
            match series.dtype() {
                DataType::Boolean => {
                    suggestions.push(format!("Consider one-hot encoding for boolean column '{}'", name));
                },
                DataType::Categorical(_) => {
                    let cardinality = series.n_unique()?;
                    if cardinality > 50 {
                        suggestions.push(format!("Consider target encoding or feature hashing for high-cardinality categorical column '{}'", name));
                    }
                },
                DataType::Utf8 => {
                    suggestions.push(format!("Consider text processing or feature extraction for text column '{}'", name));
                },
                DataType::Float64 | DataType::Float32 => {
                    if let Ok(s) = series.f64() {
                        if s.null_count() > 0 {
                            suggestions.push(format!("Consider imputing missing values in numerical column '{}'", name));
                        }
                        
                        if let (Some(min), Some(max)) = (s.min(), s.max()) {
                            if (max - min).abs() > 1000.0 {
                                suggestions.push(format!("Consider scaling/normalizing numerical column '{}' (range: {} to {})", name, min, max));
                            }
                        }
                    }
                },
                _ => {}
            }
        }
        
        // Deduplicate suggestions
        suggestions.sort();
        suggestions.dedup();
        
        Ok(suggestions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::prelude::*;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_profile_numeric_data() {
        let df = df! [
            "a" => &[1, 2, 3, 4, 5],
            "b" => &[1.0, f64::NAN, 3.0, 4.0, 5.0],
            "c" => &[Some(true), Some(false), Some(true), None, Some(false)],
            "d" => &["a", "b", "c", "d", "e"],
        ].unwrap();
        
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.csv");
        
        let mut file = std::fs::File::create(&file_path).unwrap();
        CsvWriter::new(&mut file).finish(&mut df.clone()).unwrap();
        
        let profiler = DataProfiler::new();
        let profile = profiler.profile_file(&file_path).await.unwrap();
        
        assert_eq!(profile.num_rows, 5);
        assert_eq!(profile.num_columns, 4);
        assert!(!profile.potential_issues.is_empty());
        assert!(!profile.suggested_actions.is_empty());
    }
}
