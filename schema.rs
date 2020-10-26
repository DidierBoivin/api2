

table! {
    auth_group (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    auth_group_permissions (id) {
        id -> Int4,
        group_id -> Int4,
        permission_id -> Int4,
    }
}

table! {
    auth_permission (id) {
        id -> Int4,
        name -> Varchar,
        content_type_id -> Int4,
        codename -> Varchar,
    }
}

table! {
    auth_user (id) {
        id -> Int4,
        password -> Varchar,
        last_login -> Timestamptz,
        is_superuser -> Bool,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        is_staff -> Bool,
        is_active -> Bool,
        date_joined -> Timestamptz,
    }
}

table! {
    auth_user_groups (id) {
        id -> Int4,
        user_id -> Int4,
        group_id -> Int4,
    }
}

table! {
    auth_user_user_permissions (id) {
        id -> Int4,
        user_id -> Int4,
        permission_id -> Int4,
    }
}

table! {
    django_content_type (id) {
        id -> Int4,
        name -> Varchar,
        app_label -> Varchar,
        model -> Varchar,
    }
}

table! {
    myapp_agenda (id) {
        id -> Int4,
        agendaNumber -> Int4,
        contactPerson -> Nullable<Varchar>,
        dateAgendaCreation -> Timestamptz,
        mediaTypeAgenda -> Varchar,
        levelAgenda -> Varchar,
        telephoneNumber -> Varchar,
        dateAgendaTodo -> Timestamptz,
        commentsAgenda -> Nullable<Text>,
        historyAgenda -> Nullable<Text>,
        dateAgendaClose -> Nullable<Timestamptz>,
        activityAgenda -> Varchar,
        agendaCase_id -> Varchar,
        agendaPersonAssigned_id -> Int4,
        prestation_id -> Int4,
        agendaUser_id -> Int4,
    }
}

table! {
    myapp_breakdowntype (id) {
        id -> Int4,
        breakdown -> Varchar,
    }
}

table! {
    myapp_carbrand (id) {
        id -> Int4,
        brand -> Varchar,
    }
}

table! {
    myapp_case (case_number) {
        case_number -> Varchar,
        contract_serial_number -> Nullable<Varchar>,
        place_of_damage -> Nullable<Text>,
        valid_from -> Nullable<Date>,
        valid_to -> Nullable<Date>,
        case_creation_date -> Timestamptz,
        insurance_company -> Nullable<Text>,
        foreign_case_number -> Nullable<Varchar>,
        city_case -> Nullable<Varchar>,
        zipcode_case -> Nullable<Varchar>,
        requestor_case -> Nullable<Varchar>,
        telephone_case -> Nullable<Varchar>,
        distance_case -> Nullable<Int4>,
        destination_case -> Nullable<Varchar>,
        alert_message -> Nullable<Text>,
        contract_checking -> Varchar,
        case_file -> Nullable<Varchar>,
        model_name -> Nullable<Varchar>,
        breakdown_description -> Nullable<Text>,
        place_immobilisation -> Nullable<Text>,
        registration_date -> Nullable<Date>,
        gearbox -> Varchar,
        fuel_type -> Varchar,
        license_plate_number -> Varchar,
        dealership -> Nullable<Varchar>,
        coverage_limit -> Nullable<Varchar>,
        car_serial_number -> Nullable<Varchar>,
        civility -> Varchar,
        date_of_birth -> Nullable<Date>,
        medical_center_address -> Nullable<Text>,
        pathology_type -> Nullable<Varchar>,
        doctor -> Nullable<Varchar>,
        telephone -> Nullable<Varchar>,
        hospitalisation_date -> Nullable<Date>,
        medical_information -> Nullable<Text>,
        medical_comments -> Nullable<Text>,
        last_name -> Text,
        first_name -> Nullable<Varchar>,
        cell_phone -> Nullable<Varchar>,
        home_phone -> Nullable<Varchar>,
        professional_phone -> Nullable<Varchar>,
        home_address -> Nullable<Text>,
        city -> Nullable<Varchar>,
        zipcode -> Nullable<Varchar>,
        destination -> Nullable<Varchar>,
        return_date -> Nullable<Date>,
        number_passagers -> Nullable<Int4>,
        case_brand_name_id -> Int4,
        case_breakdown_type_id -> Int4,
        case_client_id -> Int4,
        case_country_id -> Varchar,
        case_pathology_id_id -> Varchar,
        case_policyholder_id -> Varchar,
        case_protocol_id -> Int4,
        case_type_of_damage_id -> Int4,
        case_user_id -> Int4,
    }
}

table! {
    myapp_contract (contract_number) {
        contract_number -> Int4,
        contract_name -> Varchar,
    }
}

table! {
    myapp_countryname (country_id) {
        country_id -> Varchar,
        country_name -> Varchar,
    }
}

table! {
    myapp_damagetype (id) {
        id -> Int4,
        damage -> Varchar,
    }
}

table! {
    myapp_field_intervention (id) {
        id -> Int4,
        zipcode_field_intervention -> Nullable<Varchar>,
        city_field_intervention -> Varchar,
        radius_field_intervention -> Nullable<Int4>,
        longitude_field_intervention -> Nullable<Numeric>,
        latitude_field_intervention -> Nullable<Numeric>,
    }
}

table! {
    myapp_pathology (pathology_id) {
        pathology_id -> Varchar,
        type_of_pathology -> Varchar,
        name_of_pathology -> Varchar,
    }
}

table! {
    myapp_prestation (id) {
        id -> Int4,
        libellePrestation -> Varchar,
    }
}

table! {
    myapp_protocol (id) {
        id -> Int4,
        protocol_number -> Varchar,
        description -> Varchar,
        prefix -> Varchar,
        in_progress -> Bool,
        validity_date -> Nullable<Timestamptz>,
        expiration_date -> Nullable<Timestamptz>,
        customer_base -> Bool,
        contract_provision_file -> Nullable<Varchar>,
        guarranty -> Bool,
        other_document -> Nullable<Varchar>,
        alert_message -> Nullable<Text>,
        protocol_contract_id -> Int4,
    }
}

table! {
    myapp_provider (id) {
        id -> Int4,
        corporate_name -> Varchar,
        manager_name -> Varchar,
        address_provider -> Varchar,
        zip_code_provider -> Varchar,
        city_provider -> Varchar,
        date_of_visit -> Nullable<Date>,
        cell_phone -> Varchar,
        main_phone_number -> Varchar,
        telephone2 -> Nullable<Varchar>,
        telephone3 -> Nullable<Varchar>,
        fax -> Varchar,
        email_provider -> Varchar,
        registered -> Bool,
        comments_provider -> Nullable<Text>,
        provider_country_id -> Varchar,
    }
}

table! {
    myapp_provider_provider_field_intervention (id) {
        id -> Int4,
        provider_id -> Int4,
        field_intervention_id -> Int4,
    }
}

table! {
    myapp_provider_provider_providertype (id) {
        id -> Int4,
        provider_id -> Int4,
        providertype_id -> Int4,
    }
}

table! {
    myapp_providertype (id) {
        id -> Int4,
        providertype_name -> Varchar,
    }
}

table! {
    myapp_report (id) {
        id -> Int4,
        report_number -> Int4,
        contact -> Varchar,
        media_type -> Varchar,
        phone_number -> Nullable<Varchar>,
        comments -> Nullable<Text>,
        date_service_report -> Timestamptz,
        report_case_id -> Varchar,
        report_prestation_id -> Int4,
        report_user_id -> Int4,
    }
}

table! {
    myapp_serviceassignment (id) {
        id -> Int4,
        assignment_number -> Int4,
        beneficiary_reimbursement -> Nullable<Bool>,
        destination_city -> Nullable<Varchar>,
        media_type_service_assignment -> Varchar,
        assignment_phonenumber -> Nullable<Varchar>,
        assignment_email -> Nullable<Varchar>,
        cost_estimate -> Nullable<Numeric>,
        invoice -> Nullable<Numeric>,
        distance_service_assignment -> Nullable<Int4>,
        date_service_assignment -> Timestamptz,
        zipcode_service_assignment -> Nullable<Varchar>,
        city_of_damage -> Nullable<Varchar>,
        comments_service_assignment -> Text,
        assignment_deleted -> Bool,
        serviceassignment_case_id -> Varchar,
        serviceassignment_country_id -> Varchar,
        serviceassignment_prestation_id -> Int4,
        serviceassignment_provider_id -> Int4,
        serviceassignment_user_id -> Int4,
    }
}

table! {
    myapp_users (firstname) {
        firstname -> Varchar,
        password -> Varchar,
        rights -> Varchar,
        lastname -> Varchar,
        activated -> Bool,
        email -> Varchar,
        comments -> Text,
    }
}

joinable!(auth_group_permissions -> auth_group (group_id));
joinable!(auth_group_permissions -> auth_permission (permission_id));
joinable!(auth_permission -> django_content_type (content_type_id));
joinable!(auth_user_groups -> auth_group (group_id));
joinable!(auth_user_groups -> auth_user (user_id));
joinable!(auth_user_user_permissions -> auth_permission (permission_id));
joinable!(auth_user_user_permissions -> auth_user (user_id));
joinable!(myapp_agenda -> myapp_case (agendaCase_id));
joinable!(myapp_agenda -> myapp_prestation (prestation_id));
joinable!(myapp_case -> auth_user (case_user_id));
joinable!(myapp_case -> myapp_breakdowntype (case_breakdown_type_id));
joinable!(myapp_case -> myapp_carbrand (case_brand_name_id));
joinable!(myapp_case -> myapp_contract (case_client_id));
joinable!(myapp_case -> myapp_damagetype (case_type_of_damage_id));
joinable!(myapp_case -> myapp_pathology (case_pathology_id_id));
joinable!(myapp_case -> myapp_protocol (case_protocol_id));
joinable!(myapp_protocol -> myapp_contract (protocol_contract_id));
joinable!(myapp_provider -> myapp_countryname (provider_country_id));
joinable!(myapp_provider_provider_field_intervention -> myapp_field_intervention (field_intervention_id));
joinable!(myapp_provider_provider_field_intervention -> myapp_provider (provider_id));
joinable!(myapp_provider_provider_providertype -> myapp_provider (provider_id));
joinable!(myapp_provider_provider_providertype -> myapp_providertype (providertype_id));
joinable!(myapp_report -> auth_user (report_user_id));
joinable!(myapp_report -> myapp_case (report_case_id));
joinable!(myapp_report -> myapp_prestation (report_prestation_id));
joinable!(myapp_serviceassignment -> auth_user (serviceassignment_user_id));
joinable!(myapp_serviceassignment -> myapp_case (serviceassignment_case_id));
joinable!(myapp_serviceassignment -> myapp_countryname (serviceassignment_country_id));
joinable!(myapp_serviceassignment -> myapp_prestation (serviceassignment_prestation_id));
joinable!(myapp_serviceassignment -> myapp_provider (serviceassignment_provider_id));

allow_tables_to_appear_in_same_query!(
    auth_group,
    auth_group_permissions,
    auth_permission,
    auth_user,
    auth_user_groups,
    auth_user_user_permissions,
    django_content_type,
    myapp_agenda,
    myapp_breakdowntype,
    myapp_carbrand,
    myapp_case,
    myapp_contract,
    myapp_countryname,
    myapp_damagetype,
    myapp_field_intervention,
    myapp_pathology,
    myapp_prestation,
    myapp_protocol,
    myapp_provider,
    myapp_provider_provider_field_intervention,
    myapp_provider_provider_providertype,
    myapp_providertype,
    myapp_report,
    myapp_serviceassignment,
    myapp_users,
);
