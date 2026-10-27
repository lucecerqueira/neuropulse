use leptos::*;

#[component]
pub fn PlansView() -> impl IntoView {
    // Estado do plano selecionado atualmente (ex: "STARTER", "PRO", "FREE", etc.)
    let (selected_plan, set_selected_plan) = create_signal("STARTER".to_string());
    let (feedback_message, set_feedback_message) = create_signal::<Option<String>>(None);

    let handle_select = move |plan_name: &'static str| {
        set_selected_plan.set(plan_name.to_string());
        set_feedback_message.set(Some(format!("Plano {} ativado com sucesso!", plan_name)));
        
        set_timeout(
            move || {
                set_feedback_message.set(None);
            },
            std::time::Duration::from_millis(2000),
        );
    };

    view! {
        <div class="space-y-8 font-mono pb-12">
            <div class="text-center space-y-2">
                <h2 class="text-2xl font-bold text-cyan-400">"Planos de Assinatura NeuroPulse SaaS"</h2>
                <p class="text-xs text-slate-400 max-w-2xl mx-auto">
                    "Selecione o nível de acesso para laboratórios, clínicas e atletas de alta performance, com liberação modular baseada em rigor científico."
                </p>
            </div>

            {move || if let Some(ref msg) = feedback_message.get() {
                view! {
                    <div class="max-w-md mx-auto p-3 bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 text-xs rounded-lg text-center font-bold animate-pulse">
                        {msg.clone()}
                    </div>
                }.into_view()
            } else {
                view! { <div></div> }.into_view()
            }}

            <div class="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-5 gap-4">
                // 1. Trial
                <div class=move || format!(
                    "bg-[#0f172a]/60 border p-6 rounded-2xl flex flex-col justify-between transition-all {}",
                    if selected_plan.get() == "TRIAL" { "border-cyan-400 shadow-lg shadow-cyan-500/20 bg-[#0f172a]" } else { "border-slate-800/80" }
                )>
                    <div class="space-y-4">
                        <div>
                            <span class="text-[10px] text-cyan-400 font-bold uppercase tracking-wider">"Trial / 7 Dias"</span>
                            <h3 class="text-xl font-bold text-white mt-1">"Grátis"</h3>
                            <p class="text-[11px] text-slate-400 mt-2">"Avaliação temporária de todos os recursos por 7 dias."</p>
                        </div>
                        <ul class="space-y-2 text-[11px] text-slate-300">
                            <li class="flex items-center gap-2"><span>"✓"</span> "Acesso total temporário"</li>
                            <li class="flex items-center gap-2"><span>"✓"</span> "Testes SRT, CRT e Go/No-Go"</li>
                            <li class="flex items-center gap-2 text-slate-500"><span>"×"</span> "Sem exportação avançada"</li>
                        </ul>
                    </div>
                    <button
                        on:click=move |_| handle_select("TRIAL")
                        class=move || format!(
                            "w-full mt-6 py-2.5 rounded-xl font-bold text-xs transition cursor-pointer {}",
                            if selected_plan.get() == "TRIAL" { "bg-cyan-400 text-slate-950" } else { "bg-slate-800 hover:bg-slate-700 text-slate-200" }
                        )
                    >
                        {move || if selected_plan.get() == "TRIAL" { "Plano Atual" } else { "Ativar Trial" }}
                    </button>
                </div>

                // 2. Free
                <div class=move || format!(
                    "bg-[#0f172a]/60 border p-6 rounded-2xl flex flex-col justify-between transition-all {}",
                    if selected_plan.get() == "FREE" { "border-cyan-400 shadow-lg shadow-cyan-500/20 bg-[#0f172a]" } else { "border-slate-800/80" }
                )>
                    <div class="space-y-4">
                        <div>
                            <span class="text-[10px] text-slate-400 font-bold uppercase tracking-wider">"Free / Básico"</span>
                            <h3 class="text-xl font-bold text-white mt-1">"R$ 0"</h3>
                            <p class="text-[11px] text-slate-400 mt-2">"Uso livre permanente com quantidade restrita de execuções."</p>
                        </div>
                        <ul class="space-y-2 text-[11px] text-slate-300">
                            <li class="flex items-center gap-2"><span>"✓"</span> "Apenas teste SRT (3 vezes/dia)"</li>
                            <li class="flex items-center gap-2"><span>"✓"</span> "1 Trilha Motivacional"</li>
                            <li class="flex items-center gap-2 text-slate-500"><span>"×"</span> "Sem acesso ao rPPG"</li>
                        </ul>
                    </div>
                    <button
                        on:click=move |_| handle_select("FREE")
                        class=move || format!(
                            "w-full mt-6 py-2.5 rounded-xl font-bold text-xs transition cursor-pointer {}",
                            if selected_plan.get() == "FREE" { "bg-cyan-400 text-slate-950" } else { "bg-slate-800 hover:bg-slate-700 text-slate-200" }
                        )
                    >
                        {move || if selected_plan.get() == "FREE" { "Plano Atual" } else { "Usar Free" }}
                    </button>
                </div>

                // 3. Starter
                <div class=move || format!(
                    "bg-[#0f172a]/60 border p-6 rounded-2xl flex flex-col justify-between transition-all {}",
                    if selected_plan.get() == "STARTER" { "border-cyan-400 shadow-lg shadow-cyan-500/20 bg-[#0f172a]" } else { "border-slate-800/80" }
                )>
                    <div class="space-y-4">
                        <div>
                            <span class="text-[10px] text-cyan-400 font-bold uppercase tracking-wider">"Starter / Individual"</span>
                            <h3 class="text-xl font-bold text-white mt-1">"R$ 99"<span class="text-xs text-slate-400">"/mês"</span></h3>
                            <p class="text-[11px] text-slate-400 mt-2">"Ideal para profissionais autônomos e biohackers."</p>
                        </div>
                        <ul class="space-y-2 text-[11px] text-slate-300">
                            <li class="flex items-center gap-2"><span>"✓"</span> "Acesso SRT e CRT ilimitados"</li>
                            <li class="flex items-center gap-2"><span>"✓"</span> "Até 3 Trilhas Motivacionais"</li>
                            <li class="flex items-center gap-2"><span>"✓"</span> "Relatório semanal em PDF"</li>
                        </ul>
                    </div>
                    <button
                        on:click=move |_| handle_select("STARTER")
                        class=move || format!(
                            "w-full mt-6 py-2.5 rounded-xl font-bold text-xs transition cursor-pointer {}",
                            if selected_plan.get() == "STARTER" { "bg-cyan-400 text-slate-950" } else { "bg-slate-800 hover:bg-slate-700 text-slate-200" }
                        )
                    >
                        {move || if selected_plan.get() == "STARTER" { "Plano Atual" } else { "Selecionar Starter" }}
                    </button>
                </div>

                // 4. Pro (Mais Popular)
                <div class=move || format!(
                    "bg-[#0f172a] border p-6 rounded-2xl flex flex-col justify-between transition-all relative {}",
                    if selected_plan.get() == "PRO" { "border-cyan-400 shadow-xl shadow-cyan-500/30" } else { "border-cyan-500/50" }
                )>
                    <span class="absolute -top-3 left-1/2 -translate-x-1/2 bg-cyan-400 text-slate-950 text-[9px] font-black px-3 py-0.5 rounded-full uppercase tracking-wider">
                        "Mais Popular"
                    </span>
                    <div class="space-y-4 pt-1">
                        <div>
                            <span class="text-[10px] text-amber-400 font-bold uppercase tracking-wider">"Pro / Clínico"</span>
                            <h3 class="text-xl font-bold text-white mt-1">"R$ 299"<span class="text-xs text-slate-400">"/mês"</span></h3>
                            <p class="text-[11px] text-slate-400 mt-2">"Para clínicas e pesquisadores com protocolo rPPG completo."</p>
                        </div>
                        <ul class="space-y-2 text-[11px] text-slate-300">
                            <li class="flex items-center gap-2"><span>"✓"</span> "Todos os testes (SRT, CRT, Go/No-Go)"</li>
                            <li class="flex items-center gap-2"><span>"✓"</span> "13 Trilhas Motivacionais"</li>
                            <li class="flex items-center gap-2"><span>"✓"</span> "Protocolo rPPG + PINA em Tempo Real"</li>
                        </ul>
                    </div>
                    <button
                        on:click=move |_| handle_select("PRO")
                        class=move || format!(
                            "w-full mt-6 py-2.5 rounded-xl font-bold text-xs transition cursor-pointer {}",
                            if selected_plan.get() == "PRO" { "bg-cyan-400 text-slate-950" } else { "bg-cyan-500 hover:bg-cyan-400 text-slate-950" }
                        )
                    >
                        {move || if selected_plan.get() == "PRO" { "Plano Atual" } else { "Assinar Pro" }}
                    </button>
                </div>

                // 5. Enterprise
                <div class=move || format!(
                    "bg-[#0f172a]/60 border p-6 rounded-2xl flex flex-col justify-between transition-all {}",
                    if selected_plan.get() == "ENTERPRISE" { "border-cyan-400 shadow-lg shadow-cyan-500/20 bg-[#0f172a]" } else { "border-slate-800/80" }
                )>
                    <div class="space-y-4">
                        <div>
                            <span class="text-[10px] text-purple-400 font-bold uppercase tracking-wider">"Enterprise / Inst."</span>
                            <h3 class="text-xl font-bold text-white mt-1">"Sob Consulta"</h3>
                            <p class="text-[11px] text-slate-400 mt-2">"Solução customizada para grandes instituições e universidades."</p>
                        </div>
                        <ul class="space-y-2 text-[11px] text-slate-300">
                            <li class="flex items-center gap-2"><span>"✓"</span> "Usuários ilimitados e multi-tenant"</li>
                            <li class="flex items-center gap-2"><span>"✓"</span> "API REST e integrações diretas"</li>
                            <li class="flex items-center gap-2"><span>"✓"</span> "Suporte dedicado e SLA"</li>
                        </ul>
                    </div>
                    <button
                        on:click=move |_| handle_select("ENTERPRISE")
                        class=move || format!(
                            "w-full mt-6 py-2.5 rounded-xl font-bold text-xs transition cursor-pointer {}",
                            if selected_plan.get() == "ENTERPRISE" { "bg-cyan-400 text-slate-950" } else { "bg-slate-800 hover:bg-slate-700 text-slate-200" }
                        )
                    >
                        {move || if selected_plan.get() == "ENTERPRIse" { "Plano Atual" } else { "Falar com Consultor" }}
                    </button>
                </div>
            </div>
        </div>
    }
}
