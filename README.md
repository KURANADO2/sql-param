A tool for automatically replacing sql placeholders.

## Install

```bash
cargo install sql-param
```

## Usage

```bash
$ sql-param
Please input sql with placeholders.
select * from user where username = ? and email = ? and age = ? and married = ?;
Please input sql value.
zhangsan(String), null, 18(Integer), 1(Integer)
select * from user where username = 'zhangsan' and email = null and age = 18 and married = 1;
```