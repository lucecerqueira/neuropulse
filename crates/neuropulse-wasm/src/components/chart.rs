use leptos::*;
use crate::types::{TrialData, SessionRecord, MOTIVATIONAL_TRAILS};

#[component]
pub fn ReactionChart(
    #[prop(into)] trials: MaybeSignal<Vec<TrialData>>,
    #[prop(into)] sessions: MaybeSignal<Vec<SessionRecord>>,
    #[prop(into, optional)] test_filter: Option<MaybeSignal<String>>,
) -> impl IntoView {
    let (active_tags, set_active_tags) = create_signal(vec![
        "Foco Cortical Sustentado".to_string(),
        "Velocidade de Processamento".to_string(),
        "Controle Inibitório".to_string(),
        "Tomada de Decisão".to_string(),
    ]);

    let (show_mean_vector, set_show_mean_vector) = create_signal(true);
    let (show_band, set_show_band) = create_signal(true);

    let filtered_trials = create_memo(move |_| {
        let all_trials = trials.get();
        if let Some(ref filter_sig) = test_filter {
            let filter = filter_sig.get();
            if filter.is_empty() || filter == "ALL" {
                all_trials
            } else {
                all_trials.into_iter().filter(|t| t.test_type == filter).collect()
            }
        } else {
            all_trials
        }
    });

    view! {
        <div class="space-y-4 font-mono">
            // Cabeçalho da Seção de Trajetórias
            <div class="flex flex-col lg:flex-row justify-between items-start lg:items-center gap-4 border-b border-slate-800/80 pb-3">
                <div>
                    <h3 class="text-xs font-bold text-amber-400 uppercase tracking-wider">
                        "Trajetórias de Escolha Sobrepostas por Trilha"
                    </h3>
                    <span class="text-[10px] text-cyan-400 font-bold uppercase mt-0.5 block">
                        "FILTRO POR TAGS MOTIVACIONAIS:"
                    </span>
                </div>

                <div class="flex flex-wrap items-center gap-4 text-xs">
                    // Dropdown para Adicionar Trilhas
                    <div class="flex items-center gap-2">
                        <span class="text-slate-400">"+ Adicionar Trilha:"</span>
                        <select
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                if !val.is_empty() && val != "SELECT" {
                                    set_active_tags.update(|tags| {
                                        if !tags.contains(&val) {
                                            tags.push(val);
                                        }
                                    });
                                }
                            }
                            class="bg-[#1e293b] border border-slate-700 text-cyan-400 font-bold px-3 py-1 rounded-lg focus:outline-none focus:border-cyan-500 cursor-pointer"
                        >
                            <option value="SELECT">"-- Selecionar Trilha --"</option>
                            {MOTIVATIONAL_TRAILS.iter().map(|trail| {
                                let label = trail.name.to_string();
                                view! {
                                    <option value=label.clone()>{label}</option>
                                }
                            }).collect_view()}
                        </select>
                    </div>

                    // Opções de Exibição
                    <label class="flex items-center gap-1.5 text-slate-300 cursor-pointer">
                        <input
                            type="checkbox"
                            prop:checked=show_mean_vector
                            on:change=move |ev| set_show_mean_vector.set(event_target_checked(&ev))
                            class="rounded border-slate-700 text-cyan-500 focus:ring-0 bg-[#1e293b]"
                        />
                        "Vetor Médio"
                    </label>
                    <label class="flex items-center gap-1.5 text-slate-300 cursor-pointer">
                        <input
                            type="checkbox"
                            prop:checked=show_band
                            on:change=move |ev| set_show_band.set(event_target_checked(&ev))
                            class="rounded border-slate-700 text-cyan-500 focus:ring-0 bg-[#1e293b]"
                        />
                        "Banda (±1σ)"
                    </label>
                </div>
            </div>

            // Tags Ativas
            <div class="flex flex-wrap items-center gap-2 py-1 min-h-[36px]">
                {move || {
                    let tags = active_tags.get();
                    if tags.is_empty() {
                        view! {
                            <span class="text-xs text-slate-500 italic">
                                "Nenhuma trilha selecionada. Use o menu dropdown acima para escolher as curvas."
                            </span>
                        }.into_view()
                    } else {
                        tags.into_iter().map(|tag_name| {
                            let name_for_remove = tag_name.clone();
                            let dot_color = match tag_name.as_str() {
                                "Foco Cortical Sustentado" => "bg-cyan-400",
                                "Velocidade de Processamento" => "bg-purple-400",
                                "Controle Inibitório" => "bg-emerald-400",
                                "Tomada de Decisão" => "bg-amber-400",
                                _ => "bg-cyan-400",
                            };

                            view! {
                                <div class="px-3 py-1 bg-[#1e293b] border border-cyan-500/60 text-slate-200 text-xs font-bold rounded-lg flex items-center gap-2 shadow-sm">
                                    <span class=format!("w-2 h-2 rounded-full {}", dot_color)></span>
                                    <span>{tag_name.clone()}</span>
                                    <button
                                        on:click=move |_| {
                                            let target = name_for_remove.clone();
                                            set_active_tags.update(|t_list| t_list.retain(|t| t != &target));
                                        }
                                        class="text-slate-400 hover:text-rose-400 font-bold ml-1 cursor-pointer"
                                    >
                                        "×"
                                    </button>
                                </div>
                            }
                        }).collect_view()
                    }
                }}
            </div>

            // Canvas SVG para Renderização Dinâmica (Auto-scaling no Eixo Y)
            <div class="w-full h-64 bg-[#090d16] border border-slate-800/80 rounded-xl relative p-4 flex items-center justify-center overflow-hidden">
                {move || {
                    let current_trials = filtered_trials.get();
                    let active_list = active_tags.get();

                    if current_trials.is_empty() || active_list.is_empty() {
                        view! {
                            <div class="text-xs text-slate-500">
                                "Aguardando ensaios para renderizar curvas de treino sobrepostas..."
                            </div>
                        }.into_view()
                    } else {
                        // Coleta todos os tempos de reação dos ensaios filtrados e ativos
                        let active_trial_times: Vec<f64> = current_trials.iter()
                            .filter(|t| active_list.contains(&t.stimulus_type) || active_list.contains(&t.test_type))
                            .map(|t| t.reaction_time_ms)
                            .collect();

                        if active_trial_times.is_empty() {
                            return view! {
                                <div class="text-xs text-slate-500">
                                    "Aguardando ensaios da trilha selecionada..."
                                </div>
                            }.into_view();
                        }

                        // Cálculo do valor Mínimo e Máximo Real para Auto-Scaling no Eixo Y
                        let raw_min = active_trial_times.iter().cloned().fold(f64::INFINITY, f64::min);
                        let raw_max = active_trial_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                        
                        let delta = if (raw_max - raw_min).abs() < 10.0 { 50.0 } else { raw_max - raw_min };
                        let min_y = (raw_min - delta * 0.15).max(0.0);
                        let max_y = raw_max + delta * 0.15;

                        let height = 220.0f64;
                        let width = 700.0f64;

                        let trails_data = active_list.iter().map(|tag_name| {
                            let tag_trials: Vec<&TrialData> = current_trials.iter()
                                .filter(|t| t.stimulus_type == *tag_name || t.test_type == *tag_name)
                                .collect();

                            let points = tag_trials.iter().enumerate().map(|(idx, t)| {
                                let x = 40.0 + (idx as f64 * (width / (tag_trials.len().max(2) as f64)));
                                let norm_y = ((t.reaction_time_ms - min_y) / (max_y - min_y)).clamp(0.0, 1.0);
                                let y = height - (norm_y * (height - 40.0)) + 20.0;
                                (x, y, t.reaction_time_ms)
                            }).collect::<Vec<_>>();

                            let stroke_color = match tag_name.as_str() {
                                "Foco Cortical Sustentado" => "#22d3ee",
                                "Velocidade de Processamento" => "#c084fc",
                                "Controle Inibitório" => "#34d399",
                                "Tomada de Decisão" => "#fbbf24",
                                _ => "#38bdf8",
                            };

                            (tag_name.clone(), stroke_color, points)
                        }).collect::<Vec<_>>();

                        view! {
                            <svg class="w-full h-full" viewBox="0 0 750 240">
                                // Linhas de grade horizontais
                                <line x1="30" y1="40" x2="730" y2="40" stroke="#1e293b" stroke-dasharray="4" />
                                <line x1="30" y1="120" x2="730" y2="120" stroke="#1e293b" stroke-dasharray="4" />
                                <line x1="30" y1="200" x2="730" y2="200" stroke="#1e293b" stroke-dasharray="4" />

                                // Desenho das Séries com escala dinâmica
                                {trails_data.into_iter().map(|(_name, stroke_color, points)| {
                                    if points.is_empty() {
                                        return view! { <g></g> }.into_view();
                                    }

                                    let path_d = points.iter().enumerate().map(|(i, (x, y, _))| {
                                        if i == 0 {
                                            format!("M {} {}", x, y)
                                        } else {
                                            format!("L {} {}", x, y)
                                        }
                                    }).collect::<Vec<_>>().join(" ");

                                    view! {
                                        <g>
                                            <path
                                                d=path_d
                                                fill="none"
                                                stroke=stroke_color
                                                stroke-width="2.5"
                                                stroke-linejoin="round"
                                            />
                                            {points.into_iter().map(|(x, y, val)| {
                                                view! {
                                                    <g class="group">
                                                        <circle cx=x cy=y r="4" fill=stroke_color class="transition-all hover:r-6 cursor-pointer" />
                                                        <text x=x y={y - 8.0} fill="#94a3b8" font-size="9" text-anchor="middle">
                                                            {format!("{:.0}ms", val)}
                                                        </text>
                                                    </g>
                                                }
                                            }).collect_view()}
                                        </g>
                                    }.into_view()
                                }).collect_view()}
                            </svg>
                        }.into_view()
                    }
                }}
            </div>

            // Rodapé com faixa dinâmica de valores
            <div class="flex justify-between items-center text-[10px] text-slate-500 pt-1 border-t border-slate-800/80">
                <div class="flex items-center gap-4">
                    <span class="flex items-center gap-1"><span class="w-3 h-0.5 bg-amber-400"></span> "Vetor Médio Acumulado"</span>
                    <span class="flex items-center gap-1"><span class="w-3 h-0.5 bg-cyan-400"></span> "Trilhas Sobrepostas"</span>
                </div>
                <div>
                    {move || format!("{} ensaios validados | {} sessões acumuladas", filtered_trials.get().len(), sessions.get().len())}
                </div>
            </div>
        </div>
    }
}
