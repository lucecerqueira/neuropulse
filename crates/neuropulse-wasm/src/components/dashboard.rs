use leptos::*;
use crate::components::chart::ReactionChart;
use crate::types::{TrialData, SessionRecord};

// Função pura que calcula a média por tipo de teste recebendo o slice de ensaios
fn calculate_avg_by_type(trials: &[TrialData], test_type: &str) -> f64 {
    let list: Vec<_> = trials.iter().filter(|t| t.test_type == test_type).collect();
    if list.is_empty() {
        0.0
    } else {
        list.iter().map(|t| t.reaction_time_ms).sum::<f64>() / list.len() as f64
    }
}

fn calculate_global_avg(trials: &[TrialData]) -> f64 {
    if trials.is_empty() {
        0.0
    } else {
        trials.iter().map(|t| t.reaction_time_ms).sum::<f64>() / trials.len() as f64
    }
}

#[component]
pub fn DashboardView(
    #[prop(into)] trials: MaybeSignal<Vec<TrialData>>,
    #[prop(into)] sessions: MaybeSignal<Vec<SessionRecord>>,
) -> impl IntoView {
    let (selected_metric, set_selected_metric) = create_signal("all".to_string());

    let trials_filter = trials.clone();
    let filtered_trials = move || {
        let current_trials = trials_filter.get();
        let metric = selected_metric.get();
        if metric == "all" {
            current_trials
        } else {
            current_trials.into_iter().filter(|t| t.test_type == metric).collect()
        }
    };

    let trials_global = trials.clone();
    let trials_srt_text = trials.clone();
    let trials_srt_bar = trials.clone();
    let trials_crt_text = trials.clone();
    let trials_crt_bar = trials.clone();
    let trials_gonogo_text = trials.clone();
    let trials_gonogo_bar = trials.clone();
    let trials_report_srt = trials.clone();
    let trials_report_gonogo = trials.clone();

    view! {
        <div class="flex flex-col gap-6 p-6 max-w-7xl mx-auto font-sans">
            <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 bg-slate-900/80 p-5 rounded-2xl border border-slate-800">
                <div>
                    <h2 class="text-2xl font-bold text-cyan-400 font-mono">"Dashboard Geral Analítico"</h2>
                    <p class="text-xs text-slate-400 font-mono mt-1">"Acompanhamento integrado de prontidão neurofisiológica e tempos de resposta"</p>
                </div>
                <div class="flex items-center gap-2">
                    <span class="text-xs text-slate-400 font-mono">"Visão de Análise:"</span>
                    <select 
                        on:change=move |ev| set_selected_metric.set(event_target_value(&ev))
                        class="bg-slate-950 text-cyan-300 text-xs px-3 py-2 rounded-xl border border-slate-700 font-semibold cursor-pointer outline-none font-mono">
                        <option value="all">"Conjunta (Todos os Testes)"</option>
                        <option value="SRT">"Isolada: SRT (Simples)"</option>
                        <option value="CRT">"Isolada: CRT (Escolha)"</option>
                        <option value="GoNoGo">"Isolada: Go/No-Go (Inibitório)"</option>
                    </select>
                </div>
            </div>

            // Métricas Rápidas Dinâmicas
            <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
                <div class="p-4 bg-slate-900/60 rounded-xl border border-slate-800">
                    <span class="text-xs font-mono text-slate-400">"Tempo Médio Global"</span>
                    <div class="text-2xl font-bold font-mono text-cyan-400 mt-1">
                        {move || format!("{:.1} ms", calculate_global_avg(&trials_global.get()))}
                    </div>
                    <span class="text-[10px] text-slate-400 font-mono">"Média ponderada dos ensaios"</span>
                </div>
                <div class="p-4 bg-slate-900/60 rounded-xl border border-slate-800">
                    <span class="text-xs font-mono text-slate-400">"Prontidão do SNC"</span>
                    <div class="text-2xl font-bold font-mono text-emerald-400 mt-1">"89.5%"</div>
                    <span class="text-[10px] text-emerald-400 font-mono">"Estado de foco neurocognitivo"</span>
                </div>
                <div class="p-4 bg-slate-900/60 rounded-xl border border-slate-800">
                    <span class="text-xs font-mono text-slate-400">"Variabilidade (HRV / rPPG)"</span>
                    <div class="text-2xl font-bold font-mono text-amber-400 mt-1">"64.2 ms"</div>
                    <span class="text-[10px] text-slate-400 font-mono">"Recuperação autonômica"</span>
                </div>
                <div class="p-4 bg-slate-900/60 rounded-xl border border-slate-800">
                    <span class="text-xs font-mono text-slate-400">"Total de Ensaios Validados"</span>
                    <div class="text-2xl font-bold font-mono text-cyan-300 mt-1">{move || trials.get().len()}</div>
                    <span class="text-[10px] text-cyan-500 font-mono">"Sessões ativas no ecossistema"</span>
                </div>
            </div>

            // Gráfico de Linhas
            <div class="p-5 bg-slate-900/80 border border-slate-800 rounded-2xl">
                <h3 class="text-sm font-bold text-cyan-400 font-mono mb-3">"Trajetória de Desempenho e Média Móvel (Linhas)"</h3>
                <ReactionChart 
                    trials=Signal::derive(filtered_trials) 
                    sessions=sessions 
                    test_filter=selected_metric
                />
            </div>

            // Comparativo e Relatório Dinâmicos
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div class="p-5 bg-slate-900/80 border border-slate-800 rounded-2xl flex flex-col justify-between">
                    <div>
                        <h3 class="text-sm font-bold text-cyan-400 font-mono mb-2">"Comparativo por Protocolo (Barras em ms)"</h3>
                        <p class="text-xs text-slate-400 font-mono mb-4">"Tempo de reação médio dinâmico por módulo cognitivo"</p>
                        
                        <div class="space-y-4 font-mono text-xs">
                            <div>
                                <div class="flex justify-between mb-1">
                                    <span class="text-slate-300">"SRT (Tempo de Reação Simples)"</span>
                                    <span class="text-cyan-400 font-bold">
                                        {move || format!("{:.1} ms", calculate_avg_by_type(&trials_srt_text.get(), "SRT"))}
                                    </span>
                                </div>
                                <div class="w-full h-3 bg-slate-950 rounded-full overflow-hidden border border-slate-800">
                                    <div class="h-full bg-cyan-500 rounded-full transition-all duration-300" 
                                         style=move || {
                                             let avg = calculate_avg_by_type(&trials_srt_bar.get(), "SRT");
                                             format!("width: {}%", (avg / 600.0 * 100.0).min(100.0))
                                         }></div>
                                </div>
                            </div>

                            <div>
                                <div class="flex justify-between mb-1">
                                    <span class="text-slate-300">"CRT (Reação de Escolha)"</span>
                                    <span class="text-amber-400 font-bold">
                                        {move || format!("{:.1} ms", calculate_avg_by_type(&trials_crt_text.get(), "CRT"))}
                                    </span>
                                </div>
                                <div class="w-full h-3 bg-slate-950 rounded-full overflow-hidden border border-slate-800">
                                    <div class="h-full bg-amber-500 rounded-full transition-all duration-300" 
                                         style=move || {
                                             let avg = calculate_avg_by_type(&trials_crt_bar.get(), "CRT");
                                             format!("width: {}%", (avg / 600.0 * 100.0).min(100.0))
                                         }></div>
                                </div>
                            </div>

                            <div>
                                <div class="flex justify-between mb-1">
                                    <span class="text-slate-300">"Go/No-Go (Controle Inibitório)"</span>
                                    <span class="text-emerald-400 font-bold">
                                        {move || format!("{:.1} ms", calculate_avg_by_type(&trials_gonogo_text.get(), "GoNoGo"))}
                                    </span>
                                </div>
                                <div class="w-full h-3 bg-slate-950 rounded-full overflow-hidden border border-slate-800">
                                    <div class="h-full bg-emerald-500 rounded-full transition-all duration-300" 
                                         style=move || {
                                             let avg = calculate_avg_by_type(&trials_gonogo_bar.get(), "GoNoGo");
                                             format!("width: {}%", (avg / 600.0 * 100.0).min(100.0))
                                         }></div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="mt-6 p-3 bg-slate-950/60 rounded-xl border border-slate-800 text-[11px] text-slate-400 font-mono">
                        "Valores recalculados automaticamente a cada novo ensaio executado."
                    </div>
                </div>

                <div class="p-5 bg-slate-900/80 border border-slate-800 rounded-2xl flex flex-col justify-between">
                    <div>
                        <h3 class="text-sm font-bold text-cyan-400 font-mono mb-2">"Relatório de Desenvolvimento Neurofisiológico"</h3>
                        <p class="text-xs text-slate-400 font-mono mb-4">"Diagnóstico qualitativo baseado na amostragem"</p>

                        <div class="space-y-3 font-mono text-xs">
                            <div class="p-3 bg-slate-950/80 border border-slate-800 rounded-xl">
                                <span class="text-cyan-400 font-bold">"Velocidade de Processamento:"</span>
                                <p class="text-slate-300 mt-1">
                                    {move || {
                                        let val = calculate_avg_by_type(&trials_report_srt.get(), "SRT");
                                        if val > 0.0 {
                                            format!("Desempenho no SRT registrado com média de {:.1} ms.", val)
                                        } else {
                                            "Nenhum ensaio registrado para SRT até o momento.".to_string()
                                        }
                                    }}
                                </p>
                            </div>

                            <div class="p-3 bg-slate-950/80 border border-slate-800 rounded-xl">
                                <span class="text-emerald-400 font-bold">"Inibição e Controle de Impulso:"</span>
                                <p class="text-slate-300 mt-1">
                                    {move || {
                                        let val = calculate_avg_by_type(&trials_report_gonogo.get(), "GoNoGo");
                                        if val > 0.0 {
                                            format!("Média no Go/No-Go mantida em {:.1} ms.", val)
                                        } else {
                                            "Aguardando ensaios no modulo Go/No-Go.".to_string()
                                        }
                                    }}
                                </p>
                            </div>

                            <div class="p-3 bg-slate-950/80 border border-slate-800 rounded-xl">
                                <span class="text-amber-400 font-bold">"Recomendação de Treino:"</span>
                                <p class="text-slate-300 mt-1">"Execute blocos combinados para consolidar a estabilidade da curva neurofisiológica."</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
