fn main() {

    let scale:i64 = 10000;

    let loss_rate:f64 = 70.00;

    let current_price:f64 = 250.00;
    
    let (loss_rate_bp, current_price_bp) = adjust_scale(scale, loss_rate, current_price);

    let recovery_rate_bp:i64 = (loss_rate_bp * scale) / (scale - loss_rate_bp);

    let target_price = current_price_bp * (scale + ((recovery_rate_bp + 1) / 2));

    let target_price_scale:i64 = target_price / scale;

    println!("rbp : {}", recovery_rate_bp);
    println!("tp : {}", target_price_scale);
    println!("result : {}", target_price_scale as f64 / scale as f64);
    
}

fn adjust_scale(scale:i64, loss_rate:f64, current_price:f64) -> (i64, i64) {
    let loss_rate_scale:f64 = (loss_rate / 100.0) * scale as f64;
    let current_price_scale:f64 = current_price * scale as f64;

    let loss_rate_bp:i64 = loss_rate_scale.round() as i64;
    let current_price_bp:i64 = current_price_scale.round() as i64;

    (loss_rate_bp, current_price_bp)

}
