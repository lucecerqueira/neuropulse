use leptos::*;
use crate::types::TrialData;

#[component]
pub fn AnalyticsSuite(
    #[prop(into)] trials: MaybeSignal<Vec<TrialData>>,
    #[prop(into, optional)] test_filter: Option<String>,
) -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal("barras".to_string());

    let trials_clone = trials.clone();
    let filter_clone = test_filter.clone();

    let filtered_trials = create_memo(move |_| {
        let all = trials_clone.get();
        let tf = filter_clone.clone();

        all.into_iter().filter(|t| {
            match &tf {
                Some(filter) => &t.test_type == filter,
                None => true,
            }
        }).collect::<Vec<_>>()
    });

    let block_stats = create_memo(move |_| {
        let list = filtered_trials.get();
        let mut map: std::collections::HashMap<u32, Vec<f64>> = std::collections::HashMap::new();

        for t in list {
            map.entry(t.trail_number).or_default().push(t.reaction_time_ms);
        }

        let mut stats: Vec<(u32, f64, f64, f64, f64)> = map.into_iter().map(|(b_id, times)| {
            let avg = times.iter().sum::<f64>() / times.len() as f64;
            let best = times.iter().copied().fold(f64::INFINITY, f64::min);
            
            let start_avg = if times.len() >= 2 { (times[0] + times[1]) / 2.0 } else { avg };
            let end_avg = if times.len() >= 2 { (times[times.len() - 1] + times[times.len() - 2]) / 2.0 } else { avg };

            (b_id, avg, best, start_avg, end_avg)
        }).collect();

        stats.sort_by_key(|s| s.0);
        stats
    });

    let scientific_categories = create_memo(move |_| {
        let list = filtered_trials.get();
        let total = list.len() as f64;
        if total == 0.0 {
            return (0.0, 0.0, 0.0, 0.0);
        }

        let mut elite = 0.0;
        let mut normal = 0.0;
        let mut delayed = 0.0;
        let mut lapse = 0.0;

        for t in list {
            let time = t.reaction_time_ms;
            if time < 220.0 {
                elite += 1.0;
            } else if time <= 350.0 {
                normal += 1.0;
            } else if time <= 550.0 {
                delayed += 1.0;
            } else {
                lapse += 1.0;
            }
        }

        (
            (elite / total) * 100.0,
            (normal / total) * 100.0,
            (delayed / total) * 100.0,
            (lapse / total) * 100.0,
        )
    });

    view! {
        <div class="space-y-4 font-mono bg-[#090d16] border border-slate-800/80 rounded-xl p-6">
            <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 border-b border-slate-800/80 pb-4">
                <h3 class="text-xs font-bold text-amber-400 uppercase tracking-wider">
                    "Painel de Analytics Avançado (Suíte de Diagnóstico)"
                </h3>

                <div class="flex items-center gap-2 bg-[#1e293b] p-1 rounded-lg text-xs">
                    <button
                        on:click=move |_| set_active_tab.set("barras".to_string())
                        class=move || format!("px-3 py-1 rounded-md font-bold cursor-pointer transition {}", if active_tab.get() == "barras" { "bg-cyan-500 text-slate-950" } else { "text-slate-400 hover:text-slate-200" })
                    >
                        "Médias p/ Bloco"
                    </button>
                    <button
                        on:click=move |_| set_active_tab.set("fadiga".to_string())
                        class=move || format!("px-3 py-1 rounded-md font-bold cursor-pointer transition {}", if active_tab.get() == "fadiga" { "bg-cyan-500 text-slate-950" } else { "text-slate-400 hover:text-slate-200" })
                    >
                        "Resistência à Fadiga"
                    </button>
                    <button
                        on:click=move |_| set_active_tab.set("proporcao".to_string())
                        class=move || format!("px-3 py-1 rounded-md font-bold cursor-pointer transition {}", if active_tab.get() == "proporcao" { "bg-cyan-500 text-slate-950" } else { "text-slate-400 hover:text-slate-200" })
                    >
                        "Proporção (Pizza)"
                    </button>
                </div>
            </div>

            {move || match active_tab.get().as_str() {
                "proporcao" => view! {
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 items-center py-4">
                        <div class="flex justify-center">
                            <svg class="w-52 h-52" viewBox="0 0 36 36">
                                {
                                    let (c1, c2, c3, c4) = scientific_categories.get();
                                    view! {
                                        <g>
                                            <circle cx="18" cy="18" r="15.915" fill="none" stroke="#1e293b" stroke-width="3.8" />
                                            <circle cx="18" cy="18" r="15.915" fill="none" stroke="#10b981" stroke-width="3.8"
                                                stroke-dasharray=format!("{:.1} {:.1}", c1, 100.0 - c1) stroke-dashoffset="25" />
                                            <circle cx="18" cy="18" r="15.915" fill="none" stroke="#38bdf8" stroke-width="3.8"
                                                stroke-dasharray=format!("{:.1} {:.1}", c2, 100.0 - c2) stroke-dashoffset=format!("{:.1}", 25.0 - c1) />
                                            <circle cx="18" cy="18" r="15.915" fill="none" stroke="#f59e0b" stroke-width="3.8"
                                                stroke-dasharray=format!("{:.1} {:.1}", c3, 100.0 - c3) stroke-dashoffset=format!("{:.1}", 25.0 - c1 - c2) />
                                            <circle cx="18" cy="18" r="15.915" fill="none" stroke="#f43f5e" stroke-width="3.8"
                                                stroke-dasharray=format!("{:.1} {:.1}", c4, 100.0 - c4) stroke-dashoffset=format!("{:.1}", 25.0 - c1 - c2 - c3) />
                                        </g>
                                    }
                                }
                            </svg>
                        </div>

                        <div class="space-y-2.5 text-xs">
                            <h4 class="font-bold text-slate-200 mb-1">"Zonas de Latência Baseadas na Literatura:"</h4>
                            <div class="flex items-center justify-between p-2.5 bg-[#060911] border border-slate-800 rounded-lg">
                                <div class="flex items-center gap-2">
                                    <span class="w-3 h-3 rounded-full bg-emerald-500"></span>
                                    <span class="text-slate-300">"Pico Neuro-Motor (<220ms)"</span>
                                </div>
                                <span class="font-bold text-emerald-400">{move || format!("{:.1}%", scientific_categories.get().0)}</span>
                            </div>
                            <div class="flex items-center justify-between p-2.5 bg-[#060911] border border-slate-800 rounded-lg">
                                <div class="flex items-center gap-2">
                                    <span class="w-3 h-3 rounded-full bg-sky-400"></span>
                                    <span class="text-slate-300">"Janela Competitiva (220-350ms)"</span>
                                </div>
                                <span class="font-bold text-sky-400">{move || format!("{:.1}%", scientific_categories.get().1)}</span>
                            </div>
                            <div class="flex items-center justify-between p-2.5 bg-[#060911] border border-slate-800 rounded-lg">
                                <div class="flex items-center gap-2">
                                    <span class="w-3 h-3 rounded-full bg-amber-500"></span>
                                    <span class="text-slate-300">"Carga Central Elevada (350-550ms)"</span>
                                </div>
                                <span class="font-bold text-amber-400">{move || format!("{:.1}%", scientific_categories.get().2)}</span>
                            </div>
                            <div class="flex items-center justify-between p-2.5 bg-[#060911] border border-slate-800 rounded-lg">
                                <div class="flex items-center gap-2">
                                    <span class="w-3 h-3 rounded-full bg-rose-500"></span>
                                    <span class="text-slate-300">"Lapsos Atencionais (>550ms)"</span>
                                </div>
                                <span class="font-bold text-rose-400">{move || format!("{:.1}%", scientific_categories.get().3)}</span>
                            </div>
                        </div>
                    </div>
                }.into_view(),

                "fadiga" => view! {
                    <div class="space-y-4 py-2">
                        <div class="text-xs text-slate-400">
                            "Análise Intrateste de Fadiga Neural (Início vs Fim do Bloco):"
                        </div>

                        <div class="space-y-4">
                            {move || {
                                let stats = block_stats.get();
                                if stats.is_empty() {
                                    view! { <div class="text-xs text-slate-500 py-6 text-center">"Sem histórico para calcular índices de fadiga."</div> }.into_view()
                                } else {
                                    let max_val = stats.iter().map(|s| s.3.max(s.4)).fold(1.0, f64::max);
                                    stats.into_iter().map(|(b_id, _avg, _best, start_avg, end_avg)| {
                                        let w_start = (start_avg / max_val * 100.0).min(100.0);
                                        let w_end = (end_avg / max_val * 100.0).min(100.0);
                                        let delta = end_avg - start_avg;
                                        let delta_color = if delta <= 0.0 { "text-emerald-400" } else { "text-rose-400" };

                                        view! {
                                            <div class="p-3 bg-[#060911] border border-slate-800 rounded-lg space-y-2">
                                                <div class="flex justify-between items-center text-xs">
                                                    <span class="font-bold text-cyan-400">{format!("Sessão / Bloco #{}", b_id)}</span>
                                                    <span class=format!("font-bold {}", delta_color)>
                                                        {if delta <= 0.0 { format!("Ganho de Velocidade: {:.1}ms", delta.abs()) } else { format!("Fadiga/Perda: +{:.1}ms", delta) }}
                                                    </span>
                                                </div>

                                                <div class="space-y-1 text-[11px]">
                                                    <div class="flex justify-between text-slate-400">
                                                        <span>"Início (Primeiros Ensaios)"</span>
                                                        <span class="font-bold text-slate-200">{format!("{:.1}ms", start_avg)}</span>
                                                    </div>
                                                    <div class="w-full h-2.5 bg-slate-900 rounded-full border border-slate-800 overflow-hidden">
                                                        <div class="h-full bg-cyan-400 rounded-full" style=format!("width: {}%", w_start)></div>
                                                    </div>

                                                    <div class="flex justify-between text-slate-400 pt-1">
                                                        <span>"Fim (Últimos Ensaios)"</span>
                                                        <span class="font-bold text-slate-200">{format!("{:.1}ms", end_avg)}</span>
                                                    </div>
                                                    <div class="w-full h-2.5 bg-slate-900 rounded-full border border-slate-800 overflow-hidden">
                                                        <div class="h-full bg-amber-400 rounded-full" style=format!("width: {}%", w_end)></div>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()
                                }
                            }}
                        </div>
                    </div>
                }.into_view(),

                _ => view! {
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-8 pt-2">
                        <div class="space-y-3">
                            <div class="text-xs text-slate-300 font-bold">"Média de reação por bloco"</div>
                            // Adicionada separação clara com overflow-x e gap/margin entre as barras verticais
                            <div class="bg-[#111622] border border-slate-800/80 rounded-xl p-4 h-60 flex items-end justify-start overflow-x-auto gap-2">
                                {move || {
                                    let stats = block_stats.get();
                                    if stats.is_empty() {
                                        view! { <div class="text-xs text-slate-500 m-auto">"Sem dados de blocos."</div> }.into_view()
                                    } else {
                                        let max_val = stats.iter().map(|s| s.1).fold(1.0, f64::max);

                                        stats.into_iter().map(|(b_id, avg, _best, _s, _e)| {
                                            let h_pct = (avg / max_val * 100.0).min(100.0);
                                            view! {
                                                <div class="flex flex-col items-center gap-2 h-full justify-end min-w-[36px] flex-1 group">
                                                    <div class="text-[9px] text-cyan-400 opacity-0 group-hover:opacity-100 transition">{format!("{:.0}ms", avg)}</div>
                                                    <div
                                                        class="w-full bg-gradient-to-t from-cyan-500 to-emerald-400 rounded-t-md border-t border-cyan-300/40 transition-all duration-500 shadow-sm"
                                                        style=format!("height: {}%", h_pct)
                                                    ></div>
                                                    <span class="text-[10px] text-slate-400 font-semibold">{format!("#{}", b_id)}</span>
                                                </div>
                                            }
                                        }).collect_view()
                                    }
                                }}
                            </div>
                        </div>

                        <div class="space-y-3">
                            <div class="flex justify-between text-xs text-slate-300 font-bold">
                                <span>"Média de reação por bloco"</span>
                                <span class="text-cyan-400">
                                    {move || {
                                        let stats = block_stats.get();
                                        if stats.is_empty() {
                                            "Média: 0.0ms | PB: 0.0ms".to_string()
                                        } else {
                                            let avg_all = stats.iter().map(|s| s.1).sum::<f64>() / stats.len() as f64;
                                            let best_all = stats.iter().map(|s| s.2).fold(f64::INFINITY, f64::min);
                                            format!("Média: {:.1}ms | PB: {:.1}ms", avg_all, best_all)
                                        }
                                    }}
                                </span>
                            </div>

                            <div class="bg-[#111622] border border-slate-800/80 rounded-xl p-4 h-60 flex flex-col justify-around overflow-y-auto">
                                {move || {
                                    let stats = block_stats.get();
                                    if stats.is_empty() {
                                        view! { <div class="text-xs text-slate-500 m-auto">"Sem dados de blocos."</div> }.into_view()
                                    } else {
                                        let max_val = stats.iter().map(|s| s.1).fold(1.0, f64::max);

                                        stats.into_iter().map(|(b_id, avg, _best, _s, _e)| {
                                            let w_pct = (avg / max_val * 100.0).min(100.0);
                                            view! {
                                                <div class="flex items-center gap-3 text-xs py-0.5">
                                                    <span class="text-slate-400 w-6 font-bold">{format!("#{}", b_id)}</span>
                                                    <div class="w-full h-5 bg-[#090d16] rounded-md overflow-hidden border border-slate-800/90 relative">
                                                        <div
                                                            class="h-full bg-gradient-to-r from-cyan-500 to-emerald-400 rounded-sm transition-all duration-500"
                                                            style=format!("width: {}%", w_pct)
                                                        ></div>
                                                    </div>
                                                    <span class="text-[10px] text-cyan-300 w-14 text-right">{format!("{:.0}ms", avg)}</span>
                                                </div>
                                            }
                                        }).collect_view()
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                }.into_view(),
            }}
        </div>
    }
}
