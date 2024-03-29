// @generated automatically by Diesel CLI.

diesel::table! {
    event_logs (event_log_id) {
        event_log_id -> Uuid,
        #[max_length = 255]
        event_type -> Varchar,
        user_id -> Nullable<Uuid>,
        timestamp -> Timestamp,
        details -> Nullable<Text>,
    }
}

diesel::table! {
    invites (invite_id) {
        invite_id -> Uuid,
        generated_by_user_id -> Uuid,
        has_been_used -> Bool,
        date_used -> Nullable<Timestamp>,
        used_by_user_id -> Nullable<Uuid>,
        invite_code -> Text,
    }
}

diesel::table! {
    pictures (picture_id) {
        picture_id -> Uuid,
        uploaded_by_user_id -> Uuid,
        upload_date -> Timestamp,
        file_url -> Text,
        file_size -> Int8,
        file_format -> Text,
        resolution -> Text,
        tags -> Nullable<Array<Nullable<Text>>>,
        file_name -> Text,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        password_hash -> Text,
        date_registered -> Timestamp,
        invited_by_user_id -> Nullable<Uuid>,
        is_admin -> Bool,
        is_moderator -> Bool,
    }
}

diesel::table! {
    videos (video_id) {
        video_id -> Uuid,
        uploaded_by_user_id -> Uuid,
        upload_date -> Timestamp,
        file_url -> Text,
        file_size -> Int8,
        file_format -> Text,
        duration -> Int8,
        resolution -> Text,
        tags -> Nullable<Array<Nullable<Text>>>,
        file_name -> Text,
    }
}

diesel::joinable!(event_logs -> users (user_id));
diesel::joinable!(pictures -> users (uploaded_by_user_id));
diesel::joinable!(videos -> users (uploaded_by_user_id));

diesel::allow_tables_to_appear_in_same_query!(
    event_logs,
    invites,
    pictures,
    users,
    videos,
);
