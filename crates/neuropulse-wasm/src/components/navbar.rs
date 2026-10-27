use leptos::*;

#[component]
pub fn Navbar(current_view: RwSignal<String>, is_logged_in: RwSignal<bool>) -> impl IntoView {
    let nav_items = vec![
        ("Dashboard Geral", "dashboard"),
        ("Mini-Dash SRT", "srt"),
        ("Mini-Dash CRT", "crt"),
        ("Mini-Dash Go/No-Go", "gonogo"),
        ("Protocolo PINA (rPPG)", "rppg"),
        ("Planos", "plans"),
    ];

    let handle_logout = move |_| {
        is_logged_in.set(false);
    };

    view! {
        <nav class="bg-slate-950 border-b border-slate-800 px-4 py-3 sticky top-0 z-50 font-sans shadow-lg">
            <div class="max-w-7xl mx-auto flex flex-col md:flex-row justify-between items-center gap-4">
                <div class="flex items-center gap-2 cursor-pointer" on:click=move |_| current_view.set("dashboard".to_string())>
                    <div class="w-4 h-4 rounded-full bg-cyan-400 animate-pulse"></div>
                    <span class="text-lg font-black text-white font-mono tracking-wider">"NeuroPulse SaaS"</span>
                </div>

                <div class="flex flex-wrap items-center justify-center gap-2 md:gap-3">
                    {nav_items.into_iter().map(|(label, key)| {
                        let k = key.to_string();
                        let active_k = key.to_string();
                        view! {
                            <button
                                on:click=move |_| current_view.set(active_k.clone())
                                class=move || {
                                    if current_view.get() == k {
                                        "px-3 py-1.5 rounded-xl text-xs font-bold bg-cyan-500 text-slate-950 font-mono transition-all shadow-md shadow-cyan-900/30 cursor-pointer"
                                    } else {
                                        "px-3 py-1.5 rounded-xl text-xs font-semibold text-slate-300 hover:text-white hover:bg-slate-900 font-mono transition-all cursor-pointer"
                                    }
                                }
                            >
                                {label}
                            </button>
                        }
                    }).collect_view()}

                    <button
                        on:click=handle_logout
                        class="px-4 py-1.5 rounded-xl text-xs font-bold bg-rose-600 hover:bg-rose-500 text-white font-mono transition-all shadow-lg shadow-rose-900/40 cursor-pointer ml-2 border border-rose-400">
                        "SAIR DO SISTEMA"
                    </button>
                </div>
            </div>
        </nav>
    }
}
