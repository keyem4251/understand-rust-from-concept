use serde_json::json;

pub fn fn_4_6() {
    fn_json_sample();
}

fn fn_json_sample() {
    // JSONの文字列をValue型として読み込む
    let hanako = json!({
        "name": "hanako",
        "age": 8,
        "favorites": {
            "food": ["apple", "melon"],
        }
    });

    // Value型の中身を見てみる
    println!("hanako debug: {:?}", hanako);

    // Value型は[]を使ってアクセスできる
    // 文字列はダブルクォーテーションマーク付きで返される
    println!("hanako[\"name\"]: {}", hanako["name"]);

    // 文字列はダブルクォーテーションマークなしで参照するには&str
    println!("hanako[\"name\"].as_str().unwrap(): {}", hanako["name"].as_str().unwrap());

    let mut taro = json!({});
    taro["name"] = json!("taro");
    taro["age"] = json!(10);
    taro["favorites"] = json!({
        "food": ["cake"],
    });

    let mut members = json!({});
    members["members"] = json!([taro, hanako]);

    println!("JSON: {}", members);
}
