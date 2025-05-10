
use timewise_analysis::*;

fn main() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];

    let (slope, intercept) = linear_regression(&x, &y).expect("Erro na regressão");

    println!("y = {:.2}x + {:.2}", slope, intercept);

    let r2 = r_squared(&x, &y, slope, intercept);
    let mse = mean_squared_error(&x, &y, slope, intercept);

    println!("R² = {:.4}", r2);
    println!("MSE = {:.4}", mse);

    let future_x = vec![6.0, 7.0];
    let predictions = predict(&future_x, slope, intercept);
    println!("Previsões: {:?}", predictions);
}
