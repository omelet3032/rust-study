fn main() {


    let scale:i64 = 10000;

    let loss_rate:i64 = 7000;

    let current_price:i64 = 25000;
    
    let recovery_rate:f64 = ((loss_rate * scale) / (10000 - loss_rate)) as f64;

    let recovery_rate_bp:i64 = recovery_rate as i64;
    let target_price = current_price * (10000 + (recovery_rate_bp / 2));

    println!("rr : {}", recovery_rate);
    println!("target_price : {}", target_price / scale);
    
}

