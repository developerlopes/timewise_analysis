
pub fn linear_regression(x: &[f64], y: &[f64]) -> Option<(f64, f64)> {
    if x.len() != y.len() || x.is_empty() {
        return None;
    }

    let n = x.len() as f64;
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_xy: f64 = x.iter().zip(y).map(|(xi, yi)| xi * yi).sum();
    let sum_x_squared: f64 = x.iter().map(|xi| xi * xi).sum();

    let denominator = n * sum_x_squared - sum_x * sum_x;
    if denominator == 0.0 {
        return None;
    }

    let slope = (n * sum_xy - sum_x * sum_y) / denominator;
    let intercept = (sum_y - slope * sum_x) / n;

    Some((slope, intercept))
}

pub fn mean_squared_error(x: &[f64], y: &[f64], slope: f64, intercept: f64) -> f64 {
    let n = x.len() as f64;
    x.iter()
        .zip(y)
        .map(|(xi, yi)| {
            let y_pred = slope * xi + intercept;
            (yi - y_pred).powi(2)
        })
        .sum::<f64>()
        / n
}

pub fn r_squared(x: &[f64], y: &[f64], slope: f64, intercept: f64) -> f64 {
    let mean_y: f64 = y.iter().sum::<f64>() / y.len() as f64;
    let ss_total: f64 = y.iter().map(|yi| (yi - mean_y).powi(2)).sum();
    let ss_res: f64 = x
        .iter()
        .zip(y)
        .map(|(xi, yi)| {
            let y_pred = slope * xi + intercept;
            (yi - y_pred).powi(2)
        })
        .sum();
    1.0 - (ss_res / ss_total)
}

pub fn predict(x: &[f64], slope: f64, intercept: f64) -> Vec<f64> {
    x.iter().map(|xi| slope * xi + intercept).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regression_basic() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0];
        let (slope, intercept) = linear_regression(&x, &y).unwrap();
        assert!((slope - 2.0).abs() < 1e-6);
        assert!((intercept).abs() < 1e-6);
    }

    #[test]
    fn test_mse() {
        let x = vec![1.0, 2.0];
        let y = vec![2.0, 4.0];
        let (slope, intercept) = linear_regression(&x, &y).unwrap();
        let mse = mean_squared_error(&x, &y, slope, intercept);
        assert!(mse < 1e-6);
    }

    #[test]
    fn test_r_squared() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0];
        let (slope, intercept) = linear_regression(&x, &y).unwrap();
        let r2 = r_squared(&x, &y, slope, intercept);
        assert!((r2 - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_prediction() {
        let slope = 2.0;
        let intercept = 0.0;
        let x = vec![4.0, 5.0];
        let preds = predict(&x, slope, intercept);
        assert_eq!(preds, vec![8.0, 10.0]);
    }
}
