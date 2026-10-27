use leptos::*;

#[component]
pub fn AuthView(is_logged_in: RwSignal<bool>) -> impl IntoView {
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

    let handle_login = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        if !email.get().is_empty() {
            is_logged_in.set(true);
        }
    };

    view! {
        <div class="flex items-center justify-center min-h-[80vh] px-4 font-sans">
            <div class="bg-slate-900/90 border border-slate-800 rounded-2xl p-8 shadow-2xl w-full max-w-md font-mono">
                <div class="text-center mb-8">
                    <div class="w-12 h-12 rounded-full bg-cyan-400 mx-auto mb-3 animate-pulse flex items-center justify-center text-slate-950 font-black text-xl">"NP"</div>
                    <h2 class="text-2xl font-bold text-cyan-400">"Bem-vindo ao NeuroPulse"</h2>
                    <p class="text-xs text-slate-400 mt-2">"Faça login para aceder aos protocolos PINA e testes cognitivos"</p>
                </div>

                <form on:submit=handle_login class="space-y-4">
                    <div>
                        <label class="text-xs text-slate-400 block mb-1">"E-mail Profissional / Clínico"</label>
                        <input 
                            type="email"
                            placeholder="seu.email@clinica.com"
                            prop:value=email
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                            class="w-full bg-slate-950 border border-slate-800 rounded-xl px-4 py-3 text-sm text-white outline-none focus:border-cyan-400 transition-colors"
                            required
                        />
                    </div>

                    <div>
                        <label class="text-xs text-slate-400 block mb-1">"Palavra-passe"</label>
                        <input 
                            type="password"
                            placeholder="••••••••"
                            prop:value=password
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                            class="w-full bg-slate-950 border border-slate-800 rounded-xl px-4 py-3 text-sm text-white outline-none focus:border-cyan-400 transition-colors"
                            required
                        />
                    </div>

                    <button 
                        type="submit"
                        class="w-full py-3.5 bg-cyan-500 hover:bg-cyan-400 text-slate-950 font-bold rounded-xl text-xs transition-all shadow-lg shadow-cyan-900/30 cursor-pointer mt-2">
                        "Entrar no Sistema"
                    </button>
                </form>

                <div class="mt-6 text-[10px] text-slate-500 text-center">
                    "NeuroPulse SaaS - Plataforma de Avaliação Autonômica e Inibitória."
                </div>
            </div>
        </div>
    }
}
