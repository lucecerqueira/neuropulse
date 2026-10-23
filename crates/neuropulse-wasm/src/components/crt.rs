use leptos::*;
use gloo_timers::callback::Timeout;
use rand::Rng;
use crate::types::{TrialData, SessionRecord, MOTIVATIONAL_TRAILS};
use crate::components::chart::ReactionChart;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CRTState {
    Idle,
    Running,
    Completed,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TargetColor {
    Red,
    Blue,
    Green,
}

#[component]
pub fn ChoiceReactionTest(
    #[prop(into)] on_session_complete: Callback<(f64, String)>,
    #[prop(optional)] sessions_history: Option<Signal<Vec<SessionRecord>>>,
) -> impl IntoView {
    let (test_state, set_test_state) = create_signal(CRTState::Idle);
    let (total_trials, set_total_trials) = create_signal(5usize);
    let (current_trial, set_current_trial) = create_signal(0usize);
    let (target_color, set_target_color) = create_signal(TargetColor::Red);
    let (show_stimulus, set_show_stimulus) = create_signal(false);
    let (start_time, set_start_time) = create_signal(0.0f64);

    let (accumulated_trials, set_accumulated_trials) = create_signal(Vec::<TrialData>::new());
    let (current_run_times, set_current_run_times) = create_signal(Vec::<f64>::new());
    let (selected_trail_idx, set_selected_trail_idx) = create_signal(0usize);
    let (status_text, set_status_text) = create_signal("Selecione a trilha e clique em 'Iniciar CRT'.".to_string());

    let spawn_next_target = move || {
        set_show_stimulus.set(false);
        set_status_text.set("Aguarde o próximo estímulo...".to_string());

        let mut rng = rand::thread_rng();
        let color_val = rng.gen_range(0..3);
        let color = match color_val {
            0 => TargetColor::Red,
            1 => TargetColor::Blue,
            _ => TargetColor::Green,
        };
        set_target_color.set(color);

        let delay_ms = rng.gen_range(1200..2800);
        Timeout::new(delay_ms, move || {
            if test_state.get_untracked() == CRTState::Running {
                let window = web_sys::window().unwrap();
                set_start_time.set(window.performance().unwrap().now());
                set_show_stimulus.set(true);
                set_status_text.set("CLIQUE NO BOTÃO CORRESPONDENTE À COR EXIBIDA!".to_string());
            }
        }).forget();
    };

    let start_test = move |_| {
        set_accumulated_trials.set(Vec::new());
        set_current_run_times.set(Vec::new());
        set_current_trial.set(0);
        set_test_state.set(CRTState::Running);
        spawn_next_target();
    };

    let handle_choice = move |clicked_color: TargetColor| {
        if test_state.get() == CRTState::Running && show_stimulus.get() {
            let window = web_sys::window().unwrap();
            let end_time = window.performance().unwrap().now();
            let reaction_time = end_time - start_time.get();
            let is_correct = clicked_color == target_color.get();

            let mut run_times = current_run_times.get();
            if is_correct {
                run_times.push(reaction_time);
            }
            set_current_run_times.set(run_times.clone());

            let trial_num = current_trial.get() + 1;
            set_current_trial.set(trial_num);

            let mut acc = accumulated_trials.get();
            acc.push(TrialData {
                trial_number: trial_num,
                trail_number: selected_trail_idx.get(),
                reaction_time_ms: reaction_time,
                is_correct,
                timestamp: "Agora".to_string(),
            });
            set_accumulated_trials.set(acc);

            if trial_num >= total_trials.get() {
                set_test_state.set(CRTState::Completed);
                set_show_stimulus.set(false);
                let sum: f64 = run_times.iter().sum();
                let avg = if run_times.is_empty() { 0.0 } else { sum / run_times.len() as f64 };
                let trail_name = MOTIVATIONAL_TRAILS.get(selected_trail_idx.get()).map(|t| t.name).unwrap_or("Trilha Custom");
                
                set_status_text.set(format!("Teste CRT Concluído! Média: {:.2} ms [{}]", avg, trail_name));
                on_session_complete.call((avg, trail_name.to_string()));
            } else {
                spawn_next_target();
            }
        }
    };

    let cancel_test = move |_| {
        set_test_state.set(CRTState::Idle);
        set_show_stimulus.set(false);
        set_current_run_times.set(Vec::new());
        set_current_trial.set(0);
        set_status_text.set("Teste cancelado.".to_string());
    };

    view! {
        <div class="space-y-6">
            <div class="flex flex-col md:flex-row justify-between items-start md:items-center bg-slate-900 p-4 rounded-xl border border-slate-800 gap-4">
                <div>
                    <h2 class="text-xl font-bold text-purple-400 font-mono">"Teste de Reação de Escolha (CRT) + PINA"</h2>
                    <p class="text-xs text-slate-400">"Avalia tomada de decisão e sobrecarga cognitiva com telemetria rPPG integrada."</p>
                </div>

                <div class="flex flex-wrap items-center gap-4 text-xs">
                    <div>
                        <label class="block text-[10px] text-slate-500 font-mono uppercase mb-1">"Trilha Motivacional"</label>
                        <select 
                            class="bg-slate-950 border border-slate-800 text-slate-200 rounded px-2.5 py-1.5 focus:outline-none focus:border-purple-500 font-mono text-xs"
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
                            class="bg-slate-950 border border-slate-800 text-slate-200 rounded px-2.5 py-1.5 focus:outline-none focus:border-purple-500 font-mono text-xs"
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

            <div class="h-56 bg-slate-900 border border-slate-800 rounded-xl flex flex-col items-center justify-center p-6 text-center space-y-4">
                <div class="text-xs font-mono text-slate-400">
                    {move || status_text.get()}
                </div>

                <Show when=move || test_state.get() == CRTState::Running>
                    <div class="text-xs font-mono text-cyan-400">
                        {move || format!("Ensaio {} de {}", current_trial.get() + 1, total_trials.get())}
                    </div>

                    // Estímulo Alvo Exibido
                    <div class="h-16 flex items-center justify-center">
                        {move || {
                            if show_stimulus.get() {
                                match target_color.get() {
                                    TargetColor::Red => view! {
                                        <div class="px-6 py-2 rounded-lg bg-red-600/20 border-2 border-red-500 text-red-400 font-black text-xl font-mono tracking-widest animate-pulse">
                                            "ESTÍMULO: VERMELHO"
                                        </div>
                                    }.into_view(),
                                    TargetColor::Blue => view! {
                                        <div class="px-6 py-2 rounded-lg bg-blue-600/20 border-2 border-blue-500 text-blue-400 font-black text-xl font-mono tracking-widest animate-pulse">
                                            "ESTÍMULO: AZUL"
                                        </div>
                                    }.into_view(),
                                    TargetColor::Green => view! {
                                        <div class="px-6 py-2 rounded-lg bg-emerald-600/20 border-2 border-emerald-500 text-emerald-400 font-black text-xl font-mono tracking-widest animate-pulse">
                                            "ESTÍMULO: VERDE"
                                        </div>
                                    }.into_view(),
                                }
                            } else {
                                view! {
                                    <div class="text-xs font-mono text-slate-500 animate-pulse">
                                        "..."
                                    </div>
                                }.into_view()
                            }
                        }}
                    </div>

                    // Botões de Resposta
                    <div class="flex gap-4 pt-2">
                        <button 
                            class="px-6 py-3 bg-red-600 hover:bg-red-500 text-white font-bold text-xs rounded-lg transition-transform active:scale-95 shadow-lg shadow-red-950"
                            on:click=move |_| handle_choice(TargetColor::Red)
                        >
                            "Vermelho"
                        </button>
                        <button 
                            class="px-6 py-3 bg-blue-600 hover:bg-blue-500 text-white font-bold text-xs rounded-lg transition-transform active:scale-95 shadow-lg shadow-blue-950"
                            on:click=move |_| handle_choice(TargetColor::Blue)
                        >
                            "Azul"
                        </button>
                        <button 
                            class="px-6 py-3 bg-emerald-600 hover:bg-emerald-500 text-white font-bold text-xs rounded-lg transition-transform active:scale-95 shadow-lg shadow-emerald-950"
                            on:click=move |_| handle_choice(TargetColor::Green)
                        >
                            "Verde"
                        </button>
                    </div>

                    <button 
                        class="text-[10px] text-red-400 hover:underline font-mono pt-1"
                        on:click=cancel_test
                    >
                        "Cancelar Teste"
                    </button>
                </Show>

                <Show when=move || test_state.get() == CRTState::Idle || test_state.get() == CRTState::Completed>
                    <button 
                        class="px-6 py-2.5 bg-purple-600 hover:bg-purple-500 text-white font-bold text-xs rounded-lg transition-colors shadow-md shadow-purple-950"
                        on:click=start_test
                    >
                        "Iniciar Teste CRT"
                    </button>
                </Show>
            </div>

            <div class="bg-slate-900 p-4 rounded-xl border border-slate-800">
                {move || match sessions_history {
                    Some(history) => view! {
                        <ReactionChart 
                            trials=Signal::derive(move || accumulated_trials.get()) 
                            sessions_history=history
                            test_filter="CRT".to_string()
                        />
                    }.into_view(),
                    None => view! {
                        <ReactionChart 
                            trials=Signal::derive(move || accumulated_trials.get()) 
                            test_filter="CRT".to_string()
                        />
                    }.into_view(),
                }}
            </div>
        </div>
    }
}
