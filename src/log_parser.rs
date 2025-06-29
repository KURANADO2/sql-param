const SQL_KEYWORDS: [&str; 8] = [
    "SELECT", "INSERT", "UPDATE", "DELETE", "CREATE", "ALTER", "DROP", "WITH",
];

const VALUE_KEYWORDS: [&str; 4] = ["(String)", "(Integer)", "(Timestamp)", "(Long)"];

#[derive(Debug)]
enum LineTypeEnum {
    Sql,
    Value,
}

#[derive(Debug)]
struct Line {
    content: String,
    line_type: LineTypeEnum,
}

impl Line {
    fn new(content: String, line_type: LineTypeEnum) -> Self {
        Line { content, line_type }
    }
}

fn parse_lines(lines: Vec<&str>) -> Vec<Line> {
    let mut result: Vec<Line> = Vec::new();

    for line in lines {
        let upper = line.to_uppercase();
        if let Some((start_idx, _kw)) = SQL_KEYWORDS
            .iter()
            .filter_map(|kw| upper.find(kw).map(|idx| (idx, *kw)))
            .min_by_key(|(idx, _)| *idx)
        {
            let sql = line[start_idx..].trim();
            if sql.contains('?') {
                result.push(Line::new(sql.to_string(), LineTypeEnum::Sql));
            }
            continue;
        }

        if let Some(first_kw_pos) = VALUE_KEYWORDS.iter().filter_map(|kw| line.find(kw)).min() {
            let prefix_cut_pos = line[..first_kw_pos]
                .rfind(|c| c == ':')
                .map(|idx| idx + 1)
                .unwrap_or(0);

            let values_part = line[prefix_cut_pos..].trim().to_string();
            result.push(Line::new(values_part, LineTypeEnum::Value));
        }
    }

    result
}

#[cfg(test)]
mod test {
    use crate::log_parser::parse_lines;

    #[test]
    fn test_parse_lines() {
        let log = "
            iov-test-65759f684-pzpws iov-test 2025-06-28 20:18:06 --- [685fdd7ed0e3ea524bf29a38a05b72dc,0070d58fe48dd8f9] DEBUG 6 --- [  XNIO-1 task-6] c.a.model.test.yourbatis.Executor        : ==>  Preparing: SELECT * FROM user WHERE id = ? AND deleted = 0 and sex = ?;
            iov-test-65759f684-pzpws iov-test 2025-06-28 20:18:06 --- [685fdd7ed0e3ea524bf29a38a05b72dc,0070d58fe48dd8f9] DEBUG 6 --- [  XNIO-1 task-6] c.a.model.test.yourbatis.Executor        : ==> Parameters: 1(Long), male(String)
            iov-test-65759f684-pzpws iov-test 2025-06-28 20:18:06 --- [685fdd7ed0e3ea524bf29a38a05b72dc,0070d58fe48dd8f9] DEBUG 6 --- [  XNIO-1 task-6] c.a.model.test.business.Test    : ==> Update user info.
            iov-test-65759f684-pzpws iov-test 2025-06-28 20:18:06 --- [685fdd7ed0e3ea524bf29a38a05b72dc,0070d58fe48dd8f9] DEBUG 6 --- [  XNIO-1 task-6] c.a.model.test.yourbatis.Executor        : ==>  Preparing: UPDATE user SET name = ?, age = ?, update_time = ?, id_card = ? WHERE id = ? AND deleted = ?;
            iov-test-65759f684-pzpws iov-test 2025-06-28 20:18:06 --- [685fdd7ed0e3ea524bf29a38a05b72dc,0070d58fe48dd8f9] DEBUG 6 --- [  XNIO-1 task-6] c.a.model.test.yourbatis.Executor        : ==> Parameters: zhangsan(String), 18(Integer), 2025-06-13 16:44:56.499(Timestamp), 123456789(Long), 1(Integer), 0(Integer)
        ";
        let lines = log.lines().collect::<Vec<_>>();
        let lines = parse_lines(lines);
        assert_eq!(
            "SELECT * FROM user WHERE id = ? AND deleted = 0 and sex = ?;",
            lines.get(0).unwrap().content
        );
        assert_eq!("1(Long), male(String)", lines.get(1).unwrap().content);
        assert_eq!("UPDATE user SET name = ?, age = ?, update_time = ?, id_card = ? WHERE id = ? AND deleted = ?;", lines.get(2).unwrap().content);
        assert_eq!("zhangsan(String), 18(Integer), 2025-06-13 16:44:56.499(Timestamp), 123456789(Long), 1(Integer), 0(Integer)", lines.get(3).unwrap().content);
    }
}
