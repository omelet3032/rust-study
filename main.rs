fn main() {

    /* 
        목표
        1. 개별 스케일링 적용
        2. 결과값 541.65 - > 541.6666... -> 541.67 출력
     */

    let scale:i64 = 1000000;

    let loss_rate:i64 = 7024;

    let current_price:i64 = 25000;
    
    // 여기서 소수점이 .6666이어도 버림처리하면 그만큼 값이 부정확해지는거 아닌가
    // 여기서 올림처리 하려면
    let recovery_rate:i64 = (loss_rate * scale) / (scale - loss_rate);

    let target_price = current_price * (scale + (recovery_rate / 2));

    let target_price_result = target_price / scale;
    println!("rr : {}", recovery_rate);
    println!("target_price_result : {}", (target_price_result as f64 / 10000.0));
    
}

