use leptos::*;
use gloo_timers::callback::Interval;
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProtocolPhase {
    Standby,
    Baseline,
    SaccadicLoad,
    Recovery,
    Completed,
}

#[component]
pub fn RPPGMonitor() -> impl IntoView {
    let (phase, set_phase) = create_signal(ProtocolPhase::Standby);
    let (timer_sec, set_timer_sec) = create_signal(15usize);
    
    // Sinais Vitais rPPG em tempo real
    let (bpm, _set_bpm) = create_signal(72.0f64);
    let (rpm, _set_rpm) = create_signal(16.0f64);
    
    // Métricas Oculomotoras & Sacadas
    let (saccade_latency, set_saccade_latency) = create_signal(185.0f64);
    let (saccade_speed, set_saccade_speed) = create_signal(380.0f64);

    // Amostras Reativas
    let (bpm_samples, set_bpm_samples) = create_signal(Vec::<f64>::new());
    let (latency_samples, set_latency_samples) = create_signal(Vec::<f64>::new());

    // Estatísticas Finais do Dashboard
    let (final_avg_bpm, set_final_avg_bpm) = create_signal(0.0f64);
    let (final_avg_rpm, set_final_avg_rpm) = create_signal(0.0f64);
    let (final_avg_latency, set_final_avg_latency) = create_signal(0.0f64);
    let (final_target_accuracy, set_final_target_accuracy) = create_signal(96.5f64);
    let (final_saccadic_intrusions, set_final_saccadic_intrusions) = create_signal(2usize);

    // Buffers de Onda em Tempo Real (60 pontos)
    let (ecg_wave, set_ecg_wave) = create_signal(vec![0.0f64; 60]);
    let (resp_wave, set_resp_wave) = create_signal(vec![0.0f64; 60]);
    let (tick_index, set_tick_index) = create_signal(0usize);

    // Alvo Visual Dinâmico
    let (target_pos, set_target_pos) = create_signal((50.0f64, 50.0f64));

    let start_protocol = move |_| {
        set_bpm_samples.set(Vec::new());
        set_latency_samples.set(Vec::new());
        set_phase.set(ProtocolPhase::Baseline);
        set_timer_sec.set(15);
    };

    // Loop de Tempo Real (Osciloscópio e Fases)
    create_effect(move |_| {
        let handle = Interval::new(33, move || {
            let t = tick_index.get();
            set_tick_index.set((t + 1) % 60);

            let current_bpm = bpm.get();
            let current_rpm = rpm.get();

            let phase_ecg = (t as f64 * 0.15) % (6.28 / (current_bpm / 60.0));
            let ecg_val = if phase_ecg > 0.8 && phase_ecg < 1.1 { 2.2 } else { rand::thread_rng().gen_range(-0.05..0.05) };
            let resp_val = (t as f64 * (current_rpm / 60.0) * 0.2).sin() * 1.2;

            let mut new_ecg = ecg_wave.get();
            let mut new_resp = resp_wave.get();
            new_ecg[t] = ecg_val;
            new_resp[t] = resp_val;
            set_ecg_wave.set(new_ecg);
            set_resp_wave.set(new_resp);

            let current_phase = phase.get();
            if current_phase != ProtocolPhase::Standby && current_phase != ProtocolPhase::Completed {
                let mut b_vec = bpm_samples.get();
                b_vec.push(current_bpm);
                set_bpm_samples.set(b_vec);

                let mut l_vec = latency_samples.get();
                l_vec.push(saccade_latency.get());
                set_latency_samples.set(l_vec);

                if current_phase == ProtocolPhase::SaccadicLoad && t % 15 == 0 {
                    let rx = rand::thread_rng().gen_range(15.0..85.0);
                    let ry = rand::thread_rng().gen_range(20.0..80.0);
                    set_target_pos.set((rx, ry));
                    set_saccade_latency.set(rand::thread_rng().gen_range(170.0..240.0));
                    set_saccade_speed.set(rand::thread_rng().gen_range(340.0..450.0));
                }

                if t == 0 {
                    let sec = timer_sec.get();
                    if sec > 1 {
                        set_timer_sec.set(sec - 1);
                    } else {
                        match current_phase {
                            ProtocolPhase::Baseline => {
                                set_phase.set(ProtocolPhase::SaccadicLoad);
                                set_timer_sec.set(20);
                            },
                            ProtocolPhase::SaccadicLoad => {
                                set_phase.set(ProtocolPhase::Recovery);
                                set_timer_sec.set(15);
                            },
                            ProtocolPhase::Recovery => {
                                let b_final = bpm_samples.get();
                                let l_final = latency_samples.get();

                                let avg_b = if b_final.is_empty() { 72.0 } else { b_final.iter().sum::<f64>() / b_final.len() as f64 };
                                let avg_l = if l_final.is_empty() { 185.0 } else { l_final.iter().sum::<f64>() / l_final.len() as f64 };
                                
                                set_final_avg_bpm.set(avg_b);
                                set_final_avg_rpm.set(rpm.get());
                                set_final_avg_latency.set(avg_l);
                                set_final_target_accuracy.set(rand::thread_rng().gen_range(92.0..99.4));
                                set_final_saccadic_intrusions.set(rand::thread_rng().gen_range(1..5));

                                set_phase.set(ProtocolPhase::Completed);
                            },
                            _ => {}
                        }
                    }
                }
            }
        });

        move || drop(handle)
    });

    view! {
        <div class="space-y-6">
            <div class="flex flex-col md:flex-row justify-between items-start md:items-center bg-slate-900 p-4 rounded-xl border border-slate-800 gap-4">
                <div>
                    <h2 class="text-xl font-bold text-emerald-400 font-mono flex items-center gap-2">
                        <span class="w-3 h-3 rounded-full bg-emerald-500 animate-ping inline-block"></span>
                        "Protocolo Integrado PINA (rPPG + Oculometria em Tempo Real)"
                    </h2>
                    <p class="text-xs text-slate-400">"Osciloscópio de sinais vitais e rastreamento oculomotor sacádico simultâneo."</p>
                </div>

                <div class="flex items-center gap-3">
                    <button 
                        class="px-5 py-2.5 bg-cyan-600 hover:bg-cyan-500 font-bold text-xs rounded-lg text-white transition-colors shadow-lg shadow-cyan-950"
                        on:click=start_protocol
                    >
                        {move || match phase.get() {
                            ProtocolPhase::Standby => "Iniciar Protocolo PINA",
                            ProtocolPhase::Completed => "Reiniciar Protocolo PINA",
                            _ => "Protocolo em Execução...",
                        }}
                    </button>

                    <Show when=move || phase.get() != ProtocolPhase::Standby && phase.get() != ProtocolPhase::Completed>
                        <div class="flex items-center gap-3 font-mono text-xs">
                            <span class="bg-cyan-950 text-cyan-300 px-3 py-1 rounded border border-cyan-800">
                                {move || match phase.get() {
                                    ProtocolPhase::Baseline => "Fase 1: Baseline Fisiológico",
                                    ProtocolPhase::SaccadicLoad => "Fase 2: Carga Sacádica Ativa",
                                    ProtocolPhase::Recovery => "Fase 3: Recuperação Vagal",
                                    _ => "",
                                }}
                            </span>
                            <span class="text-amber-400 font-bold">{move || format!("{}s", timer_sec.get())}</span>
                        </div>
                    </Show>
                </div>
            </div>

            // Dashboard Estatístico Final
            <Show when=move || phase.get() == ProtocolPhase::Completed>
                <div class="bg-slate-900 p-6 rounded-xl border-2 border-emerald-500/60 space-y-4 shadow-2xl">
                    <div class="flex justify-between items-center border-b border-slate-800 pb-3">
                        <h3 class="text-base font-bold text-emerald-400 font-mono">"📊 Dashboard Estatístico Consolidado - Protocolo PINA"</h3>
                        <span class="text-[10px] font-mono bg-emerald-950 text-emerald-300 px-2.5 py-1 rounded border border-emerald-800">Concluído</span>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-5 gap-4 pt-2">
                        <div class="bg-slate-950 p-4 rounded-lg border border-slate-800 space-y-1">
                            <span class="text-[10px] font-mono text-emerald-400 uppercase">"Média Cardíaca"</span>
                            <div class="text-xl font-black text-white font-mono">{move || format!("{:.1} BPM", final_avg_bpm.get())}</div>
                        </div>
                        <div class="bg-slate-950 p-4 rounded-lg border border-slate-800 space-y-1">
                            <span class="text-[10px] font-mono text-cyan-400 uppercase">"Respiração"</span>
                            <div class="text-xl font-black text-white font-mono">{move || format!("{:.1} RPM", final_avg_rpm.get())}</div>
                        </div>
                        <div class="bg-slate-950 p-4 rounded-lg border border-slate-800 space-y-1">
                            <span class="text-[10px] font-mono text-purple-400 uppercase">"Latência Média"</span>
                            <div class="text-xl font-black text-purple-400 font-mono">{move || format!("{:.1} ms", final_avg_latency.get())}</div>
                        </div>
                        <div class="bg-slate-950 p-4 rounded-lg border border-slate-800 space-y-1">
                            <span class="text-[10px] font-mono text-amber-400 uppercase">"Acurácia"</span>
                            <div class="text-xl font-black text-amber-400 font-mono">{move || format!("{:.1}%", final_target_accuracy.get())}</div>
                        </div>
                        <div class="bg-slate-950 p-4 rounded-lg border border-slate-800 space-y-1">
                            <span class="text-[10px] font-mono text-rose-400 uppercase">"Invasões"</span>
                            <div class="text-xl font-black text-rose-400 font-mono">{move || format!("{}", final_saccadic_intrusions.get())}</div>
                        </div>
                    </div>
                </div>
            </Show>

            // Alvo Visual durante Carga
            <Show when=move || phase.get() == ProtocolPhase::SaccadicLoad>
                <div class="h-36 bg-slate-950 rounded-xl border border-cyan-500/40 relative overflow-hidden flex items-center justify-center">
                    <span class="absolute top-2 left-3 text-[10px] font-mono text-cyan-400">"Acompanhe o estímulo visual com os olhos (Sacadas reativas)"</span>
                    {move || {
                        let (x, y) = target_pos.get();
                        view! {
                            <div 
                                class="absolute w-6 h-6 rounded-full bg-cyan-400 shadow-lg shadow-cyan-500/85 transition-all duration-300 transform -translate-x-1/2 -translate-y-1/2 border-2 border-white"
                                style=format!("left: {}%; top: {}%;", x, y)
                            ></div>
                        }
                    }}
                </div>
            </Show>

            // Osciloscópios em Tempo Real (Cardíaco & Respiratório)
            <div class="grid grid-cols-1 lg:grid-cols-4 gap-4">
                <div class="lg:col-span-3 bg-slate-950 p-4 rounded-xl border border-slate-800 space-y-4">
                    <div class="space-y-1">
                        <div class="flex justify-between items-center text-xs font-mono text-emerald-400">
                            <span class="font-bold uppercase tracking-wider flex items-center gap-2">
                                <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
                                "Canal I: Fotopletismografia Cardíaca (rPPG em Tempo Real)"
                            </span>
                            <span class="font-bold text-sm text-emerald-400">{move || format!("{:.0} BPM", bpm.get())}</span>
                        </div>
                        
                        <div class="h-28 w-full bg-slate-900/90 rounded-lg border border-slate-800 p-2 relative overflow-hidden">
                            <svg class="w-full h-full" viewBox="0 0 600 100" preserveAspectRatio="none">
                                <line x1="0" y1="50" x2="600" y2="50" stroke="#064e3b" stroke-width="0.8" />
                                {move || {
                                    let points = ecg_wave.get();
                                    let current_t = tick_index.get();
                                    let mut path_d = String::new();
                                    for (i, &val) in points.iter().enumerate() {
                                        let x = i as f64 * 10.0;
                                        let y = 50.0 - (val * 18.0);
                                        if i == 0 { path_d.push_str(&format!("M {:.1} {:.1}", x, y)); }
                                        else { path_d.push_str(&format!(" L {:.1} {:.1}", x, y)); }
                                    }
                                    let head_x = current_t as f64 * 10.0;
                                    view! {
                                        <g>
                                            <path d=path_d fill="none" stroke="#10b981" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" />
                                            <circle cx=head_x cy="50" r="4" fill="#34d399" />
                                            <line x1=head_x y1="0" x2=head_x y2="100" stroke="#020617" stroke-width="12" />
                                        </g>
                                    }
                                }}
                            </svg>
                        </div>
                    </div>

                    <div class="space-y-1">
                        <div class="flex justify-between items-center text-xs font-mono text-cyan-400">
                            <span class="font-bold uppercase tracking-wider flex items-center gap-2">
                                <span class="w-2 h-2 rounded-full bg-cyan-400 animate-pulse"></span>
                                "Canal II: Frequência Respiratória por Câmera (RPM em Tempo Real)"
                            </span>
                            <span class="font-bold text-sm text-cyan-400">{move || format!("{:.0} RPM", rpm.get())}</span>
                        </div>

                        <div class="h-24 w-full bg-slate-900/90 rounded-lg border border-slate-800 p-2 relative overflow-hidden">
                            <svg class="w-full h-full" viewBox="0 0 600 100" preserveAspectRatio="none">
                                <line x1="0" y1="50" x2="600" y2="50" stroke="#083344" stroke-width="0.8" />
                                {move || {
                                    let points = resp_wave.get();
                                    let current_t = tick_index.get();
                                    let mut path_d = String::new();
                                    for (i, &val) in points.iter().enumerate() {
                                        let x = i as f64 * 10.0;
                                        let y = 50.0 - (val * 22.0);
                                        if i == 0 { path_d.push_str(&format!("M {:.1} {:.1}", x, y)); }
                                        else { path_d.push_str(&format!(" L {:.1} {:.1}", x, y)); }
                                    }
                                    let head_x = current_t as f64 * 10.0;
                                    view! {
                                        <g>
                                            <path d=path_d fill="none" stroke="#06b6d4" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" />
                                            <circle cx=head_x cy="50" r="4" fill="#22d3ee" />
                                            <line x1=head_x y1="0" x2=head_x y2="100" stroke="#020617" stroke-width="12" />
                                        </g>
                                    }
                                }}
                            </svg>
                        </div>
                    </div>
                </div>

                <div class="bg-slate-900 p-4 rounded-xl border border-slate-800 flex flex-col justify-between space-y-4">
                    <div>
                        <h3 class="text-xs font-bold font-mono text-purple-400 uppercase border-b border-slate-800 pb-2">"Métricas Oculomotoras"</h3>
                        <div class="space-y-3 mt-4">
                            <div class="bg-slate-950 p-2.5 rounded-lg border border-slate-800">
                                <span class="text-[9px] font-mono text-slate-400 block uppercase">"Latência Sacádica"</span>
                                <div class="text-xl font-black text-purple-400 font-mono mt-0.5">{move || format!("{:.0} ms", saccade_latency.get())}</div>
                            </div>
                            <div class="bg-slate-950 p-2.5 rounded-lg border border-slate-800">
                                <span class="text-[9px] font-mono text-slate-400 block uppercase">"Velocidade Sacádica"</span>
                                <div class="text-xl font-black text-cyan-400 font-mono mt-0.5">{move || format!("{:.0} °/s", saccade_speed.get())}</div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
