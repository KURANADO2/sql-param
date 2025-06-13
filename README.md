A tool for automatically replacing sql placeholders.

## Install

```bash
cargo install sql-param
```

## Usage

```bash
$ sql-param
> Input sql with placeholders...
UPDATE user SET name = ?, age = ?, update_time = ?, id_card = ? WHERE id = ? AND deleted = ?;   

> Input sql values with type...
zhangsan(String), 18(Integer), 2025-06-13 16:44:56.499(Timestamp), 123456789(Long), 1(Integer), 0(Integer);

> Result:
UPDATE user SET name = 'zhangsan', age = 18, update_time = '2025-06-13 16:44:56.499', id_card = 123456789 WHERE id = 1 AND deleted = 0;
```