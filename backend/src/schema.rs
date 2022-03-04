table! {
    problems (id) {
        id -> Integer,
        title -> Text,
        accepted_cnt -> Nullable<Integer>,
        submit_cnt -> Nullable<Integer>,
        description -> Text,
        input_desc -> Text,
        output_desc -> Text,
        difficulty -> Text,
        time_limit -> Nullable<Integer>,
        memory_limit -> Nullable<Integer>,
    }
}

table! {
    submits (sid) {
        sid -> Integer,
        pid -> Integer,
        uid -> Nullable<Integer>,
        result -> Integer,
        submit_at -> Nullable<Text>,
        language -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    problems,
    submits,
);
