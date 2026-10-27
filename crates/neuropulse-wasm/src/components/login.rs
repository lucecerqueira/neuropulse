use leptos::*;

#[component]
pub fn LoginView(
    #[prop(into)] on_login: Callback<()>,
) -> impl IntoView {
    let (auth_mode, set_auth_mode) = create_signal("login".to_string());
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (name, set_name) = create_signal(String::new());
    let (feedback_msg, set_feedback_msg) = create_signal::<Option<(String, bool)>>(None);

    let handleSubmit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let mode = auth_mode.get();
        let current_email = email.get();

        if current_email.is_empty() {
            set_feedback_msg.set(Some(("Por favor, insira o e-mail.".to_string(), false)));
            return;
        }

        if mode == "login" || mode == "register" {
            set_feedback_msg.set(Some(("Autenticação bem-sucedida! Entrando...".to_string(), true)));
            set_timeout(
                move || {
                    on_login.call(());
                },
                std::time::Duration::from_millis(600),
            );
        } else if mode == "forgot" {
            set_feedback_msg.set(Some((
                "Instruções de recuperação enviadas para o seu e-mail!".to_string(),
                true,
            )));
        }
    };

    view! {
        <div class="min-h-screen bg-[#060911] flex items-center justify-center p-4 font-mono">
            <div class="w-full max-w-md bg-[#0f172a]/90 border border-slate-800 p-8 rounded-2xl shadow-2xl space-y-6">
                <div class="flex justify-center">
                    <div class="w-16 h-16 bg-cyan-500/10 border border-cyan-500/30 rounded-2xl flex items-center justify-center p-3 shadow-lg shadow-cyan-500/10">
                        <img src="/favicon.svg" alt="NeuroPulse Logo" class="w-full h-full object-contain" />
                    </div>
                </div>

                <div class="text-center space-y-1">
                    <h2 class="text-xl font-bold text-slate-100">
                        {move || match auth_mode.get().as_str() {
                            "register" => "Registo Profissional",
                            "forgot" => "Recuperar Palavra-passe",
                            _ => "Bem-vindo ao NeuroPulse",
                        }}
                    </h2>
                    <p class="text-xs text-slate-400">
                        {move || match auth_mode.get().as_str() {
                            "register" => "Crie a sua conta clínica para aceder à plataforma",
                            "forgot" => "Insira o seu e-mail para receber as instruções",
                            _ => "Faça login para aceder aos protocolos PINA e testes",
                        }}
                    </p>
                </div>

                {move || if let Some((msg, is_success)) = feedback_msg.get() {
                    let color = if is_success { "bg-emerald-500/10 text-emerald-400 border-emerald-500/30" } else { "bg-rose-500/10 text-rose-400 border-rose-500/30" };
                    view! {
                        <div class=format!("p-3 border text-xs rounded-lg text-center font-bold {}", color)>
                            {msg}
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }}

                <form on:submit=handleSubmit class="space-y-4">
                    {move || if auth_mode.get() == "register" {
                        view! {
                            <div class="space-y-1">
                                <label class="text-xs text-slate-400 font-bold">"Nome Completo / Título Clínico"</label>
                                <input
                                    type="text"
                                    placeholder="Dra. Maria Silva"
                                    prop:value=name
                                    on:input=move |ev| set_name.set(event_target_value(&ev))
                                    required=true
                                    class="w-full bg-[#090d16] border border-slate-800 focus:border-cyan-500 text-slate-200 text-xs px-4 py-3 rounded-lg focus:outline-none transition"
                                />
                            </div>
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }}

                    <div class="space-y-1">
                        <label class="text-xs text-slate-400 font-bold">"E-mail Profissional / Clínico"</label>
                        <input
                            type="email"
                            placeholder="seu.email@clinica.com"
                            prop:value=email
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                            required=true
                            class="w-full bg-[#090d16] border border-slate-800 focus:border-cyan-500 text-slate-200 text-xs px-4 py-3 rounded-lg focus:outline-none transition"
                        />
                    </div>

                    {move || if auth_mode.get() != "forgot" {
                        view! {
                            <div class="space-y-1">
                                <div class="flex justify-between items-center">
                                    <label class="text-xs text-slate-400 font-bold">"Palavra-passe"</label>
                                    {if auth_mode.get() == "login" {
                                        view! {
                                            <button
                                                type="button"
                                                on:click=move |_| {
                                                    set_feedback_msg.set(None);
                                                    set_auth_mode.set("forgot".to_string());
                                                }
                                                class="text-[11px] text-cyan-400 hover:text-cyan-300 transition cursor-pointer"
                                            >
                                                "Esqueceu a senha?"
                                            </button>
                                        }.into_view()
                                    } else {
                                        view! { <div></div> }.into_view()
                                    }}
                                </div>
                                <input
                                    type="password"
                                    placeholder="••••••••"
                                    prop:value=password
                                    on:input=move |ev| set_password.set(event_target_value(&ev))
                                    required=true
                                    class="w-full bg-[#090d16] border border-slate-800 focus:border-cyan-500 text-slate-200 text-xs px-4 py-3 rounded-lg focus:outline-none transition"
                                />
                            </div>
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }}

                    <button
                        type="submit"
                        class="w-full py-3 bg-cyan-500 hover:bg-cyan-400 text-slate-950 font-bold text-xs rounded-lg transition shadow-lg shadow-cyan-500/20 cursor-pointer"
                    >
                        {move || match auth_mode.get().as_str() {
                            "register" => "Concluir Registo",
                            "forgot" => "Enviar Instruções de Recuperação",
                            _ => "Entrar no Sistema",
                        }}
                    </button>
                </form>

                <div class="text-center pt-2 border-t border-slate-800/80 text-xs">
                    {move || match auth_mode.get().as_str() {
                        "register" => view! {
                            <span class="text-slate-400">"Já tem conta? "</span>
                            <button
                                type="button"
                                on:click=move |_| {
                                    set_feedback_msg.set(None);
                                    set_auth_mode.set("login".to_string());
                                }
                                class="font-bold text-cyan-400 hover:text-cyan-300 transition cursor-pointer"
                            >
                                "Efetuar Login"
                            </button>
                        }.into_view(),
                        "forgot" => view! {
                            <button
                                type="button"
                                on:click=move |_| {
                                    set_feedback_msg.set(None);
                                    set_auth_mode.set("login".to_string());
                                }
                                class="font-bold text-cyan-400 hover:text-cyan-300 transition cursor-pointer"
                            >
                                "← Voltar ao Login"
                            </button>
                        }.into_view(),
                        _ => view! {
                            <span class="text-slate-400">"Ainda não tem conta? "</span>
                            <button
                                type="button"
                                on:click=move |_| {
                                    set_feedback_msg.set(None);
                                    set_auth_mode.set("register".to_string());
                                }
                                class="font-bold text-cyan-400 hover:text-cyan-300 transition cursor-pointer"
                            >
                                "Criar cadastro profissional"
                            </button>
                        }.into_view(),
                    }}
                </div>

                <p class="text-[10px] text-slate-500 text-center leading-relaxed">
                    "NeuroPulse SaaS - Plataforma de Avaliação Autonômica e Inibitória."
                </p>
            </div>
        </div>
    }
}
