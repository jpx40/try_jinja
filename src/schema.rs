// @generated automatically by Diesel CLI.

diesel::table! {
    arch_packages (name) {
        name -> Text,
        version -> Text,
        dependencies -> Nullable<Json>,
    }
}

diesel::table! {
    complain (id) {
        id -> Int4,
        #[max_length = 1]
        title -> Nullable<Bpchar>,
        user_id -> Nullable<Int4>,
        #[max_length = 1]
        content -> Nullable<Bpchar>,
        #[max_length = 1]
        category -> Nullable<Bpchar>,
        #[max_length = 1]
        created_at -> Nullable<Bpchar>,
        #[max_length = 1]
        updated_at -> Nullable<Bpchar>,
    }
}

diesel::table! {
    packages (name) {
        name -> Text,
        size -> Text,
        version -> Text,
    }
}

diesel::table! {
    people (id) {
        id -> Int4,
        name -> Varchar,
        gender -> Bool,
        age -> Int4,
        address -> Varchar,
        #[max_length = 11]
        phone -> Varchar,
        email -> Varchar,
    }
}

diesel::table! {
    test (name) {
        name -> Varchar,
        version -> Nullable<Varchar>,
        dependencies -> Nullable<Jsonb>,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 1]
        name -> Nullable<Bpchar>,
        #[max_length = 1]
        password -> Nullable<Bpchar>,
        #[max_length = 1]
        role -> Nullable<Bpchar>,
        #[max_length = 1]
        email -> Nullable<Bpchar>,
        id -> Int4,
        #[max_length = 1]
        login_session -> Nullable<Bpchar>,
    }
}

diesel::joinable!(complain -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    arch_packages,
    complain,
    packages,
    people,
    test,
    users,
);
