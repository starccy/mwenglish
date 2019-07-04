table! {
    exam_manage (id) {
        id -> Varchar,
        exam_name -> Varchar,
        exam_date -> Datetime,
        exam_end_date -> Datetime,
        exam_last_time -> Integer,
    }
}

table! {
    fill_questions (id) {
        id -> Varchar,
        subject_id -> Varchar,
        question_content -> Varchar,
        corrent_answer -> Varchar,
        value -> Integer,
        created_at -> Datetime,
    }
}

table! {
    paper_manage (id) {
        id -> Varchar,
        exam_id -> Varchar,
        subject -> Varchar,
        question_type_no -> Integer,
        question_id -> Varchar,
        question_score -> Integer,
    }
}

table! {
    question_type (no) {
        no -> Integer,
        #[sql_name = "type"]
        type_ -> Varchar,
    }
}

table! {
    score_manage (id) {
        id -> Varchar,
        exam_id -> Varchar,
        user_id -> Varchar,
        subject_id -> Varchar,
        score -> Integer,
    }
}

table! {
    single_questions (id) {
        id -> Varchar,
        subject_id -> Varchar,
        question_content -> Varchar,
        answer_a -> Varchar,
        answer_b -> Varchar,
        answer_c -> Nullable<Varchar>,
        answer_d -> Nullable<Varchar>,
        corrent_answer -> Varchar,
        value -> Integer,
        created_at -> Datetime,
    }
}

table! {
    subject_info (id) {
        id -> Varchar,
        subject -> Varchar,
    }
}

table! {
    tf_questions (id) {
        id -> Varchar,
        subject_id -> Varchar,
        question_content -> Varchar,
        corrent_answer -> Tinyint,
        value -> Integer,
        created_at -> Datetime,
    }
}

table! {
    users (id) {
        id -> Varchar,
        phone -> Varchar,
        password -> Varchar,
        nickname -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
        is_admin -> Tinyint,
        created_at -> Datetime,
        last_modified_at -> Datetime,
    }
}

table! {
    user_session (token) {
        token -> Varchar,
        pbkey -> Varchar,
        pvkey -> Varchar,
        user_id -> Varchar,
        created_at -> Datetime,
    }
}

joinable!(paper_manage -> exam_manage (exam_id));
joinable!(paper_manage -> question_type (question_type_no));
joinable!(paper_manage -> subject_info (subject));
joinable!(score_manage -> exam_manage (exam_id));
joinable!(score_manage -> subject_info (subject_id));
joinable!(score_manage -> users (user_id));

allow_tables_to_appear_in_same_query!(
    exam_manage,
    fill_questions,
    paper_manage,
    question_type,
    score_manage,
    single_questions,
    subject_info,
    tf_questions,
    users,
    user_session,
);
