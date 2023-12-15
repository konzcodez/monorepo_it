// @generated automatically by Diesel CLI.

diesel::table! {
    computers (id) {
        id -> Int4,
        ip -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        os -> Nullable<Varchar>,
        snum -> Nullable<Varchar>,
        notes -> Nullable<Varchar>,
        model -> Nullable<Varchar>,
        manufacturer -> Nullable<Varchar>,
        cpu -> Nullable<Varchar>,
        memory -> Nullable<Varchar>,
        storage -> Nullable<Varchar>,
        install_date -> Nullable<Varchar>,
    }
}
