fn main() {

    let rate_scale:i64 = 10000; // 비율 스케일
    let price_scale:i64 = 100; // 화폐 스케일

    let master_scale:i64 = 1000000000000; // 정밀도 스케일 
    // let master_scale:i64 = 1000000; // 정밀도 스케일 

    let loss_rate:f64 = 70.00;
    let price:f64 = 250.00;

    let loss_rate_bp:i64 = rate_scale_calculate(rate_scale, loss_rate);
    println!("loss_rate_bp : {}", loss_rate_bp); // expected 7000

    let price_bp:i64 = price_scale_calculate(price_scale, price); // expected 25000
    println!("price_bp : {}", price_bp);
    
    let required_recovery_rate:i64 = recovery_rate_calculate(master_scale,rate_scale, loss_rate_bp);
    println!("required_recovery_rate : {}", required_recovery_rate);

}

fn recovery_rate_calculate(master_scale:i64, rate_scale:i64, loss_rate_bp:i64) -> i64 {
    let loss_rate_bp_with_master_scale:i64 = loss_rate_bp * master_scale;
    let rate_scale_with_master_scale:i64 = rate_scale * master_scale;

    // let recovery_rate_final:i64 = loss_rate_bp_with_master_scale / (rate_scale_with_master_scale - loss_rate_bp_with_master_scale);
    let recovery_rate_final:i64 = loss_rate_bp_with_master_scale / (rate_scale - loss_rate_bp);

    // let numerator:i64 = loss_rate_bp_with_master_scale;
    // let denomiator:i64 = rate_scale_with_master_scale - loss_rate_bp_with_master_scale;

    // let recovery_rate_final = (numerator + denomiator / 2) / denomiator;
    
    recovery_rate_final / 100000000
}
    // let numerator:i64 = loss_rate_bp_with_master_scale;
    // let denomiator:i64 = rate_scale_with_master_scale - loss_rate_bp_with_master_scale;

    // let recovery_rate_final = (numerator + denomiator / 2) / denomiator;

fn rate_scale_calculate(rate_scale:i64, loss_rate_input:f64) -> i64 {
    
    let loss_rate_divided_100:f64 = loss_rate_input / 100.0; // 1

    let loss_rate_with_scale:f64 = loss_rate_divided_100 * rate_scale as f64; // 2 

    let loss_rate_final:i64 = loss_rate_with_scale.round() as i64; // 3

    loss_rate_final // bp 적용 mast scale 적용 x
}

fn price_scale_calculate(price_scale:i64, price_input:f64) -> i64 {

    let price_with_scale:f64 = price_input * price_scale as f64;

    let price_final:i64 = price_with_scale.round() as i64;

    price_final // bp 적용 master scale 적용x
}