use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TrialData {
    pub trial_number: u32,
    pub trail_number: u32,
    pub reaction_time_ms: f64,
    pub is_correct: bool,
    pub test_type: String,      // SRT, CRT, GoNoGo
    pub stimulus_type: String,  // Visual, Auditivo, Choque, etc.
    pub timestamp: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionRecord {
    pub id: String,
    pub user_id: String,
    pub test_type: String,
    pub timestamp: String,
    pub avg_reaction_time_ms: f64,
    pub accuracy: f64,
    pub total_trials: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TestMode {
    ByRepetitions(u32), // Ex: 10, 15 ou 20 repetições
    ByTime(u64),        // Ex: 30s, 60s, 180s (3 min)
}

#[derive(Clone, Debug)]
pub struct MotivationalTrail {
    pub index: usize,
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub color: &'static str,
}

// 13 TRILHAS MOTIVACIONAIS E COGNITIVAS
pub const MOTIVATIONAL_TRAILS: &[MotivationalTrail] = &[
    MotivationalTrail { index: 0, id: "cortical_focus", name: "Foco Cortical Sustentado", description: "Alerta sustentado e vigilância pré-motora.", color: "#06b6d4" },
    MotivationalTrail { index: 1, id: "processing_speed", name: "Velocidade de Processamento", description: "Otimização da taxa de condução axonial.", color: "#a855f7" },
    MotivationalTrail { index: 2, id: "inhibitory_control", name: "Controle Inibitório", description: "Supressão de impulsos motores reflexos.", color: "#10b981" },
    MotivationalTrail { index: 3, id: "choice_decision", name: "Tomada de Decisão", description: "Discriminação visual em escolha simples e complexa.", color: "#f59e0b" },
    MotivationalTrail { index: 4, id: "cognitive_flexibility", name: "Flexibilidade Cognitiva", description: "Alternância de conjuntos mentais e adaptação.", color: "#ec4899" },
    MotivationalTrail { index: 5, id: "working_memory", name: "Memória de Trabalho Neural", description: "Retenção temporária e manipulação de estimulo.", color: "#3b82f6" },
    MotivationalTrail { index: 6, id: "stress_resistance", name: "Resistência ao Estresse", description: "Manutenção do desempenho sob pressão cronológica.", color: "#ef4444" },
    MotivationalTrail { index: 7, id: "motor_precision", name: "Precisão Neuro-Motora", description: "Controle fino de coordenação e tempo de disparo.", color: "#84cc16" },
    MotivationalTrail { index: 8, id: "central_fatigue", name: "Resistência à Fadiga Central", description: "Avaliador de depleção de neurotransmissores.", color: "#6366f1" },
    MotivationalTrail { index: 9, id: "autonomic_recovery", name: "Recuperação Autonômica", description: "Acompanhamento HRV / rPPG pós-estímulo.", color: "#14b8a6" },
    MotivationalTrail { index: 10, id: "visual_search", name: "Busca e Varredura Visual", description: "Mecanismo de atenção top-down espacial.", color: "#d97706" },
    MotivationalTrail { index: 11, id: "dual_task", name: "Dupla Tarefa Cognitiva", description: "Divisão de recursos de atenção executiva.", color: "#8b5cf6" },
    MotivationalTrail { index: 12, id: "peak_readiness", name: "Prontidão de Pico do SNC", description: "Aferição de estado ideal para competição/foco.", color: "#22c55e" },
];
