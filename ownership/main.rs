use std::fmt;

fn main() {
    
    let country:Country = select_country();

    let price:f64 = enter_stock_price(&country);

    print_result(price, country);
}

fn print_result(price:f64, country:Country) {
    println!("print_result : {}, {}", price, country);

}

fn enter_stock_price(country:&Country) -> f64 {
    println!("enterstockprice에 입력되나? : {}", country);
    return 1.0; 
}

fn select_country() -> Country {
    println!("select_country 실행");
    return Country::KR
}

impl fmt::Display for Country {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            Country::KR => write!(f, "korea"),
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            Message::ResultCountry(price, country) => write!(f, "가격 : {}  국가 선택 : {}", price, country),
        }
    }

}

enum Country {
    KR,
}

enum Message {
    ResultCountry(f64, Country),
}
