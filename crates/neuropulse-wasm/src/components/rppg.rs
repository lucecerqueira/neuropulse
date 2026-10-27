use leptos::*;

#[derive(Clone, Copy, PartialEq)]
pub enum RPPGState {
    Idle,
    Running,
    Paused,
}

#[component]
pub fn RPPGMonitor() -> impl IntoView {
    let (rppg_state, set_rppg_state) = create_signal(RPPGState::Idle);
    let (bpm_value, set_bpm_value) = create_signal(72.0f64);
    let (_signal_quality, set_signal_quality) = create_signal(98.4f64);
    let (snc_readiness, set_snc_readiness) = create_signal(88.0f64);
    let (_lf_hf_balance, set_lf_hf_balance) = create_signal(1.25f64);
    let (resp_rate, set_resp_rate) = create_signal(16.5f64);
    let (saccade_count, set_saccade_count) = create_signal(4u32);

    let (wave_points, set_wave_points) = create_signal(vec![50.0; 45]);
    let (bpm_history, set_bpm_history) = create_signal(vec![72.0; 40]);
    let (resp_history, set_resp_history) = create_signal(vec![50.0; 40]);
    let (saccade_history, set_saccade_history) = create_signal(vec![10.0; 40]);
    let (_psd_points, _set_psd_points) = create_signal(vec![25.0, 50.0, 85.0, 60.0, 35.0, 20.0, 10.0]);

    let start_rppg = move |_| {
        set_rppg_state.set(RPPGState::Running);

        let state_chk = rppg_state;
        let set_wave = set_wave_points;
        let set_bpm = set_bpm_value;
        let set_qual = set_signal_quality;
        let set_ready = set_snc_readiness;
        let set_lfhf = set_lf_hf_balance;
        let set_resp = set_resp_rate;
        let set_sacc = set_saccade_count;
        let set_bpm_h = set_bpm_history;
        let set_resp_h = set_resp_history;
        let set_sacc_h = set_saccade_history;

        set_interval_with_handle(
            move || {
                if state_chk.get() == RPPGState::Running {
                    let time_sec = js_sys::Date::now() / 200.0;
                    
                    let new_val = 50.0 + 25.0 * (time_sec.sin()) + (js_sys::Math::random() * 4.0 - 2.0);
                    set_wave.update(|pts| {
                        pts.push(new_val);
                        if pts.len() > 45 { pts.remove(0); }
                    });

                    let new_bpm = 72.0 + (time_sec * 0.3).cos() * 3.0 + (js_sys::Math::random() * 2.0 - 1.0);
                    let rounded_bpm = ((new_bpm as f64) * 10.0).round() / 10.0;
                    set_bpm.set(rounded_bpm);
                    set_bpm_h.update(|pts| {
                        pts.push(rounded_bpm - 50.0);
                        if pts.len() > 40 { pts.remove(0); }
                    });

                    let new_resp = 50.0 + 30.0 * (time_sec * 0.4).sin();
                    set_resp_h.update(|pts| {
                        pts.push(new_resp);
                        if pts.len() > 40 { pts.remove(0); }
                    });
                    set_resp.set(15.2 + (time_sec * 0.1).sin() * 2.0);

                    let sacc = if js_sys::Math::random() > 0.75 { 80.0 } else { 15.0 + js_sys::Math::random() * 10.0 };
                    set_sacc_h.update(|pts| {
                        pts.push(sacc);
                        if pts.len() > 40 { pts.remove(0); }
                    });
                    if sacc > 70.0 {
                        set_sacc.update(|c| *c += 1);
                    }

                    set_qual.set(98.5);
                    set_ready.set(89.0);
                    set_lfhf.set(1.28);
                }
            },
            std::time::Duration::from_millis(100),
        ).ok();
    };

    let pause_rppg = move |_| set_rppg_state.set(RPPGState::Paused);
    let resume_rppg = move |_| set_rppg_state.set(RPPGState::Running);
    let cancel_rppg = move |_| set_rppg_state.set(RPPGState::Idle);

    view! {
        <div class="flex flex-col gap-4 p-4 md:p-6 max-w-7xl mx-auto font-sans w-full box-border">
            <div class="bg-slate-900/90 border border-slate-800 rounded-2xl p-4 md:p-6 shadow-xl w-full">
                <div class="flex flex-col lg:flex-row justify-between items-start lg:items-center gap-4 pb-6 border-b border-slate-800">
                    <div>
                        <h2 class="text-lg md:text-xl font-bold text-cyan-400 font-mono">"Protocolo PINA (rPPG e Fisiologia)"</h2>
                        <p class="text-xs text-slate-400 font-mono mt-1">"Monitorização multissistêmica responsiva para dispositivos móveis e desktops"</p>
                    </div>

                    <div class="flex items-center justify-between lg:justify-end w-full lg:w-auto gap-6">
                        <div class="text-left lg:text-right">
                            <span class="text-xs text-slate-400 font-mono block">"Frequência Cardíaca"</span>
                            <span class="text-2xl md:text-3xl font-black text-cyan-400 font-mono">{move || format!("{:.1} BPM", bpm_value.get())}</span>
                        </div>
                        <div class="w-3 h-3 rounded-full animate-ping bg-cyan-400"></div>
                    </div>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mt-6">
                    <div class="lg:col-span-2 flex flex-col gap-4 w-full">
                        <div class="bg-slate-950 border border-slate-800 rounded-xl p-4 flex flex-col justify-between w-full overflow-hidden">
                            <div class="flex justify-between items-center mb-2">
                                <span class="text-xs text-slate-400 font-mono">"Sinal Pletismográfico (rPPG)"</span>
                                <span class="text-[10px] px-2 py-0.5 bg-cyan-950 text-cyan-300 rounded border border-cyan-800 font-mono">
                                    {move || match rppg_state.get() {
                                        RPPGState::Idle => "Standby",
                                        RPPGState::Running => "Ao Vivo",
                                        RPPGState::Paused => "Pausado",
                                    }}
                                </span>
                            </div>
                            <div class="w-full h-24 md:h-28 flex items-center justify-center relative bg-slate-900/50 rounded-lg p-1">
                                <svg class="w-full h-full overflow-visible" viewBox="0 0 400 100" preserveAspectRatio="none">
                                    <polyline
                                        fill="none"
                                        stroke="#22d3ee"
                                        stroke-width="2.5"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        points=move || {
                                            let pts = wave_points.get();
                                            let step = 400.0 / (pts.len().max(1) as f64);
                                            pts.iter().enumerate().map(|(i, &y)| {
                                                format!("{},{}", (i as f64) * step, 100.0 - y)
                                            }).collect::<Vec<_>>().join(" ")
                                        }
                                    />
                                </svg>
                            </div>
                        </div>

                        <div class="bg-slate-950 border border-slate-800 rounded-xl p-4 flex flex-col justify-between w-full overflow-hidden">
                            <div class="flex justify-between items-center mb-2">
                                <span class="text-xs text-slate-400 font-mono">"Histórico de Frequência Cardíaca"</span>
                                <span class="text-xs font-bold text-cyan-300 font-mono">{move || format!("{:.1} BPM", bpm_value.get())}</span>
                            </div>
                            <div class="w-full h-20 md:h-24 flex items-center justify-center relative bg-slate-900/50 rounded-lg p-1">
                                <svg class="w-full h-full overflow-visible" viewBox="0 0 400 100" preserveAspectRatio="none">
                                    <polyline
                                        fill="none"
                                        stroke="#38bdf8"
                                        stroke-width="2.5"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        points=move || {
                                            let pts = bpm_history.get();
                                            let step = 400.0 / (pts.len().max(1) as f64);
                                            pts.iter().enumerate().map(|(i, &y)| {
                                                format!("{},{}", (i as f64) * step, 90.0 - (y * 1.5))
                                            }).collect::<Vec<_>>().join(" ")
                                        }
                                    />
                                </svg>
                            </div>
                        </div>

                        <div class="bg-slate-950 border border-slate-800 rounded-xl p-4 flex flex-col justify-between w-full overflow-hidden">
                            <div class="flex justify-between items-center mb-2">
                                <span class="text-xs text-slate-400 font-mono">"Curva Respiratória"</span>
                                <span class="text-xs font-bold text-emerald-400 font-mono">{move || format!("{:.1} rpm", resp_rate.get())}</span>
                            </div>
                            <div class="w-full h-20 md:h-24 flex items-center justify-center relative bg-slate-900/50 rounded-lg p-1">
                                <svg class="w-full h-full overflow-visible" viewBox="0 0 400 100" preserveAspectRatio="none">
                                    <polyline
                                        fill="none"
                                        stroke="#34d399"
                                        stroke-width="2.5"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        points=move || {
                                            let pts = resp_history.get();
                                            let step = 400.0 / (pts.len().max(1) as f64);
                                            pts.iter().enumerate().map(|(i, &y)| {
                                                format!("{},{}", (i as f64) * step, 100.0 - y)
                                            }).collect::<Vec<_>>().join(" ")
                                        }
                                    />
                                </svg>
                            </div>
                        </div>

                        <div class="bg-slate-950 border border-slate-800 rounded-xl p-4 flex flex-col justify-between w-full overflow-hidden">
                            <div class="flex justify-between items-center mb-2">
                                <span class="text-xs text-slate-400 font-mono">"Movimentos Sacádicos e Microssacadas"</span>
                                <span class="text-xs font-bold text-amber-400 font-mono">{move || format!("{} detecções", saccade_count.get())}</span>
                            </div>
                            <div class="w-full h-20 md:h-24 flex items-center justify-center relative bg-slate-900/50 rounded-lg p-1">
                                <svg class="w-full h-full overflow-visible" viewBox="0 0 400 100" preserveAspectRatio="none">
                                    <polyline
                                        fill="none"
                                        stroke="#fbbf24"
                                        stroke-width="2.5"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        points=move || {
                                            let pts = saccade_history.get();
                                            let step = 400.0 / (pts.len().max(1) as f64);
                                            pts.iter().enumerate().map(|(i, &y)| {
                                                format!("{},{}", (i as f64) * step, 100.0 - y)
                                            }).collect::<Vec<_>>().join(" ")
                                        }
                                    />
                                </svg>
                            </div>
                        </div>

                        <div class="flex flex-col sm:flex-row gap-3 pt-2 w-full">
                            {move || match rppg_state.get() {
                                RPPGState::Idle => view! {
                                    <button
                                        on:click=start_rppg
                                        class="w-full py-3 bg-cyan-500 hover:bg-cyan-400 text-slate-950 font-bold rounded-xl text-xs transition-all font-mono shadow-lg shadow-cyan-900/30 cursor-pointer">
                                        "Ativar Leitura PINA Completa"
                                    </button>
                                }.into_view(),
                                RPPGState::Running => view! {
                                    <button
                                        on:click=pause_rppg
                                        class="w-full sm:flex-1 py-3 bg-amber-600 hover:bg-amber-500 text-white font-bold rounded-xl text-xs transition-all font-mono cursor-pointer">
                                        "Pausar"
                                    </button>
                                    <button
                                        on:click=cancel_rppg
                                        class="w-full sm:flex-1 py-3 bg-rose-600 hover:bg-rose-500 text-white font-bold rounded-xl text-xs transition-all font-mono cursor-pointer">
                                        "Cancelar"
                                    </button>
                                }.into_view(),
                                RPPGState::Paused => view! {
                                    <button
                                        on:click=resume_rppg
                                        class="w-full sm:flex-1 py-3 bg-emerald-600 hover:bg-emerald-500 text-white font-bold rounded-xl text-xs transition-all font-mono cursor-pointer">
                                        "Continuar"
                                    </button>
                                    <button
                                        on:click=cancel_rppg
                                        class="w-full sm:flex-1 py-3 bg-rose-600 hover:bg-rose-500 text-white font-bold rounded-xl text-xs transition-all font-mono cursor-pointer">
                                        "Cancelar"
                                    </button>
                                }.into_view(),
                            }}
                        </div>
                    </div>

                    <div class="bg-slate-950 border border-slate-800 rounded-xl p-4 md:p-5 flex flex-col justify-between font-mono w-full">
                        <h3 class="text-sm font-bold text-cyan-400 mb-4">"Métricas Integradas"</h3>

                        <div class="space-y-4">
                            <div class="p-3 bg-slate-900 border border-slate-800 rounded-xl">
                                <span class="text-xs text-slate-400 block">"Status do Sensor:"</span>
                                <span class="text-xs font-bold text-emerald-400">
                                    {move || if rppg_state.get() == RPPGState::Running { "Ativo em Mobile/Desktop" } else { "Standby" }}
                                </span>
                            </div>

                            <div class="p-3 bg-slate-900 border border-slate-800 rounded-xl">
                                <span class="text-xs text-slate-400 block">"Frequência Respiratória:"</span>
                                <span class="text-lg font-bold text-emerald-400">
                                    {move || format!("{:.1} rpm", resp_rate.get())}
                                </span>
                            </div>

                            <div class="p-3 bg-slate-900 border border-slate-800 rounded-xl">
                                <span class="text-xs text-slate-400 block">"Microssacadas:"</span>
                                <span class="text-lg font-bold text-amber-400">
                                    {move || format!("{} eventos", saccade_count.get())}
                                </span>
                            </div>

                            <div class="p-3 bg-slate-900 border border-slate-800 rounded-xl">
                                <span class="text-xs text-slate-400 block">"Prontidão do SNC:"</span>
                                <span class="text-lg font-bold text-cyan-300">
                                    {move || format!("{:.1}%", snc_readiness.get())}
                                </span>
                            </div>
                        </div>

                        <div class="mt-6 text-[10px] text-slate-500 text-center">
                            "Design responsivo otimizado para celulares e tablets."
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

pub use RPPGMonitor as RppgView;
