use leptos::*;
use crate::types::{TrialData, SessionRecord, MOTIVATIONAL_TRAILS};
use crate::components::chart::ReactionChart;

pub use GoNoGoView as GoNoGoTest;

#[component]
pub fn GoNoGoView(
    #[prop(into)] trials: MaybeSignal<Vec<TrialData>>,
    #[prop(into)] sessions: MaybeSignal<Vec<SessionRecord>>,
    #[prop(into, optional)] on_add_trial: Option<Callback<TrialData>>,
) -> impl IntoView {
    let (state, set_state) = create_signal("idle");
    let (selected_trail, set_selected_trail) = create_signal::<String>("Controle Inibitório".to_string());
    let (stimulus_type, set_stimulus_type) = create_signal::<String>("".to_string());
    let (reaction_time, set_reaction_time) = create_signal::<Option<f64>>(None);
    let (start_time, set_start_time) = create_signal::<f64>(0.0);
    let (trial_count, set_trial_count) = create_signal::<u32>(0);
    let (target_repetitions, set_target_repetitions) = create_signal::<u32>(5);
    let (_correct_count, set_correct_count) = create_signal::<u32>(0);

    let trials_memo_sig = trials.clone();
    let gonogo_trials = create_memo(move |_| {
        trials_memo_sig.get().into_iter().filter(|t| t.test_type == "GoNoGo").collect::<Vec<_>>()
    });

    let current_block_id = create_memo(move |_| {
        let list = gonogo_trials.get();
        if list.is_empty() {
            1
        } else {
            list.iter().map(|t| t.trail_number).max().unwrap_or(0) + 1
        }
    });

    let best_time_ms = create_memo(move |_| {
        gonogo_trials.get()
            .iter()
            .filter(|t| t.is_correct && t.reaction_time_ms > 0.0)
            .map(|t| t.reaction_time_ms)
            .fold(f64::INFINITY, f64::min)
    });

    let avg_time_ms = create_memo(move |_| {
        let list: Vec<_> = gonogo_trials.get().into_iter().filter(|t| t.is_correct && t.reaction_time_ms > 0.0).collect();
        if list.is_empty() {
            0.0
        } else {
            let sum: f64 = list.iter().map(|t| t.reaction_time_ms).sum();
            sum / list.len() as f64
        }
    });

    let accuracy_percentage = create_memo(move |_| {
        let list = gonogo_trials.get();
        if list.is_empty() {
            100.0
        } else {
            let correct = list.iter().filter(|t| t.is_correct).count();
            (correct as f64 / list.len() as f64) * 100.0
        }
    });

    let (next_trigger, set_next_trigger) = create_signal(0u32);

    create_effect(move |_| {
        let trigger = next_trigger.get();
        if trigger > 0 {
            set_state.set("waiting");
            set_reaction_time.set(None);

            let delay = (js_sys::Math::random() * 1500.0 + 1000.0) as i32;

            set_timeout(
                move || {
                    let is_go = js_sys::Math::random() > 0.3;
                    set_stimulus_type.set(if is_go { "GO".to_string() } else { "NOGO".to_string() });

                    let now = web_sys::window()
                        .and_then(|w| w.performance())
                        .map(|p| p.now())
                        .unwrap_or(0.0);
                    set_start_time.set(now);
                    set_state.set("ready");

                    if !is_go {
                        set_timeout(
                            move || {
                                if state.get() == "ready" && stimulus_type.get() == "NOGO" {
                                    let next_count = trial_count.get() + 1;
                                    set_trial_count.set(next_count);
                                    set_correct_count.update(|c| *c += 1);

                                    if let Some(cb) = on_add_trial {
                                        cb.call(TrialData {
                                            trial_number: next_count,
                                            trail_number: current_block_id.get(),
                                            test_type: "GoNoGo".to_string(),
                                            reaction_time_ms: 0.0,
                                            is_correct: true,
                                            stimulus_type: selected_trail.get(),
                                            timestamp: js_sys::Date::now(),
                                        });
                                    }

                                    if next_count >= target_repetitions.get() {
                                        set_state.set("completed");
                                    } else {
                                        set_state.set("feedback");
                                        set_timeout(
                                            move || set_next_trigger.update(|t| *t += 1),
                                            std::time::Duration::from_millis(600),
                                        );
                                    }
                                }
                            },
                            std::time::Duration::from_millis(1000),
                        );
                    }
                },
                std::time::Duration::from_millis(delay as u64),
            );
        }
    });

    let start_block = move |_| {
        set_trial_count.set(0);
        set_correct_count.set(0);
        set_next_trigger.update(|t| *t += 1);
    };

    let trigger_action = move || {
        if state.get() == "ready" {
            let now = web_sys::window()
                .and_then(|w| w.performance())
                .map(|p| p.now())
                .unwrap_or(0.0);
            let elapsed = now - start_time.get();
            let current_stim = stimulus_type.get();

            let is_correct = current_stim == "GO";
            if is_correct {
                set_reaction_time.set(Some(elapsed));
                set_correct_count.update(|c| *c += 1);
            } else {
                set_reaction_time.set(Some(0.0));
            }

            let next_count = trial_count.get() + 1;
            set_trial_count.set(next_count);

            if let Some(cb) = on_add_trial {
                cb.call(TrialData {
                    trial_number: next_count,
                    trail_number: current_block_id.get(),
                    test_type: "GoNoGo".to_string(),
                    reaction_time_ms: if is_correct { elapsed } else { 0.0 },
                    is_correct,
                    stimulus_type: selected_trail.get(),
                    timestamp: js_sys::Date::now(),
                });
            }

            if next_count >= target_repetitions.get() {
                set_state.set("completed");
            } else {
                set_state.set("feedback");
                set_timeout(
                    move || set_next_trigger.update(|t| *t += 1),
                    std::time::Duration::from_millis(600),
                );
            }
        }
    };

    let progress_percentage = move || {
        let total = target_repetitions.get();
        if total == 0 { 0.0 } else { (trial_count.get() as f64 / total as f64) * 100.0 }
    };

    view! {
        <div class="p-6 space-y-6 bg-[#090d16] min-h-screen text-slate-200 font-mono select-none touch-manipulation">
            <div class="bg-[#0f172a]/80 border border-slate-800/80 p-6 rounded-xl flex flex-col lg:flex-row justify-between items-start lg:items-center gap-4">
                <div>
                    <h1 class="text-xl font-bold text-emerald-400">
                        "Protocolo Go/No-Go (Controle Inibitório Cortical)"
                    </h1>
                    <p class="text-sm text-slate-400 mt-1">
                        "Avaliação de supressão de resposta motora e impulsividade cognitiva."
                    </p>
                </div>

                <div class="flex flex-wrap items-center gap-4 text-xs">
                    <div class="flex items-center gap-2">
                        <span class="text-slate-400">"Trilha:"</span>
                        <select
                            on:change=move |ev| {
                                set_selected_trail.set(event_target_value(&ev));
                            }
                            class="bg-[#1e293b] border border-slate-700 text-slate-200 px-3 py-1.5 rounded-lg focus:outline-none focus:border-emerald-500 cursor-pointer"
                        >
                            {MOTIVATIONAL_TRAILS.iter().map(|trail| {
                                let label_text = trail.name.to_string();
                                let option_val = label_text.clone();
                                let check_val = label_text.clone();

                                view! {
                                    <option
                                        value=option_val
                                        selected=move || selected_trail.get() == check_val
                                    >
                                        {label_text}
                                    </option>
                                }
                            }).collect_view()}
                        </select>
                    </div>

                    <div class="flex items-center gap-2">
                        <span class="text-slate-400">"Repetições por Bloco:"</span>
                        <select
                            on:change=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<u32>() {
                                    set_target_repetitions.set(val);
                                }
                            }
                            class="bg-[#1e293b] border border-slate-700 text-emerald-400 font-bold px-3 py-1.5 rounded-lg focus:outline-none focus:border-emerald-500 cursor-pointer"
                        >
                            <option value="3">"3 Repetições"</option>
                            <option value="5" selected=true>"5 Repetições"</option>
                            <option value="9">"9 Repetições"</option>
                            <option value="12">"12 Repetições"</option>
                        </select>
                    </div>
                </div>
            </div>

            <div class="bg-[#0f172a]/60 border border-slate-800/80 rounded-xl p-6">
                <ReactionChart trials=trials sessions=sessions test_filter="GoNoGo".to_string() />
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <div class="lg:col-span-2 bg-[#0f172a]/60 border border-slate-800/80 rounded-xl p-6 min-h-[420px] flex flex-col justify-between relative">
                    <div class="space-y-2">
                        <div class="flex justify-between items-center text-xs">
                            <span class="text-slate-400">
                                "Progresso do Bloco Go/No-Go: " <strong class="text-emerald-400">{move || trial_count.get()}</strong> "/" {move || target_repetitions.get()}
                            </span>
                            <span class="text-slate-400">
                                {move || format!("{:.0}% Concluído", progress_percentage())}
                            </span>
                        </div>
                        <div class="w-full h-2 bg-[#090d16] rounded-full overflow-hidden border border-slate-800">
                            <div
                                class="h-full bg-gradient-to-r from-emerald-500 to-teal-400 transition-all duration-300"
                                style=move || format!("width: {}%", progress_percentage())
                            ></div>
                        </div>
                    </div>

                    <div class="flex justify-center items-center my-auto py-6">
                        {move || match state.get() {
                            "idle" => view! {
                                <div class="flex flex-col items-center space-y-4">
                                    <button
                                        on:click=start_block
                                        class="px-8 py-4 bg-emerald-500 hover:bg-emerald-400 text-slate-950 font-bold text-base rounded-lg shadow-lg shadow-emerald-500/25 hover:scale-105 active:scale-95 transition cursor-pointer touch-manipulation"
                                    >
                                        "Iniciar Bloco Go/No-Go (" {move || target_repetitions.get()} " Repetições)"
                                    </button>
                                </div>
                            }.into_view(),
                            "waiting" => view! {
                                <div class="w-full max-w-md h-56 bg-slate-900 border-2 border-slate-800 rounded-xl flex items-center justify-center text-slate-400 font-bold text-lg select-none">
                                    "Aguardando sinal..."
                                </div>
                            }.into_view(),
                            "ready" => view! {
                                <button
                                    on:click=move |_| trigger_action()
                                    on:touchstart=move |ev| {
                                        ev.prevent_default();
                                        trigger_action();
                                    }
                                    class=move || format!(
                                        "w-full max-w-md h-56 rounded-xl flex flex-col items-center justify-center space-y-2 cursor-pointer select-none transition-all border-2 shadow-xl touch-manipulation {}",
                                        if stimulus_type.get() == "GO" {
                                            "bg-emerald-500 border-emerald-400 text-slate-950 shadow-emerald-500/30 animate-pulse"
                                        } else {
                                            "bg-rose-900/80 border-rose-500 text-rose-200 shadow-rose-900/40"
                                        }
                                    )
                                >
                                    <span class="text-3xl font-black">
                                        {move || if stimulus_type.get() == "GO" { "CLIQUE AGORA (GO!)" } else { "NÃO CLIQUE! (NO-GO)" }}
                                    </span>
                                </button>
                            }.into_view(),
                            "feedback" => view! {
                                <div class="w-full max-w-md h-56 bg-[#090d16] border border-slate-800 rounded-xl flex flex-col items-center justify-center space-y-2">
                                    <span class="text-xs text-slate-400">"Ensaio " {move || trial_count.get()} ":"</span>
                                    <div class="text-3xl font-bold text-emerald-400">
                                        {move || format!("{:.1} ms", reaction_time.get().unwrap_or(0.0))}
                                    </div>
                                </div>
                            }.into_view(),
                            "completed" => view! {
                                <div class="bg-[#090d16] border border-emerald-500/40 p-8 rounded-xl flex flex-col items-center space-y-4 text-center max-w-md">
                                    <span class="px-3 py-1 bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 text-xs font-bold rounded-md uppercase tracking-wider">
                                        "STATUS: FINALIZADO"
                                    </span>
                                    <h4 class="text-lg font-bold text-white">"Bloco Concluído!"</h4>
                                    <p class="text-xs text-slate-400">
                                        "Você realizou " {move || target_repetitions.get()} " repetições com média de "
                                        <strong class="text-yellow-400">{move || format!("{:.1} ms", avg_time_ms.get())}</strong>
                                    </p>
                                    <button
                                        on:click=start_block
                                        class="px-8 py-3 bg-emerald-500 hover:bg-emerald-400 text-slate-950 font-bold text-xs rounded-lg transition cursor-pointer touch-manipulation"
                                    >
                                        "Iniciar Novo Bloco"
                                    </button>
                                </div>
                            }.into_view(),
                            _ => view! { <div></div> }.into_view(),
                        }}
                    </div>
                </div>

                <div class="bg-[#0f172a]/60 border border-slate-800/80 rounded-xl p-6 space-y-4">
                    <h3 class="text-xs font-bold text-amber-400 uppercase tracking-wider">
                        "Métricas Go/No-Go"
                    </h3>

                    <div class="bg-[#090d16] border border-slate-800/80 rounded-lg p-4 space-y-1">
                        <span class="text-xs text-slate-400">"Menor Tempo GO (PB):"</span>
                        <div class="text-xl font-bold text-emerald-400">
                            {move || {
                                let val: f64 = best_time_ms.get();
                                if val.is_infinite() {
                                    "0.0 ms".to_string()
                                } else {
                                    format!("{:.1} ms", val)
                                }
                            }}
                        </div>
                    </div>

                    <div class="bg-[#090d16] border border-slate-800/80 rounded-lg p-4 space-y-1">
                        <span class="text-xs text-slate-400">"Média da Sessão GO:"</span>
                        <div class="text-xl font-bold text-yellow-400">
                            {move || format!("{:.1} ms", avg_time_ms.get())}
                        </div>
                    </div>

                    <div class="bg-[#090d16] border border-slate-800/80 rounded-lg p-4 space-y-1">
                        <span class="text-xs text-slate-400">"Precisão Inibitória:"</span>
                        <div class="text-xl font-bold text-cyan-400">
                            {move || format!("{:.1}%", accuracy_percentage.get())}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
