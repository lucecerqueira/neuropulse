use serde::{Deserialize, Serialize};

/// Enum com todos os testes cognitivos e biomarcadores da plataforma
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentType {
    Stroop,
    GoNoGo,
    Pasat,
    ReversalLearning,
    Microsaccades,
    TrackerProjections,
}

impl AssessmentType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Stroop => "Stroop Test",
            Self::GoNoGo => "Go/No-Go Test",
            Self::Pasat => "PASAT",
            Self::ReversalLearning => "Reversal Learning",
            Self::Microsaccades => "Análise de Microssacadas",
            Self::TrackerProjections => "Projeções de Rastreamento Ocular",
        }
    }

    pub fn endpoint_slug(&self) -> &'static str {
        match self {
            Self::Stroop => "stroop",
            Self::GoNoGo => "gonogo",
            Self::Pasat => "pasat",
            Self::ReversalLearning => "reversal",
            Self::Microsaccades => "microsaccades",
            Self::TrackerProjections => "tracker/projections",
        }
    }

    pub fn measures(&self) -> Vec<&'static str> {
        match self {
            Self::Stroop => vec!["controle_inibitorio", "flexibilidade_cognitiva", "tempo_de_reacao"],
            Self::GoNoGo => vec!["impulsividade", "atencao_sustentada", "inibicao_de_resposta"],
            Self::Pasat => vec!["velocidade_de_processamento", "memoria_de_trabalho", "atencao_dividida"],
            Self::ReversalLearning => vec!["flexibilidade_comportamental", "adaptacao_a_regras"],
            Self::Microsaccades => vec!["estabilidade_de_fixacao", "main_sequence_ratio", "entropia_pne"],
            Self::TrackerProjections => vec!["coordenadas_gaze_2d_3d", "distribuicao_espacial"],
        }
    }
}

/// Enum para mapear todas as rotas e seções navegáveis (locais) do sistema
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformLocation {
    Dashboard,
    Patients,
    Assessment(AssessmentType),
    Report,
}

impl PlatformLocation {
    pub fn route_path(&self) -> String {
        match self {
            Self::Dashboard => "/".to_string(),
            Self::Patients => "/api/patients/".to_string(),
            Self::Assessment(test) => format!("/{}/", test.endpoint_slug()),
            Self::Report => "/report/".to_string(),
        }
    }
}

/// Estrutura de metadados exposta via API ou documentação interna
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentMetadata {
    pub assessment_type: AssessmentType,
    pub name: String,
    pub route_path: String,
    pub measures: Vec<String>,
}

impl From<AssessmentType> for AssessmentMetadata {
    fn from(test: AssessmentType) -> Self {
        Self {
            assessment_type: test,
            name: test.name().to_string(),
            route_path: PlatformLocation::Assessment(test).route_path(),
            measures: test.measures().into_iter().map(String::from).collect(),
        }
    }
}
