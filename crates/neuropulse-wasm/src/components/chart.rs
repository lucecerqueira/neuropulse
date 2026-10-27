use leptos::*;
use crate::types::{TrialData, SessionRecord, MOTIVATIONAL_TRAILS};
use std::cell::RefCell;

thread_local! {
    static EXCLUDED_SESSIONS_CACHE: RefCell<Vec<u32>> = RefCell::new(Vec::new());
}

#[component]
pub fn ReactionChart(
    #[prop(into)] trials: MaybeSignal<Vec<TrialData>>,
    #[prop(into)] sessions: MaybeSignal<Vec<SessionRecord>>,
    #[prop(into, optional)] test_filter: Option<String>,
) -> impl IntoView {
    let _ = sessions;

    let (_selected_tags, _set_selected_tags) = create_signal::<Vec<String>>(
        MOTIVATIONAL_TRAILS.iter().map(|t| t.name.to_string()).collect()
    );

    let initial_excluded = EXCLUDED_SESSIONS_CACHE.with(|c| c.borrow().clone());
    let (excluded_sessions, set_excluded_sessions) = create_signal::<Vec<u32>>(initial_excluded);

    let trials_clone = trials.clone();
    let filter_clone = test_filter.clone();

    let filtered_trials = create_memo(move |_| {
        let all = trials_clone.get();
        let tf = filter_clone.clone();
        let excluded = excluded_sessions.get();

        all.into_iter().filter(|t| {
            let pass_test = match &tf {
                Some(filter) => {
                    let f_lower = filter.to_lowercase();
                    let t_lower = t.test_type.to_lowercase();
                    t_lower.contains(&f_lower) || f_lower.contains(&t_lower)
                },
                None => true,
            };

            let not_excluded = !excluded.contains(&t.trail_number);
            pass_test && not_excluded
        }).collect::<Vec<_>>()
    });

    view! {
        <div class="space-y-4 font-mono">
            <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 border-b border-slate-800 pb-4">
                <div>
                    <h3 class="text-xs font-bold text-amber-400 uppercase tracking-wider">
                        "TRAJETÓRIAS DE ESCOLHA SOBREPOSTAS POR TRILHA"
                    </h3>
                    <p class="text-[11px] text-cyan-400 font-semibold mt-0.5 uppercase tracking-wide">
                        "Dica: Clique em cima de qualquer curva para removê-la."
                    </p>
                </div>

                <div class="flex items-center gap-4 text-xs">
                    {move || {
                        let excluded = excluded_sessions.get();
                        if !excluded.is_empty() {
                            view! {
                                <button
                                    on:click=move |_| {
                                        set_excluded_sessions.set(vec![]);
                                        EXCLUDED_SESSIONS_CACHE.with(|c| c.borrow_mut().clear());
                                    }
                                    class="px-2.5 py-1 bg-rose-500/20 border border-rose-500/40 text-rose-300 rounded-lg text-[11px] font-bold hover:bg-rose-500/30 transition cursor-pointer"
                                >
                                    "Restaurar Curvas Excluídas"
                                </button>
                            }.into_view()
                        } else {
                            view! { <div></div> }.into_view()
                        }
                    }}

                    <label class="flex items-center gap-1.5 cursor-pointer text-slate-300 text-[11px]">
                        <input type="checkbox" checked=true class="accent-cyan-500 rounded" />
                        "Vetor Médio"
                    </label>
                    <label class="flex items-center gap-1.5 cursor-pointer text-slate-300 text-[11px]">
                        <input type="checkbox" checked=true class="accent-cyan-500 rounded" />
                        "Banda (±1σ)"
                    </label>
                </div>
            </div>

            <div class="bg-[#111622] border border-slate-800/80 rounded-xl p-4 min-h-[300px] flex flex-col justify-center relative overflow-hidden">
                {move || {
                    let list = filtered_trials.get();
                    if list.is_empty() {
                        view! {
                            <div class="text-center py-12 text-slate-500 text-xs font-mono">
                                "Nenhuma curva visível no momento. Restaure as curvas ou realize novos testes."
                            </div>
                        }.into_view()
                    } else {
                        let mut series_map: std::collections::HashMap<u32, Vec<&TrialData>> = std::collections::HashMap::new();
                        for t in &list {
                            series_map.entry(t.trail_number).or_default().push(t);
                        }

                        let colors = vec!["#06b6d4", "#10b981", "#38bdf8", "#a855f7", "#f59e0b", "#ec4899"];
                        let mut series_rendered = Vec::new();

                        for (_color_idx, (b_id, s_trials)) in series_map.iter().enumerate() {
                            let line_color = colors[(*b_id - 1) as usize % colors.len()];
                            let current_b_id = *b_id;
                            
                            let points: Vec<(f64, f64)> = s_trials.iter().enumerate().map(|(idx, t)| {
                                let x = (idx as f64) * 130.0 + 60.0;
                                let clamped_time = t.reaction_time_ms.clamp(100.0, 1500.0);
                                let y = 220.0 - (clamped_time / 1500.0 * 170.0);
                                (x, y)
                            }).collect();

                            let mut path_d = String::new();
                            for (i, p) in points.iter().enumerate() {
                                if i == 0 {
                                    path_d.push_str(&format!("M {} {}", p.0, p.1));
                                } else {
                                    let prev = points[i - 1];
                                    let cx1 = prev.0 + 45.0;
                                    let cy1 = prev.1;
                                    let cx2 = p.0 - 45.0;
                                    let cy2 = p.1;
                                    path_d.push_str(&format!(" C {} {}, {} {}, {} {}", cx1, cy1, cx2, cy2, p.0, p.1));
                                }
                            }

                            // Renderiza a linha e apenas os pontos discretos (sem caixas de texto poluindo)
                            series_rendered.push(view! {
                                <g
                                    on:click=move |_| {
                                        set_excluded_sessions.update(|ex| {
                                            if !ex.contains(&current_b_id) {
                                                ex.push(current_b_id);
                                            }
                                        });
                                        EXCLUDED_SESSIONS_CACHE.with(|c| {
                                            let mut cache = c.borrow_mut();
                                            if !cache.contains(&current_b_id) {
                                                cache.push(current_b_id);
                                            }
                                        });
                                    }
                                    class="cursor-pointer group/curve transition-opacity hover:opacity-75"
                                    title=format!("Clique para excluir a Sessão #{}", current_b_id)
                                >
                                    <path
                                        d=path_d
                                        fill="none"
                                        stroke=line_color
                                        stroke-width="3.5"
                                        class="transition-all duration-300"
                                    />
                                    {points.into_iter().map(|(x, y)| {
                                        view! {
                                            <circle cx=x cy=y r="4.5" fill=line_color class="transition-all hover:r-7" />
                                        }
                                    }).collect_view()}
                                </g>
                            });
                        }

                        view! {
                            <div class="w-full overflow-x-auto">
                                <svg class="w-full h-64 min-w-[600px]" viewBox="0 0 650 240">
                                    <line x1="40" y1="35" x2="610" y2="35" stroke="#1e293b" stroke-dasharray="3" />
                                    <text x="10" y="38" fill="#64748b" font-size="9">"1500ms"</text>

                                    <line x1="40" y1="85" x2="610" y2="85" stroke="#1e293b" stroke-dasharray="3" />
                                    <text x="10" y="88" fill="#64748b" font-size="9">"1000ms"</text>

                                    <line x1="40" y1="135" x2="610" y2="135" stroke="#1e293b" stroke-dasharray="3" />
                                    <text x="10" y="138" fill="#64748b" font-size="9">"500ms"</text>

                                    <line x1="40" y1="205" x2="610" y2="205" stroke="#334155" stroke-width="1.5" />

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
