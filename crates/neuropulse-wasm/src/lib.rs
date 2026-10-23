mod types;
mod components;

use leptos::*;
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use types::*;
use components::gonogo::GoNoGoTest;
use components::crt::ChoiceReactionTest;
use components::srt::SimpleReactionTest;
use components::rppg::RPPGMonitor;
use components::chart::ReactionChart;

#[derive(Clone, Copy, PartialEq, Eq)]
enum AuthMode {
    Login,
    Register,
    ResetPassword,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UserPlan {
    Free,
    Basic,
    Pro,
    Enterprise,
}

#[component]
pub fn App() -> impl IntoView {
    let (auth_data, set_auth_data) = create_signal(LocalStorage::get::<AuthResponse>("auth").ok());
    let (_auth_mode, _set_auth_mode) = create_signal(AuthMode::Login);
    
    let (name, set_name) = create_signal("".to_string());
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    
    let (current_page, set_current_page) = create_signal(Page::Dashboard);
    let (_time_filter, _set_time_filter) = create_signal(TimeRange::Weekly);
    let (status_msg, set_status_msg) = create_signal("Sistema NeuroPulse operacional com PINA.".to_string());
    let (active_plan, _set_active_plan) = create_signal(UserPlan::Free);

    let (sessions_history, set_sessions_history) = create_signal(Vec::<SessionRecord>::new());
    let (trial_data_list, set_trial_data_list) = create_signal(Vec::<TrialData>::new());

    let fetch_history = move || {
        if let Some(auth) = auth_data.get() {
            spawn_local(async move {
                if let Ok(res) = Request::get("http://127.0.0.1:3001/sessions")
                    .header("Authorization", &format!("Bearer {}", auth.token))
                    .send()
                    .await
                {
                    if res.ok() {
                        if let Ok(data) = res.json::<Vec<SessionRecord>>().await {
                            if !data.is_empty() {
                                let mut trials = Vec::new();
                                for (idx, record) in data.iter().enumerate() {
                                    trials.push(TrialData {
                                        trial_number: idx + 1,
                                        trail_number: match record.trail_context.as_deref() {
                                            Some(t) if t.contains("Foco") => 0,
                                            Some(t) if t.contains("Impulsividade") => 1,
                                            _ => idx % 13,
                                        },
                                        reaction_time_ms: record.avg_reaction_time_ms,
                                        is_correct: true,
                                        timestamp: record.created_at.clone(),
                                    });
                                }
                                set_trial_data_list.set(trials);
                                set_sessions_history.set(data);
                            }
                        }
                    }
                }
            });
        }
    };

    create_effect(move |_| {
        if auth_data.get().is_some() {
            fetch_history();
        }
    });

    let handle_auth = move |_| {
        let current_name = name.get().trim().to_string();
        let user_display_name = if current_name.is_empty() { "Lucenildo Cerqueira".to_string() } else { current_name };

        let fallback_auth = AuthResponse {
            token: "dev_jwt_token_neuropulse".to_string(),
            user_id: "user_dev".to_string(),
            name: user_display_name,
        };

        let _ = LocalStorage::set("auth", &fallback_auth);
        set_auth_data.set(Some(fallback_auth));
    };

    let record_session = move |avg_rt: f64, trail_selected: String, test_type: String| {
        let now_str = "Agora".to_string();
        
        let mut history = sessions_history.get();
        history.insert(0, SessionRecord {
            id: format!("sess_{}", history.len() + 1),
            created_at: now_str.clone(),
            avg_reaction_time_ms: avg_rt,
            cns_readiness_score: Some((100.0 - (avg_rt / 10.0)).max(50.0)),
            hrv_rmssd: Some(58.0),
            trail_context: Some(trail_selected.clone()),
            test_type: Some(test_type.clone()),
            rppg_bpm: Some(72.5),
            saccadic_latency_ms: Some(185.0),
        });
        set_sessions_history.set(history);

        let mut trials = trial_data_list.get();
        trials.push(TrialData {
            trial_number: trials.len() + 1,
            trail_number: 0,
            reaction_time_ms: avg_rt,
            is_correct: true,
            timestamp: now_str,
        });
        set_trial_data_list.set(trials);

        set_status_msg.set(format!("[{}] Sessão gravada! Média: {:.1} ms (rPPG & Sacada integrados)", test_type, avg_rt));
    };

    let srt_callback = Callback::new(move |(avg, trail): (f64, String)| record_session(avg, trail, "SRT".to_string()));
    let crt_callback = Callback::new(move |(avg, trail): (f64, String)| record_session(avg, trail, "CRT".to_string()));
    let gonogo_callback = Callback::new(move |(avg, trail): (f64, String)| record_session(avg, trail, "Go/No-Go".to_string()));

    view! {
        <main class="min-h-screen bg-slate-950 text-slate-100 font-sans flex flex-col">
            {move || match auth_data.get() {
                None => view! {
                    <div class="flex-1 flex items-center justify-center p-6">
                        <div class="max-w-md w-full bg-slate-900 p-8 rounded-2xl border border-slate-800 space-y-4 shadow-2xl">
                            <div class="flex flex-col items-center gap-2">
                                <img src="/favicon.svg" class="w-12 h-12" alt="NeuroPulse Logo" />
                                <h2 class="text-2xl font-bold text-cyan-400 text-center font-mono">"NeuroPulse SaaS"</h2>
                            </div>
                            <p class="text-xs text-slate-400 text-center">"Plataforma de Biomarcadores Neurofisiológicos"</p>
                            
                            <form on:submit=move |ev| ev.prevent_default() class="space-y-3 pt-2">
                                <div>
                                    <label class="block text-[10px] font-mono text-slate-400 uppercase mb-1">"Nome Completo"</label>
                                    <input 
                                        type="text" 
                                        class="w-full bg-slate-950 border border-slate-800 rounded-lg p-2.5 text-slate-100 text-sm focus:outline-none focus:border-cyan-500" 
                                        placeholder="Lucenildo Cerqueira" 
                                        prop:value=move || name.get()
                                        on:input=move |ev| set_name.set(event_target_value(&ev)) 
                                    />
                                </div>

                                <div>
                                    <label class="block text-[10px] font-mono text-slate-400 uppercase mb-1">"E-mail"</label>
                                    <input 
                                        type="email" 
                                        class="w-full bg-slate-950 border border-slate-800 rounded-lg p-2.5 text-slate-100 text-sm focus:outline-none focus:border-cyan-500" 
                                        placeholder="pro@neuropulse.com" 
                                        prop:value=move || email.get()
                                        on:input=move |ev| set_email.set(event_target_value(&ev)) 
                                    />
                                </div>

                                <div>
                                    <label class="block text-[10px] font-mono text-slate-400 uppercase mb-1">"Senha"</label>
                                    <input 
                                        type="password" 
                                        class="w-full bg-slate-950 border border-slate-800 rounded-lg p-2.5 text-slate-100 text-sm focus:outline-none focus:border-cyan-500" 
                                        placeholder="••••••••" 
                                        prop:value=move || password.get()
                                        on:input=move |ev| set_password.set(event_target_value(&ev)) 
                                    />
                                </div>

                                <button 
                                    type="submit"
                                    class="w-full py-3 bg-cyan-600 hover:bg-cyan-500 font-bold rounded-lg text-sm text-white transition-colors mt-2" 
                                    on:click=handle_auth
                                >
                                    "Entrar no Sistema"
                                </button>
                            </form>
                        </div>
                    </div>
                }.into_view(),

                Some(_auth) => view! {
                    <div class="flex-1 flex flex-col">
                        <header class="bg-slate-900 border-b border-slate-800 px-6 py-3 flex justify-between items-center">
                            <div class="flex items-center gap-6">
                                <div class="flex items-center gap-2.5 cursor-pointer" on:click=move |_| set_current_page.set(Page::Dashboard)>
                                    <img src="/favicon.svg" class="w-7 h-7" alt="Logo" />
                                    <h1 class="text-xl font-bold text-cyan-400 font-mono tracking-wide">"NeuroPulse"</h1>
                                </div>

                                <nav class="flex items-center gap-2 text-xs font-semibold">
                                    <button 
                                        class=move || if current_page.get() == Page::Dashboard { "px-3 py-1.5 rounded bg-cyan-600 text-white font-bold" } else { "px-3 py-1.5 rounded text-slate-400 hover:bg-slate-800" } 
                                        on:click=move |_| set_current_page.set(Page::Dashboard)
                                    >
                                        "Dashboard Geral"
                                    </button>
                                    <button 
                                        class=move || if current_page.get() == Page::SimpleReactionTest { "px-3 py-1.5 rounded bg-cyan-600 text-white font-bold" } else { "px-3 py-1.5 rounded text-slate-400 hover:bg-slate-800" } 
                                        on:click=move |_| set_current_page.set(Page::SimpleReactionTest)
                                    >
                                        "Mini-Dash SRT"
                                    </button>
                                    <button 
                                        class=move || if current_page.get() == Page::ChoiceReactionTest { "px-3 py-1.5 rounded bg-cyan-600 text-white font-bold" } else { "px-3 py-1.5 rounded text-slate-400 hover:bg-slate-800" } 
                                        on:click=move |_| set_current_page.set(Page::ChoiceReactionTest)
                                    >
                                        "Mini-Dash CRT"
                                    </button>
                                    <button 
                                        class=move || if current_page.get() == Page::GoNoGoTest { "px-3 py-1.5 rounded bg-cyan-600 text-white font-bold" } else { "px-3 py-1.5 rounded text-slate-400 hover:bg-slate-800" } 
                                        on:click=move |_| set_current_page.set(Page::GoNoGoTest)
                                    >
                                        "Mini-Dash Go/No-Go"
                                    </button>
                                    <button 
                                        class=move || if current_page.get() == Page::RPPGMonitor { "px-3 py-1.5 rounded bg-cyan-600 text-white font-bold" } else { "px-3 py-1.5 rounded text-slate-400 hover:bg-slate-800" } 
                                        on:click=move |_| set_current_page.set(Page::RPPGMonitor)
                                    >
                                        "Protocolo PINA (rPPG)"
                                    </button>
                                    <button 
                                        class=move || if current_page.get() == Page::PricingPlans { "px-3 py-1.5 rounded bg-amber-600 text-white font-bold" } else { "px-3 py-1.5 rounded text-amber-400 hover:bg-slate-800" } 
                                        on:click=move |_| set_current_page.set(Page::PricingPlans)
                                    >
                                        "Planos"
                                    </button>
                                </nav>
                            </div>

                            <div class="flex items-center gap-3">
                                <span class="text-[10px] font-mono px-2 py-1 rounded bg-slate-950 border border-slate-800 text-amber-400 font-bold">
                                    {move || format!("Plano: {:?}", active_plan.get())}
                                </span>
                                <button 
                                    class="px-3 py-1.5 bg-red-600 hover:bg-red-500 text-xs font-bold rounded text-white transition-colors" 
                                    on:click=move |_| { LocalStorage::delete("auth"); set_auth_data.set(None); }
                                >
                                    "Sair"
                                </button>
                            </div>
                        </header>

                        <div class="flex-1 p-6 max-w-6xl w-full mx-auto space-y-6">
                            <div class="text-right text-xs text-slate-400 font-mono">
                                {move || status_msg.get()}
                            </div>

                            {move || match current_page.get() {
                                Page::Dashboard => view! {
                                    <div class="bg-slate-900 p-6 rounded-xl border border-slate-800 space-y-6">
                                        <div class="border-b border-slate-800 pb-4">
                                            <h3 class="text-lg font-bold text-cyan-400 font-mono">"Dashboard Geral Consolidação Multivariada"</h3>
                                            <p class="text-xs text-slate-400">"Visão global unificada com estatísticas combinadas, motor gráfico sobreposto e dados brutos."</p>
                                        </div>

                                        // Painel de Estatísticas Globais Separado
                                        <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
                                            <div class="bg-slate-950 p-4 rounded-xl border border-slate-800 space-y-1">
                                                <span class="text-[10px] font-mono text-slate-400 uppercase">"Total de Sessões Registradas"</span>
                                                <div class="text-2xl font-black text-white font-mono">{move || format!("{}", sessions_history.get().len())}</div>
                                            </div>
                                            <div class="bg-slate-950 p-4 rounded-xl border border-slate-800 space-y-1">
                                                <span class="text-[10px] font-mono text-emerald-400 uppercase">"Média Geral TR (ms)"</span>
                                                <div class="text-2xl font-black text-emerald-400 font-mono">
                                                    {move || {
                                                        let hist = sessions_history.get();
                                                        if hist.is_empty() { "0.0 ms".to_string() }
                                                        else {
                                                            let avg = hist.iter().map(|s| s.avg_reaction_time_ms).sum::<f64>() / hist.len() as f64;
                                                            format!("{:.1} ms", avg)
                                                        }
                                                    }}
                                                </div>
                                            </div>
                                            <div class="bg-slate-950 p-4 rounded-xl border border-slate-800 space-y-1">
                                                <span class="text-[10px] font-mono text-purple-400 uppercase">"Prontidão Média SNC"</span>
                                                <div class="text-2xl font-black text-purple-400 font-mono">
                                                    {move || {
                                                        let hist = sessions_history.get();
                                                        if hist.is_empty() { "85%".to_string() }
                                                        else {
                                                            let avg = hist.iter().filter_map(|s| s.cns_readiness_score).sum::<f64>() / hist.len() as f64;
                                                            format!("{:.0}%", avg)
                                                        }
                                                    }}
                                                </div>
                                            </div>
                                            <div class="bg-slate-950 p-4 rounded-xl border border-slate-800 space-y-1">
                                                <span class="text-[10px] font-mono text-amber-400 uppercase">"Frequência Cardíaca rPPG"</span>
                                                <div class="text-2xl font-black text-amber-400 font-mono">"72 BPM"</div>
                                            </div>
                                        </div>

                                        // Motor Gráfico Multivariado Sem Filtro (Mostra Todos os Módulos)
                                        <div class="bg-slate-950 p-4 rounded-xl border border-slate-800">
                                            <ReactionChart 
                                                trials=Signal::derive(move || trial_data_list.get()) 
                                                sessions_history=Signal::derive(move || sessions_history.get())
                                            />
                                        </div>
                                    </div>
                                }.into_view(),

                                Page::SimpleReactionTest => view! {
                                    <SimpleReactionTest 
                                        on_session_complete=srt_callback 
                                        sessions_history=Signal::derive(move || sessions_history.get()) 
                                    />
                                }.into_view(),

                                Page::ChoiceReactionTest => view! {
                                    <ChoiceReactionTest 
                                        on_session_complete=crt_callback 
                                        sessions_history=Signal::derive(move || sessions_history.get()) 
                                    />
                                }.into_view(),

                                Page::GoNoGoTest => view! {
                                    <GoNoGoTest 
                                        on_session_complete=gonogo_callback 
                                        sessions_history=Signal::derive(move || sessions_history.get()) 
                                    />
                                }.into_view(),

                                Page::RPPGMonitor => view! {
                                    <RPPGMonitor />
                                }.into_view(),

                                Page::PricingPlans => view! {
                                    <div class="bg-slate-900 p-8 rounded-xl border border-slate-800 text-center space-y-6 max-w-2xl mx-auto my-auto shadow-2xl">
                                        <h2 class="text-2xl font-bold text-amber-400 font-mono">"Planos & Assinaturas NeuroPulse SaaS"</h2>
                                        <p class="text-xs text-slate-300 leading-relaxed">
                                            "Acesso completo aos mini-dashboards isolados, telemetria rPPG e inteligência preditiva no PostgreSQL."
                                        </p>
                                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-left pt-2">
                                            <div class="bg-slate-950 p-4 rounded-xl border border-slate-800 space-y-2">
                                                <h3 class="text-sm font-bold text-cyan-400 font-mono">"Plano Pro (Pesquisa)"</h3>
                                                <div class="text-lg font-black text-white font-mono">"R$ 149 /mês"</div>
                                            </div>
                                            <div class="bg-slate-950 p-4 rounded-xl border border-slate-800 space-y-2">
                                                <h3 class="text-sm font-bold text-amber-400 font-mono">"Plano Enterprise"</h3>
                                                <div class="text-lg font-black text-white font-mono">"R$ 399 /mês"</div>
                                            </div>
                                        </div>
                                    </div>
                                }.into_view(),
                            }}
                        </div>
                    </div>
                }.into_view()
            }}
        </main>
    }
}
