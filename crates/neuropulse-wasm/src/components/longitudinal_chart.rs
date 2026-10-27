use leptos::*;
use crate::types::TrialData;

#[component]
pub fn LongitudinalChart(
    #[prop(into)] trials: MaybeSignal<Vec<TrialData>>,
    #[prop(into, optional)] test_filter: Option<String>,
) -> impl IntoView {
    let (selected_blocks, set_selected_blocks) = create_signal::<Vec<u32>>(vec![]);

    // Clones das variáveis para a closure available_blocks
    let trials_for_blocks = trials.clone();
    let filter_for_blocks = test_filter.clone();

    // Agrupa todos os ensaios por bloco/sessão
    let available_blocks = create_memo(move |_| {
        let all = trials_for_blocks.get();
        let tf = filter_for_blocks.clone();
        
        let filtered: Vec<_> = all.into_iter().filter(|t| {
            match &tf {
                Some(filter) => &t.test_type == filter,
                None => true,
            }
        }).collect();

        let mut blocks: Vec<u32> = filtered.iter().map(|t| t.trail_number).collect();
        blocks.sort();
        blocks.dedup();
        blocks
    });

    let toggle_block = move |block_id: u32| {
        set_selected_blocks.update(|blocks| {
            if blocks.contains(&block_id) {
                blocks.retain(|&b| b != block_id);
            } else {
                blocks.push(block_id);
            }
        });
    };

    // Clones das variáveis para a closure filtered_trials
    let trials_for_filtered = trials.clone();
    let filter_for_filtered = test_filter.clone();

    let filtered_trials = create_memo(move |_| {
        let all = trials_for_filtered.get();
        let tf = filter_for_filtered.clone();
        let active_blocks = selected_blocks.get();

        all.into_iter().filter(|t| {
            let pass_test = match &tf {
                Some(filter) => &t.test_type == filter,
                None => true,
            };
            let pass_block = active_blocks.is_empty() || active_blocks.contains(&t.trail_number);
            pass_test && pass_block
        }).collect::<Vec<_>>()
    });

    view! {
        <div class="space-y-4 font-mono bg-[#090d16] border border-slate-800/80 rounded-xl p-6">
            <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 border-b border-slate-800/80 pb-4">
                <div>
                    <h3 class="text-xs font-bold text-cyan-400 uppercase tracking-wider">
                        "Comparativo de Desempenho por Dia e Sessão"
                    </h3>
                    <p class="text-[11px] text-slate-400 mt-0.5">
                        "Selecione sessões/blocos para comparar as curvas de tempo de reação sobrepostas:"
                    </p>
                </div>
            </div>

            // Filtro de Blocos/Dias
            <div class="flex flex-wrap gap-2 py-2">
                {move || {
                    let blocks = available_blocks.get();
                    let active = selected_blocks.get();

                    if blocks.is_empty() {
                        view! { <span class="text-xs text-slate-500">"Sem históricos de blocos registrados."</span> }.into_view()
                    } else {
                        blocks.into_iter().map(|b_id| {
                            let is_sel = active.contains(&b_id) || active.is_empty();
                            view! {
                                <button
                                    on:click=move |_| toggle_block(b_id)
                                    class=format!(
                                        "px-3 py-1 rounded-lg text-xs font-bold border transition cursor-pointer {}",
                                        if is_sel {
                                            "bg-cyan-500/20 border-cyan-500 text-cyan-300"
                                        } else {
                                            "bg-slate-900 border-slate-800 text-slate-500 hover:text-slate-300"
                                        }
                                    )
                                >
                                    {format!("Sessão / Bloco #{}", b_id)}
                                </button>
                            }
                        }).collect_view()
                    }
                }}
            </div>

            // Renderização do Gráfico Secundário
            <div class="bg-[#060911] border border-slate-800/80 rounded-xl p-4 min-h-[240px] flex flex-col justify-center relative">
                {move || {
                    let list = filtered_trials.get();
                    if list.is_empty() {
                        view! {
                            <div class="text-center py-12 text-slate-500 text-xs">
                                "Nenhum dado selecionado para o gráfico comparativo."
                            </div>
                        }.into_view()
                    } else {
                        let mut series_map: std::collections::HashMap<u32, Vec<&TrialData>> = std::collections::HashMap::new();
                        for t in &list {
                            series_map.entry(t.trail_number).or_default().push(t);
                        }

                        let colors = vec!["#38bdf8", "#34d399", "#facc15", "#f43f5e", "#a855f7", "#6366f1"];
                        let mut series_rendered = Vec::new();

                        for (idx, (_b_id, s_trials)) in series_map.iter().enumerate() {
                            let line_color = colors[idx % colors.len()];

                            let points: Vec<(f64, f64)> = s_trials.iter().map(|t| {
                                let x = (t.trial_number as f64 - 1.0) * 110.0 + 40.0;
                                let y = 200.0 - (t.reaction_time_ms / 1000.0 * 100.0).min(160.0);
                                (x, y)
                            }).collect();

                            let path_d = points.iter().enumerate().fold(String::new(), |acc, (i, (x, y))| {
                                if i == 0 {
                                    format!("M {} {}", x, y)
                                } else {
                                    format!("{} L {} {}", acc, x, y)
                                }
                            });

                            series_rendered.push(view! {
                                <g>
                                    <path
                                        d=path_d
                                        fill="none"
                                        stroke=line_color
                                        stroke-width="2.5"
                                    />
                                    {points.into_iter().zip(s_trials.iter()).map(|((x, y), trial)| {
                                        let val_text = format!("{:.0}ms", trial.reaction_time_ms);
                                        view! {
                                            <g>
                                                <circle cx=x cy=y r="4" fill=line_color />
                                                <text x=x y=y - 8.0 text-anchor="middle" fill="#94a3b8" font-size="9" font-weight="bold">
                                                    {val_text}
                                                </text>
                                            </g>
                                        }
                                    }).collect_view()}
                                </g>
                            });
                        }

                        view! {
                            <div class="w-full overflow-x-auto">
                                <svg class="w-full h-56 min-w-[500px]" viewBox="0 0 600 220">
                                    <line x1="30" y1="40" x2="570" y2="40" stroke="#1e293b" stroke-dasharray="4" />
                                    <line x1="30" y1="100" x2="570" y2="100" stroke="#1e293b" stroke-dasharray="4" />
                                    <line x1="30" y1="160" x2="570" y2="160" stroke="#1e293b" stroke-dasharray="4" />
                                    <line x1="30" y1="200" x2="570" y2="200" stroke="#334155" />

                                    {series_rendered}
                                </svg>
                            </div>
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}
