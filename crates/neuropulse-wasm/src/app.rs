use leptos::*;
use crate::types::{TrialData, SessionRecord};
use crate::components::login::LoginView;
use crate::components::dashboard::DashboardView;
use crate::components::srt::SrtView;
use crate::components::crt::CrtView;
use crate::components::gonogo::GoNoGoTest;
use crate::components::plans::PlansView;

fn load_stored_trials() -> Vec<TrialData> {
    if let Some(_window) = web_sys::window() {
        if let Ok(js_val) = js_sys::eval("window.localStorage.getItem('neuropulse_trials')") {
            if let Some(json_str) = js_val.as_string() {
                if let Ok(data) = serde_json::from_str::<Vec<TrialData>>(&json_str) {
                    return data;
                }
            }
        }
    }
    vec![]
}

fn save_trials(trials: &[TrialData]) {
    if let Ok(json_str) = serde_json::to_string(trials) {
        if let Some(_) = web_sys::window() {
            let script = format!(
                "window.localStorage.setItem('neuropulse_trials', {})",
                js_sys::JSON::stringify(&wasm_bindgen::JsValue::from_str(&json_str))
                    .unwrap_or_default()
            );
            let _ = js_sys::eval(&script);
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (is_logged, set_is_logged) = create_signal(false);
    let (current_tab, set_current_tab) = create_signal("dashboard".to_string());

    let (trials, set_trials) = create_signal::<Vec<TrialData>>(load_stored_trials());
    let (sessions, _set_sessions) = create_signal::<Vec<SessionRecord>>(vec![]);

    let handle_add_trial = Callback::new(move |new_trial: TrialData| {
        set_trials.update(|t_list| {
            t_list.push(new_trial);
            save_trials(t_list);
        });
    });

    let logout = move |_| {
        set_is_logged.set(false);
    };

    view! {
        <main class="min-h-screen bg-[#060911] text-slate-100 font-mono">
            {move || {
                if !is_logged.get() {
                    view! {
                        <LoginView on_login=Callback::new(move |_| set_is_logged.set(true)) />
                    }.into_view()
                } else {
                    view! {
                        <div>
                            <header class="bg-[#090d16] border-b border-slate-800/80 px-6 py-4 flex flex-col sm:flex-row justify-between items-center gap-4 sticky top-0 z-50">
                                <div class="flex items-center gap-3">
                                    <div class="w-8 h-8 bg-cyan-500/10 border border-cyan-500/30 rounded-lg flex items-center justify-center p-1.5">
                                        <img src="/favicon.svg" alt="NeuroPulse Logo" class="w-full h-full object-contain" />
                                    </div>
                                    <span class="font-bold text-white tracking-wider text-sm">"NeuroPulse SaaS"</span>
                                </div>

                                <nav class="flex flex-wrap items-center gap-2 text-xs">
                                    <button
                                        on:click=move |_| set_current_tab.set("dashboard".to_string())
                                        class=move || format!("px-3 py-1.5 rounded-lg font-bold transition cursor-pointer {}", if current_tab.get() == "dashboard" { "bg-cyan-500 text-slate-950" } else { "text-slate-400 hover:text-slate-200" })
                                    >
                                        "Dashboard Geral"
                                    </button>
                                    <button
                                        on:click=move |_| set_current_tab.set("srt".to_string())
                                        class=move || format!("px-3 py-1.5 rounded-lg font-bold transition cursor-pointer {}", if current_tab.get() == "srt" { "bg-cyan-500 text-slate-950" } else { "text-slate-400 hover:text-slate-200" })
                                    >
                                        "Mini-Dash SRT"
                                    </button>
                                    <button
                                        on:click=move |_| set_current_tab.set("crt".to_string())
                                        class=move || format!("px-3 py-1.5 rounded-lg font-bold transition cursor-pointer {}", if current_tab.get() == "crt" { "bg-cyan-500 text-slate-950" } else { "text-slate-400 hover:text-slate-200" })
                                    >
                                        "Mini-Dash CRT"
                                    </button>
                                    <button
                                        on:click=move |_| set_current_tab.set("gonogo".to_string())
                                        class=move || format!("px-3 py-1.5 rounded-lg font-bold transition cursor-pointer {}", if current_tab.get() == "gonogo" { "bg-cyan-500 text-slate-950" } else { "text-slate-400 hover:text-slate-200" })
                                    >
                                        "Mini-Dash Go/No-Go"
                                    </button>
                                    <button
                                        on:click=move |_| set_current_tab.set("plans".to_string())
                                        class=move || format!("px-3 py-1.5 rounded-lg font-bold transition cursor-pointer {}", if current_tab.get() == "plans" { "bg-cyan-500 text-slate-950" } else { "text-slate-400 hover:text-slate-200" })
                                    >
                                        "Planos"
                                    </button>
                                </nav>

                                <div>
                                    <button
                                        on:click=logout
                                        class="px-4 py-2 bg-rose-500/10 hover:bg-rose-500/20 border border-rose-500/30 text-rose-400 font-bold text-xs rounded-lg transition cursor-pointer"
                                    >
                                        "SAIR DO SISTEMA"
                                    </button>
                                </div>
                            </header>

                            <div class="p-6 max-w-7xl mx-auto">
                                {move || match current_tab.get().as_str() {
                                    "srt" => view! { <SrtView trials=trials sessions=sessions on_add_trial=handle_add_trial /> }.into_view(),
                                    "crt" => view! { <CrtView trials=trials sessions=sessions on_add_trial=handle_add_trial /> }.into_view(),
                                    "gonogo" => view! { <GoNoGoTest trials=trials sessions=sessions on_add_trial=handle_add_trial /> }.into_view(),
                                    "plans" => view! { <PlansView /> }.into_view(),
                                    _ => view! { <DashboardView trials=trials sessions=sessions /> }.into_view(),
                                }}
                            </div>
                        </div>
                    }.into_view()
                }
            }}
        </main>
    }
}
