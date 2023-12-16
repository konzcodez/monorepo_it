// @generated automatically by Diesel CLI.

diesel::table! {
    computers (id) {
        id -> Int4,
        #[max_length = 12]
        ip -> Varchar,
        #[max_length = 25]
        name -> Varchar,
        #[max_length = 5]
        os -> Varchar,
        #[max_length = 25]
        snum -> Varchar,
        notes -> Text,
        #[max_length = 25]
        model -> Varchar,
        #[max_length = 25]
        manufacturer -> Varchar,
        #[max_length = 25]
        cpu -> Varchar,
        #[max_length = 25]
        memory -> Varchar,
        #[max_length = 25]
        storage -> Varchar,
        #[max_length = 25]
        installdate -> Varchar,
    }
}
