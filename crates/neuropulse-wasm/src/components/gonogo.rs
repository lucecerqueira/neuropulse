use leptos::*;
use gloo_timers::callback::Timeout;
use rand::Rng;
use crate::types::{TrialData, SessionRecord, MOTIVATIONAL_TRAILS};
use crate::components::chart::ReactionChart;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GoNoGoState {
    Idle,
    Running,
    Completed,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StimulusType {
    Go,
    NoGo,
}

#[component]
pub fn GoNoGoTest(
    #[prop(into)] on_session_complete: Callback<(f64, String)>,
    #[prop(optional)] sessions_history: Option<Signal<Vec<SessionRecord>>>,
) -> impl IntoView {
    let (test_state, set_test_state) = create_signal(GoNoGoState::Idle);
    let (total_trials, set_total_trials) = create_signal(5usize);
    let (current_trial, set_current_trial) = create_signal(0usize);
    let (current_stimulus, set_current_stimulus) = create_signal(StimulusType::Go);
    let (start_time, set_start_time) = create_signal(0.0f64);
    let (is_active_stimulus, set_is_active_stimulus) = create_signal(false);

    let (accumulated_trials, set_accumulated_trials) = create_signal(Vec::<TrialData>::new());
    let (current_run_times, set_current_run_times) = create_signal(Vec::<f64>::new());
    let (selected_trail_idx, set_selected_trail_idx) = create_signal(1usize);
    let (status_text, set_status_text) = create_signal("Clique em 'Iniciar Go/No-Go' para começar.".to_string());

    let (nogo_timeout_id, set_nogo_timeout_id) = create_signal(Option::<Timeout>::None);

    let spawn_stimulus_signal = create_rw_signal(0usize);

    let advance_trial = move |is_correct: bool, reaction_time: f64| {
        let mut run_times = current_run_times.get();
        if is_correct && current_stimulus.get() == StimulusType::Go {
            run_times.push(reaction_time);
        }
        set_current_run_times.set(run_times.clone());

        let trial_num = current_trial.get() + 1;
        set_current_trial.set(trial_num);

        let mut acc = accumulated_trials.get();
        acc.push(TrialData {
            trial_number: trial_num,
            trail_number: selected_trail_idx.get(),
            reaction_time_ms: if reaction_time > 0.0 { reaction_time } else { 0.0 },
            is_correct,
            timestamp: "Agora".to_string(),
        });
        set_accumulated_trials.set(acc);

        if trial_num >= total_trials.get() {
            set_test_state.set(GoNoGoState::Completed);
            set_is_active_stimulus.set(false);
            let sum: f64 = run_times.iter().sum();
            let avg = if run_times.is_empty() { 0.0 } else { sum / run_times.len() as f64 };
            let trail_name = MOTIVATIONAL_TRAILS.get(selected_trail_idx.get()).map(|t| t.name).unwrap_or("Trilha Custom");
            
            set_status_text.set(format!("Go/No-Go Concluído! Média Go: {:.2} ms [{}]", avg, trail_name));
            on_session_complete.call((avg, trail_name.to_string()));
        } else {
            spawn_stimulus_signal.update(|n| *n += 1);
        }
    };

    create_effect(move |_| {
        let trigger = spawn_stimulus_signal.get();
        if trigger > 0 && test_state.get_untracked() == GoNoGoState::Running {
            let mut rng = rand::thread_rng();
            let is_go = rng.gen_bool(0.7);
            let stim = if is_go { StimulusType::Go } else { StimulusType::NoGo };
            
            set_current_stimulus.set(stim);
            set_is_active_stimulus.set(false);
            set_status_text.set("Prepare-se...".to_string());

            let delay_ms = rng.gen_range(1200..2500);
            Timeout::new(delay_ms, move || {
                if test_state.get_untracked() == GoNoGoState::Running {
                    let window = web_sys::window().unwrap();
                    set_start_time.set(window.performance().unwrap().now());
                    set_is_active_stimulus.set(true);
                    
                    if stim == StimulusType::Go {
                        set_status_text.set("CLIQUE AGORA! (GO)".to_string());
                    } else {
                        set_status_text.set("NÃO CLIQUE! (NO-GO)".to_string());

                        let handle = Timeout::new(1300, move || {
                            if test_state.get_untracked() == GoNoGoState::Running && is_active_stimulus.get_untracked() {
                                set_is_active_stimulus.set(false);
                                set_status_text.set("Inibição Perfeita! (+1 Sucesso No-Go)".to_string());
                                advance_trial(true, 0.0);
                            }
                        });
                        set_nogo_timeout_id.set(Some(handle));
                    }
                }
            }).forget();
        }
    });

    let start_test = move |_| {
        set_accumulated_trials.set(Vec::new());
        set_current_run_times.set(Vec::new());
        set_current_trial.set(0);
        set_test_state.set(GoNoGoState::Running);
        spawn_stimulus_signal.update(|n| *n += 1);
    };

    let handle_click = move |_| {
        if test_state.get() == GoNoGoState::Running && is_active_stimulus.get() {
            let window = web_sys::window().unwrap();
            let end_time = window.performance().unwrap().now();
            let reaction_time = end_time - start_time.get();
            let stim = current_stimulus.get();

            set_is_active_stimulus.set(false);
            set_nogo_timeout_id.set(None);

            if stim == StimulusType::Go {
                advance_trial(true, reaction_time);
            } else {
                set_status_text.set("Erro de Inibição! Você clicou no No-Go (Vermelho).".to_string());
                advance_trial(false, reaction_time);
            }
        }
    };

    let cancel_test = move |_| {
        set_test_state.set(GoNoGoState::Idle);
        set_is_active_stimulus.set(false);
        set_nogo_timeout_id.set(None);
        set_current_run_times.set(Vec::new());
        set_current_trial.set(0);
        set_status_text.set("Teste cancelado.".to_string());
    };

    view! {
        <div class="space-y-6">
            <div class="flex flex-col md:flex-row justify-between items-start md:items-center bg-slate-900 p-4 rounded-xl border border-slate-800 gap-4">
                <div>
                    <h2 class="text-xl font-bold text-emerald-400 font-mono">"Teste Go/No-Go (Inibição de Resposta) + PINA"</h2>
                    <p class="text-xs text-slate-400">"Avalia o controle inibitório e a impulsividade com telemetria rPPG integrada."</p>
                </div>

                <div class="flex flex-wrap items-center gap-4 text-xs">
                    <div>
                        <label class="block text-[10px] text-slate-500 font-mono uppercase mb-1">"Trilha Motivacional"</label>
                        <select 
                            class="bg-slate-950 border border-slate-800 text-slate-200 rounded px-2.5 py-1.5 focus:outline-none focus:border-emerald-500 font-mono text-xs"
                            on:change=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                                    set_selected_trail_idx.set(val);
                                }
                            }
                        >
                            {MOTIVATIONAL_TRAILS.iter().map(|t| {
                                view! {
                                    <option value=t.index prop:selected=move || selected_trail_idx.get() == t.index>
                                        {t.name}
                                    </option>
                                }
                            }).collect_view()}
                        </select>
                    </div>

                    <div>
                        <label class="block text-[10px] text-slate-500 font-mono uppercase mb-1">"Ensaios"</label>
                        <select 
                            class="bg-slate-950 border border-slate-800 text-slate-200 rounded px-2.5 py-1.5 focus:outline-none focus:border-emerald-500 font-mono text-xs"
                            on:change=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                                    set_total_trials.set(val);
                                }
                            }
                        >
                            <option value="5" selected=true>"5 ensaios"</option>
                            <option value="10">"10 ensaios"</option>
                        </select>
                    </div>
                </div>
            </div>

            <div 
                class=move || match (test_state.get(), is_active_stimulus.get(), current_stimulus.get()) {
                    (GoNoGoState::Running, true, StimulusType::Go) => "h-52 bg-emerald-600 border-2 border-emerald-400 rounded-xl flex items-center justify-center p-6 text-center cursor-pointer shadow-2xl transition-all scale-[1.01]",
                    (GoNoGoState::Running, true, StimulusType::NoGo) => "h-52 bg-rose-600 border-2 border-rose-400 rounded-xl flex items-center justify-center p-6 text-center cursor-pointer shadow-2xl transition-all scale-[1.01]",
                    _ => "h-52 bg-slate-900 border border-slate-800 rounded-xl flex items-center justify-center p-6 text-center cursor-default",
                }
                on:click=handle_click
            >
                <div class="space-y-2 pointer-events-none">
                    <div class="text-2xl font-black text-white font-mono tracking-wide">
                        {move || status_text.get()}
                    </div>
                    <Show when=move || test_state.get() == GoNoGoState::Running>
                        <div class="text-xs text-white/80 font-mono">
                            {move || format!("Ensaio {} de {}", current_trial.get() + 1, total_trials.get())}
                        </div>
                    </Show>
                </div>
            </div>

            <div class="flex justify-between items-center bg-slate-900/50 p-3 rounded-xl border border-slate-800">
                <div class="flex gap-2">
                    <Show when=move || test_state.get() == GoNoGoState::Idle || test_state.get() == GoNoGoState::Completed>
                        <button 
                            class="px-5 py-2.5 bg-emerald-600 hover:bg-emerald-500 text-white font-bold text-xs rounded-lg transition-colors shadow-md shadow-emerald-950"
                            on:click=start_test
                        >
                            "Iniciar Go/No-Go"
                        </button>
                    </Show>

                    <Show when=move || test_state.get() == GoNoGoState::Running>
                        <button 
                            class="px-4 py-2 bg-red-600 hover:bg-red-500 text-white font-bold text-xs rounded-lg transition-colors"
                            on:click=cancel_test
                        >
                            "Cancelar Teste"
                        </button>
                    </Show>
                </div>

                <div class="text-xs font-mono text-slate-400">
                    {move || {
                        let times = current_run_times.get();
                        if times.is_empty() {
                            "Média Go: 0 ms".to_string()
                        } else {
                            let sum: f64 = times.iter().sum();
                            format!("Média Go: {:.1} ms", sum / times.len() as f64)
                        }
                    }}
                </div>
            </div>

            <div class="bg-slate-900 p-4 rounded-xl border border-slate-800">
                {move || match sessions_history {
                    Some(history) => view! {
                        <ReactionChart 
                            trials=Signal::derive(move || accumulated_trials.get()) 
                            sessions_history=history
                            test_filter="Go/No-Go".to_string()
                        />
                    }.into_view(),
                    None => view! {
                        <ReactionChart 
                            trials=Signal::derive(move || accumulated_trials.get()) 
                            test_filter="Go/No-Go".to_string()
                        />
                    }.into_view(),
                }}
            </div>
        </div>
    }
}
