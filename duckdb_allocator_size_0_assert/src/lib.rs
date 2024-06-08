#[cfg(test)]
mod tests {
    use duckdb::params;

    #[test]
    fn incorrect_behaviour() {
        use duckdb::Connection;
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "
            CREATE TYPE MyEnum AS ENUM ('A');
            CREATE TABLE purchases (
                currency MyEnum NOT NULL,
                CHECK (
                    currency = 'A'
                )
            );
        ",
        )
        .unwrap();
        conn.execute(
            "
         INSERT INTO purchases (currency) VALUES ('A'::MyEnum)
        ",
            params![],
        )
        .unwrap();
    }

    #[test]
    fn correct_behaviour() {
        use duckdb::Connection;
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "
            CREATE TYPE MyEnum AS ENUM ('A');
            CREATE TABLE purchases (
                currency MyEnum NOT NULL,
            );
        ",
        )
        .unwrap();
        conn.execute(
            "
         INSERT INTO purchases (currency) VALUES ('A'::MyEnum)
        ",
            params![],
        )
        .unwrap();
    }
}
