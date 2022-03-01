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
