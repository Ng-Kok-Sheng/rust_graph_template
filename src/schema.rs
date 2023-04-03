// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        full_name -> Varchar,
        username -> Varchar,
        password -> Varchar,
        email_address -> Varchar,
    }
}

diesel::table! {
    users_workouts (user_id, workout_id) {
        user_id -> Int4,
        workout_id -> Int4,
    }
}

diesel::table! {
    workouts (id) {
        id -> Int4,
        workout_name -> Varchar,
        muscle_group -> Nullable<Array<Nullable<Text>>>,
        split_group -> Varchar,
    }
}

diesel::joinable!(users_workouts -> users (user_id));
diesel::joinable!(users_workouts -> workouts (workout_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    users_workouts,
    workouts,
);
