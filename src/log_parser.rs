use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
pub struct LogParser {
    pub sql: Vec<String>,
    pub value: Vec<String>,
}

impl LogParser {
    pub fn parse_lines(lines: Vec<&str>) -> Option<LogParser> {
        if lines.is_empty() {
            return None;
        }

        let mut sql_lines: Vec<String> = Vec::new();
        let mut value_lines: Vec<String> = Vec::new();

        let sql_regex = Regex::new(r"Preparing:\s*(.+)").unwrap();
        let value_regex = Regex::new(r"Parameters:\s*(.+)").unwrap();

        for line in lines {
            if let Some(caps) = sql_regex.captures(line) {
                sql_lines.push(caps[1].to_string());
            }
            if let Some(caps) = value_regex.captures(line) {
                value_lines.push(caps[1].to_string() + ", ");
            }
        }

        Some(LogParser {
            sql: sql_lines,
            value: value_lines,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::log_parser::LogParser;

    #[test]
    fn test_parse_lines() {
        let log = "
            iov-test-65759f684-pzpws iov-test 2025-06-28 20:18:06 --- [685fdd7ed0e3ea524bf29a38a05b72dc,0070d58fe48dd8f9] DEBUG 6 --- [  XNIO-1 task-6] c.a.model.test.yourbatis.Executor        : ==>  Preparing: SELECT * FROM user WHERE id = ? AND deleted = 0 and sex = ?;
            iov-test-65759f684-pzpws iov-test 2025-06-28 20:18:06 --- [685fdd7ed0e3ea524bf29a38a05b72dc,0070d58fe48dd8f9] DEBUG 6 --- [  XNIO-1 task-6] c.a.model.test.yourbatis.Executor        : ==> Parameters: 1(Long), male(String)
            iov-test-65759f684-pzpws iov-test 2025-06-28 20:18:06 --- [685fdd7ed0e3ea524bf29a38a05b72dc,0070d58fe48dd8f9] DEBUG 6 --- [  XNIO-1 task-6] c.a.model.test.business.Test    : ==> Update user info.
            iov-test-65759f684-pzpws iov-test 2025-06-28 20:18:06 --- [685fdd7ed0e3ea524bf29a38a05b72dc,0070d58fe48dd8f9] DEBUG 6 --- [  XNIO-1 task-6] c.a.model.test.yourbatis.Executor        : ==>  Preparing: UPDATE user SET name = ?, age = ?, update_time = ?, id_card = ? WHERE id = ? AND deleted = ?;
            iov-test-65759f684-pzpws iov-test 2025-06-28 20:18:06 --- [685fdd7ed0e3ea524bf29a38a05b72dc,0070d58fe48dd8f9] DEBUG 6 --- [  XNIO-1 task-6] c.a.model.test.yourbatis.Executor        : ==> Parameters: zhangsan(String), 18(Integer), 2025-06-13 16:44:56.499(Timestamp), 123456789(Long), 1(Integer), 0(Integer)
        ";
        let log_parser = LogParser::parse_lines(log.lines().collect::<Vec<_>>()).unwrap();
        assert_eq!(
            "SELECT * FROM user WHERE id = ? AND deleted = 0 and sex = ?;",
            log_parser.sql[0]
        );
        assert_eq!("1(Long), male(String), ", log_parser.value[0]);
        assert_eq!("UPDATE user SET name = ?, age = ?, update_time = ?, id_card = ? WHERE id = ? AND deleted = ?;", log_parser.sql[1]);
        assert_eq!("zhangsan(String), 18(Integer), 2025-06-13 16:44:56.499(Timestamp), 123456789(Long), 1(Integer), 0(Integer), ", log_parser.value[1]);
    }

    #[test]
    fn test_empty_lines() {
        let log = "";
        let log_parser = LogParser::parse_lines(log.lines().collect::<Vec<_>>());
        assert_eq!(None, log_parser);
    }
}
