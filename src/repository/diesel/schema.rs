table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        memo -> Nullable<Text>,
        done -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
