// @generated automatically by Diesel CLI.
// Modified for Losselot

diesel::table! {
    schema_versions (id) {
        id -> Integer,
        version -> Text,
        name -> Text,
        features -> Text,
        introduced_at -> Text,
    }
}

diesel::table! {
    analysis_results (id) {
        id -> Integer,
        file_path -> Text,
        file_name -> Text,
        analyzed_at -> Text,
        schema_version -> Text,
        verdict -> Text,
        combined_score -> Integer,
        spectral_score -> Integer,
        binary_score -> Integer,
        bitrate -> Integer,
        sample_rate -> Integer,
        duration_secs -> Nullable<Double>,
        encoder -> Nullable<Text>,
        lowpass -> Nullable<Integer>,
        rms_full -> Nullable<Double>,
        rms_mid_high -> Nullable<Double>,
        rms_high -> Nullable<Double>,
        rms_upper -> Nullable<Double>,
        rms_19_20k -> Nullable<Double>,
        rms_ultrasonic -> Nullable<Double>,
        high_drop -> Nullable<Double>,
        upper_drop -> Nullable<Double>,
        ultrasonic_drop -> Nullable<Double>,
        ultrasonic_flatness -> Nullable<Double>,
        cutoff_variance -> Nullable<Double>,
        avg_cutoff_freq -> Nullable<Double>,
        rolloff_slope -> Nullable<Double>,
        transition_width -> Nullable<Double>,
        natural_rolloff -> Nullable<Integer>,
        binary_details_json -> Nullable<Text>,
        flags -> Nullable<Text>,
        error -> Nullable<Text>,
        file_hash -> Nullable<Text>,
    }
}

diesel::table! {
    decision_nodes (id) {
        id -> Integer,
        node_type -> Text,
        title -> Text,
        description -> Nullable<Text>,
        status -> Text,
        created_at -> Text,
        updated_at -> Text,
        metadata_json -> Nullable<Text>,
    }
}

diesel::table! {
    decision_edges (id) {
        id -> Integer,
        from_node_id -> Integer,
        to_node_id -> Integer,
        edge_type -> Text,
        weight -> Nullable<Double>,
        rationale -> Nullable<Text>,
        created_at -> Text,
    }
}

diesel::table! {
    decision_context (id) {
        id -> Integer,
        node_id -> Integer,
        context_type -> Text,
        content_json -> Text,
        captured_at -> Text,
    }
}

diesel::table! {
    decision_sessions (id) {
        id -> Integer,
        name -> Nullable<Text>,
        started_at -> Text,
        ended_at -> Nullable<Text>,
        root_node_id -> Nullable<Integer>,
        summary -> Nullable<Text>,
    }
}

diesel::table! {
    session_nodes (session_id, node_id) {
        session_id -> Integer,
        node_id -> Integer,
        added_at -> Text,
    }
}

diesel::table! {
    command_log (id) {
        id -> Integer,
        command -> Text,
        description -> Nullable<Text>,
        working_dir -> Nullable<Text>,
        exit_code -> Nullable<Integer>,
        stdout -> Nullable<Text>,
        stderr -> Nullable<Text>,
        started_at -> Text,
        completed_at -> Nullable<Text>,
        duration_ms -> Nullable<Integer>,
        decision_node_id -> Nullable<Integer>,
    }
}
