use leptos::*;
use std::collections::HashSet;
use crate::types::{TrialData, SessionRecord, MOTIVATIONAL_TRAILS, TRAIL_COLORS};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChartMode {
    LinesAndMarkers,
    Bars,
}

#[component]
pub fn ReactionChart(
    #[prop(into)] trials: Signal<Vec<TrialData>>,
    #[prop(optional)] sessions_history: Option<Signal<Vec<SessionRecord>>>,
    #[prop(optional)] test_filter: Option<String>,
) -> impl IntoView {
    let (chart_mode, set_chart_mode) = create_signal(ChartMode::LinesAndMarkers);
    let (show_raw_table, set_show_raw_table) = create_signal(false);
    
    // Toggles Estatísticos de Exibição
    let (show_mean_line, set_show_mean_line) = create_signal(true);
    let (show_std_band, set_show_std_band) = create_signal(true);
    let (show_markers, set_show_markers) = create_signal(true);

    // Estado do Dropdown Multi-Tag para Trilhas Motivacionais
    let (dropdown_open, set_dropdown_open) = create_signal(false);
    let (selected_trails, set_selected_trails) = create_signal({
        let mut set = HashSet::new();
        for i in 0..13 { set.insert(i); } // Padrão: Todas selecionadas
        set
    });

    let toggle_trail = move |idx: usize| {
        let mut set = selected_trails.get();
        if set.contains(&idx) {
            if set.len() > 1 { set.remove(&idx); }
        } else {
            set.insert(idx);
        }
        set_selected_trails.set(set);
    };

    let remove_trail = move |idx: usize| {
        let mut set = selected_trails.get();
        if set.len() > 1 {
            set.remove(&idx);
            set_selected_trails.set(set);
        }
    };

    let filter_val = test_filter.clone();

    view! {
        <div class="space-y-4 font-sans">
            // Cabeçalho e Seletores Principais
            <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-3 border-b border-slate-800 pb-3">
                <div>
                    <h4 class="text-sm font-bold text-cyan-400 font-mono">
                        {
                            let f_title = filter_val.clone();
                            match &f_title {
                                Some(t) => format!("Motor Gráfico Multi-Tag - Módulo {}", t),
                                None => "Dashboard Geral: Análise de Tendência & Intervalo".to_string(),
                            }
                        }
                    </h4>
                    <p class="text-[11px] text-slate-400">"Acumulação contínua com banda de desvio padrão e estatísticas por trilhas."</p>
                </div>

                <div class="flex flex-wrap items-center gap-2">
                    <div class="bg-slate-950 border border-slate-800 p-0.5 rounded-lg flex text-[10px] font-mono">
                        <button 
                            class=move || if chart_mode.get() == ChartMode::LinesAndMarkers { "px-2.5 py-1 rounded bg-cyan-600 text-white font-bold" } else { "px-2.5 py-1 rounded text-slate-400 hover:text-white" }
                            on:click=move |_| set_chart_mode.set(ChartMode::LinesAndMarkers)
                        >
                            "📈 Curvas & Marcadores"
                        </button>
                        <button 
                            class=move || if chart_mode.get() == ChartMode::Bars { "px-2.5 py-1 rounded bg-cyan-600 text-white font-bold" } else { "px-2.5 py-1 rounded text-slate-400 hover:text-white" }
                            on:click=move |_| set_chart_mode.set(ChartMode::Bars)
                        >
                            "📊 Barras por Ensaio"
                        </button>
                    </div>

                    <button 
                        class=move || if show_raw_table.get() { "px-3 py-1 bg-cyan-600 text-white font-bold text-xs rounded transition-colors" } else { "px-3 py-1 bg-slate-800 text-slate-300 hover:bg-slate-700 font-bold text-xs rounded transition-colors" }
                        on:click=move |_| set_show_raw_table.set(!show_raw_table.get())
                    >
                        {move || if show_raw_table.get() { "▾ Ocultar Tabela" } else { "▸ Exibir Tabela" }}
                    </button>
                </div>
            </div>

            // Painel de Filtro Multi-Tag / Dropdown para Trilhas
            <div class="bg-slate-900/80 p-3 rounded-xl border border-slate-800 space-y-2 relative">
                <div class="flex justify-between items-center">
                    <span class="text-[10px] font-mono text-cyan-400 font-bold uppercase tracking-wider">
                        "Filtro por Tags de Trilhas Motivacionais:"
                    </span>

                    // Toggles das Camadas Estatísticas
                    <div class="flex items-center gap-3 text-[10px] font-mono text-slate-300">
                        <label class="flex items-center gap-1 cursor-pointer">
                            <input type="checkbox" prop:checked=move || show_mean_line.get() on:change=move |_| set_show_mean_line.set(!show_mean_line.get()) class="rounded bg-slate-950 border-slate-700 text-cyan-500" />
                            "Vetor Médio"
                        </label>
                        <label class="flex items-center gap-1 cursor-pointer">
                            <input type="checkbox" prop:checked=move || show_std_band.get() on:change=move |_| set_show_std_band.set(!show_std_band.get()) class="rounded bg-slate-950 border-slate-700 text-cyan-500" />
                            "Banda Desvio (±1σ)"
                        </label>
                        <label class="flex items-center gap-1 cursor-pointer">
                            <input type="checkbox" prop:checked=move || show_markers.get() on:change=move |_| set_show_markers.set(!show_markers.get()) class="rounded bg-slate-950 border-slate-700 text-cyan-500" />
                            "Marcadores"
                        </label>
                    </div>
                </div>

                // Interface de Tags Selecionadas com Botão Dropdown
                <div class="flex flex-wrap items-center gap-1.5 bg-slate-950 border border-slate-800 p-2 rounded-lg min-h-[42px]">
                    {
                        move || {
                            let active_set = selected_trails.get();
                            MOTIVATIONAL_TRAILS.iter().filter(|t| active_set.contains(&t.index)).map(|t| {
                                let idx = t.index;
                                let color = t.color;
                                let name = t.name;
                                view! {
                                    <span class="inline-flex items-center gap-1.5 px-2 py-0.5 rounded bg-slate-900 border border-slate-700 text-[10px] font-mono text-slate-200">
                                        <span class="w-2 h-2 rounded-full" style=format!("background-color: {};", color)></span>
                                        {name}
                                        <button class="text-slate-500 hover:text-red-400 font-bold ml-1" on:click=move |_| remove_trail(idx)>"×"</button>
                                    </span>
                                }
                            }).collect_view()
                        }
                    }

                    <div class="relative inline-block ml-auto">
                        <button 
                            class="px-2.5 py-1 bg-slate-800 hover:bg-slate-700 text-slate-300 font-mono text-[10px] font-bold rounded border border-slate-700 flex items-center gap-1"
                            on:click=move |_| set_dropdown_open.set(!dropdown_open.get())
                        >
                            "+ Adicionar / Filtrar Trilha ▾"
                        </button>

                        <Show when=move || dropdown_open.get()>
                            <div class="absolute right-0 mt-1 w-64 bg-slate-900 border border-slate-700 rounded-lg shadow-2xl z-50 max-h-48 overflow-y-auto p-1 divide-y divide-slate-800">
                                {
                                    MOTIVATIONAL_TRAILS.iter().map(|t| {
                                        let idx = t.index;
                                        let color = t.color;
                                        let name = t.name;
                                        view! {
                                            <div 
                                                class="p-2 hover:bg-slate-800 cursor-pointer flex items-center justify-between text-[10px] font-mono"
                                                on:click=move |_| toggle_trail(idx)
                                            >
                                                <div class="flex items-center gap-2">
                                                    <span class="w-2 h-2 rounded-full" style=format!("background-color: {};", color)></span>
                                                    <span class="text-slate-200">{name}</span>
                                                </div>
                                                <span class="text-cyan-400">
                                                    {move || if selected_trails.get().contains(&idx) { "✓" } else { "" }}
                                                </span>
                                            </div>
                                        }
                                    }).collect_view()
                                }
                            </div>
                        </Show>
                    </div>
                </div>
            </div>

            // Área Gráfica com Proporções Corretas
            <div class="bg-slate-950 rounded-xl border border-slate-800 p-4 relative overflow-hidden">
                {
                    let f_val_outer = filter_val.clone();
                    move || {
                        let f_val_inner = f_val_outer.clone();
                        let current_trials = trials.get();
                        let active_trails_set = selected_trails.get();

                        // Histórico Filtrado Acumulativo
                        let history_records: Vec<SessionRecord> = if let Some(hist_sig) = sessions_history {
                            let raw_hist = hist_sig.get();
                            raw_hist.into_iter().filter(|s| {
                                let match_test = if let Some(ref target) = f_val_inner {
                                    if let Some(ref t) = s.test_type {
                                        t.to_uppercase().contains(&target.to_uppercase()) || target.to_uppercase().contains(&t.to_uppercase())
                                    } else { false }
                                } else { true };

                                let trail_idx = match s.trail_context.as_deref() {
                                    Some(t_str) => MOTIVATIONAL_TRAILS.iter().position(|m| m.name == t_str).unwrap_or(0),
                                    None => 0,
                                };

                                match_test && active_trails_set.contains(&trail_idx)
                            }).collect()
                        } else {
                            Vec::new()
                        };

                        match chart_mode.get() {
                            ChartMode::LinesAndMarkers => {
                                view! {
                                    <div class="space-y-2">
                                        <div class="flex justify-between text-[10px] font-mono text-slate-400 border-b border-slate-900 pb-1">
                                            <span>"Linha Ciano: Ensaio Atual | Pontos Coloridos: Histórico Acumulado por Trilha"</span>
                                            <span class="text-cyan-400">{format!("{} ensaios | {} sessões acumuladas", current_trials.len(), history_records.len())}</span>
                                        </div>

                                        <div class="w-full relative aspect-[16/6] bg-slate-950 rounded-lg">
                                            {
                                                if current_trials.is_empty() && history_records.is_empty() {
                                                    view! {
                                                        <div class="absolute inset-0 flex items-center justify-center text-xs text-slate-500 font-mono">
                                                            "Aguardando ensaios para iniciar o rastreamento gráfico..."
                                                        </div>
                                                    }.into_view()
                                                } else {
                                                    let mut all_vals: Vec<f64> = current_trials.iter().map(|t| t.reaction_time_ms).collect();
                                                    all_vals.extend(history_records.iter().map(|s| s.avg_reaction_time_ms));

                                                    let max_val = all_vals.iter().cloned().fold(500.0, f64::max);
                                                    let min_val = all_vals.iter().cloned().fold(0.0, f64::min);
                                                    let range = if max_val == min_val { 1.0 } else { max_val - min_val };

                                                    // Cálculo de Média e Desvio Padrão
                                                    let n = all_vals.len() as f64;
                                                    let mean_cohered_val = if n > 0.0 { all_vals.iter().sum::<f64>() / n } else { 0.0 };
                                                    let variance = if n > 1.0 {
                                                        all_vals.iter().map(|v| (v - mean_cohered_val).powi(2)).sum::<f64>() / (n - 1.0)
                                                    } else { 0.0 };
                                                    let std_dev = variance.sqrt();

                                                    let mean_y = 130.0 - ((mean_cohered_val - min_val) / range * 100.0);
                                                    let std_upper_y = 130.0 - (((mean_cohered_val + std_dev) - min_val) / range * 100.0);
                                                    let std_lower_y = 130.0 - (((mean_cohered_val - std_dev) - min_val) / range * 100.0);

                                                    // Pontos e Linha do Ensaio Atual
                                                    let mut cur_path = String::new();
                                                    let mut cur_pts = Vec::new();
                                                    let c_count = current_trials.len();
                                                    for (i, t) in current_trials.iter().enumerate() {
                                                        let x = if c_count <= 1 { 50.0 } else { (i as f64 / (c_count - 1) as f64) * 520.0 + 40.0 };
                                                        let y = 130.0 - ((t.reaction_time_ms - min_val) / range * 100.0);
                                                        cur_pts.push((x, y, t.reaction_time_ms, t.trial_number));
                                                        if i == 0 { cur_path.push_str(&format!("M {:.1} {:.1}", x, y)); }
                                                        else { cur_path.push_str(&format!(" L {:.1} {:.1}", x, y)); }
                                                    }

                                                    // Pontos do Histórico por Trilha
                                                    let mut hist_elements = Vec::new();
                                                    let h_count = history_records.len();
                                                    if h_count > 0 {
                                                        for (i, s) in history_records.iter().enumerate() {
                                                            let x = if h_count <= 1 { 50.0 } else { (i as f64 / (h_count - 1) as f64) * 520.0 + 40.0 };
                                                            let y = 130.0 - ((s.avg_reaction_time_ms - min_val) / range * 100.0);
                                                            let t_idx = match s.trail_context.as_deref() {
                                                                Some(t_str) => MOTIVATIONAL_TRAILS.iter().position(|m| m.name == t_str).unwrap_or(0),
                                                                None => 0,
                                                            };
                                                            let t_color = TRAIL_COLORS.get(t_idx).unwrap_or(&"#06b6d4");
                                                            hist_elements.push((x, y, s.avg_reaction_time_ms, *t_color));
                                                        }
                                                    }

                                                    let has_cur = !cur_path.is_empty();

                                                    view! {
                                                        <svg class="w-full h-full" viewBox="0 0 600 150" preserveAspectRatio="xMidYMid meet">
                                                            // Banda de Desvio Padrão (±1σ)
                                                            <Show when=move || show_std_band.get()>
                                                                <rect x="20" y=std_upper_y width="560" height=(std_lower_y - std_upper_y).abs() fill="#f59e0b" fill-opacity="0.08" />
                                                            </Show>

                                                            // Vetor Médio Tracejado
                                                            <Show when=move || show_mean_line.get()>
                                                                <line x1="20" y1=mean_y x2="580" y2=mean_y stroke="#f59e0b" stroke-width="1.8" stroke-dasharray="4,4" />
                                                                <text x="585" y=mean_y fill="#f59e0b" font-size="8" font-family="monospace" text-anchor="start">{format!("{:.1}ms", mean_cohered_val)}</text>
                                                            </Show>

                                                            // Pontos do Histórico Acumulado
                                                            {
                                                                hist_elements.into_iter().map(|(x, y, val, color)| {
                                                                    let label_text = format!("{:.0}ms", val);
                                                                    view! {
                                                                        <g>
                                                                            <circle cx=x cy=y r="4" fill=color stroke="#ffffff" stroke-width="1" />
                                                                            <Show when=move || show_markers.get()>
                                                                                {
                                                                                    let lbl = label_text.clone();
                                                                                    view! {
                                                                                        <text x=x y=y-8.0 fill=color font-size="8" font-family="monospace" text-anchor="middle">{lbl}</text>
                                                                                    }
                                                                                }
                                                                            </Show>
                                                                        </g>
                                                                    }
                                                                }).collect_view()
                                                            }

                                                            // Curva do Ensaio Ativo
                                                            <Show when=move || has_cur>
                                                                <path d=cur_path.clone() fill="none" stroke="#06b6d4" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" />
                                                                {
                                                                    let c_nodes = cur_pts.clone();
                                                                    c_nodes.into_iter().map(|(x, y, val, num)| {
                                                                        let label_text = format!("{:.1}ms", val);
                                                                        view! {
                                                                            <g>
                                                                                <circle cx=x cy=y r="5" fill="#22d3ee" stroke="#ffffff" stroke-width="2" />
                                                                                <Show when=move || show_markers.get()>
                                                                                    {
                                                                                        let lbl = label_text.clone();
                                                                                        view! {
                                                                                            <text x=x y=y-10.0 fill="#ffffff" font-size="9" font-weight="bold" font-family="monospace" text-anchor="middle">{lbl}</text>
                                                                                        }
                                                                                    }
                                                                                </Show>
                                                                                <text x=x y="145" fill="#64748b" font-size="8" font-family="monospace" text-anchor="middle">{format!("E{}", num)}</text>
                                                                            </g>
                                                                        }
                                                                    }).collect_view()
                                                                }
                                                            </Show>
                                                        </svg>
                                                    }.into_view()
                                                }
                                            }
                                        </div>
                                    </div>
                                }.into_view()
                            }
                            ChartMode::Bars => {
                                view! {
                                    <div class="space-y-2">
                                        <div class="flex justify-between text-[10px] font-mono text-slate-400 border-b border-slate-900 pb-1">
                                            <span>"Modo: Barras por Ensaio"</span>
                                            <span class="text-cyan-400">{format!("{} ensaios acumulados", current_trials.len())}</span>
                                        </div>
                                        <div class="h-44 flex items-end gap-2 overflow-x-auto pt-2">
                                            {if current_trials.is_empty() {
                                                view! {
                                                    <div class="w-full h-full flex items-center justify-center text-xs text-slate-500 font-mono">
                                                        "Aguardando a execução dos ensaios nesta sessão..."
                                                    </div>
                                                }.into_view()
                                            } else {
                                                current_trials.into_iter().map(|t| {
                                                    let h_pct = (t.reaction_time_ms / 1000.0 * 100.0).clamp(8.0, 95.0);
                                                    let color = if t.is_correct { "bg-cyan-500 hover:bg-cyan-400" } else { "bg-rose-500 hover:bg-rose-400" };
                                                    let tooltip = format!("{:.1} ms", t.reaction_time_ms);
                                                    let idx_str = format!("E{}", t.trial_number);
                                                    view! {
                                                        <div class="flex flex-col items-center gap-1 h-full justify-end group relative min-w-[36px]">
                                                            <div class="absolute -top-7 bg-slate-900 border border-slate-700 text-[9px] font-mono text-white px-1.5 py-0.5 rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap z-20 shadow-xl">
                                                                {tooltip}
                                                            </div>
                                                            <div class=format!("w-7 rounded-t transition-all {}", color) style=format!("height: {}%;", h_pct)></div>
                                                            <span class="text-[9px] font-mono text-slate-400 font-bold">{idx_str}</span>
                                                        </div>
                                                    }
                                                }).collect_view().into_view()
                                            }}
                                        </div>
                                    </div>
                                }.into_view()
                            }
                        }
                    }
                }
            </div>

            // Tabela de Dados Brutos
            <Show when=move || show_raw_table.get() && sessions_history.is_some()>
                <div class="bg-slate-900 p-4 rounded-xl border border-slate-800 space-y-2 animate-fade-in">
                    <span class="text-xs font-mono text-cyan-400 font-bold block">"Dados Brutos das Sessões Filtradas"</span>
                    <div class="max-h-36 overflow-y-auto divide-y divide-slate-800 font-mono text-xs">
                        {
                            let hist_opt = sessions_history;
                            let target_type = filter_val.clone();
                            
                            move || {
                                if let Some(hist_sig) = hist_opt {
                                    let history = hist_sig.get();
                                    let active_trails_set = selected_trails.get();

                                    let filtered: Vec<_> = history.into_iter().filter(|s| {
                                        let match_test = if let Some(ref target) = target_type {
                                            if let Some(ref t) = s.test_type {
                                                t.to_uppercase().contains(&target.to_uppercase()) || target.to_uppercase().contains(&t.to_uppercase())
                                            } else { false }
                                        } else { true };

                                        let trail_idx = match s.trail_context.as_deref() {
                                            Some(t_str) => MOTIVATIONAL_TRAILS.iter().position(|m| m.name == t_str).unwrap_or(0),
                                            None => 0,
                                        };

                                        match_test && active_trails_set.contains(&trail_idx)
                                    }).collect();

                                    if filtered.is_empty() {
                                        view! {
                                            <div class="p-3 text-slate-500 text-center">
                                                "Nenhum registro encontrado para as tags selecionadas."
                                            </div>
                                        }.into_view()
                                    } else {
                                        filtered.into_iter().map(move |item| {
                                            let t_type = item.test_type.unwrap_or_else(|| "Sessão".to_string());
                                            let t_context = item.trail_context.unwrap_or_else(|| "Geral".to_string());
                                            let time_str = item.created_at;
                                            let rt_str = format!("{:.1} ms", item.avg_reaction_time_ms);
                                            let bpm_str = format!("{:.0} BPM", item.rppg_bpm.unwrap_or(72.0));

                                            view! {
                                                <div class="py-2 flex justify-between items-center px-2 hover:bg-slate-950 rounded">
                                                    <div class="flex items-center gap-3">
                                                        <span class="text-cyan-400 font-bold">{t_type}</span>
                                                        <span class="text-slate-400">"(" {t_context} ")"</span>
                                                        <span class="text-slate-500 text-[10px]">{time_str}</span>
                                                    </div>
                                                    <div class="flex items-center gap-4">
                                                        <span class="text-emerald-400 font-bold">{rt_str}</span>
                                                        <span class="text-purple-400 text-[11px]">{bpm_str}</span>
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view().into_view()
                                    }
                                } else {
                                    view! { <div></div> }.into_view()
                                }
                            }
                        }
                    </div>
                </div>
            </Show>
        </div>
    }
}
