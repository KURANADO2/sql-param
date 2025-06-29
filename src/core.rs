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

pub fn replace_placeholder(sql: &str, value: &str) -> String {
    if sql.is_empty() || value.is_empty() {
        return String::from("");
    }

    let mut result = String::new();

    let values = values(value);

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

#[cfg(test)]
mod test {
    use crate::core::replace_placeholder;

    #[test]
    fn test() {
        let sql = String::from("UPDATE user SET name = ?, age = ?, update_time = ?, id_card = ? WHERE id = ? AND deleted = ?;");
        let value = String::from("zhangsan(String), 18(Integer), 2025-06-13 16:44:56.499(Timestamp), 123456789(Long), 1(Integer), 0(Integer)");
        assert_eq!("UPDATE user SET name = 'zhangsan', age = 18, update_time = '2025-06-13 16:44:56.499', id_card = 123456789 WHERE id = 1 AND deleted = 0;", replace_placeholder(sql.as_str(), value.as_str()));
    }

    #[test]
    fn test_multiple_sql() {
        let sql = String::from("SELECT * FROM user WHERE id = ? AND deleted = 0 and sex = ?;\
        UPDATE user SET name = ?, age = ?, update_time = ?, id_card = ? WHERE id = ? AND deleted = ?;");
        let value = String::from("1(Long), male(String), \
        zhangsan(String), 18(Integer), 2025-06-13 16:44:56.499(Timestamp), 123456789(Long), 1(Integer), 0(Integer), ");
        assert_eq!("SELECT * FROM user WHERE id = 1 AND deleted = 0 and sex = 'male';UPDATE user SET name = 'zhangsan', age = 18, update_time = '2025-06-13 16:44:56.499', id_card = 123456789 WHERE id = 1 AND deleted = 0;", replace_placeholder(sql.as_str(), value.as_str()));
    }

    #[test]
    fn test_empty_sql() {
        let sql = String::from("");
        let value = String::from("zhangsan(String), 18(Integer), 2025-06-13 16:44:56.499(Timestamp), 123456789(Long), 1(Integer), 0(Integer);");
        assert_eq!("", replace_placeholder(sql.as_str(), value.as_str()));
    }

    #[test]
    fn test_empty_value() {
        let sql = String::from("UPDATE user SET name = ?, age = ?, update_time = ?, id_card = ? WHERE id = ? AND deleted = ?;");
        let value = String::from("");
        assert_eq!("", replace_placeholder(sql.as_str(), value.as_str()));
    }

    #[test]
    fn test_empty_all() {
        let sql = String::from("");
        let value = String::from("");
        assert_eq!("", replace_placeholder(sql.as_str(), value.as_str()));
    }
}
