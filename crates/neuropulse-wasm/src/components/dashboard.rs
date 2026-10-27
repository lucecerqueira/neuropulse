use leptos::*;
use crate::types::{TrialData, SessionRecord};
use crate::components::chart::ReactionChart;
use crate::components::analytics_suite::AnalyticsSuite;

#[component]
pub fn DashboardView(
    #[prop(into)] trials: MaybeSignal<Vec<TrialData>>,
    #[prop(into)] sessions: MaybeSignal<Vec<SessionRecord>>,
) -> impl IntoView {
    let (selected_metric, set_selected_metric) = create_signal("SRT".to_string());

    let trials_memo = trials.clone();
    let filtered_trials = create_memo(move |_| {
        let current_filter = selected_metric.get();
        trials_memo.get().into_iter().filter(|t| t.test_type == current_filter).collect::<Vec<_>>()
    });

    let best_time_ms = create_memo(move |_| {
        filtered_trials.get()
            .iter()
            .map(|t| t.reaction_time_ms)
            .fold(f64::INFINITY, f64::min)
    });

    let avg_time_ms = create_memo(move |_| {
        let list = filtered_trials.get();
        if list.is_empty() {
            0.0
        } else {
            let sum: f64 = list.iter().map(|t| t.reaction_time_ms).sum();
            sum / list.len() as f64
        }
    });

    let total_tests = create_memo(move |_| filtered_trials.get().len());

    view! {
        <div class="flex flex-col gap-6 p-6 max-w-7xl mx-auto font-sans">
            // Barra superior e títulos com fontes maiores e de alto destaque
            <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 border-b border-slate-800 pb-4">
                <div>
                    <h1 class="text-3xl font-black text-white tracking-tight">"Dashboard de Treino Cognitivo"</h1>
                    <p class="text-sm text-slate-300 mt-1 font-medium">"Análise histórica consolidada de velocidade de reação e consistência cortical."</p>
                </div>

                <div class="flex items-center gap-3">
                    <span class="text-sm text-slate-300 font-bold">"Filtrar Teste:"</span>
                    <select
                        on:change=move |ev| {
                            set_selected_metric.set(event_target_value(&ev));
                        }
                        class="bg-[#1e293b] border border-slate-700 text-cyan-400 text-sm font-bold px-4 py-2 rounded-xl focus:outline-none focus:border-cyan-500 cursor-pointer shadow-md"
                    >
                        <option value="SRT" selected=move || selected_metric.get() == "SRT">"SRT (Reação Simples)"</option>
                        <option value="CRT" selected=move || selected_metric.get() == "CRT">"CRT (Escolha Multidirecional)"</option>
                        <option value="GoNoGo" selected=move || selected_metric.get() == "GoNoGo">"Go/No-Go (Controle Inibitório)"</option>
                    </select>
                </div>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                <div class="bg-[#0f172a]/80 border border-slate-800 p-5 rounded-xl flex flex-col gap-1">
                    <span class="text-sm text-slate-300 font-bold">"Menor Tempo (PB)"</span>
                    <span class="text-3xl font-black text-emerald-400">
                        {move || {
                            let val = best_time_ms.get();
                            if val.is_infinite() { "0.0 ms".to_string() } else { format!("{:.1} ms", val) }
                        }}
                    </span>
                </div>

                <div class="bg-[#0f172a]/80 border border-slate-800 p-5 rounded-xl flex flex-col gap-1">
                    <span class="text-sm text-slate-300 font-bold">"Média da Métrica"</span>
                    <span class="text-3xl font-black text-amber-400">
                        {move || format!("{:.1} ms", avg_time_ms.get())}
                    </span>
                </div>

                <div class="bg-[#0f172a]/80 border border-slate-800 p-5 rounded-xl flex flex-col gap-1">
                    <span class="text-sm text-slate-300 font-bold">"Total de Ensaios Registrados"</span>
                    <span class="text-3xl font-black text-cyan-400">
                        {move || total_tests.get()}
                    </span>
                </div>
            </div>

            // Gráfico 1: Trajetórias Principais por Trilha
            <div class="bg-[#0f172a]/60 border border-slate-800/80 rounded-xl p-6">
                <ReactionChart
                    trials=Signal::derive(move || filtered_trials.get())
                    sessions=sessions
                    test_filter=selected_metric.get()
                />
            </div>

            // Suíte Estatística: Barras Comparativas e Proporção em Donut/Pizza
            <AnalyticsSuite
                trials=trials
                test_filter=selected_metric.get()
            />
        </div>
    }
}
