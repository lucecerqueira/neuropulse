use leptos::*;
use gloo_timers::callback::Timeout;
use rand::Rng;
use crate::types::{TrialData, SessionRecord, MOTIVATIONAL_TRAILS};
use crate::components::chart::ReactionChart;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TestState {
    Idle,
    Waiting,
    Ready,
    Paused,
    Completed,
}

#[component]
pub fn SimpleReactionTest(
    #[prop(into)] on_session_complete: Callback<(f64, String)>,
    #[prop(optional)] sessions_history: Option<Signal<Vec<SessionRecord>>>,
) -> impl IntoView {
    let (test_state, set_test_state) = create_signal(TestState::Idle);
    let (total_trials, set_total_trials) = create_signal(5usize);
    let (current_trial, set_current_trial) = create_signal(0usize);
    let (start_time, set_start_time) = create_signal(0.0f64);
    
    let (accumulated_trials, set_accumulated_trials) = create_signal(Vec::<TrialData>::new());
    let (current_run_times, set_current_run_times) = create_signal(Vec::<f64>::new());
    let (selected_trail_idx, set_selected_trail_idx) = create_signal(0usize);
    let (status_text, set_status_text) = create_signal("Pressione 'Iniciar Teste' para começar.".to_string());

    let start_test = move |_| {
        set_accumulated_trials.set(Vec::new());
        set_current_run_times.set(Vec::new());
        set_current_trial.set(0);
        set_test_state.set(TestState::Waiting);
        set_status_text.set("Aguarde o sinal verde... Não clique ainda!".to_string());

        let delay_ms = rand::thread_rng().gen_range(1500..4000);
        Timeout::new(delay_ms, move || {
            if test_state.get_untracked() == TestState::Waiting {
                set_test_state.set(TestState::Ready);
                let window = web_sys::window().unwrap();
                set_start_time.set(window.performance().unwrap().now());
                set_status_text.set("CLIQUE AGORA!".to_string());
            }
        }).forget();
    };

    let handle_click = move |_| {
        match test_state.get() {
            TestState::Waiting => {
                set_status_text.set("Antecipado! Clique apenas quando ficar verde.".to_string());
                set_test_state.set(TestState::Idle);
            }
            TestState::Ready => {
                let window = web_sys::window().unwrap();
                let end_time = window.performance().unwrap().now();
                let reaction_time = end_time - start_time.get();

                let mut run_times = current_run_times.get();
                run_times.push(reaction_time);
                set_current_run_times.set(run_times.clone());

                let trial_num = current_trial.get() + 1;
                set_current_trial.set(trial_num);

                // ATUALIZAÇÃO EM TEMPO REAL PARA O GRÁFICO
                let mut acc = accumulated_trials.get();
                acc.push(TrialData {
                    trial_number: trial_num,
                    trail_number: selected_trail_idx.get(),
                    reaction_time_ms: reaction_time,
                    is_correct: true,
                    timestamp: "Agora".to_string(),
                });
                set_accumulated_trials.set(acc);

                if trial_num >= total_trials.get() {
                    set_test_state.set(TestState::Completed);
                    let sum: f64 = run_times.iter().sum();
                    let avg = sum / run_times.len() as f64;
                    let trail_name = MOTIVATIONAL_TRAILS.get(selected_trail_idx.get()).map(|t| t.name).unwrap_or("Trilha Custom");
                    
                    set_status_text.set(format!("Teste SRT Concluído! Média: {:.2} ms [{}]", avg, trail_name));
                    on_session_complete.call((avg, trail_name.to_string()));
                } else {
                    set_test_state.set(TestState::Waiting);
                    set_status_text.set("Aguarde o próximo sinal verde...".to_string());

                    let delay_ms = rand::thread_rng().gen_range(1500..3500);
                    Timeout::new(delay_ms, move || {
                        if test_state.get_untracked() == TestState::Waiting {
                            set_test_state.set(TestState::Ready);
                            let window = web_sys::window().unwrap();
                            set_start_time.set(window.performance().unwrap().now());
                            set_status_text.set("CLIQUE AGORA!".to_string());
                        }
                    }).forget();
                }
            }
            _ => {}
        }
    };

    let pause_test = move |_| {
        if test_state.get() == TestState::Waiting || test_state.get() == TestState::Ready {
            set_test_state.set(TestState::Paused);
            set_status_text.set("Teste em pausa.".to_string());
        }
    };

    let resume_test = move |_| {
        if test_state.get() == TestState::Paused {
            set_test_state.set(TestState::Waiting);
            set_status_text.set("Retomando... Aguarde o sinal verde.".to_string());
            let delay_ms = rand::thread_rng().gen_range(1500..3000);
            Timeout::new(delay_ms, move || {
                if test_state.get_untracked() == TestState::Waiting {
                    set_test_state.set(TestState::Ready);
                    let window = web_sys::window().unwrap();
                    set_start_time.set(window.performance().unwrap().now());
                    set_status_text.set("CLIQUE AGORA!".to_string());
                }
            }).forget();
        }
    };

    let cancel_test = move |_| {
        set_test_state.set(TestState::Idle);
        set_current_run_times.set(Vec::new());
        set_current_trial.set(0);
        set_status_text.set("Teste cancelado. Pressione 'Iniciar Teste' para reiniciar.".to_string());
    };

    view! {
        <div class="space-y-6">
            <div class="flex flex-col md:flex-row justify-between items-start md:items-center bg-slate-900 p-4 rounded-xl border border-slate-800 gap-4">
                <div>
                    <h2 class="text-xl font-bold text-cyan-400 font-mono">"Teste de Reação Simples (SRT) + PINA"</h2>
                    <p class="text-xs text-slate-400">"Mede a velocidade pura do reflexo neuromuscular com telemetria rPPG integrada."</p>
                </div>

                <div class="flex flex-wrap items-center gap-4 text-xs">
                    <div>
                        <label class="block text-[10px] text-slate-500 font-mono uppercase mb-1">"Trilha Motivacional"</label>
                        <select 
                            class="bg-slate-950 border border-slate-800 text-slate-200 rounded px-2.5 py-1.5 focus:outline-none focus:border-cyan-500 font-mono text-xs"
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
                        <label class="block text-[10px] text-slate-500 font-mono uppercase mb-1">"Repetições"</label>
                        <select 
                            class="bg-slate-950 border border-slate-800 text-slate-200 rounded px-2.5 py-1.5 focus:outline-none focus:border-cyan-500 font-mono text-xs"
                            on:change=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                                    set_total_trials.set(val);
                                }
                            }
                        >
                            <option value="5" selected=true>"5 ensaios"</option>
                            <option value="10">"10 ensaios"</option>
                            <option value="15">"15 ensaios"</option>
                        </select>
                    </div>
                </div>
            </div>

            <div 
                class=move || match test_state.get() {
                    TestState::Idle => "h-48 bg-slate-900 border border-slate-800 rounded-xl flex items-center justify-center p-6 text-center transition-all cursor-default",
                    TestState::Waiting => "h-48 bg-red-950/80 border-2 border-red-600/80 rounded-xl flex items-center justify-center p-6 text-center transition-all cursor-pointer shadow-lg shadow-red-900/20",
                    TestState::Ready => "h-48 bg-emerald-600 border-2 border-emerald-400 rounded-xl flex items-center justify-center p-6 text-center transition-all cursor-pointer shadow-2xl shadow-emerald-500/40 scale-[1.01]",
                    TestState::Paused => "h-48 bg-amber-950/60 border-2 border-amber-600/60 rounded-xl flex items-center justify-center p-6 text-center transition-all cursor-default",
                    TestState::Completed => "h-48 bg-slate-900 border border-slate-800 rounded-xl flex items-center justify-center p-6 text-center transition-all cursor-default",
                }
                on:click=handle_click
            >
                <div class="space-y-2 pointer-events-none">
                    <div class="text-2xl font-black text-white font-mono tracking-wide">
                        {move || status_text.get()}
                    </div>
                    
                    <Show when=move || test_state.get() == TestState::Waiting || test_state.get() == TestState::Ready>
                        <div class="text-xs text-white/80 font-mono">
                            {move || format!("Ensaio {} de {}", current_trial.get() + 1, total_trials.get())}
                        </div>
                    </Show>
                </div>
            </div>

            <div class="flex justify-between items-center bg-slate-900/50 p-3 rounded-xl border border-slate-800">
                <div class="flex gap-2">
                    <Show when=move || test_state.get() == TestState::Idle || test_state.get() == TestState::Completed>
                        <button 
                            class="px-5 py-2.5 bg-cyan-600 hover:bg-cyan-500 text-white font-bold text-xs rounded-lg transition-colors shadow-md shadow-cyan-950"
                            on:click=start_test
                        >
                            "Iniciar Teste"
                        </button>
                    </Show>

                    <Show when=move || test_state.get() == TestState::Waiting || test_state.get() == TestState::Ready>
                        <button 
                            class="px-4 py-2 bg-amber-600 hover:bg-amber-500 text-white font-bold text-xs rounded-lg transition-colors"
                            on:click=pause_test
                        >
                            "Pausar"
                        </button>
                    </Show>

                    <Show when=move || test_state.get() == TestState::Paused>
                        <button 
                            class="px-4 py-2 bg-emerald-600 hover:bg-emerald-500 text-white font-bold text-xs rounded-lg transition-colors"
                            on:click=resume_test
                        >
                            "Continuar"
                        </button>
                    </Show>

                    <Show when=move || test_state.get() != TestState::Idle && test_state.get() != TestState::Completed>
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
                            "Média Atual: 0 ms".to_string()
                        } else {
                            let sum: f64 = times.iter().sum();
                            format!("Média Atual: {:.1} ms", sum / times.len() as f64)
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
                            test_filter="SRT".to_string()
                        />
                    }.into_view(),
                    None => view! {
                        <ReactionChart 
                            trials=Signal::derive(move || accumulated_trials.get()) 
                            test_filter="SRT".to_string()
                        />
                    }.into_view(),
                }}
            </div>
        </div>
    }
}
