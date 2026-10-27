script_code = '''import glob

files = glob.glob("migrations/*.sql")
target = files[0] if files else "migrations/0001_init_schema.sql"

sql_content = """-- 1. Enums de Autenticação e Níveis
DO $$ BEGIN
    CREATE TYPE user_role AS ENUM (\'client\', \'clinician\', \'admin\');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE subscription_tier AS ENUM (\'free_community\', \'pro_clinician\', \'enterprise\');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- 2. Tabela de Usuários (Neuropulse Server)
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    full_name TEXT NOT NULL,
    role user_role NOT NULL DEFAULT \'client\',
    tier subscription_tier NOT NULL DEFAULT \'free_community\',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 3. Tabela de Sessões Cognitivas Gerais (Neuropulse Server)
CREATE TABLE IF NOT EXISTS cognitive_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    level_completed INT NOT NULL,
    avg_reaction_time_ms REAL NOT NULL,
    saccadic_latency_ms REAL NOT NULL,
    heart_rate_bpm REAL NOT NULL DEFAULT 72.0,
    respiration_rate_bpm REAL NOT NULL DEFAULT 16.0,
    cns_readiness_score REAL NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 4. Pacientes (Neurosync Platform)
CREATE TABLE IF NOT EXISTS patients (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    full_name TEXT NOT NULL,
    medical_record_id TEXT NOT NULL,
    birth_date DATE NOT NULL,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 5. Sessões Neurofisiológicas
CREATE TABLE IF NOT EXISTS assessment_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_id UUID REFERENCES patients(id),
    session_id UUID,
    bcea_68_score REAL NOT NULL DEFAULT 0.0,
    main_sequence_ratio_avg REAL NOT NULL DEFAULT 0.0,
    pne_entropy_avg REAL NOT NULL DEFAULT 0.0,
    blink_rate_per_min REAL NOT NULL DEFAULT 0.0,
    vergence_z_cm_avg REAL NOT NULL DEFAULT 0.0,
    started_at TIMESTAMPTZ DEFAULT NOW(),
    ended_at TIMESTAMPTZ
);

-- 6. Eye Tracking
CREATE TABLE IF NOT EXISTS eye_tracking_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id UUID NOT NULL,
    frame_number INT NOT NULL,
    gaze_x REAL NOT NULL,
    gaze_y REAL NOT NULL,
    gaze_z REAL DEFAULT 0.0,
    ear_left REAL DEFAULT 0.0,
    ear_right REAL DEFAULT 0.0,
    confidence REAL NOT NULL DEFAULT 1.0,
    time TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 7. Stroop
CREATE TABLE IF NOT EXISTS stroop_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id UUID NOT NULL,
    trial_number INT NOT NULL,
    is_congruent BOOLEAN NOT NULL,
    displayed_word TEXT NOT NULL,
    displayed_color TEXT NOT NULL,
    selected_color TEXT NOT NULL,
    is_correct BOOLEAN NOT NULL,
    response_time_ms REAL NOT NULL,
    bpm REAL DEFAULT 72.0,
    cardiac_acceleration REAL DEFAULT 0.0,
    cardiac_deceleration REAL DEFAULT 0.0,
    hrv_rmssd REAL DEFAULT 0.0
);

-- 8. Go/No-Go
CREATE TABLE IF NOT EXISTS emotional_gonogo_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id UUID NOT NULL,
    trial_number INT NOT NULL,
    stimulus_type TEXT NOT NULL,
    emotion_label TEXT NOT NULL,
    user_action TEXT NOT NULL,
    is_correct BOOLEAN NOT NULL,
    reaction_time_ms REAL NOT NULL,
    hold_time_ms REAL NOT NULL DEFAULT 0.0,
    gaze_latency_ms REAL NOT NULL DEFAULT 0.0,
    zygomaticus_activation REAL NOT NULL DEFAULT 0.0,
    corrugator_activation REAL NOT NULL DEFAULT 0.0,
    bpm REAL DEFAULT 72.0,
    cardiac_acceleration REAL DEFAULT 0.0,
    cardiac_deceleration REAL DEFAULT 0.0,
    hrv_rmssd REAL DEFAULT 0.0
);

-- 9. PASAT
CREATE TABLE IF NOT EXISTS pasat_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id UUID NOT NULL,
    trial_number INT NOT NULL,
    current_digit INT NOT NULL,
    previous_digit INT NOT NULL,
    expected_sum INT NOT NULL,
    user_sum INT,
    is_correct BOOLEAN NOT NULL,
    response_time_ms REAL NOT NULL,
    inter_digit_interval_ms INT NOT NULL DEFAULT 0,
    blink_count_in_trial INT NOT NULL DEFAULT 0,
    corrugator_tension REAL NOT NULL DEFAULT 0.0,
    bpm REAL DEFAULT 72.0,
    cardiac_acceleration REAL DEFAULT 0.0,
    cardiac_deceleration REAL DEFAULT 0.0,
    hrv_rmssd REAL DEFAULT 0.0
);

-- 10. Reversal Learning
CREATE TABLE IF NOT EXISTS reversal_learning_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id UUID NOT NULL,
    trial_number INT NOT NULL,
    selected_option TEXT NOT NULL,
    is_correct BOOLEAN NOT NULL,
    received_feedback TEXT NOT NULL,
    is_reversal_phase BOOLEAN NOT NULL,
    response_time_ms REAL NOT NULL,
    mouse_path_auc REAL NOT NULL DEFAULT 0.0,
    mouse_jitter_score REAL NOT NULL DEFAULT 0.0,
    post_feedback_corrugator REAL NOT NULL DEFAULT 0.0,
    bpm REAL DEFAULT 72.0,
    cardiac_acceleration REAL DEFAULT 0.0,
    cardiac_deceleration REAL DEFAULT 0.0,
    hrv_rmssd REAL DEFAULT 0.0
);
"""

with open(target, "w") as f:
    f.write(sql_content)

print(f"Migracao completa gravada em: {target}")
'''

with open("update_full_migration.py", "w") as f:
    f.write(script_code)

print("update_full_migration.py atualizado.")
