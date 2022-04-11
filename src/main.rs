use chrono::Duration;
use human_duration::human_duration;
use bcrypt::hash;

fn main() {
    for cost in 0..=24 {
        let span = Duration::span(|| {
            let _ = hash("e54847adfdc8fc9e7a0fc06e", cost);
        });
        println!("cost: {} duration: {}", cost, human_duration(&span.to_std().unwrap()));
    }
}
