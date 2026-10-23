use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: String,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TrialData {
    pub trial_number: usize,
    pub trail_number: usize,
    pub reaction_time_ms: f64,
    pub is_correct: bool,
    pub timestamp: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SessionRecord {
    pub id: String,
    pub created_at: String,
    pub avg_reaction_time_ms: f64,
    pub cns_readiness_score: Option<f64>,
    pub hrv_rmssd: Option<f64>,
    pub trail_context: Option<String>,
    pub test_type: Option<String>,
    pub rppg_bpm: Option<f64>,
    pub saccadic_latency_ms: Option<f64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SaveSessionPayload {
    pub level_completed: usize,
    pub avg_reaction_time_ms: f64,
    pub saccadic_latency_ms: Option<f64>,
    pub heart_rate_bpm: Option<f64>,
    pub respiration_rate_bpm: Option<f64>,
    pub hrv_rmssd: Option<f64>,
    pub cns_readiness_score: Option<f64>,
    pub trail_context: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Page {
    Dashboard,
    SimpleReactionTest,
    ChoiceReactionTest,
    GoNoGoTest,
    RPPGMonitor,
    PricingPlans,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TimeRange {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

pub struct MotivationalTrail {
    pub index: usize,
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub color: &'static str,
}

pub const MOTIVATIONAL_TRAILS: &[MotivationalTrail] = &[
    MotivationalTrail { index: 0, id: "Trilha 1", name: "Foco Cortical Sustentado", description: "Otimização de ondas alfa/beta para atenção contínua.", color: "#06b6d4" },
    MotivationalTrail { index: 1, id: "Trilha 2", name: "Controle de Impulsividade", description: "Inibição de resposta e modulação do córtex pré-frontal.", color: "#10b981" },
    MotivationalTrail { index: 2, id: "Trilha 3", name: "Velocidade de Processamento", description: "Tempos de trânsito sináptico e resposta motora rápida.", color: "#a855f7" },
    MotivationalTrail { index: 3, id: "Trilha 4", name: "Resiliência sob Fadiga", description: "Manutenção de desempenho cognitivo sob carga autonômica.", color: "#f59e0b" },
    MotivationalTrail { index: 4, id: "Trilha 5", name: "Sincronia Mão-Olho", description: "Precisão sacádica e acoplamento visuomotor.", color: "#ef4444" },
    MotivationalTrail { index: 5, id: "Trilha 6", name: "Equilíbrio Autonômico Vagal", description: "Modulação parassimpática e coerência cardíaca.", color: "#3b82f6" },
    MotivationalTrail { index: 6, id: "Trilha 7", name: "Ativação de Alerta Fásico", description: "Prontidão do sistema reticular ativador ascendente.", color: "#ec4899" },
    MotivationalTrail { index: 7, id: "Trilha 8", name: "Flexibilidade Cognitiva", description: "Alternância rápida de set mental e inibição setorial.", color: "#14b8a6" },
    MotivationalTrail { index: 8, id: "Trilha 9", name: "Estabilidade Foveal", description: "Redução de intrusões sacádicas e micro-movimentos.", color: "#8b5cf6" },
    MotivationalTrail { index: 9, id: "Trilha 10", name: "Capacidade de Inibição Seletiva", description: "Filtro de estímulos distractores periféricos.", color: "#f97316" },
    MotivationalTrail { index: 10, id: "Trilha 11", name: "Recuperação Pós-Esforço", description: "Eficiência de reestabelecimento da homeostase.", color: "#6366f1" },
    MotivationalTrail { index: 11, id: "Trilha 12", name: "Tonificação Noradrenérgica", description: "Alerta tônico e otimização de ganho sináptico.", color: "#84cc16" },
    MotivationalTrail { index: 12, id: "Trilha 13", name: "Integração Neuro-Motora Total", description: "Sinergia completa entre biomarcadores e resposta física.", color: "#06b6d4" },
];

pub const TRAIL_COLORS: &[&str] = &[
    "#06b6d4", "#10b981", "#a855f7", "#f59e0b", "#ef4444", 
    "#3b82f6", "#ec4899", "#14b8a6", "#8b5cf6", "#f97316", 
    "#6366f1", "#84cc16", "#06b6d4"
];
