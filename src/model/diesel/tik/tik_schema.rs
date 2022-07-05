table! {
    check_list (id) {
        id -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
        deleted -> Int4,
        name -> Varchar,
        user_id -> Int8,
        is_complete -> Int4,
    }
}

table! {
    todo (id) {
        id -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
        deleted -> Int4,
        name -> Varchar,
        tags -> Varchar,
        user_id -> Int8,
        is_complete -> Int4,
        priority -> Int4,
        schedule_time -> Int8,
        description -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    check_list,
    todo,
);
