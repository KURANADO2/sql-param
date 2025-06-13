use std::io::stdin;

#[derive(Debug)]
struct Value {
    field_value: String,
    field_type: String,
}

impl Value {
    fn is_string(&self) -> bool {
        self.field_type == "String" || self.field_type == "Timestamp"
    }
}

fn values(value: &str) -> Vec<Value> {
    value
        .split(",")
        .map(|v| {
            let mut split = v.trim().split('(');
            return Value {
                field_value: String::from(split.next().unwrap()),
                field_type: String::from(
                    split.next().unwrap_or("").split(')').next().unwrap_or(""),
                ),
            };
        })
        .collect::<Vec<Value>>()
}

fn replace_placeholder(sql: &str, values: Vec<Value>) -> String {
    let mut result = String::new();

    let mut i = 0;
    for char in sql.chars() {
        if char == '?' {
            if let Some(v) = values.get(i) {
                if v.is_string() {
                    result.push_str(format!("'{}'", v.field_value.as_str()).as_str());
                } else {
                    result.push_str(format!("{}", v.field_value.as_str()).as_str());
                }
            }
            i += 1;
        } else {
            result.push(char);
        }
    }

    result
}

fn main() {
    println!("> Input sql with placeholders...");
    let mut sql = String::new();
    stdin().read_line(&mut sql).expect("Failed to read sql");

    println!("\n> Input sql values with type...");
    let mut value = String::new();
    stdin().read_line(&mut value).expect("Failed to read value");

    println!("\n> Result:");
    print!(
        "{}",
        replace_placeholder(sql.as_str(), values(value.as_str()))
    );
}
