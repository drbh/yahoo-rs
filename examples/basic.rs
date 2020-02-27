use yahoo_rs::*;

fn main() {
    let data = YahooFinance::new("TVIX")
        .interval("1m")
        // .include_pre_post(true) // throwing error on deser
        .set_period1(1580406600)
        .set_period2(1581011340)
        // .events("div")
        // .events("split")
        .fetch();

    println!("[X] Fetch Complete");
    // println!("{:#?}", data);
}
