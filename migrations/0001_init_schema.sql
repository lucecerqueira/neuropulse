CREATE TYPE user_role AS ENUM ('admin', 'client', 'clinician');
CREATE TYPE subscription_tier AS ENUM ('free_guest', 'free_community', 'pro_athlete', 'performance_lab', 'clinical');

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    role user_role DEFAULT 'client' NOT NULL,
    tier subscription_tier DEFAULT 'free_community' NOT NULL,
    stripe_customer_id VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE cognitive_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    level_completed INT CHECK (level_completed BETWEEN 1 AND 5),
    avg_reaction_time_ms FLOAT NOT NULL,
    saccadic_latency_ms FLOAT,
    heart_rate_bpm FLOAT,
    respiration_rate_bpm FLOAT,
    hrv_rmssd FLOAT,
    cns_readiness_score FLOAT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE admin_audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    admin_id UUID REFERENCES users(id),
    action VARCHAR(255) NOT NULL,
    target_user_id UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);
